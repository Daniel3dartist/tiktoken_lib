# C example - list encodings

Enumerates encodings registered in the library.

**Code:** [`list_encodings.c`](list_encodings.c)

## Expected output

```
tiktoken version: 0.1.0
encodings (7):
  - gpt2
  - r50k_base
  - p50k_base
  - p50k_edit
  - cl100k_base
  - o200k_base
  - o200k_harmony
```

## API used

```c
const char* ver = tiktoken_version();

size_t count = 0;
char** names = tiktoken_list_encoding_names(&count);
/* ... use names[i] ... */
strings_array_free(names, count);
```

This example does **not** download vocabularies - it only reads in-memory metadata.

## Compile

Same commands as [basic.md](basic.md), with `list_encodings.c`.
