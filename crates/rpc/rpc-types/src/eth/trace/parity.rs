#![allow(missing_docs)]
//! Types for trace module.
//!
//! See <https://openethereum.github.io/JSONRPC-trace-module>

use reth_primitives::{Address, Bytes, H256, U256, U64};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Result type for parity style transaction trace
pub type TraceResult = crate::trace::common::TraceResult<TraceOutput, String>;

// === impl TraceResult ===

impl TraceResult {
    /// Wraps the result type in a [TraceResult::Success] variant
    pub fn parity_success(result: TraceOutput) -> Self {
        TraceResult::Success { result }
    }

    /// Wraps the result type in a [TraceResult::Error] variant
    pub fn parity_error(error: String) -> Self {
        TraceResult::Error { error }
    }
}

/// Different Trace diagnostic targets.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TraceType {
    /// Default trace
    Trace,
    /// Provides a full trace of the VM’s state throughout the execution of the transaction,
    /// including for any subcalls.
    VmTrace,
    /// Provides information detailing all altered portions of the Ethereum state made due to the
    /// execution of the transaction.
    StateDiff,
}

/// The Outcome of a traced transaction with optional settings
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceResults {
    /// Output of the trace
    pub output: Bytes,
    /// Enabled if [TraceType::Trace] is provided
    pub trace: Option<Vec<TransactionTrace>>,
    /// Enabled if [TraceType::VmTrace] is provided
    pub vm_trace: Option<VmTrace>,
    /// Enabled if [TraceType::StateDiff] is provided
    pub state_diff: Option<StateDiff>,
}

/// A `FullTrace` with an additional transaction hash
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceResultsWithTransactionHash {
    #[serde(flatten)]
    pub full_trace: TraceResults,
    pub transaction_hash: H256,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ChangedType<T> {
    pub from: T,
    pub to: T,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum Delta<T> {
    #[default]
    #[serde(rename = "=")]
    Unchanged,
    #[serde(rename = "+")]
    Added(T),
    #[serde(rename = "-")]
    Removed(T),
    #[serde(rename = "*")]
    Changed(ChangedType<T>),
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountDiff {
    pub balance: Delta<U256>,
    pub nonce: Delta<U64>,
    pub code: Delta<Bytes>,
    pub storage: BTreeMap<H256, Delta<H256>>,
}

/// New-type for list of account diffs
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct StateDiff(pub BTreeMap<Address, AccountDiff>);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "action")]
pub enum Action {
    Call(CallAction),
    Create(CreateAction),
    Selfdestruct(SelfdestructAction),
    Reward(RewardAction),
}

/// An external action type.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ActionType {
    /// Contract call.
    Call,
    /// Contract creation.
    Create,
    /// Contract suicide/selfdestruct.
    Selfdestruct,
    /// A block reward.
    Reward,
}

/// Call type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CallType {
    /// None
    #[default]
    None,
    /// Call
    Call,
    /// Call code
    CallCode,
    /// Delegate call
    DelegateCall,
    /// Static call
    StaticCall,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallAction {
    pub from: Address,
    pub to: Address,
    pub value: U256,
    pub gas: U64,
    pub input: Bytes,
    pub call_type: CallType,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAction {
    pub from: Address,
    pub value: U256,
    pub gas: U64,
    pub init: Bytes,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RewardType {
    Block,
    Uncle,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RewardAction {
    pub author: Address,
    pub value: U256,
    pub reward_type: RewardType,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfdestructAction {
    pub address: Address,
    pub refund_address: Address,
    pub balance: U256,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallOutput {
    pub gas_used: U64,
    pub output: Bytes,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOutput {
    pub gas_used: U64,
    pub code: Bytes,
    pub address: Address,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TraceOutput {
    Call(CallOutput),
    Create(CreateOutput),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionTrace {
    pub trace_address: Vec<usize>,
    pub subtraces: usize,
    #[serde(flatten)]
    pub action: Action,
    #[serde(flatten)]
    pub result: Option<TraceResult>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalizedTransactionTrace {
    #[serde(flatten)]
    pub trace: TransactionTrace,
    /// Transaction index within the block, None if pending.
    pub transaction_position: Option<u64>,
    /// Hash of the transaction
    pub transaction_hash: Option<H256>,
    /// Block number the transaction is included in, None if pending.
    ///
    /// Note: this deviates from <https://openethereum.github.io/JSONRPC-trace-module#trace_transaction> which always returns a block number
    pub block_number: Option<u64>,
    /// Hash of the block, if not pending
    ///
    /// Note: this deviates from <https://openethereum.github.io/JSONRPC-trace-module#trace_transaction> which always returns a block number
    pub block_hash: Option<H256>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VmTrace {
    pub code: Bytes,
    pub ops: Vec<VmInstruction>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VmInstruction {
    pub pc: usize,
    pub cost: u64,
    pub ex: Option<VmExecutedOperation>,
    pub sub: Option<VmTrace>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VmExecutedOperation {
    pub used: u64,
    pub push: Option<H256>,
    pub mem: Option<MemoryDelta>,
    pub store: Option<StorageDelta>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryDelta {
    pub off: usize,
    pub data: Bytes,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageDelta {
    pub key: U256,
    pub val: U256,
}
