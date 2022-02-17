package lib

import (
	"fmt"
	"math/big"
)

type Address = [20]byte

func newAddress(data []byte) Address {
	res := Address{}
	copy(res[:], data[:20])
	return res
}

type HostEnv interface {
	SetStorage(*GasMeter, []byte, []byte)
	GetStorage(*GasMeter, []byte) []byte
	RemoveStorage(*GasMeter, []byte)
	BlockNumber(*GasMeter) uint64
	BlockTimestamp(*GasMeter) int64
	Send(*GasMeter, Address, *big.Int) error
	MinFeePerGas() *big.Int
	Balance(address Address) *big.Int
	BlockSeed() []byte
	NetworkSize(meter *GasMeter) uint64
	IdentityState(meter *GasMeter, address Address) byte
}
type GasMeter struct {
	gasLimit    uint64
	gasConsumed uint64
}

func (g *GasMeter) GasConsumed() uint64 {
	return g.gasConsumed
}

func (g *GasMeter) SetRemainingGas(newLimit uint64) {
	g.gasLimit = newLimit
	println(fmt.Sprintf("set new gas limit %v", newLimit))
}
