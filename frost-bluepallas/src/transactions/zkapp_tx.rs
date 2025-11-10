use ark_ff::Field as ArkField;
use mina_signer::CompressedPubKey;
use serde::{Deserialize, Serialize};

use crate::transactions::MEMO_BYTES;

mod commit;
mod constants;
mod hash;
pub mod zkapp_display;
pub mod zkapp_emptiable;
pub mod zkapp_packable;
pub mod zkapp_serde;

#[cfg(test)]
mod test_vectors;

// The final transaction structure for a ZkApp transaction
// FeePayer is a field which may be signed by the same key as in the account updates
// or by a different key
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ZKAppCommand {
    pub fee_payer: FeePayer,
    pub account_updates: Vec<AccountUpdate>,
    #[serde(
        serialize_with = "zkapp_serde::memo_serde",
        deserialize_with = "zkapp_serde::memo_deser"
    )]
    pub memo: [u8; MEMO_BYTES],
}

impl Default for ZKAppCommand {
    fn default() -> Self {
        Self {
            fee_payer: FeePayer::default(),
            account_updates: Vec::default(),
            memo: [0u8; MEMO_BYTES],
        }
    }
}

// Fee payer
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct FeePayer {
    pub body: FeePayerBody,
    pub authorization: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct FeePayerBody {
    pub public_key: PublicKey,
    pub fee: UInt64,
    pub valid_until: Option<UInt32>,
    pub nonce: UInt32,
}

// Account update

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AccountUpdate {
    pub body: AccountUpdateBody,
    pub authorization: Authorization,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
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

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
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

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
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

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SetVerificationKey {
    pub auth: AuthRequired,
    pub txn_version: UInt32,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Preconditions {
    pub network: NetworkPreconditions,
    pub account: AccountPreconditions,
    pub valid_while: Option<RangeCondition<UInt32>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
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

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct NetworkPreconditions {
    pub snarked_ledger_hash: Option<Field>,
    pub blockchain_length: Option<RangeCondition<UInt32>>,
    pub min_window_density: Option<RangeCondition<UInt32>>,
    pub total_currency: Option<RangeCondition<UInt64>>,
    pub global_slot_since_genesis: Option<RangeCondition<UInt32>>,
    pub staking_epoch_data: EpochData,
    pub next_epoch_data: EpochData,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Events {
    pub data: Vec<Vec<Field>>,
    pub hash: Field,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Actions {
    pub data: Vec<Vec<Field>>,
    pub hash: Field,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Authorization {
    pub proof: Option<String>,
    pub signature: Option<String>,
}

// Supporting types

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RangeCondition<T> {
    pub lower: T,
    pub upper: T,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct VerificationKeyData {
    pub data: String,
    pub hash: VerificationKeyHash,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TimingData {
    pub initial_minimum_balance: UInt64,
    pub cliff_time: UInt32,
    pub cliff_amount: UInt64,
    pub vesting_period: UInt32,
    pub vesting_increment: UInt64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct EpochData {
    pub ledger: EpochLedger,
    pub seed: Option<Field>,
    pub start_checkpoint: Option<Field>,
    pub lock_checkpoint: Option<Field>,
    pub epoch_length: Option<RangeCondition<UInt32>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct EpochLedger {
    pub hash: Option<Field>,
    pub total_currency: Option<RangeCondition<UInt64>>,
}

// Wrappers for base types that need additional traits implemented
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicKey(pub CompressedPubKey);

impl Default for PublicKey {
    fn default() -> Self {
        Self(CompressedPubKey::empty())
    }
}

impl From<CompressedPubKey> for PublicKey {
    fn from(key: CompressedPubKey) -> Self {
        Self(key)
    }
}

impl From<PublicKey> for CompressedPubKey {
    fn from(wrapper: PublicKey) -> Self {
        wrapper.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Default)]
pub struct Field(pub mina_hasher::Fp);

impl From<mina_hasher::Fp> for Field {
    fn from(field: mina_hasher::Fp) -> Self {
        Self(field)
    }
}

impl From<Field> for mina_hasher::Fp {
    fn from(wrapper: Field) -> Self {
        wrapper.0
    }
}

// Base types from transaction-leaves-bigint.ts
pub type Bool = bool;
pub type UInt64 = u64;
pub type UInt32 = u32;
pub type Sign = i8; // -1 or 1

// Wrapper structs
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenId(pub Field);
impl Default for TokenId {
    fn default() -> Self {
        TokenId(Field(mina_hasher::Fp::ONE))
    }
}

// Derived types
pub type StateHash = Field;
pub type ActionState = Field;
pub type VerificationKeyHash = Field;
pub type ReceiptChainHash = Field;
pub type TransactionVersion = UInt32;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AuthRequired {
    None,
    Either,
    Proof,
    Signature,
    Impossible,
    Both, // Legacy only
}

pub struct AuthRequiredEncoded<T> {
    pub constant: T,
    pub signature_necessary: T,
    pub signature_sufficient: T,
}

impl AuthRequired {
    pub fn encode(self) -> AuthRequiredEncoded<bool> {
        let (constant, signature_necessary, signature_sufficient) = match self {
            AuthRequired::None => (true, false, true),
            AuthRequired::Either => (false, false, true),
            AuthRequired::Proof => (false, false, false),
            AuthRequired::Signature => (false, true, true),
            AuthRequired::Impossible => (true, true, false),
            AuthRequired::Both => (false, true, false),
        };

        AuthRequiredEncoded {
            constant,
            signature_necessary,
            signature_sufficient,
        }
    }
}

impl Default for AuthRequired {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TokenSymbol(pub Vec<u8>);

impl TokenSymbol {
    pub fn to_bytes(&self, bytes: &mut [u8]) {
        if self.0.is_empty() {
            return;
        }
        let len = self.0.len();
        let s: &[u8] = &self.0;
        bytes[..len].copy_from_slice(&s[..len.min(6)]);
    }
}

impl std::str::FromStr for TokenSymbol {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() <= 6 {
            Ok(Self(s.as_bytes().to_vec()))
        } else {
            Err("Token symbol must be at most 6 characters".to_string())
        }
    }
}

// Default is derived for TokenSymbol

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ZkappUri(pub Vec<u8>);

impl std::str::FromStr for ZkappUri {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() <= 32 {
            Ok(Self(s.as_bytes().to_vec()))
        } else {
            Err("Zkapp URI must be at most 32 characters".to_string())
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct MayUseToken {
    pub parents_own_token: Bool,
    pub inherit_from_parent: Bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BalanceChange {
    pub magnitude: UInt64,
    pub sgn: Sign,
}
impl Default for BalanceChange {
    fn default() -> Self {
        Self {
            magnitude: 0,
            sgn: 1,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AuthorizationKind {
    pub is_signed: Bool,
    pub is_proved: Bool,
    pub verification_key_hash: VerificationKeyHash,
}
