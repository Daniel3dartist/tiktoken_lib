use tiktoken::get_encoding;

#[test]
fn gpt2_hello_world() {
    let enc = get_encoding("gpt2").expect("load gpt2");
    assert_eq!(enc.encode_ordinary("hello world"), vec![31373, 995]);
    assert_eq!(
        enc.decode(&[31373, 995]).expect("decode"),
        "hello world"
    );
}

#[test]
fn gpt2_endoftext() {
    let enc = get_encoding("gpt2").expect("load gpt2");
    let allowed: std::collections::HashSet<&str> =
        std::iter::once("<|endoftext|>").collect();
    let (tokens, _) = enc.encode("hello <|endoftext|>", &allowed).expect("encode");
    assert_eq!(tokens, vec![31373, 220, 50256]);
}

#[test]
fn r50k_matches_gpt2_hello_world() {
    let enc = get_encoding("r50k_base").expect("load r50k_base");
    assert_eq!(enc.encode_ordinary("hello world"), vec![31373, 995]);
}

#[test]
fn list_encodings_contains_gpt2() {
    let names = tiktoken::list_encoding_names();
    assert!(names.contains(&"gpt2"));
    assert!(names.contains(&"cl100k_base"));
}
