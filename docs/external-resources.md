# External resources

Same sources as [Python tiktoken](https://github.com/openai/tiktoken). The Rust library downloads and caches these files automatically.

## Vocabularies (Azure Blob Storage)

| URL | Description |
|-----|-------------|
| https://openaipublic.blob.core.windows.net/gpt-2/encodings/main/vocab.bpe | Original GPT-2 BPE merge table |
| https://openaipublic.blob.core.windows.net/gpt-2/encodings/main/encoder.json | GPT-2 token→index map; validates consistency with vocab.bpe |
| https://openaipublic.blob.core.windows.net/encodings/r50k_base.tiktoken | Precompiled r50k vocabulary (`.tiktoken` format) |
| https://openaipublic.blob.core.windows.net/encodings/p50k_base.tiktoken | p50k vocabulary |
| https://openaipublic.blob.core.windows.net/encodings/cl100k_base.tiktoken | cl100k vocabulary (GPT-3.5/4) |
| https://openaipublic.blob.core.windows.net/encodings/o200k_base.tiktoken | o200k vocabulary (GPT-4o) |

`.tiktoken` format: one line per token — `base64(token_bytes) rank`.

SHA-256 hashes are verified on download (same values as Python).

## References

| URL | Description |
|-----|-------------|
| https://en.wikipedia.org/wiki/Byte_pair_encoding | BPE algorithm |
| https://github.com/openai/tiktoken | Official OpenAI repository |
| https://pypi.org/project/tiktoken | Python PyPI package |
| https://github.com/openai/openai-cookbook/blob/main/examples/How_to_count_tokens_with_tiktoken.ipynb | Token counting examples |
| https://github.com/openai/tiktoken/issues | Upstream issues |
