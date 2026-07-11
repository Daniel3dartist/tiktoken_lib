/*
 * Special tokens: <|endoftext|> as a single token when allowed.
 *
 * Expected output:
 *   tokens: 31373 220 50256
 */
#include "tiktoken.h"

#include <stdio.h>
#include <stdlib.h>

int main(void) {
    CoreBPEError err = {0};
    CoreBPE* enc = tiktoken_get_encoding("gpt2", &err);
    if (!enc) {
        fprintf(stderr, "%s\n", err.message ? err.message : "load failed");
        corebpe_error_free(&err);
        return EXIT_FAILURE;
    }

    const char* text = "hello <|endoftext|>";
    const char* allowed[] = {"<|endoftext|>"};

    EncodeResult encoded = corebpe_encode(
        enc,
        text,
        allowed,
        1,
        &err
    );

    if (!encoded.tokens) {
        fprintf(stderr, "encode failed: %s\n", err.message ? err.message : "?");
        corebpe_free(enc);
        corebpe_error_free(&err);
        return EXIT_FAILURE;
    }

    printf("tokens (%zu):", encoded.len);
    for (size_t i = 0; i < encoded.len; ++i) {
        printf(" %u", encoded.tokens[i]);
    }
    printf("\n");

    /* Equivalent to allowed_special="all" in Python: */
    EncodeResult all_special = corebpe_encode_with_special_tokens(enc, text);
    printf("encode_with_special_tokens (%zu):", all_special.len);
    for (size_t i = 0; i < all_special.len; ++i) {
        printf(" %u", all_special.tokens[i]);
    }
    printf("\n");

    encode_result_free(all_special);
    encode_result_free(encoded);
    corebpe_free(enc);
    corebpe_error_free(&err);

    return EXIT_SUCCESS;
}
