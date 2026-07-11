#pragma once

#include "tiktoken.h"

#include <cstddef>
#include <cstdint>
#include <memory>
#include <stdexcept>
#include <string>
#include <vector>

namespace tiktoken {

class Error : public std::runtime_error {
public:
    explicit Error(const std::string& message, bool is_key_error = false)
        : std::runtime_error(message), is_key_error_(is_key_error) {}

    bool is_key_error() const { return is_key_error_; }

private:
    bool is_key_error_;
};

class Encoding {
public:
    Encoding() : handle_(nullptr) {}

    explicit Encoding(const char* name) {
        CoreBPEError err{};
        handle_ = tiktoken_get_encoding(name, &err);
        if (!handle_) {
            throw Error(err.message ? err.message : "failed to load encoding", err.is_key_error);
        }
        corebpe_error_free(&err);
    }

    ~Encoding() {
        if (handle_) {
            corebpe_free(handle_);
        }
    }

    Encoding(const Encoding&) = delete;
    Encoding& operator=(const Encoding&) = delete;

    Encoding(Encoding&& other) noexcept : handle_(other.handle_) {
        other.handle_ = nullptr;
    }

    Encoding& operator=(Encoding&& other) noexcept {
        if (this != &other) {
            if (handle_) {
                corebpe_free(handle_);
            }
            handle_ = other.handle_;
            other.handle_ = nullptr;
        }
        return *this;
    }

    std::vector<uint32_t> encode_ordinary(const std::string& text) const {
        EncodeResult result = corebpe_encode_ordinary(handle_, text.c_str());
        std::vector<uint32_t> tokens(result.tokens, result.tokens + result.len);
        encode_result_free(result);
        return tokens;
    }

    std::vector<uint32_t> encode_with_special_tokens(const std::string& text) const {
        EncodeResult result = corebpe_encode_with_special_tokens(handle_, text.c_str());
        std::vector<uint32_t> tokens(result.tokens, result.tokens + result.len);
        encode_result_free(result);
        return tokens;
    }

    std::vector<uint32_t> encode(
        const std::string& text,
        const std::vector<std::string>& allowed_special = {}
    ) const {
        std::vector<const char*> c_allowed;
        c_allowed.reserve(allowed_special.size());
        for (const auto& token : allowed_special) {
            c_allowed.push_back(token.c_str());
        }

        CoreBPEError err{};
        EncodeResult result = corebpe_encode(
            handle_,
            text.c_str(),
            c_allowed.empty() ? nullptr : c_allowed.data(),
            c_allowed.size(),
            &err
        );
        if (!result.tokens) {
            throw Error(err.message ? err.message : "encode failed", err.is_key_error);
        }
        std::vector<uint32_t> tokens(result.tokens, result.tokens + result.len);
        encode_result_free(result);
        corebpe_error_free(&err);
        return tokens;
    }

    std::string decode(const std::vector<uint32_t>& tokens) const {
        CoreBPEError err{};
        char* text = corebpe_decode(
            handle_,
            tokens.data(),
            tokens.size(),
            &err
        );
        if (!text) {
            throw Error(err.message ? err.message : "decode failed", err.is_key_error);
        }
        std::string out(text);
        tiktoken_free_string(text);
        corebpe_error_free(&err);
        return out;
    }

    std::vector<uint8_t> decode_bytes(const std::vector<uint32_t>& tokens) const {
        CoreBPEError err{};
        size_t len = 0;
        uint8_t* bytes = corebpe_decode_bytes(
            handle_,
            tokens.data(),
            tokens.size(),
            &len,
            &err
        );
        if (!bytes) {
            throw Error(err.message ? err.message : "decode_bytes failed", err.is_key_error);
        }
        std::vector<uint8_t> out(bytes, bytes + len);
        tiktoken_free_bytes(bytes, len);
        corebpe_error_free(&err);
        return out;
    }

    CoreBPE* raw() const { return handle_; }

private:
    CoreBPE* handle_;
};

inline std::vector<std::string> list_encoding_names() {
    size_t count = 0;
    char** names = tiktoken_list_encoding_names(&count);
    std::vector<std::string> out;
    out.reserve(count);
    for (size_t i = 0; i < count; ++i) {
        out.emplace_back(names[i]);
    }
    strings_array_free(names, count);
    return out;
}

}  // namespace tiktoken
