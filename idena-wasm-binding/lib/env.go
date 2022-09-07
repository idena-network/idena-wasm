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
	MinFeePerGas(meter *GasMeter) *big.Int
	Balance(meter *GasMeter, address Address) *big.Int
	BlockSeed(meter *GasMeter) []byte
	NetworkSize(meter *GasMeter) uint64
	IdentityState(meter *GasMeter, address Address) byte
	Identity(meter *GasMeter, address Address) []byte
	CreateSubEnv(contract Address, payAmount *big.Int, isDeploy bool) (HostEnv, error)
	GetCode(addr Address) []byte
	Commit()
	Clear()
	Caller(meter *GasMeter) Address
	OriginalCaller(meter *GasMeter) Address
	SubBalance(meter *GasMeter, amount *big.Int) error
	AddBalance(meter *GasMeter, address Address, bytes *big.Int)
	ContractAddress(meter *GasMeter) Address
	ContractCode(meter *GasMeter, addr Address) []byte
	ContractAddr(meter *GasMeter, code []byte, args []byte, nonce []byte) Address
	Deploy(code []byte)
	ContractAddrByHash(meter *GasMeter, hash []byte, args []byte, nonce []byte) Address
	OwnCode(meter *GasMeter) []byte
	CodeHash(meter *GasMeter) []byte
	Event(meter *GasMeter, name string, args ...[]byte)
	ReadContractData(meter *GasMeter, address Address, key []byte) []byte
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

func (g *GasMeter) ConsumeGas(gas uint64) {
	g.gasConsumed += gas
	if g.gasLimit >= 0 && g.gasLimit < g.gasConsumed {
		panic(OutOfGas{})
	}
}
