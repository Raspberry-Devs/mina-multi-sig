use mina_signer::CompressedPubKey;

#[derive(Clone)]
pub struct AccountUpdate {
    pub body: AccountUpdateBody,
    pub authorization: Authorization,
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Update {
    pub app_state: Vec<Option<Field>>,
    pub delegate: Option<PublicKey>,
    pub verification_key: Option<VerificationKeyData>,
    pub permissions: Option<Permissions>,
    pub zkapp_uri: Option<ZkappUri>,
    pub token_symbol: Option<TokenSymbol>,
    pub timing: Option<TimingData>,
    pub voting_for: Option<Field>,
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct SetVerificationKey {
    pub auth: AuthRequired,
    pub txn_version: UInt32,
}

#[derive(Clone)]
pub struct Preconditions {
    pub network: NetworkPreconditions,
    pub account: AccountPreconditions,
    pub valid_while: Option<RangeCondition<UInt32>>,
}

#[derive(Clone)]
pub struct AccountPreconditions {
    pub balance: Option<RangeCondition<UInt64>>,
    pub nonce: Option<RangeCondition<UInt32>>,
    pub receipt_chain_hash: Option<ReceiptChainHash>,
    pub delegate: Option<PublicKey>,
    pub state: Vec<Option<Field>>,
    pub action_state: Option<ActionState>,
    pub proved_state: Option<Bool>,
    pub is_new: Option<Bool>,
}

#[derive(Clone)]
pub struct NetworkPreconditions {
    pub snarked_ledger_hash: Option<Field>,
    pub blockchain_length: Option<RangeCondition<UInt32>>,
    pub min_window_density: Option<RangeCondition<UInt32>>,
    pub total_currency: Option<RangeCondition<UInt64>>,
    pub global_slot_since_genesis: Option<RangeCondition<UInt32>>,
    pub staking_epoch_data: EpochData,
    pub next_epoch_data: EpochData,
}

#[derive(Clone)]
pub struct Events {
    pub data: Vec<Vec<Field>>,
    pub hash: Field,
}

#[derive(Clone)]
pub struct Actions {
    pub data: Vec<Vec<Field>>,
    pub hash: Field,
}

#[derive(Clone)]
pub struct Authorization {
    pub proof: Option<String>,
    pub signature: Option<String>,
}

// Supporting types

#[derive(Clone)]
pub struct RangeCondition<T> {
    pub lower: T,
    pub upper: T,
}

#[derive(Clone)]
pub struct VerificationKeyData {
    pub data: String,
    pub hash: VerificationKeyHash,
}

#[derive(Clone)]
pub struct TimingData {
    pub initial_minimum_balance: UInt64,
    pub cliff_time: UInt32,
    pub cliff_amount: UInt64,
    pub vesting_period: UInt32,
    pub vesting_increment: UInt64,
}

#[derive(Clone)]
pub struct EpochData {
    pub ledger: EpochLedger,
    pub seed: Option<Field>,
    pub start_checkpoint: Option<Field>,
    pub lock_checkpoint: Option<Field>,
    pub epoch_length: Option<RangeCondition<UInt32>>,
}

#[derive(Clone)]
pub struct EpochLedger {
    pub hash: Option<Field>,
    pub total_currency: Option<RangeCondition<UInt64>>,
}

// Base types from transaction-leaves-bigint.ts
pub type PublicKey = CompressedPubKey;
pub type Field = mina_hasher::Fp;
pub type Bool = bool;
pub type UInt64 = u64;
pub type UInt32 = u32;
pub type Sign = i8; // -1 or 1

// Derived types
pub type TokenId = Field;
pub type StateHash = Field;
pub type ActionState = Field;
pub type VerificationKeyHash = Field;
pub type ReceiptChainHash = Field;
pub type TransactionVersion = UInt32;

#[derive(Clone)]
pub struct AuthRequired {
    pub constant: Bool,
    pub signature_necessary: Bool,
    pub signature_sufficient: Bool,
}

pub type TokenSymbol = String;

pub type ZkappUri = String;

#[derive(Clone)]
pub struct MayUseToken {
    pub parents_own_token: Bool,
    pub inherit_from_parent: Bool,
}

#[derive(Clone)]
pub struct BalanceChange {
    pub magnitude: UInt64,
    pub sgn: Sign,
}

#[derive(Clone)]
pub struct AuthorizationKind {
    pub is_signed: Bool,
    pub is_proved: Bool,
    pub verification_key_hash: VerificationKeyHash,
}
