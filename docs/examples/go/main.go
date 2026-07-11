// Basic example: load GPT-2 via CGO, tokenize "hello world", and decode.
//
// Run from the repository root (after scons):
//
//	cd docs/examples/go
//	CGO_ENABLED=1 go run .
package main

/*
#cgo linux CFLAGS: -I${SRCDIR}/../../../include -I${SRCDIR}/../../../build
#cgo linux LDFLAGS: -L${SRCDIR}/../../../build -Wl,--whole-archive -ltiktoken -Wl,--no-whole-archive -lpthread -ldl -lm

#cgo darwin CFLAGS: -I${SRCDIR}/../../../include -I${SRCDIR}/../../../build
#cgo darwin LDFLAGS: -L${SRCDIR}/../../../build -Wl,-force_load,${SRCDIR}/../../../build/libtiktoken.a -lpthread -ldl -lm

#cgo windows CFLAGS: -I${SRCDIR}/../../../include -I${SRCDIR}/../../../build
#cgo windows LDFLAGS: -L${SRCDIR}/../../../build tiktoken.lib ws2_32.lib userenv.lib bcrypt.lib advapi32.lib ntdll.lib

#include "tiktoken.h"
#include <stdlib.h>
*/
import "C"

import (
	"fmt"
	"os"
	"unsafe"
)

func main() {
	if err := run(); err != nil {
		fmt.Fprintf(os.Stderr, "%v\n", err)
		os.Exit(1)
	}
}

func run() error {
	name := C.CString("gpt2")
	defer C.free(unsafe.Pointer(name))

	var cErr C.CoreBPEError
	enc := C.tiktoken_get_encoding(name, &cErr)
	if enc == nil {
		return loadError(&cErr)
	}
	defer C.corebpe_free(enc)

	text := C.CString("hello world")
	defer C.free(unsafe.Pointer(text))

	encoded := C.corebpe_encode_ordinary(enc, text)
	defer C.encode_result_free(encoded)

	if encoded.tokens == nil || encoded.len == 0 {
		return fmt.Errorf("encode failed")
	}

	tokens := unsafe.Slice(encoded.tokens, encoded.len)
	fmt.Printf("tokens (%d):", len(tokens))
	for _, t := range tokens {
		fmt.Printf(" %d", t)
	}
	fmt.Println()

	decoded, err := decode(enc, tokens)
	if err != nil {
		return err
	}
	defer C.tiktoken_free_string(decoded)

	fmt.Printf("decoded: %s\n", C.GoString(decoded))
	return nil
}

func loadError(cErr *C.CoreBPEError) error {
	defer C.corebpe_error_free(cErr)
	if cErr.message != nil {
		return fmt.Errorf("failed to load gpt2: %s", C.GoString(cErr.message))
	}
	return fmt.Errorf("failed to load gpt2: (no message)")
}

func decode(enc *C.CoreBPE, tokens []C.uint32_t) (*C.char, error) {
	var cErr C.CoreBPEError
	out := C.corebpe_decode(enc, &tokens[0], C.size_t(len(tokens)), &cErr)
	if out == nil {
		defer C.corebpe_error_free(&cErr)
		if cErr.message != nil {
			return nil, fmt.Errorf("decode failed: %s", C.GoString(cErr.message))
		}
		return nil, fmt.Errorf("decode failed")
	}
	return out, nil
}
