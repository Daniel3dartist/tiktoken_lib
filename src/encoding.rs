use rustc_hash::FxHashMap as HashMap;

use crate::load::{data_gym_to_mergeable_bpe_ranks, load_tiktoken_bpe};
use crate::{CoreBPE, Rank};

pub const ENDOFTEXT: &str = "<|endoftext|>";
pub const FIM_PREFIX: &str = "<|fim_prefix|>";
pub const FIM_MIDDLE: &str = "<|fim_middle|>";
pub const FIM_SUFFIX: &str = "<|fim_suffix|>";
pub const ENDOFPROMPT: &str = "<|endofprompt|>";

/// GPT-2 / r50k regex pattern (equivalent to the original GPT-2 release pattern).
pub const R50K_PAT_STR: &str =
    r"'(?:[sdmt]|ll|ve|re)| ?\p{L}++| ?\p{N}++| ?[^\s\p{L}\p{N}]++|\s++$|\s+(?!\S)|\s";

pub const CL100K_PAT_STR: &str = r"'(?i:[sdmt]|ll|ve|re)|[^\r\n\p{L}\p{N}]?+\p{L}++|\p{N}{1,3}+| ?[^\s\p{L}\p{N}]++[\r\n]*+|\s++$|\s*[\r\n]|\s+(?!\S)|\s";

const GPT2_VOCAB_BPE: &str =
    "https://openaipublic.blob.core.windows.net/gpt-2/encodings/main/vocab.bpe";
const GPT2_ENCODER_JSON: &str =
    "https://openaipublic.blob.core.windows.net/gpt-2/encodings/main/encoder.json";
const GPT2_VOCAB_BPE_HASH: &str =
    "1ce1664773c50f3e0cc8842619a93edc4624525b728b188a9e0be33b7726adc5";
const GPT2_ENCODER_JSON_HASH: &str =
    "196139668be63f3b5d6574427317ae82f612a97c5d1cdaf36ed2256dbf636783";

const R50K_BASE_URL: &str =
    "https://openaipublic.blob.core.windows.net/encodings/r50k_base.tiktoken";
const R50K_BASE_HASH: &str =
    "306cd27f03c1a714eca7108e03d66b7dc042abe8c258b44c199a7ed9838dd930";

const P50K_BASE_URL: &str =
    "https://openaipublic.blob.core.windows.net/encodings/p50k_base.tiktoken";
const P50K_BASE_HASH: &str =
    "94b5ca7dff4d00767bc256fdd1b27e5b17361d7b8a5f968547f9f23eb70d2069";

const CL100K_BASE_URL: &str =
    "https://openaipublic.blob.core.windows.net/encodings/cl100k_base.tiktoken";
const CL100K_BASE_HASH: &str =
    "223921b76ee99bde995b7ff738513eef100fb51d18c93597a113bcffe865b2a7";

const O200K_BASE_URL: &str =
    "https://openaipublic.blob.core.windows.net/encodings/o200k_base.tiktoken";
const O200K_BASE_HASH: &str =
    "446a9538cb6c348e3516120d7c08b09f57c36495e2acfffe59a5bf8b0cfb1a2d";

fn o200k_pat_str() -> String {
    [
        r"[^\r\n\p{L}\p{N}]?[\p{Lu}\p{Lt}\p{Lm}\p{Lo}\p{M}]*[\p{Ll}\p{Lm}\p{Lo}\p{M}]+(?i:'s|'t|'re|'ve|'m|'ll|'d)?",
        r"[^\r\n\p{L}\p{N}]?[\p{Lu}\p{Lt}\p{Lm}\p{Lo}\p{M}]+[\p{Ll}\p{Lm}\p{Lo}\p{M}]*(?i:'s|'t|'re|'ve|'m|'ll|'d)?",
        r"\p{N}{1,3}",
        r" ?[^\s\p{L}\p{N}]+[\r\n/]*",
        r"\s*[\r\n]+",
        r"\s+(?!\S)",
        r"\s+",
    ]
    .join("|")
}

struct EncodingSpec {
    name: &'static str,
    pat_str: &'static str,
    explicit_n_vocab: Option<usize>,
    special_tokens: fn() -> HashMap<String, Rank>,
    loader: fn() -> Result<(HashMap<Vec<u8>, Rank>, Option<&'static str>), Box<dyn std::error::Error + Send + Sync>>,
}

fn load_gpt2() -> Result<(HashMap<Vec<u8>, Rank>, Option<&'static str>), Box<dyn std::error::Error + Send + Sync>> {
    let ranks = data_gym_to_mergeable_bpe_ranks(
        GPT2_VOCAB_BPE,
        GPT2_ENCODER_JSON,
        Some(GPT2_VOCAB_BPE_HASH),
        Some(GPT2_ENCODER_JSON_HASH),
    )?;
    Ok((ranks, Some(R50K_PAT_STR)))
}

fn load_r50k_base() -> Result<(HashMap<Vec<u8>, Rank>, Option<&'static str>), Box<dyn std::error::Error + Send + Sync>> {
    let ranks = load_tiktoken_bpe(R50K_BASE_URL, Some(R50K_BASE_HASH))?;
    Ok((ranks, Some(R50K_PAT_STR)))
}

fn load_p50k_base() -> Result<(HashMap<Vec<u8>, Rank>, Option<&'static str>), Box<dyn std::error::Error + Send + Sync>> {
    let ranks = load_tiktoken_bpe(P50K_BASE_URL, Some(P50K_BASE_HASH))?;
    Ok((ranks, Some(R50K_PAT_STR)))
}

fn load_cl100k_base() -> Result<(HashMap<Vec<u8>, Rank>, Option<&'static str>), Box<dyn std::error::Error + Send + Sync>> {
    let ranks = load_tiktoken_bpe(CL100K_BASE_URL, Some(CL100K_BASE_HASH))?;
    Ok((ranks, Some(CL100K_PAT_STR)))
}

fn load_o200k_base() -> Result<(HashMap<Vec<u8>, Rank>, Option<&'static str>), Box<dyn std::error::Error + Send + Sync>> {
    let ranks = load_tiktoken_bpe(O200K_BASE_URL, Some(O200K_BASE_HASH))?;
    Ok((ranks, None))
}

fn gpt2_special_tokens() -> HashMap<String, Rank> {
    HashMap::from([(ENDOFTEXT.to_string(), 50256)])
}

fn r50k_special_tokens() -> HashMap<String, Rank> {
    gpt2_special_tokens()
}

fn p50k_special_tokens() -> HashMap<String, Rank> {
    gpt2_special_tokens()
}

fn p50k_edit_special_tokens() -> HashMap<String, Rank> {
    HashMap::from([
        (ENDOFTEXT.to_string(), 50256),
        (FIM_PREFIX.to_string(), 50281),
        (FIM_MIDDLE.to_string(), 50282),
        (FIM_SUFFIX.to_string(), 50283),
    ])
}

fn cl100k_special_tokens() -> HashMap<String, Rank> {
    HashMap::from([
        (ENDOFTEXT.to_string(), 100257),
        (FIM_PREFIX.to_string(), 100258),
        (FIM_MIDDLE.to_string(), 100259),
        (FIM_SUFFIX.to_string(), 100260),
        (ENDOFPROMPT.to_string(), 100276),
    ])
}

fn o200k_special_tokens() -> HashMap<String, Rank> {
    HashMap::from([
        (ENDOFTEXT.to_string(), 199999),
        (ENDOFPROMPT.to_string(), 200018),
    ])
}

fn o200k_harmony_special_tokens() -> HashMap<String, Rank> {
    let mut tokens = o200k_special_tokens();
    tokens.insert("<|startoftext|>".to_string(), 199998);
    tokens.insert(ENDOFTEXT.to_string(), 199999);
    tokens.insert("<|reserved_200000|>".to_string(), 200000);
    tokens.insert("<|reserved_200001|>".to_string(), 200001);
    tokens.insert("<|return|>".to_string(), 200002);
    tokens.insert("<|constrain|>".to_string(), 200003);
    tokens.insert("<|reserved_200004|>".to_string(), 200004);
    tokens.insert("<|channel|>".to_string(), 200005);
    tokens.insert("<|start|>".to_string(), 200006);
    tokens.insert("<|end|>".to_string(), 200007);
    tokens.insert("<|message|>".to_string(), 200008);
    tokens.insert("<|reserved_200009|>".to_string(), 200009);
    tokens.insert("<|reserved_200010|>".to_string(), 200010);
    tokens.insert("<|reserved_200011|>".to_string(), 200011);
    tokens.insert("<|call|>".to_string(), 200012);
    for i in 200013..=201087 {
        tokens.insert(format!("<|reserved_{i}|>"), i);
    }
    tokens
}

static ENCODINGS: &[EncodingSpec] = &[
    EncodingSpec {
        name: "gpt2",
        pat_str: R50K_PAT_STR,
        explicit_n_vocab: Some(50257),
        special_tokens: gpt2_special_tokens,
        loader: load_gpt2,
    },
    EncodingSpec {
        name: "r50k_base",
        pat_str: R50K_PAT_STR,
        explicit_n_vocab: Some(50257),
        special_tokens: r50k_special_tokens,
        loader: load_r50k_base,
    },
    EncodingSpec {
        name: "p50k_base",
        pat_str: R50K_PAT_STR,
        explicit_n_vocab: Some(50281),
        special_tokens: p50k_special_tokens,
        loader: load_p50k_base,
    },
    EncodingSpec {
        name: "p50k_edit",
        pat_str: R50K_PAT_STR,
        explicit_n_vocab: None,
        special_tokens: p50k_edit_special_tokens,
        loader: load_p50k_base,
    },
    EncodingSpec {
        name: "cl100k_base",
        pat_str: CL100K_PAT_STR,
        explicit_n_vocab: None,
        special_tokens: cl100k_special_tokens,
        loader: load_cl100k_base,
    },
    EncodingSpec {
        name: "o200k_base",
        pat_str: "",
        explicit_n_vocab: None,
        special_tokens: o200k_special_tokens,
        loader: load_o200k_base,
    },
    EncodingSpec {
        name: "o200k_harmony",
        pat_str: "",
        explicit_n_vocab: None,
        special_tokens: o200k_harmony_special_tokens,
        loader: load_o200k_base,
    },
];

pub fn list_encoding_names() -> Vec<&'static str> {
    ENCODINGS.iter().map(|spec| spec.name).collect()
}

pub fn get_encoding(name: &str) -> Result<CoreBPE, Box<dyn std::error::Error + Send + Sync>> {
    let spec = ENCODINGS
        .iter()
        .find(|spec| spec.name == name)
        .ok_or_else(|| format!("unknown encoding {name:?}"))?;

    let (mergeable_ranks, pat_override) = (spec.loader)()?;
    let special_tokens = (spec.special_tokens)();

    if let Some(n_vocab) = spec.explicit_n_vocab {
        if mergeable_ranks.len() + special_tokens.len() != n_vocab {
            return Err(format!(
                "encoding {name} expected {n_vocab} tokens, got {}",
                mergeable_ranks.len() + special_tokens.len()
            )
            .into());
        }
        let max_token = mergeable_ranks
            .values()
            .chain(special_tokens.values())
            .copied()
            .max()
            .unwrap_or(0);
        if max_token != n_vocab as u32 - 1 {
            return Err(format!(
                "encoding {name} max token value {max_token} != {}",
                n_vocab - 1
            )
            .into());
        }
    }

    let o200k_pat_owned = if pat_override.is_none() && spec.name.starts_with("o200k") {
        Some(o200k_pat_str())
    } else {
        None
    };
    let pat_str = pat_override.unwrap_or_else(|| {
        o200k_pat_owned
            .as_deref()
            .unwrap_or(spec.pat_str)
    });

    CoreBPE::new(mergeable_ranks, special_tokens, pat_str)
}
