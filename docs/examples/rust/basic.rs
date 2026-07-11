//! Rust example - direct crate usage (no C FFI).
//!
//! Run from the repository root:
//!   cargo run --example basic --manifest-path docs/examples/rust/Cargo.toml
//!
//! Or add `tiktoken = { path = "../.." }` in your Cargo.toml.

use tiktoken::get_encoding;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let enc = get_encoding("gpt2")?;

    let text = "hello world";
    let tokens = enc.encode_ordinary(text);
    println!("tokens ({}):" , tokens.len());
    for t in &tokens {
        print!(" {t}");
    }
    println!();

    let decoded = enc.decode(&tokens)?;
    println!("decoded: {decoded}");

    let allowed: std::collections::HashSet<&str> = ["<|endoftext|>"].into();
    let (eot_tokens, _) = enc.encode("hello <|endoftext|>", &allowed)?;
    println!("with eot: {eot_tokens:?}");

    println!("encodings: {:?}", tiktoken::list_encoding_names());

    Ok(())
}
