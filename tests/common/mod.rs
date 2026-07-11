use std::collections::HashSet;

use tiktoken::{get_encoding, CoreBPE};

/// Encodings exercised in the upstream parametrized suite (`test_helpers.ENCODINGS`).
pub const PARAM_ENCODINGS: &[&str] = &["r50k_base", "cl100k_base"];

pub fn load(name: &str) -> CoreBPE {
    get_encoding(name).unwrap_or_else(|e| panic!("load encoding {name}: {e}"))
}

pub fn all_special(enc: &CoreBPE) -> HashSet<&str> {
    enc.special_tokens()
}

pub fn encode_all_special(enc: &CoreBPE, text: &str) -> Vec<u32> {
    let allowed = all_special(enc);
    enc.encode(text, &allowed).expect("encode").0
}

/// Python `Encoding.decode` default (`errors="replace"`).
pub fn decode_python_default(enc: &CoreBPE, tokens: &[u32]) -> String {
    enc.decode_lossy(tokens).expect("decode_lossy")
}

pub fn roundtrip_encode_decode(enc: &CoreBPE, text: &str) -> String {
    let allowed = HashSet::new();
    let disallowed = all_special(enc);
    let tokens = tiktoken::validation::encode_checked(enc, text, &allowed, &disallowed)
        .expect("encode_checked");
    decode_python_default(enc, &tokens)
}
