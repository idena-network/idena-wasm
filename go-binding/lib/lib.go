package lib

// #include <stdlib.h>
// #include "bindings.h"
import "C"
import (
	"errors"
	"fmt"
	"syscall"
	"unsafe"
)

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

func copyAndDestroyUnmanagedVector(v C.UnmanagedVector) []byte {
	var out []byte
	if v.is_none {
		out = nil
	} else if v.cap == cusize(0) {
		// There is no allocation we can copy
		out = []byte{}
	} else {
		// C.GoBytes create a copy (https://stackoverflow.com/a/40950744/2013738)
		out = C.GoBytes(unsafe.Pointer(v.ptr), cint(v.len))
	}
	//C.destroy_unmanaged_vector(v)
	return out
}

func errorWithMessage(err error, b C.UnmanagedVector) error {
	// this checks for out of gas as a special case
	if errno, ok := err.(syscall.Errno); ok && int(errno) == 2 {
		return errors.New("out of gas")
	}
	msg := copyAndDestroyUnmanagedVector(b)
	if msg == nil {
		return err
	}
	return fmt.Errorf("%s", string(msg))
}

func Execute(code []byte, api *GoAPI) error {
	errmsg := newUnmanagedVector(nil)
	err, winerr  := C.execute(buildAPI(api), makeView(code), &errmsg)
	if err == 0 {
		return nil
	}
	return errorWithMessage(winerr, errmsg)
}
