# Test suite

Rust ports of the public upstream tests in `.samples/tiktoken/tests/`.

## Run

**Windows:** `cargo test` needs the MSVC linker (`link.exe`). Install [Build Tools for Visual Studio](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with the **Desktop development with C++** workload. See [docs/installation.md](../docs/installation.md).

```bash
cargo test
# limit property-test cases (default 100, same env as upstream)
TIKTOKEN_MAX_EXAMPLES=50 cargo test
```

First run downloads vocabulary files (requires network); cache via `TIKTOKEN_CACHE_DIR`.

## Coverage map

| Upstream | Rust | Notes |
|----------|------|-------|
| `test_encoding.py` (deterministic) | `encoding_compat.rs` | Uses `validation::encode_checked` for `disallowed_special` policy |
| `test_encoding.py` (hypothesis) | `property_roundtrip.rs` | `proptest` instead of `hypothesis` |
| `test_simple_public.py` | `encoding_compat.rs` | |
| `test_misc.py` (`encoding_for_model`) | `model_tests.rs` | |
| `src/lib.rs` BPE unit tests | `src/lib.rs` | unchanged |
| `test_offsets.py` | — | `decode_with_offsets` not implemented |
| `test_pickle.py` | — | Python-only |
| `test_misc.py` (blobfile) | — | Python import check |
| Lone UTF-16 surrogates | — | Cannot exist in Rust `&str`; emoji case covered |

## Intentional differences

- **`test_large_repeated`**: Python raises `ValueError` on 1M chars for `o200k_base`; Rust currently succeeds (`test_o200k_large_input` documents this).
- **`decode`**: roundtrip tests use `decode_lossy` (Python default `errors="replace"`).
