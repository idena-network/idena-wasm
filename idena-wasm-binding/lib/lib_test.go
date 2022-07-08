package lib

import (
	"fmt"
	"github.com/stretchr/testify/require"
	db "github.com/tendermint/tm-db"
	"math"
	"math/big"
	"testing"
)

type HostEnvMock struct {
	db db.DB
}

func (e *HostEnvMock) ContractCode(meter *GasMeter, addr Address) []byte {
	panic("implement me")
}

func (e *HostEnvMock) ContractAddress(meter *GasMeter) Address {
	panic("implement me")
}

func (e *HostEnvMock) AddBalance(meter *GasMeter, address Address, bytes *big.Int) {
	panic("implement me")
}

func (e *HostEnvMock) SubBalance(meter *GasMeter, amount *big.Int) error {
	panic("implement me")
}

func (e *HostEnvMock) Revert() {
	panic("implement me")
}

func (e *HostEnvMock) Caller(meter *GasMeter) Address {
	panic("implement me")
}

func (e *HostEnvMock) OriginalCaller(meter *GasMeter) Address {
	panic("implement me")
}

func (e *HostEnvMock) CreateSubEnv(Address, *big.Int) (*HostEnvMock, error) {
	return &HostEnvMock{
		db: e.db,
	}, nil
}

func (e *HostEnvMock) Commit() {
}

func (e *HostEnvMock) GetCode(addr Address) []byte {
	code, _ := Testdata1()
	return code
}

func (e *HostEnvMock) Identity(meter *GasMeter, address Address) []byte {
	panic("implement me")
}

func (e *HostEnvMock) IdentityState(meter *GasMeter, address Address) byte {
	return 3
}

func (e *HostEnvMock) NetworkSize(meter *GasMeter) uint64 {
	return 117
}

func (e *HostEnvMock) BlockSeed(meter *GasMeter) []byte {
	return []byte{1, 2, 3, 0x0a}
}

func (e *HostEnvMock) Balance(address Address) *big.Int {
	return big.NewInt(15)
}

func (e *HostEnvMock) MinFeePerGas(meter *GasMeter) *big.Int {
	return big.NewInt(10)
}

func (e *HostEnvMock) Send(meter *GasMeter, address Address, amount *big.Int) error {
	println(fmt.Sprintf("send amount %v", amount.String()))
	meter.gasConsumed += 20
	return nil
}

func (e *HostEnvMock) BlockNumber(meter *GasMeter) uint64 {
	meter.gasConsumed += 1
	return math.MaxUint64 - 1
}

func (e *HostEnvMock) BlockTimestamp(meter *GasMeter) int64 {
	meter.gasConsumed += 1
	return 121020131
}

func (e *HostEnvMock) SetStorage(meter *GasMeter, key []byte, data []byte) {
	meter.gasConsumed += uint64(len(key))
	e.db.Set(key, data)
}

func (e *HostEnvMock) GetStorage(meter *GasMeter, key []byte) []byte {
	data, _ := e.db.Get(key)
	meter.gasConsumed += uint64(len(data))
	return data
}

func (e *HostEnvMock) RemoveStorage(meter *GasMeter, key []byte) {
	meter.gasConsumed += uint64(len(key))
	e.db.Delete(key)
}

func NewMockAPI() *GoAPI {
	meter := GasMeter{}
	return &GoAPI{
		&HostEnvMock{
			db: db.NewMemDB(),
		},
		&meter,
	}
}

func TestExecute(t *testing.T) {
	code, _ := Testdata1()
	gas, err := Execute(NewMockAPI(), code, "inc", [][]byte{{0}}, 100000000)
	require.NoError(t, err)
	require.True(t, gas > 0)
	t.Logf("gas used=%v", gas)
}
