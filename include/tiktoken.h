#ifndef TIKTOKEN_H
#define TIKTOKEN_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct CoreBPE CoreBPE;

typedef struct {
    char* message;
    bool is_key_error;
} CoreBPEError;

typedef struct {
    uint32_t* tokens;
    size_t len;
    size_t last_piece_token_len;
} EncodeResult;

typedef struct {
    const uint8_t** encoder_keys;
    const size_t* encoder_key_lengths;
    const uint32_t* encoder_values;
    size_t encoder_len;
    const char** special_tokens_keys;
    const uint32_t* special_tokens_values;
    size_t special_tokens_len;
    const char* pattern;
} CoreBPEConfig;

typedef struct {
    uint32_t* tokens;
    size_t tokens_len;
    uint32_t** completions;
    size_t* completions_lens;
    size_t completions_count;
} UnstableEncodeResult;

/* Built-in encodings (gpt2, r50k_base, cl100k_base, ...) */
const char* tiktoken_version(void);
char** tiktoken_list_encoding_names(size_t* out_len);
CoreBPE* tiktoken_get_encoding(const char* name, CoreBPEError* error);

CoreBPE* corebpe_new(const CoreBPEConfig* config, CoreBPEError* error);
void corebpe_free(CoreBPE* bpe);

EncodeResult corebpe_encode_ordinary(CoreBPE* bpe, const char* text);
EncodeResult corebpe_encode(
    CoreBPE* bpe,
    const char* text,
    const char** allowed_special,
    size_t allowed_special_len,
    CoreBPEError* error
);
EncodeResult corebpe_encode_with_special_tokens(CoreBPE* bpe, const char* text);
UnstableEncodeResult corebpe_encode_unstable_native(
    CoreBPE* bpe,
    const char* text,
    const char** allowed_special,
    size_t allowed_special_len
);

uint8_t* corebpe_decode_bytes(
    CoreBPE* bpe,
    const uint32_t* tokens,
    size_t tokens_len,
    size_t* out_len,
    CoreBPEError* error
);
char* corebpe_decode(
    CoreBPE* bpe,
    const uint32_t* tokens,
    size_t tokens_len,
    CoreBPEError* error
);

char** corebpe_special_tokens(CoreBPE* bpe, size_t* out_len);

void encode_result_free(EncodeResult result);
void unstable_encode_result_free(UnstableEncodeResult result);
void strings_array_free(char** array, size_t len);
void corebpe_error_free(CoreBPEError* error);

void tiktoken_free_bytes(uint8_t* ptr, size_t len);
void tiktoken_free_string(char* ptr);
void tiktoken_free_u32_array(uint32_t* ptr, size_t len);
void tiktoken_free_usize_array(size_t* ptr, size_t len);

size_t* byte_pair_merge(
    const uint8_t** ranks_keys,
    const size_t* ranks_key_lengths,
    const uint32_t* ranks_values,
    size_t ranks_len,
    const uint8_t* piece,
    size_t piece_len,
    size_t* out_len
);
uint32_t* byte_pair_encode(
    const uint8_t** ranks_keys,
    const size_t* ranks_key_lengths,
    const uint32_t* ranks_values,
    size_t ranks_len,
    const uint8_t* piece,
    size_t piece_len,
    size_t* out_len
);
uint8_t** byte_pair_split(
    const uint8_t** ranks_keys,
    const size_t* ranks_key_lengths,
    const uint32_t* ranks_values,
    size_t ranks_len,
    const uint8_t* piece,
    size_t piece_len,
    size_t* out_len,
    size_t** out_sublens
);
void byte_pair_split_free(uint8_t** pieces, size_t* piece_lens, size_t count);

#ifdef __cplusplus
}
#endif

#endif /* TIKTOKEN_H */
