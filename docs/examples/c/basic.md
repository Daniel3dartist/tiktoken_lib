# C example — basic (GPT-2)

Loads the `gpt2` encoding, tokenizes `"hello world"`, and decodes back.

**Code:** [`basic.c`](basic.c)

## Expected output

```
tokens (2): 31373 995
decoded: hello world
```

These IDs match `tiktoken.get_encoding("gpt2").encode("hello world")` in Python.

## Step by step

### 1. Load the encoding

```c
CoreBPEError err = {0};
CoreBPE* enc = tiktoken_get_encoding("gpt2", &err);
```

On first run, the GPT-2 vocabulary is downloaded and cached. Set `TIKTOKEN_CACHE_DIR` to control the directory.

### 2. Ordinary encode

```c
EncodeResult encoded = corebpe_encode_ordinary(enc, "hello world");
```

`encode_ordinary` does **not** treat strings like `<|endoftext|>` as special tokens — it only applies regex + BPE.

### 3. Decode

```c
char* decoded = corebpe_decode(enc, encoded.tokens, encoded.len, &err);
```

### 4. Free memory

```c
tiktoken_free_string(decoded);
encode_result_free(encoded);
corebpe_free(enc);
corebpe_error_free(&err);
```

See [memory.md](../../memory.md) for the full table.

## Compile

From the repository root, after `scons`:

**Windows (MSVC):**

```powershell
cl /I build /I include docs\examples\c\basic.c build\tiktoken.lib ^
   ws2_32.lib userenv.lib bcrypt.lib advapi32.lib ntdll.lib /Fe:basic.exe
.\basic.exe
```

**Linux:**

```bash
gcc -Iinclude -Ibuild docs/examples/c/basic.c \
  -Wl,--whole-archive build/libtiktoken.a -Wl,--no-whole-archive \
  -lpthread -ldl -lm -o basic
./basic
```

More linking options: [linking.md](../../linking.md).

## Next steps

- [special_tokens.md](special_tokens.md) — use `<|endoftext|>`
- [list_encodings.md](list_encodings.md) — enumerate available encodings
