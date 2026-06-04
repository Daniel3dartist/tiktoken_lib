# Examples

Reference programs for integrating TikToken Lib. Build after `scons` at the repository root.

| Folder | Language | Files |
|--------|----------|-------|
| [c/](c/) | C | `basic`, `special_tokens`, `list_encodings` |
| [cpp/](cpp/) | C++ | `basic` |
| [rust/](rust/) | Rust | `basic` (crate path) |

## Build all (reference)

### Windows (MSVC, static)

```powershell
cd ..\..   # repository root
scons

cl /I build /I include docs\examples\c\basic.c build\tiktoken.lib ws2_32.lib userenv.lib bcrypt.lib advapi32.lib ntdll.lib /Fe:build\ex_basic.exe
build\ex_basic.exe
```

### Linux

```bash
cd ../..   # root
scons

gcc -Iinclude -Ibuild docs/examples/c/basic.c \
  -Wl,--whole-archive build/libtiktoken.a -Wl,--no-whole-archive \
  -lpthread -ldl -lm -o build/ex_basic
./build/ex_basic
```

Detailed instructions in each `.md` under the subfolders.
