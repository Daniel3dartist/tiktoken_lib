# Installation and build

## Requirements

| Tool | Required | Notes |
|------|----------|-------|
| [Rust stable](https://www.rust-lang.org/tools/install) | Yes | Builds the core and FFI |
| [SCons](https://scons.org/) | Recommended | Default project build |
| MSVC Build Tools (Windows) or GCC/Clang (Unix) | For C/C++ examples | Link against the library |
| HTTP network | First run | Download `.tiktoken` / GPT-2 files |

### Windows — MSVC

Install **Build Tools for Visual Studio** with the **Desktop development with C++** workload (provides `link.exe`, required by Rust on the MSVC target).

### Vocabulary cache

Avoid re-downloading vocabularies on every run:

```powershell
# Windows
$env:TIKTOKEN_CACHE_DIR = "C:\cache\tiktoken"
```

```bash
# Linux / macOS
export TIKTOKEN_CACHE_DIR=/var/cache/tiktoken
```

Python-compatible: `DATA_GYM_CACHE_DIR`.

## Build with SCons

From the repository root:

```bash
# Static library + headers + test binary
scons

# Run GPT-2 compatibility test
scons test=1

# Shared library (DLL / .so / .dylib)
scons shared=1

# Copy to build/install (or prefix=/usr/local)
scons install
```

### Artifacts in `build/`

| File | Mode |
|------|------|
| `tiktoken.lib` / `libtiktoken.a` | Static (default) |
| `tiktoken.dll` / `libtiktoken.so` | `shared=1` |
| `tiktoken.h`, `tiktoken.hpp` | Always |
| `test_gpt2.exe` / `test_gpt2` | Included test |

## Cargo-only build

```bash
cargo build --release
```

Output in `target/release/`:

- Static: `tiktoken.lib` (Windows) or `libtiktoken.a` (Unix)
- Shared: `tiktoken.dll` / `libtiktoken.so` (crate-type includes `cdylib`)

## Docker

```bash
docker build -t tiktoken-lib .
```

The image builds the project and runs `scons test=1`.

## Compile a documentation example

After `scons`, build a C example (Windows, static):

```powershell
cl /I build /I include docs\examples\c\basic.c build\tiktoken.lib ws2_32.lib userenv.lib bcrypt.lib advapi32.lib ntdll.lib /Fe:basic.exe
.\basic.exe
```

Linux (static):

```bash
gcc -Iinclude -Ibuild docs/examples/c/basic.c \
  -Wl,--whole-archive build/libtiktoken.a -Wl,--no-whole-archive \
  -lpthread -ldl -lm -o basic
./basic
```

Platform-specific link flags: [linking.md](linking.md).
