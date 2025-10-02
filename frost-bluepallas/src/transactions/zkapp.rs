use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountUpdate {
    pub body: AccountUpdateBody,
    pub authorization: Authorization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountUpdateBody {
    pub public_key: PublicKey,
    pub token_id: TokenId,
    pub update: Update,
    pub balance_change: BalanceChange,
    pub increment_nonce: Bool,
    pub events: Events,
    pub actions: Actions,
    pub call_data: Field,
    pub call_depth: u32,
    pub preconditions: Preconditions,
    pub use_full_commitment: Bool,
    pub implicit_account_creation_fee: Bool,
    pub may_use_token: MayUseToken,
    pub authorization_kind: AuthorizationKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Update {
    pub app_state: Vec<Option<Field>>,
    pub delegate: Option<PublicKey>,
    pub verification_key: Option<VerificationKeyData>,
    pub permissions: Option<Permissions>,
    pub zkapp_uri: Option<ZkappUriData>,
    pub token_symbol: Option<TokenSymbolData>,
    pub timing: Option<TimingData>,
    pub voting_for: Option<Field>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permissions {
    pub edit_state: AuthRequired,
    pub access: AuthRequired,
    pub send: AuthRequired,
    pub receive: AuthRequired,
    pub set_delegate: AuthRequired,
    pub set_permissions: AuthRequired,
    pub set_verification_key: SetVerificationKey,
    pub set_zkapp_uri: AuthRequired,
    pub edit_action_state: AuthRequired,
    pub set_token_symbol: AuthRequired,
    pub increment_nonce: AuthRequired,
    pub set_voting_for: AuthRequired,
    pub set_timing: AuthRequired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetVerificationKey {
    pub auth: AuthRequired,
    pub txn_version: UInt32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preconditions {
    pub network: NetworkPreconditions,
    pub account: AccountPreconditions,
    pub valid_while: Option<RangeCondition<UInt32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountPreconditions {
    pub balance: Option<RangeCondition<UInt64>>,
    pub nonce: Option<RangeCondition<UInt32>>,
    pub receipt_chain_hash: Option<Field>,
    pub delegate: Option<PublicKey>,
    pub state: Vec<Option<Field>>,
    pub action_state: Option<Field>,
    pub proved_state: Option<Bool>,
    pub is_new: Option<Bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPreconditions {
    pub snarked_ledger_hash: Option<Field>,
    pub blockchain_length: Option<RangeCondition<UInt32>>,
    pub min_window_density: Option<RangeCondition<UInt32>>,
    pub total_currency: Option<RangeCondition<UInt64>>,
    pub global_slot_since_genesis: Option<RangeCondition<UInt32>>,
    pub staking_epoch_data: EpochData,
    pub next_epoch_data: EpochData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Events {
    pub data: Vec<Vec<Field>>,
    pub hash: Field,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Authorization {
    pub proof: Option<String>,
    pub signature: Option<String>,
}

// Supporting types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeCondition<T> {
    pub lower: T,
    pub upper: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationKeyData {
    pub data: String,
    pub hash: Field,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkappUriData {
    pub data: String,
    pub hash: Field,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenSymbolData {
    pub symbol: String,
    pub field: Field,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingData {
    pub initial_minimum_balance: UInt64,
    pub cliff_time: UInt32,
    pub cliff_amount: UInt64,
    pub vesting_period: UInt32,
    pub vesting_increment: UInt64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpochData {
    pub ledger: EpochLedger,
    pub seed: Option<Field>,
    pub start_checkpoint: Option<Field>,
    pub lock_checkpoint: Option<Field>,
    pub epoch_length: Option<RangeCondition<UInt32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpochLedger {
    pub hash: Option<Field>,
    pub total_currency: Option<RangeCondition<UInt64>>,
}

pub type PublicKey = String;
pub type TokenId = u64;
pub type Field = String;
pub type UInt64 = u64;
pub type UInt32 = u32;
pub type Bool = bool;
pub type AuthRequired = String;
pub type BalanceChange = String;
pub type Actions = Events;
pub type MayUseToken = String;
pub type AuthorizationKind = String;
