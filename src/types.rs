use crate::proto::models::{Action as protoAction, ActionResult as protoActionResult, InvocationContext as protoContext, PromiseResult as protoPromiseResult};

pub const ACTION_FUNCTION_CALL: u8 = 1;
pub const ACTION_TRANSFER: u8 = 2;
pub const ACTION_DEPLOY_CONTRACT: u8 = 3;
pub const ACTION_READ_CONTRACT_DATA: u8 = 4;
pub const ACTION_READ_IDENTITY: u8 = 5;

pub type IDNA = Vec<u8>;

pub type Address = Vec<u8>;
pub type Hash = [u8; 32];
pub type Gas = u64;

#[derive(Clone, Debug)]
pub enum Action {
    None,
    DeployContract(DeployContractAction),
    FunctionCall(FunctionCallAction),
    ReadShardedData(ReadShardedDataAction),
    Transfer(TransferAction),
}

#[derive(Clone, Debug)]
pub struct FunctionCallAction {
    pub method_name: String,
    pub args: Vec<u8>,
    pub gas_limit: Gas,
    pub deposit: IDNA,
}

#[derive(Clone, Debug)]
pub struct DeployContractAction {
    pub code: Vec<u8>,
    pub nonce: Vec<u8>,
    pub args: Vec<u8>,
    pub gas_limit: Gas,
    pub deposit: IDNA,
}

#[derive(Clone, Debug)]
pub struct TransferAction {
    pub amount: IDNA,
}

#[derive(Clone, Debug)]
pub struct ReadContractDataAction {
    pub gas_limit: u64,
    pub key: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct GetIdentityAction {
    pub gas_limit: u64,
    pub addr: Vec<u8>,
}

#[derive(Clone, Debug)]
pub enum ReadShardedDataAction {
    ReadContractData(ReadContractDataAction),
    GetIdentity(GetIdentityAction),
}


/*pub struct OutputData {
    pub data_id: Hash,
    pub data: Vec<u8>,
}


pub struct ActionReceipt {
    pub sender: Address,
    pub gas_price: IDNA,
    pub input_dependencies: Vec<Hash>,
    pub output_data: Vec<OutputData>,
    pub actions: Vec<Action>,
}

pub enum ReceiptEnum {
    ActionReceipt(ActionReceipt),
    DataReceipt(OutputData),
}
*/

#[derive(Clone, Debug)]
pub struct ActionResult {
    pub input_action: Action,
    pub gas_used: Gas,
    pub remaining_gas: Gas,
    pub success: bool,
    pub error: String,
    pub output_data: Vec<u8>,
    pub sub_action_results: Vec<ActionResult>,
    pub contract: Address,
}

impl Into<protoActionResult> for &ActionResult {
    fn into(self) -> protoActionResult {
        let mut proto = protoActionResult::default();
        proto.set_input_action((&self.input_action).into());
        proto.gas_used = self.gas_used;
        proto.output_data = self.output_data.clone();
        proto.success = self.success;
        proto.error = self.error.clone();
        proto.remaining_gas = self.remaining_gas;
        proto.contract = self.contract.clone();
        for sub_res in self.sub_action_results.iter() {
            proto.sub_action_results.push(sub_res.into());
        }

        proto
    }
}

impl From<protoActionResult> for ActionResult {
    fn from(action_res: protoActionResult) -> Self {
        ActionResult {
            input_action: action_res.input_action.unwrap_or_default().into(),
            success: action_res.success,
            remaining_gas: action_res.remaining_gas,
            gas_used: action_res.gas_used,
            sub_action_results: action_res.sub_action_results.into_iter().map(|a| a.into()).collect(),
            error: action_res.error,
            output_data: action_res.output_data,
            contract: action_res.contract,
        }
    }
}


impl Into<protoAction> for &Action {
    fn into(self) -> protoAction {
        match self {
            Action::FunctionCall(call) =>
                {
                    let mut proto = protoAction::default();
                    proto.action_type = ACTION_FUNCTION_CALL as u32;
                    proto.method = call.method_name.clone();
                    proto.amount = call.deposit.clone();
                    proto.args = call.args.clone();
                    proto.gas_limit = call.gas_limit;
                    proto
                }
            Action::DeployContract(deploy) => {
                let mut proto = protoAction::default();
                proto.action_type = ACTION_DEPLOY_CONTRACT as u32;
                proto.code = deploy.code.clone();
                proto.amount = deploy.deposit.clone();
                proto.args = deploy.args.clone();
                proto.gas_limit = deploy.gas_limit;
                proto.nonce = deploy.nonce.clone();
                proto
            }
            Action::Transfer(transfer) => {
                let mut proto = protoAction::default();
                proto.action_type = ACTION_TRANSFER as u32;
                proto.amount = transfer.amount.clone();
                proto
            }
            Action::ReadShardedData(read) => {
                let mut proto = protoAction::default();

                match read {
                    ReadShardedDataAction::ReadContractData(r) => {
                        proto.action_type = ACTION_READ_CONTRACT_DATA as u32;
                        proto.gas_limit = r.gas_limit;
                        proto.key = r.key.clone();
                    }
                    ReadShardedDataAction::GetIdentity(r) => {
                        proto.action_type = ACTION_READ_IDENTITY as u32;
                        proto.gas_limit = r.gas_limit;
                        proto.key = r.addr.clone();
                    }
                }
                proto
            }
            _ => protoAction::default()
        }
    }
}

impl From<protoAction> for Action {
    fn from(action: protoAction) -> Self {
        match action.action_type as u8 {
            ACTION_FUNCTION_CALL => {
                Action::FunctionCall(FunctionCallAction {
                    gas_limit: action.gas_limit,
                    args: action.args,
                    method_name: action.method,
                    deposit: action.amount,
                })
            }

            ACTION_DEPLOY_CONTRACT => {
                Action::DeployContract(DeployContractAction {
                    gas_limit: action.gas_limit,
                    args: action.args,
                    code: action.code,
                    deposit: action.amount,
                    nonce: action.nonce,
                })
            }

            ACTION_TRANSFER => Action::Transfer(TransferAction {
                amount: action.amount,
            }),
            ACTION_READ_CONTRACT_DATA => Action::ReadShardedData(ReadShardedDataAction::ReadContractData(ReadContractDataAction {
                key: action.key,
                gas_limit: action.gas_limit,
            })),
            ACTION_READ_IDENTITY => Action::ReadShardedData(ReadShardedDataAction::GetIdentity(GetIdentityAction {
                addr: action.key,
                gas_limit: action.gas_limit,
            })),
            _ => Action::None
        }
    }
}

impl Default for Action {
    fn default() -> Self {
        Action::None
    }
}


impl ActionResult {
    pub(crate) fn append_sub_action_results(&mut self, results: Vec<ActionResult>) {
        self.sub_action_results.extend_from_slice(&results);
    }
}

#[derive(Clone, Debug)]
pub enum PromiseResult {
    Empty,
    Value(Vec<u8>),
    Failed,
}

#[derive(Clone)]
pub struct Promise {
    pub predecessor_id: Address,
    pub receiver_id: Address,
    pub action: Action,
    pub action_callback: Option<Action>,
}
#[derive(Clone)]
pub struct InvocationContext {
    pub is_callback: bool,
    pub promise_result: Option<PromiseResult>,
}

impl From<protoPromiseResult> for PromiseResult {
    fn from(promise_res: protoPromiseResult) -> Self {
        if !promise_res.success {
            return PromiseResult::Failed;
        }
        if promise_res.data.is_empty() {
            return PromiseResult::Empty;
        }
        return PromiseResult::Value(promise_res.data);
    }
}

impl Into<protoPromiseResult> for &PromiseResult {
    fn into(self) -> protoPromiseResult {
        let mut res = protoPromiseResult::default();
        match self {
            PromiseResult::Empty => {
                res.success = true
            }
            PromiseResult::Value(val) => {
                res.success = true;
                res.set_data(val.clone());
            }
            PromiseResult::Failed => {
                res.success = false;
            }
        };
        res
    }
}

impl From<protoContext> for InvocationContext {
    fn from(ctx: protoContext) -> Self {
        InvocationContext {
            is_callback: ctx.is_callback,
            promise_result: Some(ctx.promise_result.unwrap_or_default().into()),
        }
    }
}

impl Into<protoContext> for InvocationContext {
    fn into(self) -> protoContext {
        let mut ctx = protoContext::default();
        ctx.is_callback = self.is_callback;
        match self.promise_result {
            Some(v) => ctx.set_promise_result((&v).into()),
            _ => {}
        }
        ctx
    }
}

impl Default for InvocationContext {
    fn default() -> Self {
        InvocationContext {
            is_callback: false,
            promise_result: None,
        }
    }
}









