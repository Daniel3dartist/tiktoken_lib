# Go example — basic (GPT-2)

Uses **CGO** to call the C API from Go. Suitable when you want a Go service or CLI without a separate Rust dependency at runtime.

**Code:** [`main.go`](main.go)

## Expected output

```
tokens (2): 31373 995
decoded: hello world
```

These IDs match `tiktoken.get_encoding("gpt2").encode("hello world")` in Python.

## Prerequisites

| Platform | Requirements |
|----------|----------------|
| Linux | `go` 1.21+, `gcc`, static lib from `scons` |
| macOS | `go` 1.21+, Xcode CLI tools, static lib from `scons` |
| Windows | `go` 1.21+, MSVC (`cl`) on `PATH`, `set CC=cl`, `scons` with MSVC |

`CGO_ENABLED=1` is required (default on native builds).

## Build and run

From the repository root:

```bash
scons
cd docs/examples/go
CGO_ENABLED=1 go run .
```

Or build a binary:

```bash
CGO_ENABLED=1 go build -o tiktoken-basic .
./tiktoken-basic
```

On first run, the GPT-2 vocabulary is downloaded and cached. Set `TIKTOKEN_CACHE_DIR` to control the cache directory.

### Windows (MSVC)

```powershell
scons
cd docs\examples\go
$env:CC = "cl"
$env:CGO_ENABLED = "1"
go run .
```

The `#cgo windows` directives in `main.go` link `tiktoken.lib` and the Rust runtime system libraries (same set as the [C example](../c/basic.md)).

## How it works

### 1. CGO preamble

`main.go` embeds `tiktoken.h` and sets include/library paths relative to the example directory (`../../../include`, `../../../build`).

### 2. Load encoding

```go
name := C.CString("gpt2")
defer C.free(unsafe.Pointer(name))

var cErr C.CoreBPEError
enc := C.tiktoken_get_encoding(name, &cErr)
defer C.corebpe_free(enc)
```

### 3. Encode and decode

```go
text := C.CString("hello world")
defer C.free(unsafe.Pointer(text))

encoded := C.corebpe_encode_ordinary(enc, text)
defer C.encode_result_free(encoded)

tokens := unsafe.Slice(encoded.tokens, encoded.len)
decoded := C.corebpe_decode(enc, &tokens[0], C.size_t(len(tokens)), &cErr)
defer C.tiktoken_free_string(decoded)
```

Every pointer allocated by the library or `C.CString` must be freed with the matching API. See [memory.md](../../memory.md).

## Integrate in your project

1. Copy the `#cgo` `CFLAGS` / `LDFLAGS` block from `main.go` into your package (adjust `${SRCDIR}` paths to your layout).
2. Run `scons` (or `cargo build --release`) in tiktoken_lib and point `-L` at `build/` or `target/release/`.
3. Ship `tiktoken.h` (and the static or shared library) with your binary.

For production, consider a thin Go package that wraps load/encode/decode and centralizes `defer` cleanup.

More link flags: [linking.md](../../linking.md).

## Next steps

- [c/special_tokens.md](../c/special_tokens.md) — special-token behavior (same C API from Go)
- [c/list_encodings.md](../c/list_encodings.md) — `tiktoken_list_encoding_names`
