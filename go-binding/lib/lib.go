package lib

// #include <stdlib.h>
// #include "bindings.h"
import "C"

// Value types
type cint = C.int
type cbool = C.bool
type cusize = C.size_t
type cu8 = C.uint8_t
type cu32 = C.uint32_t
type cu64 = C.uint64_t
type ci8 = C.int8_t
type ci32 = C.int32_t
type ci64 = C.int64_t

// Pointers
type cu8_ptr = *C.uint8_t

func NewMockAPI() *GoAPI {
	return &GoAPI{
		SetStorage: func(bytes []byte, bytes2 []byte) int {
			println(bytes)
			return 0
		},
	}
}

func Execute(code []byte) {
	api := NewMockAPI()
	res := C.execute(buildAPI(api), makeView(code))
	println(res)
}
