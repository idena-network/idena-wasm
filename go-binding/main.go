package main

import "go-binding/lib"

func main() {
	a, _ := lib.Testdata1()
	lib.Execute(a)
}
