package lib

/*
#include "bindings.h"


typedef GoResult (*set_storage_fn)(api_t *ptr, U8SliceView key,  U8SliceView value,  uint64_t *used_gas);

GoResult cset_storage_cgo(api_t *ptr, U8SliceView key,  U8SliceView value,  uint64_t *used_gas);

*/
import "C"
import (
	"unsafe"
)

type SetStorage func([]byte, []byte) int

type GoAPI struct {
	SetStorage SetStorage
}

var api_vtable = C.GoApi_vtable{
	set_storage: (C.set_storage_fn)(C.cset_storage_cgo),
}

// contract: original pointer/struct referenced must live longer than C.GoApi struct
// since this is only used internally, we can verify the code that this is the case
func buildAPI(api *GoAPI) C.GoApi {
	return C.GoApi{
		state:  (*C.api_t)(unsafe.Pointer(api)),
		vtable: api_vtable,
	}
}


//export cset_storage
func cset_storage(ptr *C.api_t, key C.U8SliceView, value C.U8SliceView, gasUsed cu64) (ret C.GoResult) {
	api := (*GoAPI)(unsafe.Pointer(ptr))
	k := copyU8Slice(key)
	v := copyU8Slice(value)
	api.SetStorage(k, v)
	return C.GoResult_Ok
}
