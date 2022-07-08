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

typedef GoResult (*call_fn)(api_t *ptr,  U8SliceView addr, U8SliceView method,U8SliceView args, U8SliceView amount, U8SliceView ctx, uint64_t gas_limit, uint64_t *used_gas, UnmanagedVector *actionResult);
GoResult ccall(api_t *ptr, U8SliceView addr, U8SliceView method,U8SliceView args, U8SliceView amount, U8SliceView ctx, uint64_t gas_limit, uint64_t *used_gas, UnmanagedVector *actionResult);

typedef GoResult (*caller_fn)(api_t *ptr,  uint64_t *used_gas,  UnmanagedVector *data);
GoResult ccaller(api_t *ptr, uint64_t *used_gas, UnmanagedVector *data);

typedef GoResult (*original_caller_fn)(api_t *ptr,  uint64_t *used_gas,  UnmanagedVector *data);
GoResult coriginal_caller(api_t *ptr, uint64_t *used_gas, UnmanagedVector *data);

typedef GoResult (*commit_fn)(api_t *ptr);
GoResult ccommit(api_t *ptr);

typedef GoResult (*deduct_balance_fn)(api_t *ptr, U8SliceView amount,  uint64_t *used_gas,  UnmanagedVector *err_out);
GoResult cdeduct_balance(api_t *ptr, U8SliceView amount,  uint64_t *used_gas,  UnmanagedVector *err_out);

typedef GoResult (*add_balance_fn)(api_t *ptr, U8SliceView addr, U8SliceView amount,  uint64_t *used_gas);
GoResult cadd_balance(api_t *ptr, U8SliceView addr, U8SliceView amount,  uint64_t *used_gas);

typedef GoResult (*contract_fn)(api_t *ptr, uint64_t *used_gas,  UnmanagedVector *result);
GoResult ccontract(api_t *ptr, uint64_t *used_gas,  UnmanagedVector *result);

typedef GoResult (*contract_code_fn)(api_t *ptr, U8SliceView addr, uint64_t *used_gas,  UnmanagedVector *result);
GoResult ccontract_code(api_t *ptr, U8SliceView addr, uint64_t *used_gas,  UnmanagedVector *result);


typedef GoResult (*contract_addr_fn)(api_t *ptr, U8SliceView code, U8SliceView args, U8SliceView nonce, uint64_t *used_gas,  UnmanagedVector *result);
GoResult ccontract_addr(api_t *ptr, U8SliceView code, U8SliceView args, U8SliceView nonce, uint64_t *used_gas,  UnmanagedVector *result);

*/
import "C"
import (
	"fmt"
	"log"
	"math/big"
	"unsafe"
)

type GoAPI struct {
	host     HostEnv
	gasMeter *GasMeter
}

func NewGoAPI(env HostEnv, gasMeter *GasMeter) *GoAPI {
	return &GoAPI{
		host:     env,
		gasMeter: gasMeter,
	}
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
	caller:            (C.caller_fn)(C.ccaller),
	original_caller:   (C.original_caller_fn)(C.coriginal_caller),
	commit:            (C.commit_fn)(C.ccommit),
	deduct_balance:    (C.deduct_balance_fn)(C.cdeduct_balance),
	add_balance:       (C.add_balance_fn)(C.cadd_balance),
	contract:          (C.contract_fn)(C.ccontract),
	contract_code:     (C.contract_code_fn)(C.ccontract_code),
	call:              (C.call_fn)(C.ccall),
	contract_addr:     (C.contract_addr_fn)(C.ccontract_addr),
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

func recoverPanic(ret *C.GoResult, api *GoAPI, gasUsed *cu64) {
	if rec := recover(); rec != nil {
		switch rec.(type) {
		case OutOfGas:
			*ret = C.GoResult_OutOfGas
			if gasUsed != nil {
				*gasUsed = cu64(api.gasMeter.gasLimit)
			}
		default:
			log.Printf("Panic in Go callback: %#v\n", rec)
			*ret = C.GoResult_Panic
		}
	}
}

//export cset_remaining_gas
func cset_remaining_gas(ptr *C.api_t, remainingGas cu64) (ret C.GoResult) {
	api := (*GoAPI)(unsafe.Pointer(ptr))
	defer recoverPanic(&ret, api, nil)
	api.gasMeter.SetRemainingGas(uint64(remainingGas))
	return C.GoResult_Ok
}

//export cset_storage
func cset_storage(ptr *C.api_t, key C.U8SliceView, value C.U8SliceView, gasUsed *cu64) (ret C.GoResult) {
	api := (*GoAPI)(unsafe.Pointer(ptr))
	defer recoverPanic(&ret, api, gasUsed)

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
	defer recoverPanic(&ret, api, gasUsed)

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
	defer recoverPanic(&ret, api, gasUsed)

	k := copyU8Slice(key)
	gasBefore := api.gasMeter.GasConsumed()
	api.host.RemoveStorage(api.gasMeter, k)
	*gasUsed = cu64(api.gasMeter.GasConsumed() - gasBefore)
	return C.GoResult_Ok
}

//export csend
func csend(ptr *C.api_t, addr C.U8SliceView, amount C.U8SliceView, gasUsed *cu64, errOut *C.UnmanagedVector) (ret C.GoResult) {
	api := (*GoAPI)(unsafe.Pointer(ptr))
	defer recoverPanic(&ret, api, gasUsed)

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
	defer recoverPanic(&ret, api, gasUsed)

	gasBefore := api.gasMeter.GasConsumed()

	*blockTimestamp = ci64(api.host.BlockTimestamp(api.gasMeter))
	*gasUsed = cu64(api.gasMeter.GasConsumed() - gasBefore)
	return C.GoResult_Ok
}

//export cblock_number
func cblock_number(ptr *C.api_t, gasUsed *cu64, blockNumer *cu64) (ret C.GoResult) {

	api := (*GoAPI)(unsafe.Pointer(ptr))
	defer recoverPanic(&ret, api, gasUsed)
	gasBefore := api.gasMeter.GasConsumed()

	*blockNumer = cu64(api.host.BlockNumber(api.gasMeter))

	*gasUsed = cu64(api.gasMeter.GasConsumed() - gasBefore)
	return C.GoResult_Ok
}

//export cmin_fee_per_gas
func cmin_fee_per_gas(ptr *C.api_t, gasUsed *cu64, data *C.UnmanagedVector) (ret C.GoResult) {

	api := (*GoAPI)(unsafe.Pointer(ptr))
	defer recoverPanic(&ret, api, gasUsed)

	gasBefore := api.gasMeter.GasConsumed()
	feePerGas := api.host.MinFeePerGas(api.gasMeter)
	*data = newUnmanagedVector(feePerGas.Bytes())
	*gasUsed = cu64(api.gasMeter.GasConsumed() - gasBefore)
	return C.GoResult_Ok
}

//export cbalance
func cbalance(ptr *C.api_t, addr C.U8SliceView, gasUsed *cu64, data *C.UnmanagedVector) (ret C.GoResult) {

	api := (*GoAPI)(unsafe.Pointer(ptr))
	defer recoverPanic(&ret, api, gasUsed)

	address := newAddress(copyU8Slice(addr))
	gasBefore := api.gasMeter.GasConsumed()
	balance := api.host.Balance(api.gasMeter, address)

	*data = newUnmanagedVector(balance.Bytes())
	*gasUsed = cu64(api.gasMeter.GasConsumed() - gasBefore)
	return C.GoResult_Ok
}

//export cblock_seed
func cblock_seed(ptr *C.api_t, gasUsed *cu64, data *C.UnmanagedVector) (ret C.GoResult) {

	api := (*GoAPI)(unsafe.Pointer(ptr))
	defer recoverPanic(&ret, api, gasUsed)

	gasBefore := api.gasMeter.GasConsumed()
	seed := api.host.BlockSeed(api.gasMeter)
	*data = newUnmanagedVector(seed)
	*gasUsed = cu64(api.gasMeter.GasConsumed() - gasBefore)
	return C.GoResult_Ok
}

//export cnetwork_size
func cnetwork_size(ptr *C.api_t, gasUsed *cu64, network *cu64) (ret C.GoResult) {

	api := (*GoAPI)(unsafe.Pointer(ptr))
	defer recoverPanic(&ret, api, gasUsed)

	gasBefore := api.gasMeter.GasConsumed()

	*network = cu64(api.host.NetworkSize(api.gasMeter))

	*gasUsed = cu64(api.gasMeter.GasConsumed() - gasBefore)
	return C.GoResult_Ok
}

//export cidentity_state
func cidentity_state(ptr *C.api_t, addr C.U8SliceView, gasUsed *cu64, state *cu8) (ret C.GoResult) {

	api := (*GoAPI)(unsafe.Pointer(ptr))
	defer recoverPanic(&ret, api, gasUsed)

	address := newAddress(copyU8Slice(addr))
	gasBefore := api.gasMeter.GasConsumed()

	*state = cu8(api.host.IdentityState(api.gasMeter, address))

	*gasUsed = cu64(api.gasMeter.GasConsumed() - gasBefore)
	return C.GoResult_Ok
}

//export cidentity
func cidentity(ptr *C.api_t, addr C.U8SliceView, gasUsed *cu64, result *C.UnmanagedVector) (ret C.GoResult) {
	api := (*GoAPI)(unsafe.Pointer(ptr))
	defer recoverPanic(&ret, api, gasUsed)

	address := newAddress(copyU8Slice(addr))
	gasBefore := api.gasMeter.GasConsumed()

	*result = newUnmanagedVector(api.host.Identity(api.gasMeter, address))

	*gasUsed = cu64(api.gasMeter.GasConsumed() - gasBefore)
	return C.GoResult_Ok
}

//export ccall
func ccall(ptr *C.api_t, addr C.U8SliceView, method C.U8SliceView, args C.U8SliceView, amount C.U8SliceView, invocationContext C.U8SliceView, gasLimit cu64, gasUsed *cu64, actionResult *C.UnmanagedVector) (ret C.GoResult) {

	api := (*GoAPI)(unsafe.Pointer(ptr))
	defer recoverPanic(&ret, api, gasUsed)
	address := newAddress(copyU8Slice(addr))

	code := api.host.GetCode(address)
	if len(code) == 0 {
		*gasUsed = gasLimit
		return C.GoResult_Other
	}

	subHost, err := api.host.CreateSubEnv(address, big.NewInt(0).SetBytes(copyU8Slice(amount)))
	if err != nil {
		*gasUsed = gasLimit
		return C.GoResult_Other
	}
	meter := GasMeter{}
	subApi := &GoAPI{
		host:     subHost,
		gasMeter: &meter,
	}
	subCallGasUsed, actionResultBytes, err := execute(subApi, code, copyU8Slice(method), copyU8Slice(args), copyU8Slice(invocationContext), uint64(gasLimit))
	println(fmt.Sprintf("sub call err: %v", err))
	if err != nil {
		api.host.Commit()
	} else {
		api.host.Revert()
	}
	*gasUsed = cu64(subCallGasUsed)
	*actionResult = newUnmanagedVector(actionResultBytes)
	if err != nil {
		return C.GoResult_Other
	}
	return C.GoResult_Ok
}

//export cdeploy
func cdeploy(ptr *C.api_t, code C.U8SliceView, args C.U8SliceView, nonce C.U8SliceView, amount C.U8SliceView, gasLimit cu64, gasUsed *cu64, actionResult *C.UnmanagedVector) (ret C.GoResult) {

	api := (*GoAPI)(unsafe.Pointer(ptr))
	defer recoverPanic(&ret, api, gasUsed)
	codeBytes := copyU8Slice(code)
	argsBytes := copyU8Slice(args)
	nonceBytes := copyU8Slice(nonce)

	addr := api.host.ContractAddr(api.gasMeter, codeBytes, argsBytes, nonceBytes)

	subHost, err := api.host.CreateSubEnv(addr, big.NewInt(0).SetBytes(copyU8Slice(amount)))
	if err != nil {
		*gasUsed = gasLimit
		return C.GoResult_Other
	}
	meter := GasMeter{}
	subApi := &GoAPI{
		host:     subHost,
		gasMeter: &meter,
	}
	subCallGasUsed, actionResultBytes, err := deploy(subApi, codeBytes, argsBytes, uint64(gasLimit))
	println(fmt.Sprintf("deploy err: %v", err))
	if err != nil {
		api.host.Commit()
	} else {
		api.host.Revert()
	}
	*gasUsed = cu64(subCallGasUsed)
	*actionResult = newUnmanagedVector(actionResultBytes)
	if err != nil {
		return C.GoResult_Other
	}
	return C.GoResult_Ok
}

//export ccaller
func ccaller(ptr *C.api_t, gasUsed *cu64, result *C.UnmanagedVector) (ret C.GoResult) {
	api := (*GoAPI)(unsafe.Pointer(ptr))
	defer recoverPanic(&ret, api, gasUsed)

	gasBefore := api.gasMeter.GasConsumed()

	addr := api.host.Caller(api.gasMeter)
	*result = newUnmanagedVector(addr[:])

	*gasUsed = cu64(api.gasMeter.GasConsumed() - gasBefore)
	return C.GoResult_Ok
}

//export coriginal_caller
func coriginal_caller(ptr *C.api_t, gasUsed *cu64, result *C.UnmanagedVector) (ret C.GoResult) {

	api := (*GoAPI)(unsafe.Pointer(ptr))
	defer recoverPanic(&ret, api, gasUsed)

	gasBefore := api.gasMeter.GasConsumed()

	addr := api.host.OriginalCaller(api.gasMeter)
	*result = newUnmanagedVector(addr[:])

	*gasUsed = cu64(api.gasMeter.GasConsumed() - gasBefore)
	return C.GoResult_Ok
}

//export ccommit
func ccommit(ptr *C.api_t) (ret C.GoResult) {

	api := (*GoAPI)(unsafe.Pointer(ptr))
	defer recoverPanic(&ret, api, nil)
	api.host.Commit()
	return C.GoResult_Ok
}

//export cdeduct_balance
func cdeduct_balance(ptr *C.api_t, amount C.U8SliceView, gasUsed *cu64, errOut *C.UnmanagedVector) (ret C.GoResult) {

	api := (*GoAPI)(unsafe.Pointer(ptr))
	defer recoverPanic(&ret, api, gasUsed)

	amountBytes := copyU8Slice(amount)
	gasBefore := api.gasMeter.GasConsumed()

	if err := api.host.SubBalance(api.gasMeter, big.NewInt(0).SetBytes(amountBytes)); err != nil {
		*errOut = newUnmanagedVector([]byte(err.Error()))
		return C.GoResult_Other
	}

	*gasUsed = cu64(api.gasMeter.GasConsumed() - gasBefore)

	return C.GoResult_Ok
}

//export cadd_balance
func cadd_balance(ptr *C.api_t, addr C.U8SliceView, amount C.U8SliceView, gasUsed *cu64) (ret C.GoResult) {

	api := (*GoAPI)(unsafe.Pointer(ptr))
	defer recoverPanic(&ret, api, gasUsed)

	address := newAddress(copyU8Slice(addr))
	amountBytes := copyU8Slice(amount)
	gasBefore := api.gasMeter.GasConsumed()
	api.host.AddBalance(api.gasMeter, address, big.NewInt(0).SetBytes(amountBytes))
	*gasUsed = cu64(api.gasMeter.GasConsumed() - gasBefore)

	return C.GoResult_Ok
}

//export ccontract
func ccontract(ptr *C.api_t, gasUsed *cu64, result *C.UnmanagedVector) (ret C.GoResult) {
	api := (*GoAPI)(unsafe.Pointer(ptr))
	defer recoverPanic(&ret, api, gasUsed)

	gasBefore := api.gasMeter.GasConsumed()
	addr := api.host.ContractAddress(api.gasMeter)
	*result = newUnmanagedVector(addr[:])
	*gasUsed = cu64(api.gasMeter.GasConsumed() - gasBefore)

	return C.GoResult_Ok
}

//export ccontract_code
func ccontract_code(ptr *C.api_t, addr C.U8SliceView, gasUsed *cu64, result *C.UnmanagedVector) (ret C.GoResult) {
	api := (*GoAPI)(unsafe.Pointer(ptr))
	defer recoverPanic(&ret, api, gasUsed)

	address := newAddress(copyU8Slice(addr))

	gasBefore := api.gasMeter.GasConsumed()
	code := api.host.ContractCode(api.gasMeter, address)
	*result = newUnmanagedVector(code)
	*gasUsed = cu64(api.gasMeter.GasConsumed() - gasBefore)

	return C.GoResult_Ok
}

//export ccontract_addr
func ccontract_addr(ptr *C.api_t, code C.U8SliceView, args C.U8SliceView, nonce C.U8SliceView, gasUsed *cu64, result *C.UnmanagedVector) (ret C.GoResult) {
	api := (*GoAPI)(unsafe.Pointer(ptr))
	defer recoverPanic(&ret, api, gasUsed)
	codeBytes := copyU8Slice(code)
	argsBytes := copyU8Slice(args)
	nonceBytes := copyU8Slice(nonce)

	gasBefore := api.gasMeter.GasConsumed()

	address := api.host.ContractAddr(api.gasMeter, codeBytes, argsBytes, nonceBytes)
	*result = newUnmanagedVector(address[:])
	*gasUsed = cu64(api.gasMeter.GasConsumed() - gasBefore)

	return C.GoResult_Ok
}
