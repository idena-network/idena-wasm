package lib

// #include <stdlib.h>
// #include "bindings.h"
import "C"
import (
	"errors"
	"fmt"
	"github.com/golang/protobuf/proto"
	models "github.com/idena-network/idena-wasm-binding/lib/protobuf"
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

const ArgsPlainFormat = 0x0
const ArgsProtobufFormat = 0x1

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

func errorWithMessage(err int, b C.UnmanagedVector) error {
	// this checks for out of gas as a special case
	if err == 2 {
		return errors.New("out of gas")
	}
	msg := copyAndDestroyUnmanagedVector(b)
	if msg == nil {
		return errors.New("error without description")
	}
	return fmt.Errorf("%s", string(msg))
}

func executeInternal(api *GoAPI, code []byte, method []byte, args []byte, gasLimit uint64) (uint64, error) {
	return execute(api, code, makeView(method), args, gasLimit)
}

func Execute(api *GoAPI, code []byte, method string, args [][]byte, gasLimit uint64) (uint64, error) {

	argsMsg := models.ProtoCallContractArgs{
		Args: args,
	}
	argsBytes, err := proto.Marshal(&argsMsg)
	if err != nil {
		return 0, err
	}
	return execute(api, code, makeView([]byte(method)), append([]byte{ArgsProtobufFormat}, argsBytes...), gasLimit)
}

func Deploy(api *GoAPI, code []byte, args [][]byte, gasLimit uint64) (uint64, error) {
	argsMsg := models.ProtoCallContractArgs{
		Args: args,
	}
	argsBytes, err := proto.Marshal(&argsMsg)
	if err != nil {
		return 0, err
	}
	return deploy(api, code, append([]byte{ArgsProtobufFormat}, argsBytes...), gasLimit)
}

func execute(api *GoAPI, code []byte, method C.ByteSliceView, args []byte, gasLimit uint64) (uint64, error) {
	errmsg := newUnmanagedVector(nil)
	var gasUsed cu64
	errno := C.execute(buildAPI(api), makeView(code), method, makeView(args), cu64(gasLimit), &gasUsed, &errmsg)
	if errno == 0 {
		return uint64(gasUsed), nil
	}
	return uint64(gasUsed), errorWithMessage(int(errno), errmsg)
}

func deploy(api *GoAPI, code []byte, args []byte, gasLimit uint64) (uint64, error) {
	errmsg := newUnmanagedVector(nil)
	var gasUsed cu64
	errno := C.deploy(buildAPI(api), makeView(code), makeView(args), cu64(gasLimit), &gasUsed, &errmsg)
	if errno == 0 {
		return uint64(gasUsed), nil
	}
	return uint64(gasUsed), errorWithMessage(int(errno), errmsg)
}


