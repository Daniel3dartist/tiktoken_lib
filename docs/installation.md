# Installation and build

## Requirements

| Tool | Required | Notes |
|------|----------|-------|
| [Rust stable](https://www.rust-lang.org/tools/install) | Yes | Builds the core and FFI |
| [SCons](https://scons.org/) | Recommended | Default project build |
| MSVC Build Tools (Windows) or GCC/Clang (Unix) | For C/C++ examples | Link against the library |
| HTTP network | First run | Download `.tiktoken` / GPT-2 files |

### Windows - MSVC (required for `cargo test` / `cargo build`)

Rust on Windows defaults to **`x86_64-pc-windows-msvc`**, which needs Microsoft’s linker **`link.exe`** (not included with `rustup` alone).

**Install (pick one):**

1. [Build Tools for Visual Studio 2022](https://visualstudio.microsoft.com/visual-cpp-build-tools/) → workload **“Desenvolvimento para desktop com C++”** / **Desktop development with C++** (includes MSVC, `link.exe`, Windows SDK).
2. Full Visual Studio 2022 with the same workload.

**Verify** (new terminal after install):

```powershell
where.exe link.exe
# e.g. C:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\...\bin\Hostx64\x64\link.exe
```

Then:

```powershell
cargo test
```

**If `link.exe` is still not found**, open **“Developer PowerShell for VS 2022”** (or **x64 Native Tools Command Prompt**) and run `cargo test` from there so `VC\Tools\MSVC\...\bin` is on `PATH`.

**Alternative (GNU toolchain, no `link.exe`):** install [MSYS2](https://www.msys2.org/) or MinGW-w64, then:

```powershell
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
# ensure gcc is on PATH, e.g. C:\msys64\mingw64\bin
cargo test
```

Note: `scons` / MSVC examples in this repo still expect the **MSVC** toolchain on Windows unless you adapt link flags.

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
