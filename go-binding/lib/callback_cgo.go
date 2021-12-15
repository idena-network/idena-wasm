package lib

/*
#include "bindings.h"
#include <stdio.h>

GoResult cset_storage(api_t *ptr, U8SliceView key, U8SliceView value, uint64_t *used_gas);

GoResult cset_storage_cgo(api_t *ptr, U8SliceView key, U8SliceView value,  uint64_t *used_gas) {
    return cset_storage(ptr, key, value, used_gas);
}
*/
import "C"

// We need these gateway functions to allow calling back to a go function from the c code.
// At least I didn't discover a cleaner way.
// Also, this needs to be in a different file than `callbacks.go`, as we cannot create functions
// in the same file that has //export directives. Only import header types
