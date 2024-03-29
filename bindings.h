/* (c) 2019 Confio UO. Licensed under Apache-2.0 */

/* Generated with cbindgen:0.20.0 */

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define MAX_RETURN_VALUE_SIZE (64 * 1024)

#define BASE_PROMISE_COST 100000

#define BASE_DEPLOY_COST 3000000

#define BASE_CALL_COST 100000

#define BASE_BYTES_TO_HEX_COST 10000

#define ACTION_FUNCTION_CALL 1

#define ACTION_TRANSFER 2

#define ACTION_DEPLOY_CONTRACT 3

#define ACTION_READ_CONTRACT_DATA 4

#define ACTION_READ_IDENTITY 5

/**
 * This enum gives names to the status codes returned from Go callbacks to Rust.
 *
 * The go code will return one of these variants when returning.
 *
 */
enum GoResult {
  GoResult_Ok = 0,
  /**
   * Go panicked for an unexpected reason.
   */
  GoResult_Panic = 1,
  /**
   * Go received a bad argument from Rust
   */
  GoResult_BadArgument = 2,
  /**
   * Ran out of gas while using the SDK (e.g. storage)
   */
  GoResult_OutOfGas = 3,
  /**
   * An error happened during normal operation of a Go callback, which should abort the contract
   */
  GoResult_Other = 4,
  /**
   * An error happened during normal operation of a Go callback, which should be fed back to the contract
   */
  GoResult_User = 5,
};
typedef int32_t GoResult;

typedef struct UnmanagedVector {
  /**
   * True if and only if this is None. If this is true, the other fields must be ignored.
   */
  bool is_none;
  uint8_t *ptr;
  uintptr_t len;
  uintptr_t cap;
} UnmanagedVector;

typedef struct api_t {
  uint8_t _private[0];
} api_t;

typedef struct gas_meter_t {
  uint8_t _private[0];
} gas_meter_t;

typedef struct U8SliceView {
  /**
   * True if and only if this is None. If this is true, the other fields must be ignored.
   */
  bool is_none;
  const uint8_t *ptr;
  uintptr_t len;
} U8SliceView;

typedef struct GoApi_vtable {
  int32_t (*set_remaining_gas)(const struct api_t*, uint64_t);
  int32_t (*set_storage)(const struct api_t*, struct U8SliceView, struct U8SliceView, uint64_t*);
  int32_t (*get_storage)(const struct api_t*, struct U8SliceView, uint64_t*, struct UnmanagedVector*);
  int32_t (*remove_storage)(const struct api_t*, struct U8SliceView, uint64_t*);
  int32_t (*block_number)(const struct api_t*, uint64_t*, uint64_t*);
  int32_t (*block_timestamp)(const struct api_t*, uint64_t*, int64_t*);
  int32_t (*min_fee_per_gas)(const struct api_t*, uint64_t*, struct UnmanagedVector*);
  int32_t (*balance)(const struct api_t*, uint64_t*, struct UnmanagedVector*);
  int32_t (*block_seed)(const struct api_t*, uint64_t*, struct UnmanagedVector*);
  int32_t (*network_size)(const struct api_t*, uint64_t*, uint64_t*);
  int32_t (*burn)(const struct api_t*, struct U8SliceView, uint64_t*);
  int32_t (*epoch)(const struct api_t*, uint64_t*, uint16_t*);
  int32_t (*identity)(const struct api_t*, struct U8SliceView, uint64_t*, struct UnmanagedVector*);
  int32_t (*caller)(const struct api_t*, uint64_t*, struct UnmanagedVector*);
  int32_t (*original_caller)(const struct api_t*, uint64_t*, struct UnmanagedVector*);
  int32_t (*deduct_balance)(const struct api_t*, struct U8SliceView, uint64_t*, struct UnmanagedVector*);
  int32_t (*add_balance)(const struct api_t*, struct U8SliceView, struct U8SliceView, uint64_t*);
  int32_t (*contract)(const struct api_t*, uint64_t*, struct UnmanagedVector*);
  int32_t (*call)(const struct api_t*, struct U8SliceView, struct U8SliceView, struct U8SliceView, struct U8SliceView, struct U8SliceView, uint64_t, uint64_t*, struct UnmanagedVector*);
  int32_t (*deploy)(const struct api_t*, struct U8SliceView, struct U8SliceView, struct U8SliceView, struct U8SliceView, uint64_t, uint64_t*, struct UnmanagedVector*);
  int32_t (*contract_addr)(const struct api_t*, struct U8SliceView, struct U8SliceView, struct U8SliceView, uint64_t*, struct UnmanagedVector*);
  int32_t (*contract_addr_by_hash)(const struct api_t*, struct U8SliceView, struct U8SliceView, struct U8SliceView, uint64_t*, struct UnmanagedVector*);
  int32_t (*own_code)(const struct api_t*, uint64_t*, struct UnmanagedVector*);
  int32_t (*code_hash)(const struct api_t*, uint64_t*, struct UnmanagedVector*);
  int32_t (*event)(const struct api_t*, struct U8SliceView, struct U8SliceView, uint64_t*);
  int32_t (*read_contract_data)(const struct api_t*, struct U8SliceView, struct U8SliceView, uint64_t*, struct UnmanagedVector*);
  int32_t (*pay_amount)(const struct api_t*, uint64_t*, struct UnmanagedVector*);
  int32_t (*block_header)(const struct api_t*, uint64_t, uint64_t*, struct UnmanagedVector*);
  int32_t (*keccak256)(const struct api_t*, struct U8SliceView, uint64_t*, struct UnmanagedVector*);
  int32_t (*global_state)(const struct api_t*, uint64_t*, struct UnmanagedVector*);
  int32_t (*ecrecover)(const struct api_t*, struct U8SliceView, struct U8SliceView, uint64_t*, struct UnmanagedVector*);
} GoApi_vtable;

typedef struct GoApi {
  const struct api_t *state;
  const struct gas_meter_t *gas_meter;
  struct GoApi_vtable vtable;
} GoApi;

typedef struct ByteSliceView {
  /**
   * True if and only if the byte slice is nil in Go. If this is true, the other fields must be ignored.
   */
  bool is_nil;
  const uint8_t *ptr;
  uintptr_t len;
} ByteSliceView;

void destroy_unmanaged_vector(struct UnmanagedVector v);

struct UnmanagedVector new_unmanaged_vector(bool nil, const uint8_t *ptr, uintptr_t length);

uint8_t execute(struct GoApi api,
                struct ByteSliceView code,
                struct ByteSliceView method_name,
                struct ByteSliceView args,
                struct ByteSliceView invocation_context,
                struct ByteSliceView contract_addr,
                uint64_t gas_limit,
                uint64_t *gas_used,
                struct UnmanagedVector *action_result,
                bool is_debug);

uint8_t deploy(struct GoApi api,
               struct ByteSliceView code,
               struct ByteSliceView args,
               struct ByteSliceView contract_addr,
               uint64_t gas_limit,
               uint64_t *gas_used,
               struct UnmanagedVector *action_result,
               bool is_debug);
