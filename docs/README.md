# Documentation — TikToken Lib

Guide to the native BPE tokenization library (compatible with OpenAI tiktoken).

## Index

| Document | Contents |
|----------|----------|
| [Installation and build](installation.md) | Requirements, SCons, Cargo, vocabulary cache |
| [Linking](linking.md) | Static vs shared — C, C++, Rust, Windows/Linux/macOS |
| [C API](api-c.md) | Functions, types, encode/decode flow |
| [Memory management](memory.md) | What to allocate, what to free |
| [Encodings](encodings.md) | Built-in names and when to use each |
| [External resources](external-resources.md) | Vocabulary URLs and references |

## Examples

Runnable code in [`examples/`](examples/):

| Example | Language | Description |
|---------|----------|-------------|
| [c/basic](examples/c/basic.md) | C | Load GPT-2, encode and decode |
| [c/special_tokens](examples/c/special_tokens.md) | C | `<\|endoftext\|>` and allowed tokens |
| [c/list_encodings](examples/c/list_encodings.md) | C | List available encodings |
| [cpp/basic](examples/cpp/basic.md) | C++ | RAII wrapper with `tiktoken.hpp` |
| [rust/basic](examples/rust/basic.md) | Rust | Direct crate usage |

## Quick start (C)

```c
#include "tiktoken.h"

CoreBPEError err = {0};
CoreBPE* enc = tiktoken_get_encoding("gpt2", &err);
if (!enc) { /* handle err.message */ }

EncodeResult r = corebpe_encode_ordinary(enc, "hello world");
// r.tokens → [31373, 995]

encode_result_free(r);
corebpe_free(enc);
corebpe_error_free(&err);
```

See the full walkthrough in [examples/c/basic.md](examples/c/basic.md).
