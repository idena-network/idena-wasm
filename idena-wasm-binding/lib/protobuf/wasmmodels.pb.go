//protoc --go_opt=paths=source_relative --go_out=./ ./protobuf/*.proto

// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.23.0
// 	protoc        v3.12.3
// source: protobuf/wasmmodels.proto

package models

import (
	proto "github.com/golang/protobuf/proto"
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

// This is a compile-time assertion that a sufficiently up-to-date version
// of the legacy proto package is being used.
const _ = proto.ProtoPackageIsVersion4

type ProtoCallContractArgs struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Args [][]byte `protobuf:"bytes,1,rep,name=args,proto3" json:"args,omitempty"`
}

func (x *ProtoCallContractArgs) Reset() {
	*x = ProtoCallContractArgs{}
	if protoimpl.UnsafeEnabled {
		mi := &file_protobuf_wasmmodels_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ProtoCallContractArgs) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ProtoCallContractArgs) ProtoMessage() {}

func (x *ProtoCallContractArgs) ProtoReflect() protoreflect.Message {
	mi := &file_protobuf_wasmmodels_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ProtoCallContractArgs.ProtoReflect.Descriptor instead.
func (*ProtoCallContractArgs) Descriptor() ([]byte, []int) {
	return file_protobuf_wasmmodels_proto_rawDescGZIP(), []int{0}
}

func (x *ProtoCallContractArgs) GetArgs() [][]byte {
	if x != nil {
		return x.Args
	}
	return nil
}

type Action struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	ActionType uint32 `protobuf:"varint,1,opt,name=action_type,json=actionType,proto3" json:"action_type,omitempty"`
	Amount     []byte `protobuf:"bytes,2,opt,name=amount,proto3" json:"amount,omitempty"`
	Method     string `protobuf:"bytes,3,opt,name=method,proto3" json:"method,omitempty"`
	Args       []byte `protobuf:"bytes,4,opt,name=args,proto3" json:"args,omitempty"`
	GasLimit   uint64 `protobuf:"varint,5,opt,name=gas_limit,json=gasLimit,proto3" json:"gas_limit,omitempty"`
}

func (x *Action) Reset() {
	*x = Action{}
	if protoimpl.UnsafeEnabled {
		mi := &file_protobuf_wasmmodels_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Action) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Action) ProtoMessage() {}

func (x *Action) ProtoReflect() protoreflect.Message {
	mi := &file_protobuf_wasmmodels_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Action.ProtoReflect.Descriptor instead.
func (*Action) Descriptor() ([]byte, []int) {
	return file_protobuf_wasmmodels_proto_rawDescGZIP(), []int{1}
}

func (x *Action) GetActionType() uint32 {
	if x != nil {
		return x.ActionType
	}
	return 0
}

func (x *Action) GetAmount() []byte {
	if x != nil {
		return x.Amount
	}
	return nil
}

func (x *Action) GetMethod() string {
	if x != nil {
		return x.Method
	}
	return ""
}

func (x *Action) GetArgs() []byte {
	if x != nil {
		return x.Args
	}
	return nil
}

func (x *Action) GetGasLimit() uint64 {
	if x != nil {
		return x.GasLimit
	}
	return 0
}

type ActionResult struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	InputAction      *Action         `protobuf:"bytes,1,opt,name=input_action,json=inputAction,proto3" json:"input_action,omitempty"`
	Success          bool            `protobuf:"varint,2,opt,name=success,proto3" json:"success,omitempty"`
	Error            string          `protobuf:"bytes,3,opt,name=error,proto3" json:"error,omitempty"`
	GasUsed          uint64          `protobuf:"varint,4,opt,name=gas_used,json=gasUsed,proto3" json:"gas_used,omitempty"`
	OutputData       []byte          `protobuf:"bytes,5,opt,name=output_data,json=outputData,proto3" json:"output_data,omitempty"`
	RemainGasCost    []byte          `protobuf:"bytes,6,opt,name=remain_gas_cost,json=remainGasCost,proto3" json:"remain_gas_cost,omitempty"`
	SubActionResults []*ActionResult `protobuf:"bytes,7,rep,name=sub_action_results,json=subActionResults,proto3" json:"sub_action_results,omitempty"`
}

func (x *ActionResult) Reset() {
	*x = ActionResult{}
	if protoimpl.UnsafeEnabled {
		mi := &file_protobuf_wasmmodels_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ActionResult) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ActionResult) ProtoMessage() {}

func (x *ActionResult) ProtoReflect() protoreflect.Message {
	mi := &file_protobuf_wasmmodels_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ActionResult.ProtoReflect.Descriptor instead.
func (*ActionResult) Descriptor() ([]byte, []int) {
	return file_protobuf_wasmmodels_proto_rawDescGZIP(), []int{2}
}

func (x *ActionResult) GetInputAction() *Action {
	if x != nil {
		return x.InputAction
	}
	return nil
}

func (x *ActionResult) GetSuccess() bool {
	if x != nil {
		return x.Success
	}
	return false
}

func (x *ActionResult) GetError() string {
	if x != nil {
		return x.Error
	}
	return ""
}

func (x *ActionResult) GetGasUsed() uint64 {
	if x != nil {
		return x.GasUsed
	}
	return 0
}

func (x *ActionResult) GetOutputData() []byte {
	if x != nil {
		return x.OutputData
	}
	return nil
}

func (x *ActionResult) GetRemainGasCost() []byte {
	if x != nil {
		return x.RemainGasCost
	}
	return nil
}

func (x *ActionResult) GetSubActionResults() []*ActionResult {
	if x != nil {
		return x.SubActionResults
	}
	return nil
}

var File_protobuf_wasmmodels_proto protoreflect.FileDescriptor

var file_protobuf_wasmmodels_proto_rawDesc = []byte{
	0x0a, 0x19, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x77, 0x61, 0x73, 0x6d, 0x6d,
	0x6f, 0x64, 0x65, 0x6c, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x06, 0x6d, 0x6f, 0x64,
	0x65, 0x6c, 0x73, 0x22, 0x2b, 0x0a, 0x15, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x43, 0x61, 0x6c, 0x6c,
	0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x41, 0x72, 0x67, 0x73, 0x12, 0x12, 0x0a, 0x04,
	0x61, 0x72, 0x67, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0c, 0x52, 0x04, 0x61, 0x72, 0x67, 0x73,
	0x22, 0x8a, 0x01, 0x0a, 0x06, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x1f, 0x0a, 0x0b, 0x61,
	0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d,
	0x52, 0x0a, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x54, 0x79, 0x70, 0x65, 0x12, 0x16, 0x0a, 0x06,
	0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x06, 0x61, 0x6d,
	0x6f, 0x75, 0x6e, 0x74, 0x12, 0x16, 0x0a, 0x06, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x18, 0x03,
	0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x12, 0x12, 0x0a, 0x04,
	0x61, 0x72, 0x67, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x61, 0x72, 0x67, 0x73,
	0x12, 0x1b, 0x0a, 0x09, 0x67, 0x61, 0x73, 0x5f, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x18, 0x05, 0x20,
	0x01, 0x28, 0x04, 0x52, 0x08, 0x67, 0x61, 0x73, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x22, 0x99, 0x02,
	0x0a, 0x0c, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x31,
	0x0a, 0x0c, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x5f, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x01,
	0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x73, 0x2e, 0x41, 0x63,
	0x74, 0x69, 0x6f, 0x6e, 0x52, 0x0b, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x41, 0x63, 0x74, 0x69, 0x6f,
	0x6e, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x18, 0x02, 0x20, 0x01,
	0x28, 0x08, 0x52, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x12, 0x14, 0x0a, 0x05, 0x65,
	0x72, 0x72, 0x6f, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f,
	0x72, 0x12, 0x19, 0x0a, 0x08, 0x67, 0x61, 0x73, 0x5f, 0x75, 0x73, 0x65, 0x64, 0x18, 0x04, 0x20,
	0x01, 0x28, 0x04, 0x52, 0x07, 0x67, 0x61, 0x73, 0x55, 0x73, 0x65, 0x64, 0x12, 0x1f, 0x0a, 0x0b,
	0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x05, 0x20, 0x01, 0x28,
	0x0c, 0x52, 0x0a, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x44, 0x61, 0x74, 0x61, 0x12, 0x26, 0x0a,
	0x0f, 0x72, 0x65, 0x6d, 0x61, 0x69, 0x6e, 0x5f, 0x67, 0x61, 0x73, 0x5f, 0x63, 0x6f, 0x73, 0x74,
	0x18, 0x06, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0d, 0x72, 0x65, 0x6d, 0x61, 0x69, 0x6e, 0x47, 0x61,
	0x73, 0x43, 0x6f, 0x73, 0x74, 0x12, 0x42, 0x0a, 0x12, 0x73, 0x75, 0x62, 0x5f, 0x61, 0x63, 0x74,
	0x69, 0x6f, 0x6e, 0x5f, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x73, 0x18, 0x07, 0x20, 0x03, 0x28,
	0x0b, 0x32, 0x14, 0x2e, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x73, 0x2e, 0x41, 0x63, 0x74, 0x69, 0x6f,
	0x6e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x52, 0x10, 0x73, 0x75, 0x62, 0x41, 0x63, 0x74, 0x69,
	0x6f, 0x6e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x73, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x33,
}

var (
	file_protobuf_wasmmodels_proto_rawDescOnce sync.Once
	file_protobuf_wasmmodels_proto_rawDescData = file_protobuf_wasmmodels_proto_rawDesc
)

func file_protobuf_wasmmodels_proto_rawDescGZIP() []byte {
	file_protobuf_wasmmodels_proto_rawDescOnce.Do(func() {
		file_protobuf_wasmmodels_proto_rawDescData = protoimpl.X.CompressGZIP(file_protobuf_wasmmodels_proto_rawDescData)
	})
	return file_protobuf_wasmmodels_proto_rawDescData
}

var file_protobuf_wasmmodels_proto_msgTypes = make([]protoimpl.MessageInfo, 3)
var file_protobuf_wasmmodels_proto_goTypes = []interface{}{
	(*ProtoCallContractArgs)(nil), // 0: models.ProtoCallContractArgs
	(*Action)(nil),                // 1: models.Action
	(*ActionResult)(nil),          // 2: models.ActionResult
}
var file_protobuf_wasmmodels_proto_depIdxs = []int32{
	1, // 0: models.ActionResult.input_action:type_name -> models.Action
	2, // 1: models.ActionResult.sub_action_results:type_name -> models.ActionResult
	2, // [2:2] is the sub-list for method output_type
	2, // [2:2] is the sub-list for method input_type
	2, // [2:2] is the sub-list for extension type_name
	2, // [2:2] is the sub-list for extension extendee
	0, // [0:2] is the sub-list for field type_name
}

func init() { file_protobuf_wasmmodels_proto_init() }
func file_protobuf_wasmmodels_proto_init() {
	if File_protobuf_wasmmodels_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_protobuf_wasmmodels_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ProtoCallContractArgs); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_protobuf_wasmmodels_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Action); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_protobuf_wasmmodels_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ActionResult); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_protobuf_wasmmodels_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   3,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_protobuf_wasmmodels_proto_goTypes,
		DependencyIndexes: file_protobuf_wasmmodels_proto_depIdxs,
		MessageInfos:      file_protobuf_wasmmodels_proto_msgTypes,
	}.Build()
	File_protobuf_wasmmodels_proto = out.File
	file_protobuf_wasmmodels_proto_rawDesc = nil
	file_protobuf_wasmmodels_proto_goTypes = nil
	file_protobuf_wasmmodels_proto_depIdxs = nil
}
