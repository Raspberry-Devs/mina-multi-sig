use mina_signer::CompressedPubKey;

mod commit;
mod constants;
pub mod zkapp_trait;

// The final transaction structure for a ZkApp transaction
// FeePayer is a field which may be signed by the same key as in the account updates
// or by a different key
#[derive(Clone)]
pub struct ZKAppCommand {
    pub fee_payer: FeePayer,
    pub account_updates: Vec<AccountUpdate>,
    pub memo: String,
}

#[derive(Clone)]
pub struct FeePayer {
    pub body: FeePayerBody,
    pub authorization: String,
}

#[derive(Clone)]
pub struct FeePayerBody {
    pub public_key: PublicKey,
    pub fee: UInt64,
    pub valid_until: Option<UInt32>,
    pub nonce: UInt32,
}

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
    pub app_state: Vec<OptionalValue<Field>>,
    pub delegate: OptionalValue<PublicKey>,
    pub verification_key: OptionalValue<VerificationKeyData>,
    pub permissions: OptionalValue<Permissions>,
    pub zkapp_uri: OptionalValue<ZkappUriData>,
    pub token_symbol: OptionalValue<TokenSymbolData>,
    pub timing: OptionalValue<TimingData>,
    pub voting_for: OptionalValue<Field>,
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
    pub valid_while: OptionalValue<RangeCondition<UInt32>>,
}

#[derive(Clone)]
pub struct AccountPreconditions {
    pub balance: OptionalValue<RangeCondition<UInt64>>,
    pub nonce: OptionalValue<RangeCondition<UInt32>>,
    pub receipt_chain_hash: OptionalValue<ReceiptChainHash>,
    pub delegate: OptionalValue<PublicKey>,
    pub state: Vec<OptionalValue<Field>>,
    pub action_state: OptionalValue<ActionState>,
    pub proved_state: OptionalValue<Bool>,
    pub is_new: OptionalValue<Bool>,
}

#[derive(Clone)]
pub struct NetworkPreconditions {
    pub snarked_ledger_hash: OptionalValue<Field>,
    pub blockchain_length: OptionalValue<RangeCondition<UInt32>>,
    pub min_window_density: OptionalValue<RangeCondition<UInt32>>,
    pub total_currency: OptionalValue<RangeCondition<UInt64>>,
    pub global_slot_since_genesis: OptionalValue<RangeCondition<UInt32>>,
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
    pub seed: OptionalValue<Field>,
    pub start_checkpoint: OptionalValue<Field>,
    pub lock_checkpoint: OptionalValue<Field>,
    pub epoch_length: OptionalValue<RangeCondition<UInt32>>,
}

#[derive(Clone)]
pub struct EpochLedger {
    pub hash: OptionalValue<Field>,
    pub total_currency: OptionalValue<RangeCondition<UInt64>>,
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

#[derive(Clone)]
pub struct OptionalValue<T> {
    pub is_some: Bool,
    pub value: T,
}

#[derive(Clone)]
pub struct ZkappUriData {
    pub data: String,
    pub hash: Field,
}

#[derive(Clone)]
pub struct TokenSymbolData {
    pub symbol: String,
    pub field: Field,
}

// Additional structs for Account type
#[derive(Clone)]
pub struct Account {
    pub public_key: PublicKey,
    pub token_id: TokenId,
    pub token_symbol: String,
    pub balance: UInt64,
    pub nonce: UInt32,
    pub receipt_chain_hash: Field,
    pub delegate: Option<PublicKey>,
    pub voting_for: Field,
    pub timing: AccountTiming,
    pub permissions: Permissions,
    pub zkapp: Option<ZkappAccount>,
}

#[derive(Clone)]
pub struct AccountTiming {
    pub is_timed: Bool,
    pub initial_minimum_balance: UInt64,
    pub cliff_time: UInt32,
    pub cliff_amount: UInt64,
    pub vesting_period: UInt32,
    pub vesting_increment: UInt64,
}

#[derive(Clone)]
pub struct ZkappAccount {
    pub app_state: Vec<Field>,
    pub verification_key: Option<VerificationKeyData>,
    pub zkapp_version: UInt32,
    pub action_state: Vec<Field>,
    pub last_action_slot: UInt32,
    pub proved_state: Bool,
    pub zkapp_uri: String,
}
