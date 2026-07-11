/*
 * C++ example using the RAII wrapper (tiktoken.hpp).
 */
#include "tiktoken.hpp"

#include <iostream>
#include <string>

int main() {
    try {
        tiktoken::Encoding enc("gpt2");

        const std::string text = "hello world";
        auto tokens = enc.encode_ordinary(text);

        std::cout << "tokens (" << tokens.size() << "):";
        for (uint32_t t : tokens) {
            std::cout << " " << t;
        }
        std::cout << "\n";

        std::string decoded = enc.decode(tokens);
        std::cout << "decoded: " << decoded << "\n";

        /* cl100k to compare the same text */
        tiktoken::Encoding cl100k("cl100k_base");
        auto cl_tokens = cl100k.encode_ordinary(text);
        std::cout << "cl100k tokens (" << cl_tokens.size() << "):";
        for (uint32_t t : cl_tokens) {
            std::cout << " " << t;
        }
        std::cout << "\n";

        auto names = tiktoken::list_encoding_names();
        std::cout << "available encodings: " << names.size() << "\n";

    } catch (const tiktoken::Error& e) {
        std::cerr << "error: " << e.what() << "\n";
        return 1;
    }

    return 0;
}
