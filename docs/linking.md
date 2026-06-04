# Linking

The library can be linked **statically** (default) or **dynamically**. Always include the header:

```c
#include "tiktoken.h"   /* C */
#include "tiktoken.hpp" /* C++ */
```

Typical directories after `scons`:

- Headers: `include/` or `build/`
- Library: `build/`

---

## C — static

### Windows (MSVC)

```
cl your_code.c /I include /I build ^
   build\tiktoken.lib ^
   ws2_32.lib userenv.lib bcrypt.lib advapi32.lib ntdll.lib
```

| Library | Reason |
|---------|--------|
| `tiktoken.lib` | Core + FFI |
| `ws2_32.lib` | Network (vocabulary download via `ureq`) |
| `userenv.lib`, `bcrypt.lib`, `advapi32.lib`, `ntdll.lib` | Rust runtime / system |

### Linux

```bash
gcc your_code.c -Iinclude -Lbuild \
  -Wl,--whole-archive build/libtiktoken.a -Wl,--no-whole-archive \
  -lpthread -ldl -lm -o app
```

### macOS

```bash
clang your_code.c -Iinclude \
  -Wl,-force_load,build/libtiktoken.a \
  -lpthread -ldl -lm -o app
```

---

## C — shared

Build: `scons shared=1`

### Windows

```
cl your_code.c /I include /I build build\tiktoken.dll.lib
```

Place `tiktoken.dll` on `PATH` or next to the executable.

### Linux

```bash
gcc your_code.c -Iinclude -Lbuild -ltiktoken -Wl,-rpath,build -o app
export LD_LIBRARY_PATH=build:$LD_LIBRARY_PATH
```

### macOS

```bash
clang your_code.c -Iinclude -Lbuild -ltiktoken -Wl,-rpath,build -o app
```

---

## C++

Same link flags as C. Use `tiktoken.hpp` for RAII:

```cpp
#include "tiktoken.hpp"

tiktoken::Encoding enc("gpt2");
auto tokens = enc.encode_ordinary("hello world");
```

Full example: [examples/cpp/basic.md](examples/cpp/basic.md).

---

## Rust (another crate)

### Path dependency (development)

```toml
[dependencies]
tiktoken = { path = "../tiktoken_lib" }
```

```rust
use tiktoken::get_encoding;

let enc = get_encoding("gpt2")?;
let tokens = enc.encode_ordinary("hello world");
```

### Consuming the static C library

1. `cargo build --release` in tiktoken_lib
2. Your crate's `build.rs`:

```rust
fn main() {
    println!("cargo:rustc-link-search=native=../tiktoken_lib/build");
    println!("cargo:rustc-link-lib=static=tiktoken");
    // Windows: also link ws2_32, userenv, bcrypt, advapi32, ntdll
}
```

3. Generate bindings with [`bindgen`](https://github.com/rust-lang/rust-bindgen) from `include/tiktoken.h` if you prefer the C API.

---

## Summary

| Mode | Windows | Linux/macOS |
|------|---------|-------------|
| Static | `tiktoken.lib` + Rust runtime libs | `libtiktoken.a` + `-lpthread -ldl -lm` |
| Shared | `tiktoken.dll` + `tiktoken.dll.lib` | `-ltiktoken` + rpath |
