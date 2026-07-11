# C example - special tokens

Demonstrates `<|endoftext|>` (ID **50256** in GPT-2) when explicitly allowed.

**Code:** [`special_tokens.c`](special_tokens.c)

## Behavior

| Function | `"hello <|endoftext|>"` |
|----------|-------------------------|
| `corebpe_encode_ordinary` | Tokenizes `\|endoftext\|` as text (multiple tokens) |
| `corebpe_encode` + `allowed` | `[31373, 220, 50256]` |
| `corebpe_encode_with_special_tokens` | Same as Python with `allowed_special="all"` |

## Main snippet

```c
const char* allowed[] = {"<|endoftext|>"};
EncodeResult r = corebpe_encode(enc, "hello <|endoftext|>", allowed, 1, &err);
/* r.tokens → 31373, 220, 50256 */
```

`220` is the space token before `<|endoftext|>`.

## Compile

Same commands as [basic.md](basic.md), replacing the source file with `special_tokens.c`.

## Note vs Python

In Python, `encode(text)` **raises** if it finds a special token that is not allowed. In the C API, specials not listed in `allowed_special` are tokenized as ordinary text (Rust core behavior). Validate in your code if you need Python's strict mode.
