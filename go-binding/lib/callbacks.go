package lib

/*
#include "bindings.h"


typedef GoResult (*set_remaining_gas_fn)(api_t *ptr, uint64_t remaining_gas);
GoResult cset_remaining_gas(api_t *ptr, uint64_t remaining_gas);

typedef GoResult (*set_storage_fn)(api_t *ptr, U8SliceView key,  U8SliceView value,  uint64_t *used_gas);
GoResult cset_storage(api_t *ptr, U8SliceView key,  U8SliceView value,  uint64_t *used_gas);

typedef GoResult (*get_storage_fn)(api_t *ptr, U8SliceView key,   uint64_t *used_gas, UnmanagedVector *value);
GoResult cget_storage(api_t *ptr, U8SliceView key, uint64_t *used_gas,  UnmanagedVector *value);

typedef GoResult (*remove_storage_fn)(api_t *ptr, U8SliceView key, uint64_t *used_gas);
GoResult cremove_storage(api_t *ptr, U8SliceView key,  uint64_t *used_gas);

typedef GoResult (*send_fn)(api_t *ptr, U8SliceView to,  U8SliceView amount, uint64_t *used_gas,  UnmanagedVector *error);
GoResult csend(api_t *ptr, U8SliceView to,  U8SliceView amount, uint64_t *used_gas,  UnmanagedVector *error);

typedef GoResult (*block_timestamp_fn)(api_t *ptr, uint64_t *used_gas, int64_t *block_timestamp);
GoResult cblock_timestamp(api_t *ptr, uint64_t *used_gas,  int64_t *block_timestamp);

typedef GoResult (*block_number_fn)(api_t *ptr, uint64_t *used_gas,  uint64_t *block_number);
GoResult cblock_number(api_t *ptr, uint64_t *used_gas,  uint64_t *block_number);

typedef GoResult (*min_fee_per_gas_fn)(api_t *ptr, uint64_t *used_gas,  UnmanagedVector *data);
GoResult cmin_fee_per_gas(api_t *ptr, uint64_t *used_gas,   UnmanagedVector *data);

typedef GoResult (*balance_fn)(api_t *ptr, U8SliceView addr, uint64_t *used_gas,  UnmanagedVector *data);
GoResult cbalance(api_t *ptr, U8SliceView addr, uint64_t *used_gas,   UnmanagedVector *data);

typedef GoResult (*block_seed_fn)(api_t *ptr, uint64_t *used_gas,  UnmanagedVector *seed);
GoResult cblock_seed(api_t *ptr, uint64_t *used_gas,   UnmanagedVector *seed);

typedef GoResult (*network_size_fn)(api_t *ptr, uint64_t *used_gas,  uint64_t *size);
GoResult cnetwork_size(api_t *ptr, uint64_t *used_gas,  uint64_t *size);

typedef GoResult (*identity_state_fn)(api_t *ptr,  U8SliceView addr, uint64_t *used_gas,  uint8_t *state);
GoResult cidentity_state(api_t *ptr,  U8SliceView addr, uint64_t *used_gas,  uint8_t *state);

typedef GoResult (*identity_fn)(api_t *ptr,  U8SliceView addr, uint64_t *used_gas,  UnmanagedVector *data);
GoResult cidentity(api_t *ptr,  U8SliceView addr, uint64_t *used_gas, UnmanagedVector *data);

typedef GoResult (*call_fn)(api_t *ptr,  U8SliceView addr, U8SliceView method,U8SliceView args, uint64_t gas_limit, uint64_t *used_gas);
GoResult ccall(api_t *ptr, U8SliceView addr, U8SliceView method,U8SliceView args, uint64_t gas_limit, uint64_t *used_gas);

*/
import "C"
import (
	"fmt"
	"math/big"
	"unsafe"
)

type GoAPI struct {
	host     HostEnv
	gasMeter *GasMeter
}

var api_vtable = C.GoApi_vtable{
	set_remaining_gas: (C.set_remaining_gas_fn)(C.cset_remaining_gas),
	set_storage:       (C.set_storage_fn)(C.cset_storage),
	get_storage:       (C.get_storage_fn)(C.cget_storage),
	remove_storage:    (C.remove_storage_fn)(C.cremove_storage),
	block_timestamp:   (C.block_timestamp_fn)(C.cblock_timestamp),
	block_number:      (C.block_number_fn)(C.cblock_number),
	min_fee_per_gas:   (C.min_fee_per_gas_fn)(C.cmin_fee_per_gas),
	balance:           (C.balance_fn)(C.cbalance),
	block_seed:        (C.block_seed_fn)(C.cblock_seed),
	network_size:      (C.network_size_fn)(C.cnetwork_size),
	identity_state:    (C.identity_state_fn)(C.cidentity_state),
	send:              (C.send_fn)(C.csend),
	identity:          (C.identity_fn)(C.cidentity),
	call:              (C.call_fn)(C.ccall),
}

// contract: original pointer/struct referenced must live longer than C.GoApi struct
// since this is only used internally, we can verify the code that this is the case
func buildAPI(api *GoAPI) C.GoApi {
	return C.GoApi{
		state:    (*C.api_t)(unsafe.Pointer(api)),
		gasMeter: (*C.gas_meter_t)(unsafe.Pointer(api.gasMeter)),
		vtable:   api_vtable,
	}
}

//export cset_remaining_gas
func cset_remaining_gas(ptr *C.api_t, remainingGas cu64) (ret C.GoResult) {
	api := (*GoAPI)(unsafe.Pointer(ptr))
	api.gasMeter.SetRemainingGas(uint64(remainingGas))
	return C.GoResult_Ok
}

//export cset_storage
func cset_storage(ptr *C.api_t, key C.U8SliceView, value C.U8SliceView, gasUsed *cu64) (ret C.GoResult) {
	api := (*GoAPI)(unsafe.Pointer(ptr))
	k := copyU8Slice(key)
	v := copyU8Slice(value)
	gasBefore := api.gasMeter.GasConsumed()
	api.host.SetStorage(api.gasMeter, k, v)
	*gasUsed = cu64(api.gasMeter.GasConsumed() - gasBefore)
	return C.GoResult_Ok
}

//export cget_storage
func cget_storage(ptr *C.api_t, key C.U8SliceView, gasUsed *cu64, value *C.UnmanagedVector) (ret C.GoResult) {
	api := (*GoAPI)(unsafe.Pointer(ptr))
	k := copyU8Slice(key)
	gasBefore := api.gasMeter.GasConsumed()
	v := api.host.GetStorage(api.gasMeter, k)
	*gasUsed = cu64(api.gasMeter.GasConsumed() - gasBefore)
	*value = newUnmanagedVector(v)
	return C.GoResult_Ok
}

//export cremove_storage
func cremove_storage(ptr *C.api_t, key C.U8SliceView, gasUsed *cu64) (ret C.GoResult) {
	api := (*GoAPI)(unsafe.Pointer(ptr))
	k := copyU8Slice(key)
	gasBefore := api.gasMeter.GasConsumed()
	api.host.RemoveStorage(api.gasMeter, k)
	*gasUsed = cu64(api.gasMeter.GasConsumed() - gasBefore)
	return C.GoResult_Ok
}

//export csend
func csend(ptr *C.api_t, addr C.U8SliceView, amount C.U8SliceView, gasUsed *cu64, errOut *C.UnmanagedVector) (ret C.GoResult) {
	api := (*GoAPI)(unsafe.Pointer(ptr))
	address := newAddress(copyU8Slice(addr))
	amountBytes := copyU8Slice(amount)
	gasBefore := api.gasMeter.GasConsumed()

	api.host.Send(api.gasMeter, address, big.NewInt(0).SetBytes(amountBytes))
	*gasUsed = cu64(api.gasMeter.GasConsumed() - gasBefore)
	return C.GoResult_Ok
}

//export cblock_timestamp
func cblock_timestamp(ptr *C.api_t, gasUsed *cu64, blockTimestamp *ci64) (ret C.GoResult) {
	api := (*GoAPI)(unsafe.Pointer(ptr))
	gasBefore := api.gasMeter.GasConsumed()

	*blockTimestamp = ci64(api.host.BlockTimestamp(api.gasMeter))
	*gasUsed = cu64(api.gasMeter.GasConsumed() - gasBefore)
	return C.GoResult_Ok
}

//export cblock_number
func cblock_number(ptr *C.api_t, gasUsed *cu64, blockNumer *cu64) (ret C.GoResult) {
	api := (*GoAPI)(unsafe.Pointer(ptr))
	gasBefore := api.gasMeter.GasConsumed()

	*blockNumer = cu64(api.host.BlockNumber(api.gasMeter))

	*gasUsed = cu64(api.gasMeter.GasConsumed() - gasBefore)
	return C.GoResult_Ok
}

//export cmin_fee_per_gas
func cmin_fee_per_gas(ptr *C.api_t, gasUsed *cu64, data *C.UnmanagedVector) C.GoResult {
	api := (*GoAPI)(unsafe.Pointer(ptr))
	gasBefore := api.gasMeter.GasConsumed()
	feePerGas := api.host.MinFeePerGas()
	*data = newUnmanagedVector(feePerGas.Bytes())
	*gasUsed = cu64(api.gasMeter.GasConsumed() - gasBefore)
	return C.GoResult_Ok
}

//export cbalance
func cbalance(ptr *C.api_t, addr C.U8SliceView, gasUsed *cu64, data *C.UnmanagedVector) C.GoResult {
	api := (*GoAPI)(unsafe.Pointer(ptr))
	address := newAddress(copyU8Slice(addr))
	gasBefore := api.gasMeter.GasConsumed()
	balance := api.host.Balance(address)

	*data = newUnmanagedVector(balance.Bytes())
	*gasUsed = cu64(api.gasMeter.GasConsumed() - gasBefore)
	return C.GoResult_Ok
}

//export cblock_seed
func cblock_seed(ptr *C.api_t, gasUsed *cu64, data *C.UnmanagedVector) C.GoResult {
	api := (*GoAPI)(unsafe.Pointer(ptr))
	gasBefore := api.gasMeter.GasConsumed()
	seed := api.host.BlockSeed()
	*data = newUnmanagedVector(seed)
	*gasUsed = cu64(api.gasMeter.GasConsumed() - gasBefore)
	return C.GoResult_Ok
}

//export cnetwork_size
func cnetwork_size(ptr *C.api_t, gasUsed *cu64, network *cu64) (ret C.GoResult) {
	api := (*GoAPI)(unsafe.Pointer(ptr))
	gasBefore := api.gasMeter.GasConsumed()

	*network = cu64(api.host.NetworkSize(api.gasMeter))

	*gasUsed = cu64(api.gasMeter.GasConsumed() - gasBefore)
	return C.GoResult_Ok
}

//export cidentity_state
func cidentity_state(ptr *C.api_t, addr C.U8SliceView, gasUsed *cu64, state *cu8) (ret C.GoResult) {
	address := newAddress(copyU8Slice(addr))
	api := (*GoAPI)(unsafe.Pointer(ptr))
	gasBefore := api.gasMeter.GasConsumed()

	*state = cu8(api.host.IdentityState(api.gasMeter, address))

	*gasUsed = cu64(api.gasMeter.GasConsumed() - gasBefore)
	return C.GoResult_Ok
}

//export cidentity
func cidentity(ptr *C.api_t, addr C.U8SliceView, gasUsed *cu64, result *C.UnmanagedVector) (ret C.GoResult) {
	address := newAddress(copyU8Slice(addr))
	api := (*GoAPI)(unsafe.Pointer(ptr))
	gasBefore := api.gasMeter.GasConsumed()

	*result = newUnmanagedVector(api.host.Identity(api.gasMeter, address))

	*gasUsed = cu64(api.gasMeter.GasConsumed() - gasBefore)
	return C.GoResult_Ok
}

//export ccall
func ccall(ptr *C.api_t, addr C.U8SliceView, method C.U8SliceView, args C.U8SliceView, gasLimit cu64, gasUsed *cu64) (ret C.GoResult) {
	address := newAddress(copyU8Slice(addr))
	api := (*GoAPI)(unsafe.Pointer(ptr))

	code := api.host.GetCode(address)
	if len(code) == 0 {
		*gasUsed = gasLimit
		return C.GoResult_Other
	}

	subHost := api.host.CreateSubEnv()
	meter := GasMeter{}
	subApi := &GoAPI{
		host:     subHost,
		gasMeter: &meter,
	}
	subCallGasUsed, err := executeInternal(subApi, code, copyU8Slice(method), copyU8Slice(args), uint64(gasLimit))
	println(fmt.Sprintf("sub call err: %v", err))
	*gasUsed = cu64(subCallGasUsed)
	if err != nil {
		return C.GoResult_Other
	}
	return C.GoResult_Ok
}
