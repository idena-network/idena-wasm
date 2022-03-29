package lib

type OutOfGas struct {

}

func (o OutOfGas) Error() string {
	panic("out of gas")
}

