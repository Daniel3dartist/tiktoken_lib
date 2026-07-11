#include "tiktoken.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static int expect_encode(const char* label, CoreBPE* enc, const char* text,
                         const uint32_t* expected, size_t expected_len) {
    EncodeResult result = corebpe_encode_ordinary(enc, text);
    if (result.len != expected_len) {
        fprintf(stderr, "%s: expected %zu tokens, got %zu\n", label, expected_len, result.len);
        encode_result_free(result);
        return 1;
    }
    for (size_t i = 0; i < expected_len; ++i) {
        if (result.tokens[i] != expected[i]) {
            fprintf(stderr, "%s: token[%zu] expected %u, got %u\n",
                    label, i, expected[i], result.tokens[i]);
            encode_result_free(result);
            return 1;
        }
    }
    encode_result_free(result);
    return 0;
}

static int expect_decode(const char* label, CoreBPE* enc, const uint32_t* tokens,
                         size_t tokens_len, const char* expected) {
    CoreBPEError err = {0};
    char* text = corebpe_decode(enc, tokens, tokens_len, &err);
    if (!text) {
        fprintf(stderr, "%s: decode failed: %s\n", label, err.message ? err.message : "unknown");
        corebpe_error_free(&err);
        return 1;
    }
    int ok = strcmp(text, expected) == 0;
    if (!ok) {
        fprintf(stderr, "%s: expected %s, got %s\n", label, expected, text);
    }
    tiktoken_free_string(text);
    corebpe_error_free(&err);
    return ok ? 0 : 1;
}

int main(void) {
    CoreBPEError err = {0};
    CoreBPE* enc = tiktoken_get_encoding("gpt2", &err);
    if (!enc) {
        fprintf(stderr, "failed to load gpt2 encoding: %s\n",
                err.message ? err.message : "unknown");
        corebpe_error_free(&err);
        return 1;
    }
    corebpe_error_free(&err);

    const uint32_t hello_world[] = {31373, 995};
    if (expect_encode("gpt2 hello world", enc, "hello world", hello_world, 2) != 0) {
        corebpe_free(enc);
        return 1;
    }

    if (expect_decode("gpt2 hello world decode", enc, hello_world, 2, "hello world") != 0) {
        corebpe_free(enc);
        return 1;
    }

    const char* allowed[] = {"<|endoftext|>"};
    const uint32_t hello_eot[] = {31373, 220, 50256};
    EncodeResult eot_result = corebpe_encode(
        enc,
        "hello <|endoftext|>",
        allowed,
        1,
        &err
    );
    if (!eot_result.tokens || eot_result.len != 3) {
        fprintf(stderr, "gpt2 eot encode failed\n");
        encode_result_free(eot_result);
        corebpe_free(enc);
        return 1;
    }
    for (size_t i = 0; i < 3; ++i) {
        if (eot_result.tokens[i] != hello_eot[i]) {
            fprintf(stderr, "gpt2 eot token[%zu] mismatch\n", i);
            encode_result_free(eot_result);
            corebpe_free(enc);
            return 1;
        }
    }
    encode_result_free(eot_result);

    corebpe_free(enc);
    printf("All GPT-2 compatibility tests passed.\n");
    return 0;
}
