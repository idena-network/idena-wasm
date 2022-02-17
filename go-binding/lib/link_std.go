//go:build windows

package lib

// #cgo LDFLAGS: -L${SRCDIR} -lidena_wasm
// #cgo LDFLAGS: -lws2_32 -lbcrypt -luserenv
import "C"
