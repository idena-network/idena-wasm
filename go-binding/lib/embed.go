package lib

import "embed"

//go:embed testdata/optimized.wasm
var content embed.FS


func Testdata1() ([]byte,error) {
	return content.ReadFile("testdata/optimized.wasm")
}