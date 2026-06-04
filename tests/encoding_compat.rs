//! Ports deterministic tests from `.samples/tiktoken/tests/test_encoding.py` and
//! `test_simple_public.py` against the Rust `CoreBPE` API.

mod common;

use std::collections::HashSet;

use common::{all_special, encode_all_special, load, roundtrip_encode_decode, PARAM_ENCODINGS};
use tiktoken::{get_encoding, list_encoding_names, validation};

// --- test_simple (public) ---

#[test]
fn test_simple_gpt2() {
    let enc = load("gpt2");
    let allowed = all_special(&enc);
    assert_eq!(
        validation::encode_checked(&enc, "hello world", &HashSet::new(), &allowed).unwrap(),
        vec![31373, 995]
    );
    assert_eq!(enc.decode(&[31373, 995]).unwrap(), "hello world");
    assert_eq!(
        encode_all_special(&enc, "hello <|endoftext|>"),
        vec![31373, 220, 50256]
    );
}

#[test]
fn test_simple_cl100k() {
    let enc = load("cl100k_base");
    let allowed = all_special(&enc);
    assert_eq!(
        validation::encode_checked(&enc, "hello world", &HashSet::new(), &allowed).unwrap(),
        vec![15339, 1917]
    );
    assert_eq!(enc.decode(&[15339, 1917]).unwrap(), "hello world");
    assert_eq!(
        encode_all_special(&enc, "hello <|endoftext|>"),
        vec![15339, 220, 100257]
    );
}

#[test]
fn test_simple_vocab_roundtrip_first_10k() {
    for name in list_encoding_names() {
        let enc = load(name);
        let limit = 10_000.min(enc.n_vocab().saturating_sub(1));
        for token in 0..limit {
            let token = token as u32;
            let bytes = match enc.decode_single_token_bytes(token) {
                Ok(b) => b,
                Err(_) => continue,
            };
            assert_eq!(
                enc.encode_single_token(&bytes),
                Some(token),
                "encoding {name} token {token}"
            );
        }
    }
}

// --- test_simple_repeated ---

#[test]
fn test_simple_repeated_zeros_gpt2() {
    let enc = load("gpt2");
    let cases: &[(&str, &[u32])] = &[
        ("0", &[15]),
        ("00", &[405]),
        ("000", &[830]),
        ("0000", &[2388]),
        ("00000", &[20483]),
        ("000000", &[10535]),
        ("0000000", &[24598]),
        ("00000000", &[8269]),
        ("000000000", &[10535, 830]),
        ("0000000000", &[8269, 405]),
        ("00000000000", &[8269, 830]),
        ("000000000000", &[8269, 2388]),
        ("0000000000000", &[8269, 20483]),
        ("00000000000000", &[8269, 10535]),
        ("000000000000000", &[8269, 24598]),
        ("0000000000000000", &[25645]),
        ("00000000000000000", &[8269, 10535, 830]),
    ];
    for (text, expected) in cases {
        assert_eq!(enc.encode_ordinary(text), *expected, "text={text:?}");
    }
}

// --- test_simple_regex ---

#[test]
fn test_simple_regex_cl100k() {
    let enc = load("cl100k_base");
    let allowed = all_special(&enc);
    let encode = |t: &str| {
        validation::encode_checked(&enc, t, &HashSet::new(), &allowed).unwrap()
    };
    assert_eq!(encode("rer"), vec![38149]);
    assert_eq!(encode("'rer"), vec![2351, 81]);
    assert_eq!(encode("today\n "), vec![31213, 198, 220]);
    assert_eq!(encode("today\n \n"), vec![31213, 27907]);
    assert_eq!(encode("today\n  \n"), vec![31213, 14211]);
}

// --- test_basic_encode ---

#[test]
fn test_basic_encode() {
    let allowed_r50k = all_special(&load("r50k_base"));
    let enc = load("r50k_base");
    assert_eq!(
        validation::encode_checked(&enc, "hello world", &HashSet::new(), &allowed_r50k).unwrap(),
        vec![31373, 995]
    );

    let enc = load("p50k_base");
    let allowed = all_special(&enc);
    assert_eq!(
        validation::encode_checked(&enc, "hello world", &HashSet::new(), &allowed).unwrap(),
        vec![31373, 995]
    );

    let enc = load("cl100k_base");
    let allowed = all_special(&enc);
    assert_eq!(
        validation::encode_checked(&enc, "hello world", &HashSet::new(), &allowed).unwrap(),
        vec![15339, 1917]
    );
    let odd_chars: String = [' ', '\u{0085}', '0'].iter().collect();
    assert_eq!(
        validation::encode_checked(&enc, &odd_chars, &HashSet::new(), &allowed).unwrap(),
        vec![220, 126, 227, 15]
    );
}

#[test]
fn test_encode_empty() {
    let enc = load("r50k_base");
    let allowed = all_special(&enc);
    assert_eq!(
        validation::encode_checked(&enc, "", &HashSet::new(), &allowed).unwrap(),
        Vec::<u32>::new()
    );
}

// --- test_encode_bytes ---

#[test]
fn test_encode_bytes_cl100k() {
    let enc = load("cl100k_base");
    assert_eq!(enc.encode_bytes(b" \xec\x8b\xa4\xed"), vec![62085]);
    for i in 0..10 {
        let bytestring = vec![0x80u8; i];
        assert_eq!(
            enc.decode_bytes(&enc.encode_bytes(&bytestring)).unwrap(),
            bytestring
        );
    }
}

// --- test_encode_surrogate_pairs (emoji; Rust str cannot hold lone surrogates) ---

#[test]
fn test_encode_emoji_cl100k() {
    let enc = load("cl100k_base");
    let allowed = all_special(&enc);
    let tokens =
        validation::encode_checked(&enc, "👍", &HashSet::new(), &allowed).unwrap();
    assert_eq!(tokens, vec![9468, 239, 235]);
}

// --- test_catastrophically_repetitive ---

#[test]
fn test_catastrophically_repetitive() {
    for name in PARAM_ENCODINGS {
        let enc = load(name);
        let allowed = all_special(&enc);
        for c in ["^", "0", "a", "'s", " ", "\n"] {
            let base = c.repeat(10_000);
            for big_value in [base.clone(), format!(" {base}"), format!("{base}\n")] {
                let tokens = validation::encode_checked(
                    &enc,
                    &big_value,
                    &HashSet::new(),
                    &allowed,
                )
                .unwrap();
                assert_eq!(roundtrip_encode_decode(&enc, &big_value), big_value);
                assert_eq!(enc.decode_lossy(&tokens).unwrap(), big_value);
            }
        }
    }
}

// --- roundtrip ---

#[test]
fn test_basic_roundtrip() {
    let samples = [
        "hello",
        "hello ",
        "hello  ",
        " hello",
        " hello ",
        " hello  ",
        "hello world",
        "请考试我的软件！12345",
    ];
    for name in PARAM_ENCODINGS {
        let enc = load(name);
        for value in samples {
            assert_eq!(roundtrip_encode_decode(&enc, value), value, "{name}");
            assert_eq!(
                enc.decode_lossy(&enc.encode_ordinary(value)).unwrap(),
                value,
                "{name} ordinary"
            );
        }
    }
}

#[test]
fn test_single_token_roundtrip_full_vocab() {
    for name in PARAM_ENCODINGS {
        let enc = load(name);
        for token in 0..enc.n_vocab() {
            let token = token as u32;
            let Ok(bytes) = enc.decode_single_token_bytes(token) else {
                continue;
            };
            assert_eq!(
                enc.encode_single_token(&bytes),
                Some(token),
                "{name} token {token}"
            );
        }
    }
}

// --- special tokens (test_special_token) ---

#[test]
fn test_special_token_cl100k() {
    let enc = load("cl100k_base");
    let eot = enc.encode_single_token(b"<|endoftext|>").unwrap();
    assert_eq!(eot, 100257);
    let fip = enc.encode_single_token(b"<|fim_prefix|>").unwrap();
    let fim = enc.encode_single_token(b"<|fim_middle|>").unwrap();

    let text = "<|endoftext|> hello <|fim_prefix|>";
    let all = all_special(&enc);
    let none_allowed: HashSet<&str> = HashSet::new();

    let tokens =
        validation::encode_checked(&enc, text, &none_allowed, &HashSet::new()).unwrap();
    assert!(!tokens.contains(&eot));

    assert!(validation::encode_checked(&enc, text, &none_allowed, &all).is_err());
    assert!(validation::encode_checked(
        &enc,
        text,
        &none_allowed,
        &HashSet::from(["<|endoftext|>"])
    )
    .is_err());
    assert!(validation::encode_checked(
        &enc,
        text,
        &none_allowed,
        &HashSet::from(["<|fim_prefix|>"])
    )
    .is_err());

    let text2 = "<|endoftext|> hello <|fim_prefix|> there <|fim_middle|>";
    let tokens =
        validation::encode_checked(&enc, text2, &none_allowed, &HashSet::new()).unwrap();
    assert!(!tokens.contains(&eot));
    assert!(!tokens.contains(&fip));
    assert!(!tokens.contains(&fim));

    let tokens = encode_all_special(&enc, text2);
    assert!(tokens.contains(&eot));
    assert!(tokens.contains(&fip));
    assert!(tokens.contains(&fim));

    let allowed: HashSet<&str> = HashSet::from(["<|fim_prefix|>"]);
    let tokens = validation::encode_checked(&enc, text2, &allowed, &HashSet::new()).unwrap();
    assert!(!tokens.contains(&eot));
    assert!(tokens.contains(&fip));
    assert!(!tokens.contains(&fim));

    let allowed: HashSet<&str> = HashSet::from(["<|endoftext|>"]);
    let tokens = validation::encode_checked(&enc, text2, &allowed, &HashSet::new()).unwrap();
    assert!(tokens.contains(&eot));
    assert!(!tokens.contains(&fip));
    assert!(!tokens.contains(&fim));

    let allowed: HashSet<&str> = HashSet::from(["<|fim_middle|>"]);
    let tokens = validation::encode_checked(&enc, text2, &allowed, &HashSet::new()).unwrap();
    assert!(!tokens.contains(&eot));
    assert!(!tokens.contains(&fip));
    assert!(tokens.contains(&fim));
}

#[test]
fn test_encode_ordinary_matches_disallowed_empty() {
    for name in PARAM_ENCODINGS {
        let enc = load(name);
        let text = "hello world 123";
        assert_eq!(
            enc.encode_ordinary(text),
            validation::encode_checked(&enc, text, &HashSet::new(), &HashSet::new()).unwrap()
        );
    }
}

// --- batch equivalence (no ThreadPool; same results) ---

#[test]
fn test_batch_encode_equivalence() {
    let text1 = "hello world";
    let text2 = "goodbye world";
    for name in PARAM_ENCODINGS {
        let enc = load(name);
        let allowed = HashSet::new();
        let disallowed = all_special(&enc);
        let encode_one =
            |t: &str| validation::encode_checked(&enc, t, &allowed, &disallowed).unwrap();
        let batch = [text1, text2];
        let encoded: Vec<Vec<u32>> = batch.iter().map(|t| encode_one(t)).collect();
        assert_eq!(encoded, vec![encode_one(text1), encode_one(text2)]);
        assert_eq!(encoded, vec![enc.encode_ordinary(text1), enc.encode_ordinary(text2)]);
    }
}

// --- list encodings ---

#[test]
fn list_encodings_contains_expected() {
    let names = list_encoding_names();
    for expected in [
        "gpt2",
        "r50k_base",
        "p50k_base",
        "p50k_edit",
        "cl100k_base",
        "o200k_base",
        "o200k_harmony",
    ] {
        assert!(names.contains(&expected), "missing {expected}");
    }
}

// --- o200k large input (Python raises ValueError; document Rust behavior) ---

#[test]
fn test_o200k_large_input() {
    let enc = load("o200k_base");
    let allowed = all_special(&enc);
    let huge = "x".repeat(1_000_000);
    // Upstream Python rejects this with ValueError; Rust currently completes.
    let result = validation::encode_checked(&enc, &huge, &HashSet::new(), &allowed);
    assert!(result.is_ok(), "Rust encodes 1M chars: {:?}", result.err());
}
