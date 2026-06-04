# C API

Header: `include/tiktoken.h`

## Lifecycle

```c
CoreBPEError err = {0};

/* Option 1: built-in encoding (recommended) */
CoreBPE* enc = tiktoken_get_encoding("gpt2", &err);

/* Option 2: custom vocabulary */
CoreBPEConfig config = { /* encoder_keys, pattern, ... */ };
CoreBPE* enc = corebpe_new(&config, &err);

corebpe_free(enc);
corebpe_error_free(&err);
```

### Built-in encodings

```c
const char* ver = tiktoken_version();

size_t count = 0;
char** names = tiktoken_list_encoding_names(&count);
/* names[0..count-1] */
strings_array_free(names, count);
```

Available names: see [encodings.md](encodings.md).

---

## Encode

### Ordinary (ignores special tokens)

Does not treat `<|endoftext|>` as a single token — tokenizes it as normal text.

```c
EncodeResult r = corebpe_encode_ordinary(enc, "hello world");
/* r.tokens[0]=31373, r.tokens[1]=995 for gpt2 */
encode_result_free(r);
```

### With special-token control

```c
const char* allowed[] = {"<|endoftext|>"};
EncodeResult r = corebpe_encode(enc, "hello <|endoftext|>", allowed, 1, &err);
encode_result_free(r);
```

### All specials allowed

```c
EncodeResult r = corebpe_encode_with_special_tokens(enc, text);
encode_result_free(r);
```

---

## Decode

### UTF-8 string

```c
uint32_t tokens[] = {31373, 995};
char* text = corebpe_decode(enc, tokens, 2, &err);
tiktoken_free_string(text);
```

### Raw bytes

Useful when the byte sequence is not valid UTF-8.

```c
size_t len = 0;
uint8_t* bytes = corebpe_decode_bytes(enc, tokens, n, &len, &err);
tiktoken_free_bytes(bytes, len);
```

---

## Instance special tokens

```c
size_t n = 0;
char** specials = corebpe_special_tokens(enc, &n);
strings_array_free(specials, n);
```

---

## Return types

| Type | Main fields | Free with |
|------|-------------|-----------|
| `EncodeResult` | `tokens`, `len`, `last_piece_token_len` | `encode_result_free` |
| `UnstableEncodeResult` | `tokens`, `completions`, ... | `unstable_encode_result_free` |
| `CoreBPEError` | `message`, `is_key_error` | `corebpe_error_free` |
| `char*` / `uint8_t*` | allocated by lib | `tiktoken_free_string` / `tiktoken_free_bytes` |

Details: [memory.md](memory.md).

---

## Low-level BPE

Utility functions that operate only on the merge table (no regex):

- `byte_pair_merge`
- `byte_pair_encode`
- `byte_pair_split` (+ `byte_pair_split_free`)

Useful for debugging or unit tests of the BPE algorithm.
