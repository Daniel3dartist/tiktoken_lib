# Built-in encodings

Load with `tiktoken_get_encoding("name", &err)` or, in C++, `tiktoken::Encoding enc("name")`.

| Name | Vocabulary | Regex | Typical use |
|------|------------|-------|-------------|
| `gpt2` | vocab.bpe + encoder.json (original GPT-2) | r50k | GPT-2, legacy training compatibility |
| `r50k_base` | r50k_base.tiktoken | r50k | r50k base (50257 tokens) |
| `p50k_base` | p50k_base.tiktoken | r50k | text-davinci-003, etc. |
| `p50k_edit` | p50k_base.tiktoken + FIM | r50k | Code edit models |
| `cl100k_base` | cl100k_base.tiktoken | cl100k | GPT-3.5-turbo, GPT-4 |
| `o200k_base` | o200k_base.tiktoken | o200k | GPT-4o |
| `o200k_harmony` | o200k + harmony reserved | o200k | GPT-OSS / harmony |

## GPT-2 / r50k — reference values

To validate compatibility with Python tiktoken:

| Input | Tokens (`gpt2` / `r50k_base`) |
|-------|-------------------------------|
| `"hello world"` | `[31373, 995]` |
| `"hello world"` (`cl100k_base`) | `[15339, 1917]` |
| `"hello <|endoftext|>"` with special allowed | `[31373, 220, 50256]` |

## Default special token

| Encoding | `<\|endoftext\|>` ID |
|----------|----------------------|
| gpt2, r50k, p50k | 50256 |
| cl100k_base | 100257 |
| o200k_base | 199999 |

## Choosing an encoding

- **GPT-2 model:** use `gpt2` or `r50k_base` (same regex pattern; equivalent vocabulary).
- **Classic GPT-3.5 / GPT-4:** `cl100k_base`.
- **GPT-4o:** `o200k_base`.

The encoding name must match both the vocabulary **and** special tokens — do not mix one encoding's vocabulary with another's regex.

## Downloaded files

On first load, files are fetched from URLs in [external-resources.md](external-resources.md) and cached in `TIKTOKEN_CACHE_DIR`.
