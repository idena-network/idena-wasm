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

func (r *HostEnvMock) Identity(meter *GasMeter, address Address) []byte {
	panic("implement me")
}

func (r *HostEnvMock) IdentityState(meter *GasMeter, address Address) byte {
	return 3
}

func (r *HostEnvMock) NetworkSize(meter *GasMeter) uint64 {
	return 117
}

func (h *HostEnvMock) BlockSeed() []byte {
	return []byte{1, 2, 3, 0x0a}
}

func (h *HostEnvMock) Balance(address Address) *big.Int {
	return big.NewInt(15)
}

func (h *HostEnvMock) MinFeePerGas() *big.Int {
	return big.NewInt(10)
}

func (h *HostEnvMock) Send(meter *GasMeter, address Address, amount *big.Int) error {
	println(fmt.Sprintf("send amount %v", amount.String()))
	meter.gasConsumed += 20
	return nil
}

func (h *HostEnvMock) BlockNumber(meter *GasMeter) uint64 {
	meter.gasConsumed += 1
	return math.MaxUint64 - 1
}

func (h *HostEnvMock) BlockTimestamp(meter *GasMeter) int64 {
	meter.gasConsumed += 1
	return 121020131
}

func (h *HostEnvMock) SetStorage(meter *GasMeter, key []byte, data []byte) {
	meter.gasConsumed += uint64(len(key))
	h.db.Set(key, data)
}

func (h *HostEnvMock) GetStorage(meter *GasMeter, key []byte) []byte {
	data, _ := h.db.Get(key)
	meter.gasConsumed += uint64(len(data))
	return data
}

func (h *HostEnvMock) RemoveStorage(meter *GasMeter, key []byte) {
	meter.gasConsumed += uint64(len(key))
	h.db.Delete(key)
}

func NewMockAPI() *GoAPI {
	meter := GasMeter{}
	return &GoAPI{
		host: &HostEnvMock{
			db: db.NewMemDB(),
		},
		gasMeter: &meter,
	}
}

func TestExecute(t *testing.T) {
	code, _ := Testdata1()
	gas, err := Execute(NewMockAPI(), code, "main", [][]byte{nil, {0x12, 0x13}}, 10000000)
	require.NoError(t, err)
	require.True(t, gas > 0)
}
