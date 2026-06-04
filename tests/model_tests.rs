//! Ports `test_misc.py` / `test_simple_public.py::test_encoding_for_model`.

use tiktoken::model::encoding_name_for_model;

#[test]
fn test_encoding_name_for_model() {
    assert_eq!(encoding_name_for_model("gpt2").unwrap(), "gpt2");
    assert_eq!(
        encoding_name_for_model("text-davinci-003").unwrap(),
        "p50k_base"
    );
    assert_eq!(
        encoding_name_for_model("text-davinci-edit-001").unwrap(),
        "p50k_edit"
    );
    assert_eq!(
        encoding_name_for_model("gpt-3.5-turbo-0301").unwrap(),
        "cl100k_base"
    );
    assert_eq!(encoding_name_for_model("gpt-4").unwrap(), "cl100k_base");
    assert_eq!(encoding_name_for_model("gpt-4o").unwrap(), "o200k_base");
    assert_eq!(
        encoding_name_for_model("gpt-oss-120b").unwrap(),
        "o200k_harmony"
    );
}

#[test]
fn test_encoding_for_model_loads() {
    use tiktoken::model::encoding_for_model;
    let enc = encoding_for_model("gpt2").expect("gpt2");
    assert_eq!(enc.encode_ordinary("hello world"), vec![31373, 995]);
}
