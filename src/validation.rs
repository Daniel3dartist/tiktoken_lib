use std::collections::HashSet;

use fancy_regex::Regex;

/// Returns the first disallowed special token substring found in `text`, if any.
///
/// Mirrors the Python `Encoding.encode` check using a regex over `disallowed_special`.
pub fn find_disallowed_special(text: &str, disallowed_special: &HashSet<&str>) -> Option<String> {
    if disallowed_special.is_empty() {
        return None;
    }
    let inner: String = disallowed_special
        .iter()
        .map(|s| fancy_regex::escape(s))
        .collect::<Vec<_>>()
        .join("|");
    let pattern = format!("({inner})");
    let re = Regex::new(&pattern).ok()?;
    re.find(text)
        .ok()
        .flatten()
        .map(|m| text[m.start()..m.end()].to_string())
}

/// Encodes with `allowed_special`, after validating `disallowed_special` (Python `Encoding.encode` policy).
pub fn encode_checked(
    bpe: &crate::CoreBPE,
    text: &str,
    allowed_special: &HashSet<&str>,
    disallowed_special: &HashSet<&str>,
) -> Result<Vec<crate::Rank>, String> {
    if let Some(token) = find_disallowed_special(text, disallowed_special) {
        return Err(format!(
            "Encountered text corresponding to disallowed special token {token:?}"
        ));
    }
    bpe.encode(text, allowed_special)
        .map(|(tokens, _)| tokens)
        .map_err(|e| e.message)
}
