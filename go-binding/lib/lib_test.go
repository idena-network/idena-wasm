package lib

import "testing"

func TestExecute(t *testing.T) {
	code, _ := Testdata1()
	Execute(code)
}