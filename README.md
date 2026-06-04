# TikToken Lib

Reescrita em **Rust** do núcleo do [OpenAI tiktoken](https://github.com/openai/tiktoken), exposta como biblioteca nativa para **C**, **C++** e outras linguagens compatíveis com `.a`, `.lib`, `.dll`, `.so` e `.dylib`.

O comportamento de tokenização é compatível com o projeto Python original (incluindo GPT-2 / `r50k_base`). A referência upstream está em `.samples/tiktoken` (gitignored).

## Requisitos

| Ferramenta | Uso |
|------------|-----|
| [Rust](https://www.rust-lang.org/tools/install) (stable) | Compila o núcleo BPE + FFI |
| [SCons](https://scons.org/) | Build padrão deste projeto |
| MSVC Build Tools (Windows) ou GCC/Clang (Linux/macOS) | Linkar testes/exemplos em C |
| Acesso HTTP (primeira execução) | Download dos arquivos de vocabulário |

### Cache de vocabulário

Na primeira carga de um encoding (`gpt2`, `cl100k_base`, etc.), os arquivos são baixados e cacheados:

```bash
# Windows (PowerShell)
$env:TIKTOKEN_CACHE_DIR = "C:\cache\tiktoken"

# Linux/macOS
export TIKTOKEN_CACHE_DIR=/var/cache/tiktoken
```

Alternativa compatível com o Python: `DATA_GYM_CACHE_DIR`.

## Build

```bash
# Biblioteca estática + headers + teste (não executa)
scons

# Executar teste de compatibilidade GPT-2 (baixa vocabulário na 1ª vez)
scons test=1

# Biblioteca dinâmica (DLL/SO)
scons shared=1

# Instalar em build/install (ou prefix=/usr/local)
scons install
```

Artefatos gerados em `build/`:

| Arquivo | Descrição |
|---------|-----------|
| `tiktoken.lib` / `libtiktoken.a` | Biblioteca estática |
| `tiktoken.dll` / `libtiktoken.so` | Biblioteca dinâmica (`shared=1`) |
| `tiktoken.h` | API C |
| `tiktoken.hpp` | Wrapper C++ (RAII) |
| `test_gpt2` / `test_gpt2.exe` | Teste de compatibilidade |

Build direto com Cargo (sem SCons):

```bash
cargo build --release
# Estático: target/release/tiktoken.lib (Windows) ou libtiktoken.a (Unix)
# Dinâmico:  target/release/tiktoken.dll / libtiktoken.so
```

## Encodings embutidos

Carregados via `tiktoken_get_encoding("nome", &err)` ou `tiktoken::Encoding enc("nome")`:

| Nome | Uso típico |
|------|------------|
| `gpt2` | GPT-2 (vocabulário legacy via vocab.bpe + encoder.json) |
| `r50k_base` | Base GPT-3 davinci / modelos r50k |
| `p50k_base` | text-davinci-003 e similares |
| `p50k_edit` | Modelos de edição (FIM tokens) |
| `cl100k_base` | GPT-4, GPT-3.5-turbo |
| `o200k_base` | GPT-4o |
| `o200k_harmony` | GPT-OSS / harmony |

## Linkagem

### C — estático (Windows)

```c
#include "tiktoken.h"

// cl ... /I path/to/include test.c tiktoken.lib ws2_32.lib userenv.lib bcrypt.lib advapi32.lib ntdll.lib
```

Bibliotecas extras são dependências transitivas do runtime Rust (rede para download de vocabulário).

### C — estático (Linux/macOS)

```bash
gcc -Iinclude test.c -Lbuild -Wl,--whole-archive build/libtiktoken.a -Wl,--no-whole-archive -lpthread -ldl -lm -o test
```

### C — dinâmico

```bash
# Linux
gcc -Iinclude test.c -Lbuild -ltiktoken -Wl,-rpath,build -o test

# Windows: tiktoken.dll no PATH + tiktoken.dll.lib no link
```

### C++ 

Inclua `tiktoken.hpp` e linke da mesma forma que C:

```cpp
#include "tiktoken.hpp"

tiktoken::Encoding enc("gpt2");
auto tokens = enc.encode_ordinary("hello world");  // {31373, 995}
auto text = enc.decode(tokens);                    // "hello world"
```

### Rust (mesmo crate)

```rust
use tiktoken::get_encoding;

let enc = get_encoding("gpt2")?;
assert_eq!(enc.encode_ordinary("hello world"), vec![31373, 995]);
```

Para outro crate Rust consumindo a lib estática, use `links` + `build.rs` apontando para `libtiktoken.a` e inclua `tiktoken.h` via `bindgen` se necessário.

## API C (resumo)

```c
CoreBPEError err = {0};
CoreBPE* enc = tiktoken_get_encoding("gpt2", &err);

EncodeResult r = corebpe_encode_ordinary(enc, "hello world");
// r.tokens[0] == 31373, r.tokens[1] == 995
encode_result_free(r);

char* text = corebpe_decode(enc, (uint32_t[]){31373, 995}, 2, &err);
tiktoken_free_string(text);

corebpe_free(enc);
corebpe_error_free(&err);
```

Toda memória alocada pela biblioteca deve ser liberada com as funções `encode_result_free`, `tiktoken_free_string`, `tiktoken_free_bytes`, `strings_array_free` ou `corebpe_error_free`.

## Compatibilidade GPT-2

Testes verificam os valores golden do upstream:

| Entrada | Tokens esperados |
|---------|------------------|
| `"hello world"` | `[31373, 995]` |
| `"hello <|endoftext|>"` (special permitido) | `[31373, 220, 50256]` |

```bash
scons test=1
cargo test --test gpt2_compat
```

## Recursos externos (vocabulários e referências)

Links usados pelo projeto Python original — o que é cada um:

### Vocabulários (Azure Blob Storage)

| URL | O que é |
|-----|---------|
| https://openaipublic.blob.core.windows.net/gpt-2/encodings/main/vocab.bpe | **Merge table GPT-2**: pares de bytes BPE e ordem de merge usada no treinamento original do GPT-2. |
| https://openaipublic.blob.core.windows.net/gpt-2/encodings/main/encoder.json | **Vocabulário GPT-2**: mapa token→índice; usado para validar que `vocab.bpe` produz os mesmos ranks. |
| https://openaipublic.blob.core.windows.net/encodings/r50k_base.tiktoken | **Vocabulário r50k pré-compilado**: 50 257 tokens mergeáveis + regex r50k; equivalente ao GPT-2 para modelos base. |
| https://openaipublic.blob.core.windows.net/encodings/p50k_base.tiktoken | **Vocabulário p50k**: extensão do r50k com tokens extras (50281 total). |
| https://openaipublic.blob.core.windows.net/encodings/cl100k_base.tiktoken | **Vocabulário cl100k**: usado por GPT-3.5/4 (100 256 mergeáveis + especiais). |
| https://openaipublic.blob.core.windows.net/encodings/o200k_base.tiktoken | **Vocabulário o200k**: usado por GPT-4o e modelos recentes. |

Formato `.tiktoken`: uma linha por token, `base64(token_bytes) rank`.

### Documentação e projeto

| URL | O que é |
|-----|---------|
| https://en.wikipedia.org/wiki/Byte_pair_encoding | Explicação do algoritmo **Byte Pair Encoding (BPE)**. |
| https://github.com/openai/tiktoken | Repositório **oficial OpenAI tiktoken** (Python + Rust). |
| https://pypi.org/project/tiktoken | Pacote **PyPI** do tiktoken Python. |
| https://github.com/openai/openai-cookbook/blob/main/examples/How_to_count_tokens_with_tiktoken.ipynb | **Notebook** de exemplos para contar tokens. |
| https://github.com/openai/tiktoken/issues | **Issues** / suporte do projeto upstream. |
| https://github.com/rust-lang/regex/blob/master/PERFORMANCE.md | Notas de performance do motor regex (referenciado no core Rust). |

## Licenças

- Este projeto: MIT — ver [LICENSE](LICENSE)
- Core derivado do OpenAI tiktoken: MIT — ver [licenses/OpenAI/LICENSE](licenses/OpenAI/LICENSE)

## Estrutura

```
include/          tiktoken.h, tiktoken.hpp
src/
  lib.rs          Core BPE (encode/decode)
  ffi.rs          Bindings C
  load.rs         Download/cache de vocabulários
  encoding.rs     Encodings embutidos (gpt2, cl100k, ...)
tests/            test_gpt2.c, gpt2_compat.rs
SConstruct        Build SCons
Cargo.toml        Build Rust
```
