use crate::encoding::get_encoding;
use crate::CoreBPE;

/// Model name prefix → encoding name (newer / versioned models).
const MODEL_PREFIX_TO_ENCODING: &[(&str, &str)] = &[
    ("o1-", "o200k_base"),
    ("o3-", "o200k_base"),
    ("o4-mini-", "o200k_base"),
    ("gpt-5-", "o200k_base"),
    ("gpt-4.5-", "o200k_base"),
    ("gpt-4.1-", "o200k_base"),
    ("chatgpt-4o-", "o200k_base"),
    ("gpt-4o-", "o200k_base"),
    ("gpt-4-", "cl100k_base"),
    ("gpt-3.5-turbo-", "cl100k_base"),
    ("gpt-35-turbo-", "cl100k_base"),
    ("gpt-oss-", "o200k_harmony"),
    ("ft:gpt-4o", "o200k_base"),
    ("ft:gpt-4", "cl100k_base"),
    ("ft:gpt-3.5-turbo", "cl100k_base"),
    ("ft:davinci-002", "cl100k_base"),
    ("ft:babbage-002", "cl100k_base"),
];

/// Exact model name → encoding name.
const MODEL_TO_ENCODING: &[(&str, &str)] = &[
    ("o1", "o200k_base"),
    ("o3", "o200k_base"),
    ("o4-mini", "o200k_base"),
    ("gpt-5", "o200k_base"),
    ("gpt-4.1", "o200k_base"),
    ("gpt-4o", "o200k_base"),
    ("gpt-4", "cl100k_base"),
    ("gpt-3.5-turbo", "cl100k_base"),
    ("gpt-3.5", "cl100k_base"),
    ("gpt-35-turbo", "cl100k_base"),
    ("davinci-002", "cl100k_base"),
    ("babbage-002", "cl100k_base"),
    ("text-embedding-ada-002", "cl100k_base"),
    ("text-embedding-3-small", "cl100k_base"),
    ("text-embedding-3-large", "cl100k_base"),
    ("text-davinci-003", "p50k_base"),
    ("text-davinci-002", "p50k_base"),
    ("text-davinci-001", "r50k_base"),
    ("text-curie-001", "r50k_base"),
    ("text-babbage-001", "r50k_base"),
    ("text-ada-001", "r50k_base"),
    ("davinci", "r50k_base"),
    ("curie", "r50k_base"),
    ("babbage", "r50k_base"),
    ("ada", "r50k_base"),
    ("code-davinci-002", "p50k_base"),
    ("code-davinci-001", "p50k_base"),
    ("code-cushman-002", "p50k_base"),
    ("code-cushman-001", "p50k_base"),
    ("davinci-codex", "p50k_base"),
    ("cushman-codex", "p50k_base"),
    ("text-davinci-edit-001", "p50k_edit"),
    ("code-davinci-edit-001", "p50k_edit"),
    ("text-similarity-davinci-001", "r50k_base"),
    ("text-similarity-curie-001", "r50k_base"),
    ("text-similarity-babbage-001", "r50k_base"),
    ("text-similarity-ada-001", "r50k_base"),
    ("text-search-davinci-doc-001", "r50k_base"),
    ("text-search-curie-doc-001", "r50k_base"),
    ("text-search-babbage-doc-001", "r50k_base"),
    ("text-search-ada-doc-001", "r50k_base"),
    ("code-search-babbage-code-001", "r50k_base"),
    ("code-search-ada-code-001", "r50k_base"),
    ("gpt2", "gpt2"),
    ("gpt-2", "gpt2"),
];

/// Returns the encoding name used by an OpenAI API model id.
pub fn encoding_name_for_model(model_name: &str) -> Result<&'static str, String> {
    if let Some(&(_, enc)) = MODEL_TO_ENCODING
        .iter()
        .find(|(name, _)| *name == model_name)
    {
        return Ok(enc);
    }
    for (prefix, enc) in MODEL_PREFIX_TO_ENCODING {
        if model_name.starts_with(prefix) {
            return Ok(enc);
        }
    }
    Err(format!(
        "Could not automatically map {model_name:?} to a tokeniser. \
         Use `get_encoding` to explicitly get the tokeniser you expect."
    ))
}

/// Loads the [`CoreBPE`] used by an OpenAI API model id.
pub fn encoding_for_model(
    model_name: &str,
) -> Result<CoreBPE, Box<dyn std::error::Error + Send + Sync>> {
    get_encoding(encoding_name_for_model(model_name)?)
}
