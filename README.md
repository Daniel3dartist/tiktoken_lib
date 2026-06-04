# TikToken Lib

**Rust** rewrite of the [OpenAI tiktoken](https://github.com/openai/tiktoken) core, exposed as a native library for **C**, **C++**, and other languages that link against `.a`, `.lib`, `.dll`, `.so`, and `.dylib`.

Tokenization behavior matches the original Python project (including GPT-2 / `r50k_base`). The upstream reference lives in `.samples/tiktoken` (gitignored).

**Full documentation and examples:** [`docs/`](docs/README.md) — installation, linking, C API, and sample code in C, C++, Go, and Rust.

## Requirements

| Tool | Purpose |
|------|---------|
| [Rust](https://www.rust-lang.org/tools/install) (stable) | Builds the BPE core + FFI |
| [SCons](https://scons.org/) | Default build for this project |
| MSVC Build Tools (Windows) or GCC/Clang (Linux/macOS) | Link C tests/examples |
| HTTP access (first run) | Downloads vocabulary files |

### Vocabulary cache

On first load of an encoding (`gpt2`, `cl100k_base`, etc.), files are downloaded and cached:

```bash
# Windows (PowerShell)
$env:TIKTOKEN_CACHE_DIR = "C:\cache\tiktoken"

# Linux/macOS
export TIKTOKEN_CACHE_DIR=/var/cache/tiktoken
```

Python-compatible alternative: `DATA_GYM_CACHE_DIR`.

## Build

```bash
# Static library + headers + test binary (does not run)
scons

# Run GPT-2 compatibility test (downloads vocabulary on first run)
scons test=1

# Shared library (DLL/SO)
scons shared=1

# Install to build/install (or prefix=/usr/local)
scons install
```

Artifacts in `build/`:

| File | Description |
|------|-------------|
| `tiktoken.lib` / `libtiktoken.a` | Static library |
| `tiktoken.dll` / `libtiktoken.so` | Shared library (`shared=1`) |
| `tiktoken.h` | C API |
| `tiktoken.hpp` | C++ wrapper (RAII) |
| `test_gpt2` / `test_gpt2.exe` | Compatibility test |

Direct Cargo build (without SCons):

```bash
cargo build --release
# Static: target/release/tiktoken.lib (Windows) or libtiktoken.a (Unix)
# Shared: target/release/tiktoken.dll / libtiktoken.so
```

## Built-in encodings

Load with `tiktoken_get_encoding("name", &err)` or `tiktoken::Encoding enc("name")`:

| Name | Typical use |
|------|-------------|
| `gpt2` | GPT-2 (legacy vocabulary via vocab.bpe + encoder.json) |
| `r50k_base` | Base GPT-3 davinci / r50k models |
| `p50k_base` | text-davinci-003 and similar |
| `p50k_edit` | Edit models (FIM tokens) |
| `cl100k_base` | GPT-4, GPT-3.5-turbo |
| `o200k_base` | GPT-4o |
| `o200k_harmony` | GPT-OSS / harmony |

## Linking

### C — static (Windows)

```c
#include "tiktoken.h"

// cl ... /I path/to/include test.c tiktoken.lib ws2_32.lib userenv.lib bcrypt.lib advapi32.lib ntdll.lib
```

Extra libraries are transitive dependencies of the Rust runtime (network for vocabulary download).

### C — static (Linux/macOS)

```bash
gcc -Iinclude test.c -Lbuild -Wl,--whole-archive build/libtiktoken.a -Wl,--no-whole-archive -lpthread -ldl -lm -o test
```

### C — shared

```bash
# Linux
gcc -Iinclude test.c -Lbuild -ltiktoken -Wl,-rpath,build -o test

# Windows: tiktoken.dll on PATH + tiktoken.dll.lib at link time
```

### C++

Include `tiktoken.hpp` and link the same way as C:

```cpp
#include "tiktoken.hpp"

tiktoken::Encoding enc("gpt2");
auto tokens = enc.encode_ordinary("hello world");  // {31373, 995}
auto text = enc.decode(tokens);                    // "hello world"
```

### Rust (same crate)

```rust
use tiktoken::get_encoding;

let enc = get_encoding("gpt2")?;
assert_eq!(enc.encode_ordinary("hello world"), vec![31373, 995]);
```

For another Rust crate consuming the static library, use `links` + `build.rs` pointing at `libtiktoken.a` and include `tiktoken.h` via `bindgen` if needed.

## C API (summary)

```c
CoreBPEError err = {0};
CoreBPE* enc = tiktoken_get_encoding("gpt2", &err);

EncodeResult r = corebpe_encode_ordinary(enc, "hello world");
// r.tokens[0] == 31373, r.tokens[1] == 995
encode_result_free(r);

char* text = corebpe_decode(enc, (uint32_t[]){31373, 995}, 2, &err);
tiktoken_free_string(text);

corebpe_free(enc);
corebpe_error_free(&err);
```

All memory allocated by the library must be freed with `encode_result_free`, `tiktoken_free_string`, `tiktoken_free_bytes`, `strings_array_free`, or `corebpe_error_free`.

## GPT-2 compatibility

Tests verify upstream golden values:

| Input | Expected tokens |
|-------|-----------------|
| `"hello world"` | `[31373, 995]` |
| `"hello <|endoftext|>"` (special allowed) | `[31373, 220, 50256]` |

```bash
scons test=1
cargo test --test gpt2_compat
```

## External resources (vocabularies and references)

Links used by the original Python project — what each one is:

### Vocabularies (Azure Blob Storage)

| URL | What it is |
|-----|------------|
| https://openaipublic.blob.core.windows.net/gpt-2/encodings/main/vocab.bpe | **GPT-2 merge table**: BPE byte pairs and merge order from original GPT-2 training. |
| https://openaipublic.blob.core.windows.net/gpt-2/encodings/main/encoder.json | **GPT-2 vocabulary**: token→index map; used to verify `vocab.bpe` produces the same ranks. |
| https://openaipublic.blob.core.windows.net/encodings/r50k_base.tiktoken | **Precompiled r50k vocabulary**: 50 257 mergeable tokens + r50k regex; equivalent to GPT-2 for base models. |
| https://openaipublic.blob.core.windows.net/encodings/p50k_base.tiktoken | **p50k vocabulary**: r50k extension with extra tokens (50281 total). |
| https://openaipublic.blob.core.windows.net/encodings/cl100k_base.tiktoken | **cl100k vocabulary**: used by GPT-3.5/4 (100 256 mergeable + specials). |
| https://openaipublic.blob.core.windows.net/encodings/o200k_base.tiktoken | **o200k vocabulary**: used by GPT-4o and recent models. |

`.tiktoken` format: one line per token, `base64(token_bytes) rank`.

### Documentation and project

| URL | What it is |
|-----|------------|
| https://en.wikipedia.org/wiki/Byte_pair_encoding | **Byte Pair Encoding (BPE)** algorithm overview. |
| https://github.com/openai/tiktoken | **Official OpenAI tiktoken** repository (Python + Rust). |
| https://pypi.org/project/tiktoken | **PyPI** package for Python tiktoken. |
| https://github.com/openai/openai-cookbook/blob/main/examples/How_to_count_tokens_with_tiktoken.ipynb | **Notebook** with token-counting examples. |
| https://github.com/openai/tiktoken/issues | Upstream **issues** / support. |
| https://github.com/rust-lang/regex/blob/master/PERFORMANCE.md | Regex engine performance notes (referenced in the Rust core). |

## Licenses

- This project: MIT — see [LICENSE](LICENSE)
- Core derived from OpenAI tiktoken: MIT — see [licenses/OpenAI/LICENSE](licenses/OpenAI/LICENSE)

## Layout

```
docs/             Documentation and examples (C, C++, Go, Rust)
include/          tiktoken.h, tiktoken.hpp
src/
  lib.rs          Core BPE (encode/decode)
  ffi.rs          C bindings
  load.rs         Vocabulary download/cache
  encoding.rs     Built-in encodings (gpt2, cl100k, ...)
tests/            test_gpt2.c, gpt2_compat.rs
SConstruct        SCons build
Cargo.toml        Rust build
```
