# Rust example - basic

Native crate usage without the C FFI. Best when the consumer is already Rust.

**Code:** [`basic.rs`](basic.rs)

## Run

From the repository root:

```bash
cargo run --manifest-path docs/examples/rust/Cargo.toml
```

Or, with compatibility tests:

```bash
cargo test
```

## Code

```rust
use tiktoken::get_encoding;

let enc = get_encoding("gpt2")?;
let tokens = enc.encode_ordinary("hello world");
assert_eq!(tokens, vec![31373, 995]);
let text = enc.decode(&tokens)?;
```

### Special tokens

```rust
use std::collections::HashSet;

let allowed: HashSet<&str> = ["<|endoftext|>"].into();
let (tokens, _) = enc.encode("hello <|endoftext|>", &allowed)?;
assert_eq!(tokens, vec![31373, 220, 50256]);
```

## Integrate in another project

```toml
[dependencies]
tiktoken = { path = "../path/to/tiktoken_lib" }
```

## Rust consuming the C API

If you need the C interface (e.g. stable layout or `bindgen`):

1. Static build: `scons`
2. `build.rs` with `cargo:rustc-link-lib=static=tiktoken`
3. Bindings from `include/tiktoken.h`

Details: [linking.md](../../linking.md).
