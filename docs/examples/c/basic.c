/*
 * Minimal example: load GPT-2, tokenize "hello world", and decode.
 *
 * Expected output:
 *   tokens: 31373 995
 *   decoded: hello world
 */
#include "tiktoken.h"

#include <stdio.h>
#include <stdlib.h>

int main(void) {
    CoreBPEError err = {0};

    CoreBPE* enc = tiktoken_get_encoding("gpt2", &err);
    if (!enc) {
        fprintf(stderr, "Failed to load gpt2: %s\n",
                err.message ? err.message : "(no message)");
        corebpe_error_free(&err);
        return EXIT_FAILURE;
    }

    const char* text = "hello world";
    EncodeResult encoded = corebpe_encode_ordinary(enc, text);

    if (!encoded.tokens || encoded.len == 0) {
        fprintf(stderr, "encode failed\n");
        corebpe_free(enc);
        return EXIT_FAILURE;
    }

    printf("tokens (%zu):", encoded.len);
    for (size_t i = 0; i < encoded.len; ++i) {
        printf(" %u", encoded.tokens[i]);
    }
    printf("\n");

    char* decoded = corebpe_decode(enc, encoded.tokens, encoded.len, &err);
    if (!decoded) {
        fprintf(stderr, "decode failed: %s\n", err.message ? err.message : "?");
        encode_result_free(encoded);
        corebpe_free(enc);
        corebpe_error_free(&err);
        return EXIT_FAILURE;
    }

    printf("decoded: %s\n", decoded);

    tiktoken_free_string(decoded);
    encode_result_free(encoded);
    corebpe_free(enc);
    corebpe_error_free(&err);

    return EXIT_SUCCESS;
}
