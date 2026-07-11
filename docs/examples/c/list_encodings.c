/*
 * Lists built-in encodings and prints the library version.
 */
#include "tiktoken.h"

#include <stdio.h>
#include <stdlib.h>

int main(void) {
    printf("tiktoken version: %s\n", tiktoken_version());

    size_t count = 0;
    char** names = tiktoken_list_encoding_names(&count);
    if (!names) {
        fprintf(stderr, "failed to list encodings\n");
        return EXIT_FAILURE;
    }

    printf("encodings (%zu):\n", count);
    for (size_t i = 0; i < count; ++i) {
        printf("  - %s\n", names[i]);
    }

    strings_array_free(names, count);
    return EXIT_SUCCESS;
}
