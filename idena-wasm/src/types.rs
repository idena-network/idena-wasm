use crate::proto::models::{Action as protoAction, ActionResult as protoActionResult};

pub const ACTION_FUNCTION_CALL: u8 = 1;
pub const ACTION_TRANSFER: u8 = 2;


pub type IDNA = Vec<u8>;

pub type Address = Vec<u8>;
pub type Hash = [u8; 32];
pub type Gas = u64;

#[derive(Clone, Debug)]
pub enum Action {
    FunctionCall(FunctionCallAction),
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
pub struct TransferAction {
    pub amount: IDNA,
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
    pub success: bool,
    pub error: String,
    pub output_data: Vec<u8>,
    pub sub_action_results: Vec<ActionResult>,
}

impl Into<protoActionResult> for &ActionResult {
    fn into(self) -> protoActionResult {
        let mut proto = protoActionResult::default();
        proto.set_input_action((&self.input_action).into());
        proto.gas_used = self.gas_used;
        proto.output_data = self.output_data.clone();
        proto.success = self.success;
        proto.error = self.error.clone();

        for sub_res in self.sub_action_results.iter() {
            proto.sub_action_results.push(sub_res.into());
        }

        proto
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
            Action::Transfer(transfer) => {
                let mut proto = protoAction::default();
                proto.action_type = ACTION_TRANSFER as u32;
                proto.amount = transfer.amount.clone();
                proto
            }
        }
    }
}

impl ActionResult {
    pub(crate) fn append_sub_action_results(&mut self, results: Vec<ActionResult>) {
        self.sub_action_results.extend_from_slice(&results);
    }
}

#[derive(Clone)]
pub enum PromiseResult {
    Empty,
    Value(Vec<u8>),
    Failed,
}


#[derive(Clone)]
pub struct Promise {
    pub receiver_Id: Address,
    pub action: Action,
    pub action_callback: Option<Action>,
}





