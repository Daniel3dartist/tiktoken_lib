# C++ example - basic

Uses the RAII wrapper in `include/tiktoken.hpp`.

**Code:** [`basic.cpp`](basic.cpp)

## Expected output (excerpt)

```
tokens (2): 31373 995
decoded: hello world
cl100k tokens (2): 15339 1917
available encodings: 7
```

## Using the wrapper

```cpp
#include "tiktoken.hpp"

tiktoken::Encoding enc("gpt2");
auto tokens = enc.encode_ordinary("hello world");
std::string text = enc.decode(tokens);
```

`tiktoken::Error` exceptions wrap load, encode, and decode failures. The destructor frees the C handle automatically.

### Encode with specials

```cpp
std::vector<std::string> allowed = {"<|endoftext|>"};
auto tokens = enc.encode("hello <|endoftext|>", allowed);
```

## Compile

**Windows (MSVC):**

```powershell
cl /EHsc /I build /I include docs\examples\cpp\basic.cpp build\tiktoken.lib ^
   ws2_32.lib userenv.lib bcrypt.lib advapi32.lib ntdll.lib /Fe:basic_cpp.exe
```

**Linux:**

```bash
g++ -std=c++17 -Iinclude -Ibuild docs/examples/cpp/basic.cpp \
  -Wl,--whole-archive build/libtiktoken.a -Wl,--no-whole-archive \
  -lpthread -ldl -lm -o basic_cpp
```

Dynamic linking: [linking.md](../../linking.md).

## When to use C vs C++

| API | When |
|-----|------|
| `tiktoken.h` | Pure C projects, FFI from other languages |
| `tiktoken.hpp` | C++ projects - less memory boilerplate |
