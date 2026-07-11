# Memory management

All memory **allocated by the library** must be freed with the matching functions. Do not use the C runtime `free()` on pointers returned by the lib (unless you allocated the buffer yourself).

## General rule

| You received | Free with |
|--------------|-----------|
| `EncodeResult` | `encode_result_free(result)` |
| `UnstableEncodeResult` | `unstable_encode_result_free(result)` |
| `char*` (decode, errors, lists) | `tiktoken_free_string(ptr)` or `strings_array_free` |
| `uint8_t*` (decode_bytes) | `tiktoken_free_bytes(ptr, len)` |
| standalone `uint32_t*` | `tiktoken_free_u32_array(ptr, len)` |
| standalone `size_t*` | `tiktoken_free_usize_array(ptr, len)` |
| `CoreBPE*` | `corebpe_free(enc)` |
| `CoreBPEError.message` | `corebpe_error_free(&err)` |

## Full example

```c
CoreBPEError err = {0};
CoreBPE* enc = tiktoken_get_encoding("gpt2", &err);
if (!enc) {
    if (err.message) {
        fprintf(stderr, "%s\n", err.message);
        corebpe_error_free(&err);
    }
    return 1;
}

EncodeResult encoded = corebpe_encode_ordinary(enc, "hello world");
if (encoded.tokens) {
    char* decoded = corebpe_decode(enc, encoded.tokens, encoded.len, &err);
    if (decoded) {
        printf("%s\n", decoded);
        tiktoken_free_string(decoded);
    } else {
        corebpe_error_free(&err);
    }
    encode_result_free(encoded);
}

corebpe_free(enc);
```

## Errors

`CoreBPEError` is a struct passed by value. After a failed call:

```c
if (err.message) {
    printf("Error: %s (key=%d)\n", err.message, err.is_key_error);
    corebpe_error_free(&err);
}
```

`is_key_error == true` indicates an invalid token during decode (`DecodeKeyError`).

## Empty EncodeResult

If `encode` fails, `result.tokens` may be `NULL` and `result.len == 0`. The current implementation handles `tokens == NULL` safely in `encode_result_free`, but always check the operation return value before using tokens.
