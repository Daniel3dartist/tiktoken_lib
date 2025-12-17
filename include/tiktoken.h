#ifndef COREBPE_H
#define COREBPE_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// Opaque handle para CoreBPE
typedef struct CoreBPE CoreBPE;

// Tipos de erro
typedef struct {
    char* message;
    bool is_key_error; // se for um erro de chave (token inválido) ou outro tipo
} CoreBPEError;

// Resultado da codificação
typedef struct {
    uint32_t* tokens;
    size_t len;
    size_t last_piece_token_len;
} EncodeResult;

// Configuração para criação do CoreBPE
typedef struct {
    // Encoder regular: pares de (bytes, rank)
    const uint8_t** encoder_keys;
    const size_t* encoder_key_lengths;
    const uint32_t* encoder_values;
    size_t encoder_len;
    
    // Tokens especiais: pares de (string, rank)
    const char** special_tokens_keys;
    const uint32_t* special_tokens_values;
    size_t special_tokens_len;
    
    // Padrão regex para tokenização
    const char* pattern;
} CoreBPEConfig;

// Cria uma nova instância de CoreBPE
CoreBPE* corebpe_new(const CoreBPEConfig* config, CoreBPEError* error);

// Libera uma instância de CoreBPE
void corebpe_free(CoreBPE* bpe);

// Codificação ordinária (sem tokens especiais)
EncodeResult corebpe_encode_ordinary(CoreBPE* bpe, const char* text);

// Codificação com tokens especiais
EncodeResult corebpe_encode(
    CoreBPE* bpe,
    const char* text,
    const char** allowed_special,
    size_t allowed_special_len,
    CoreBPEError* error
);

// Codificação nativa instável (retorna tokens e conjunto de completions)
typedef struct {
    uint32_t* tokens;
    size_t tokens_len;
    uint32_t** completions; // array de arrays
    size_t* completions_lens; // comprimento de cada completion
    size_t completions_count;
} UnstableEncodeResult;

UnstableEncodeResult corebpe_encode_unstable_native(
    CoreBPE* bpe,
    const char* text,
    const char** allowed_special,
    size_t allowed_special_len
);

// Decodificação: tokens -> bytes
uint8_t* corebpe_decode_bytes(
    CoreBPE* bpe,
    const uint32_t* tokens,
    size_t tokens_len,
    size_t* out_len,
    CoreBPEError* error
);

// Decodificação para string (assume UTF-8 válido)
char* corebpe_decode(
    CoreBPE* bpe,
    const uint32_t* tokens,
    size_t tokens_len,
    CoreBPEError* error
);

// Codificação com todos os tokens especiais permitidos
EncodeResult corebpe_encode_with_special_tokens(CoreBPE* bpe, const char* text);

// Obtém a lista de tokens especiais
char** corebpe_special_tokens(CoreBPE* bpe, size_t* out_len);

// Funções auxiliares para manipulação de resultados

// Libera resultado de codificação
void encode_result_free(EncodeResult result);

// Libera resultado de codificação instável
void unstable_encode_result_free(UnstableEncodeResult result);

// Libera array de strings
void strings_array_free(char** array, size_t len);

// Libera erro
void corebpe_error_free(CoreBPEError* error);

// Funções de BPE de baixo nível (úteis para testes/debug)

// Aplica merge de byte pair
size_t* byte_pair_merge(
    const uint8_t** ranks_keys,
    const size_t* ranks_key_lengths,
    const uint32_t* ranks_values,
    size_t ranks_len,
    const uint8_t* piece,
    size_t piece_len,
    size_t* out_len
);

// Codificação byte pair
uint32_t* byte_pair_encode(
    const uint8_t** ranks_keys,
    const size_t* ranks_key_lengths,
    const uint32_t* ranks_values,
    size_t ranks_len,
    const uint8_t* piece,
    size_t piece_len,
    size_t* out_len
);

// Divisão byte pair
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

#ifdef __cplusplus
}
#endif

#endif // COREBPE_H