//! Property-based tests mirroring `test_hyp_roundtrip` and `test_hyp_encode_bytes`.

mod common;

use common::load;
use proptest::prelude::*;
use tiktoken::validation;

proptest! {
    #![proptest_config(ProptestConfig {
        cases: std::env::var("TIKTOKEN_MAX_EXAMPLES")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(100),
        ..ProptestConfig::default()
    })]

    #[test]
    fn hyp_roundtrip_r50k(text in "\\PC*") {
        let enc = load("r50k_base");
        let allowed = std::collections::HashSet::new();
        let disallowed = enc.special_tokens();
        if validation::encode_checked(&enc, &text, &allowed, &disallowed).is_err() {
            return Ok(());
        }
        let tokens = validation::encode_checked(&enc, &text, &allowed, &disallowed).unwrap();
        prop_assert_eq!(enc.decode_lossy(&tokens).unwrap(), text);
    }

    #[test]
    fn hyp_roundtrip_cl100k(text in "\\PC*") {
        let enc = load("cl100k_base");
        let allowed = std::collections::HashSet::new();
        let disallowed = enc.special_tokens();
        if validation::encode_checked(&enc, &text, &allowed, &disallowed).is_err() {
            return Ok(());
        }
        let tokens = validation::encode_checked(&enc, &text, &allowed, &disallowed).unwrap();
        prop_assert_eq!(enc.decode_lossy(&tokens).unwrap(), text);
    }

    #[test]
    fn hyp_encode_bytes_cl100k(bytestring in proptest::collection::vec(any::<u8>(), 0..256)) {
        let enc = load("cl100k_base");
        let tokens = enc.encode_bytes(&bytestring);
        prop_assert_eq!(enc.decode_bytes(&tokens).unwrap(), bytestring);
    }
}
