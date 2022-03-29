package main

import "idena-wasm-binding/lib"

func main() {
	a, _ := lib.Testdata1()
	lib.Execute(a)
}
