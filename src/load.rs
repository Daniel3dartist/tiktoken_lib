use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use base64::Engine;
use rustc_hash::FxHashMap as HashMap;
use sha1::{Digest as Sha1Digest, Sha1};
use sha2::Sha256;

use crate::Rank;

const DEFAULT_CACHE_DIR: &str = "data-gym-cache";

fn cache_dir() -> PathBuf {
    if let Ok(dir) = env::var("TIKTOKEN_CACHE_DIR") {
        return PathBuf::from(dir);
    }
    if let Ok(dir) = env::var("DATA_GYM_CACHE_DIR") {
        return PathBuf::from(dir);
    }
    env::temp_dir().join(DEFAULT_CACHE_DIR)
}

fn user_specified_cache_dir() -> bool {
    env::var("TIKTOKEN_CACHE_DIR").is_ok() || env::var("DATA_GYM_CACHE_DIR").is_ok()
}

pub fn read_file(path: &str) -> io::Result<Vec<u8>> {
    if !path.contains("://") {
        return fs::read(path);
    }

    if path.starts_with("http://") || path.starts_with("https://") {
        let response = ureq::get(path).call().map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("failed to download {path}: {e}"),
            )
        })?;
        let mut bytes = Vec::new();
        response
            .into_reader()
            .read_to_end(&mut bytes)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        return Ok(bytes);
    }

    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        format!("unsupported URL scheme in {path} (use http/https or a local path)"),
    ))
}

fn check_hash(data: &[u8], expected_hash: &str) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize()) == expected_hash
}

pub fn read_file_cached(path: &str, expected_hash: Option<&str>) -> io::Result<Vec<u8>> {
    let user_cache = user_specified_cache_dir();
    let cache_root = cache_dir();

    if user_cache && cache_root.as_os_str().is_empty() {
        return read_file(path);
    }

    if !user_cache {
        // Default cache: best-effort only (mirrors Python behavior on permission errors).
    }

    let cache_key = format!("{:x}", Sha1::digest(path.as_bytes()));
    let cache_path = cache_root.join(cache_key);

    if cache_path.exists() {
        let data = fs::read(&cache_path)?;
        if expected_hash.is_none_or(|hash| check_hash(&data, hash)) {
            return Ok(data);
        }
        let _ = fs::remove_file(&cache_path);
    }

    let contents = read_file(path)?;
    if let Some(expected_hash) = expected_hash {
        if !check_hash(&contents, expected_hash) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "hash mismatch for data downloaded from {path} (expected {expected_hash})"
                ),
            ));
        }
    }

    if user_cache || !cache_root.as_os_str().is_empty() {
        if let Err(err) = write_cache_file(&cache_root, &cache_path, &contents) {
            if user_cache {
                return Err(err);
            }
        }
    }

    Ok(contents)
}

fn write_cache_file(cache_root: &Path, cache_path: &Path, contents: &[u8]) -> io::Result<()> {
    fs::create_dir_all(cache_root)?;
    let tmp_path = cache_path.with_extension(format!(
        "tmp.{}",
        uuid_simple()
    ));
    fs::write(&tmp_path, contents)?;
    fs::rename(tmp_path, cache_path)
}

fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("{nanos:x}")
}

fn py_isprintable(b: u8) -> bool {
    let c = char::from_u32(u32::from(b)).unwrap_or('\0');
    !c.is_control() && c != ' ' && c != '\u{7f}'
}

pub fn data_gym_to_mergeable_bpe_ranks(
    vocab_bpe_file: &str,
    encoder_json_file: &str,
    vocab_bpe_hash: Option<&str>,
    encoder_json_hash: Option<&str>,
) -> io::Result<HashMap<Vec<u8>, Rank>> {
    let mut rank_to_intbyte: Vec<u8> = (0u8..=255).filter(|&b| py_isprintable(b)).collect();

    let mut data_gym_byte_to_byte: HashMap<char, u8> = rank_to_intbyte
        .iter()
        .map(|&b| (b as char, b))
        .collect();

    let mut n = 0usize;
    for b in 0u8..=255 {
        if !rank_to_intbyte.contains(&b) {
            rank_to_intbyte.push(b);
            data_gym_byte_to_byte.insert(char::from_u32(256 + n as u32).unwrap(), b);
            n += 1;
        }
    }
    assert_eq!(rank_to_intbyte.len(), 256);

    let vocab_bpe_contents = String::from_utf8(read_file_cached(vocab_bpe_file, vocab_bpe_hash)?)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let bpe_merges: Vec<(String, String)> = vocab_bpe_contents
        .split('\n')
        .skip(1)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            (parts[0].to_string(), parts[1].to_string())
        })
        .collect();

    let decode_data_gym = |value: &str| -> Vec<u8> {
        value
            .chars()
            .map(|c| data_gym_byte_to_byte[&c])
            .collect()
    };

    let mut bpe_ranks: HashMap<Vec<u8>, Rank> = rank_to_intbyte
        .into_iter()
        .enumerate()
        .map(|(i, b)| (vec![b], i as Rank))
        .collect();

    let mut next_rank = bpe_ranks.len() as Rank;
    for (first, second) in bpe_merges {
        let mut merged = decode_data_gym(&first);
        merged.extend_from_slice(&decode_data_gym(&second));
        bpe_ranks.insert(merged, next_rank);
        next_rank += 1;
    }

    let encoder_json_bytes = read_file_cached(encoder_json_file, encoder_json_hash)?;
    let encoder_json: HashMap<String, i64> = serde_json::from_slice(&encoder_json_bytes)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let mut encoder_json_loaded: HashMap<Vec<u8>, Rank> = encoder_json
        .into_iter()
        .map(|(k, v)| (decode_data_gym(&k), v as Rank))
        .collect();

    encoder_json_loaded.remove(b"<|endoftext|>".as_slice());
    encoder_json_loaded.remove(b"<|startoftext|>".as_slice());

    if bpe_ranks != encoder_json_loaded {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "vocab.bpe merges do not match encoder.json ranks",
        ));
    }

    Ok(bpe_ranks)
}

pub fn load_tiktoken_bpe(
    tiktoken_bpe_file: &str,
    expected_hash: Option<&str>,
) -> io::Result<HashMap<Vec<u8>, Rank>> {
    let contents = read_file_cached(tiktoken_bpe_file, expected_hash)?;
    let mut ret = HashMap::default();

    for line in contents.split(|&b| b == b'\n') {
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split(|&b| b == b' ');
        let token_b64 = parts
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "empty line in .tiktoken"))?;
        let rank_bytes = parts
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "missing rank in line"))?;
        let token = base64::engine::general_purpose::STANDARD
            .decode(token_b64)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let rank: Rank = std::str::from_utf8(rank_bytes)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        ret.insert(token, rank);
    }

    Ok(ret)
}
