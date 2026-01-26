//! GraphQL input types for zkApp transactions.
//!
//! This module provides Rust structs that mirror the GraphQL input types required
//! for submitting zkApp transactions via the Mina GraphQL API. It follows the same
//! pattern established in [`crate::graphql`] for legacy transactions.
//!
//! Many internal types from [`super`] are reused directly when their serde
//! serialization already matches the GraphQL schema requirements.

use crate::{
    base58::{to_base58_check, MEMO_VERSION_BYTE},
    graphql::GraphqlRequest,
    transactions::{
        zkapp_tx::{
            AccountPreconditions, AccountUpdate, AuthRequired, EpochData, EpochLedger, FeePayer,
            Field, NetworkPreconditions, Permissions, Preconditions, RangeCondition,
            SetVerificationKey, TimingData, Update, VerificationKeyData, ZKAppCommand,
        },
        MEMO_BYTES,
    },
};
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use serde::Serialize;

// -------------------------------------------------------------------------------------------------
// -------------------------------- GraphQL Variables Wrapper --------------------------------------
// -------------------------------------------------------------------------------------------------

/// Variables for the sendZkapp mutation
#[derive(Serialize)]
pub struct SendZkappVariables {
    pub input: SendZkappInput,
}

// -------------------------------------------------------------------------------------------------
// -------------------------------- Top-Level Input Types ------------------------------------------
// -------------------------------------------------------------------------------------------------

/// Input type for sendZkapp mutation
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendZkappInput {
    pub zkapp_command: ZkappCommandInput,
}

/// The main zkApp command input structure.
/// Note: This differs from [`ZKAppCommand`] because memo must be a base58check string.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ZkappCommandInput {
    pub fee_payer: FeePayer,
    pub account_updates: Vec<AccountUpdate>,
    pub memo: String,
}

// -------------------------------------------------------------------------------------------------
// -------------------------------- GraphQL-Specific Input Types -----------------------------------
// These types exist because the internal types serialize differently than GraphQL expects.
// -------------------------------------------------------------------------------------------------

/// Account update modification input (fields that can be updated).
/// Differs from [`Update`] because `app_state` needs `Option<String>` instead of `Option<Field>`.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountUpdateModificationInput {
    pub app_state: Vec<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_key: Option<VerificationKeyWithHashInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PermissionsInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zkapp_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing: Option<TimingInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voting_for: Option<String>,
}

/// Verification key with hash input.
/// Differs from [`VerificationKeyData`] because hash must be a string.
#[derive(Serialize)]
pub struct VerificationKeyWithHashInput {
    pub data: String,
    pub hash: String,
}

/// Timing input for account updates.
/// Differs from [`TimingData`] because all fields must be strings.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimingInput {
    pub initial_minimum_balance: String,
    pub cliff_time: String,
    pub cliff_amount: String,
    pub vesting_period: String,
    pub vesting_increment: String,
}

/// Permissions input.
/// Differs from [`Permissions`] because [`AuthRequired`] fields must be strings.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionsInput {
    pub edit_state: String,
    pub access: String,
    pub send: String,
    pub receive: String,
    pub set_delegate: String,
    pub set_permissions: String,
    pub set_verification_key: VerificationKeyPermissionInput,
    pub set_zkapp_uri: String,
    pub edit_action_state: String,
    pub set_token_symbol: String,
    pub increment_nonce: String,
    pub set_voting_for: String,
    pub set_timing: String,
}

/// Verification key permission input.
/// Differs from [`SetVerificationKey`] because fields must be strings.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationKeyPermissionInput {
    pub auth: String,
    pub txn_version: String,
}

/// Preconditions input.
/// Differs from [`Preconditions`] because nested types need string serialization.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreconditionsInput {
    pub network: NetworkPreconditionInput,
    pub account: AccountPreconditionInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_while: Option<IntervalInput>,
}

/// Network precondition input.
/// Differs from [`NetworkPreconditions`] because intervals need string bounds.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPreconditionInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snarked_ledger_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockchain_length: Option<IntervalInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_window_density: Option<IntervalInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_currency: Option<IntervalInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_slot_since_genesis: Option<IntervalInput>,
    pub staking_epoch_data: EpochDataPreconditionInput,
    pub next_epoch_data: EpochDataPreconditionInput,
}

/// Account precondition input.
/// Differs from [`AccountPreconditions`] because intervals and fields need string serialization.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountPreconditionInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<IntervalInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<IntervalInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_chain_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegate: Option<String>,
    pub state: Vec<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proved_state: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_new: Option<bool>,
}

/// Epoch data precondition input.
/// Differs from [`EpochData`] because fields need string serialization.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EpochDataPreconditionInput {
    pub ledger: EpochLedgerPreconditionInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_checkpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_checkpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch_length: Option<IntervalInput>,
}

/// Epoch ledger precondition input.
/// Differs from [`EpochLedger`] because fields need string serialization.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EpochLedgerPreconditionInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_currency: Option<IntervalInput>,
}

/// Generic interval input with string bounds.
/// Used for all interval types (balance, nonce, length, currency amount, global slot).
/// The GraphQL schema uses different scalar types but they all serialize as strings.
#[derive(Serialize)]
pub struct IntervalInput {
    pub lower: String,
    pub upper: String,
}

// -------------------------------------------------------------------------------------------------
// -------------------------------- Helper Functions -----------------------------------------------
// -------------------------------------------------------------------------------------------------

/// Encode a memo byte array as base58check string
fn encode_memo(memo: &[u8; MEMO_BYTES]) -> String {
    to_base58_check(memo, MEMO_VERSION_BYTE)
}

/// Convert Field to string representation
fn field_to_string(field: &Field) -> String {
    field.0.to_string()
}

/// Convert AuthRequired to GraphQL string
fn auth_required_to_string(auth: &AuthRequired) -> String {
    match auth {
        AuthRequired::None => "None",
        AuthRequired::Either => "Either",
        AuthRequired::Proof => "Proof",
        AuthRequired::Signature => "Signature",
        AuthRequired::Impossible => "Impossible",
        AuthRequired::Both => "Signature", // GraphQL schema doesn't have Both, map to Signature
    }
    .to_string()
}

/// Convert a RangeCondition to IntervalInput with string bounds
fn range_to_interval<T: ToString>(range: &RangeCondition<T>) -> IntervalInput {
    IntervalInput {
        lower: range.lower.to_string(),
        upper: range.upper.to_string(),
    }
}

// -------------------------------------------------------------------------------------------------
// -------------------------------- From Implementations -------------------------------------------
// -------------------------------------------------------------------------------------------------

impl From<&ZKAppCommand> for SendZkappInput {
    fn from(cmd: &ZKAppCommand) -> Self {
        SendZkappInput {
            zkapp_command: ZkappCommandInput::from(cmd),
        }
    }
}

impl From<&ZKAppCommand> for ZkappCommandInput {
    fn from(cmd: &ZKAppCommand) -> Self {
        ZkappCommandInput {
            fee_payer: cmd.fee_payer.clone(),
            account_updates: cmd.account_updates.clone(),
            memo: encode_memo(&cmd.memo),
        }
    }
}

impl From<&Update> for AccountUpdateModificationInput {
    fn from(update: &Update) -> Self {
        AccountUpdateModificationInput {
            app_state: update
                .app_state
                .iter()
                .map(|opt| opt.as_ref().map(field_to_string))
                .collect(),
            delegate: update.delegate.as_ref().map(|pk| pk.0.into_address()),
            verification_key: update
                .verification_key
                .as_ref()
                .map(VerificationKeyWithHashInput::from),
            permissions: update.permissions.as_ref().map(PermissionsInput::from),
            zkapp_uri: update
                .zkapp_uri
                .as_ref()
                .map(|uri| String::from_utf8_lossy(&uri.0).to_string()),
            token_symbol: update
                .token_symbol
                .as_ref()
                .map(|sym| String::from_utf8_lossy(&sym.0).to_string()),
            timing: update.timing.as_ref().map(TimingInput::from),
            voting_for: update.voting_for.as_ref().map(field_to_string),
        }
    }
}

impl From<&VerificationKeyData> for VerificationKeyWithHashInput {
    fn from(vk: &VerificationKeyData) -> Self {
        VerificationKeyWithHashInput {
            data: vk.data.clone(),
            hash: field_to_string(&vk.hash),
        }
    }
}

impl From<&TimingData> for TimingInput {
    fn from(timing: &TimingData) -> Self {
        TimingInput {
            initial_minimum_balance: timing.initial_minimum_balance.to_string(),
            cliff_time: timing.cliff_time.to_string(),
            cliff_amount: timing.cliff_amount.to_string(),
            vesting_period: timing.vesting_period.to_string(),
            vesting_increment: timing.vesting_increment.to_string(),
        }
    }
}

impl From<&Permissions> for PermissionsInput {
    fn from(perms: &Permissions) -> Self {
        PermissionsInput {
            edit_state: auth_required_to_string(&perms.edit_state),
            access: auth_required_to_string(&perms.access),
            send: auth_required_to_string(&perms.send),
            receive: auth_required_to_string(&perms.receive),
            set_delegate: auth_required_to_string(&perms.set_delegate),
            set_permissions: auth_required_to_string(&perms.set_permissions),
            set_verification_key: VerificationKeyPermissionInput::from(&perms.set_verification_key),
            set_zkapp_uri: auth_required_to_string(&perms.set_zkapp_uri),
            edit_action_state: auth_required_to_string(&perms.edit_action_state),
            set_token_symbol: auth_required_to_string(&perms.set_token_symbol),
            increment_nonce: auth_required_to_string(&perms.increment_nonce),
            set_voting_for: auth_required_to_string(&perms.set_voting_for),
            set_timing: auth_required_to_string(&perms.set_timing),
        }
    }
}

impl From<&SetVerificationKey> for VerificationKeyPermissionInput {
    fn from(svk: &SetVerificationKey) -> Self {
        VerificationKeyPermissionInput {
            auth: auth_required_to_string(&svk.auth),
            txn_version: svk.txn_version.to_string(),
        }
    }
}

impl From<&Preconditions> for PreconditionsInput {
    fn from(pre: &Preconditions) -> Self {
        PreconditionsInput {
            network: NetworkPreconditionInput::from(&pre.network),
            account: AccountPreconditionInput::from(&pre.account),
            valid_while: pre.valid_while.as_ref().map(range_to_interval),
        }
    }
}

impl From<&NetworkPreconditions> for NetworkPreconditionInput {
    fn from(np: &NetworkPreconditions) -> Self {
        NetworkPreconditionInput {
            snarked_ledger_hash: np.snarked_ledger_hash.as_ref().map(field_to_string),
            blockchain_length: np.blockchain_length.as_ref().map(range_to_interval),
            min_window_density: np.min_window_density.as_ref().map(range_to_interval),
            total_currency: np.total_currency.as_ref().map(range_to_interval),
            global_slot_since_genesis: np.global_slot_since_genesis.as_ref().map(range_to_interval),
            staking_epoch_data: EpochDataPreconditionInput::from(&np.staking_epoch_data),
            next_epoch_data: EpochDataPreconditionInput::from(&np.next_epoch_data),
        }
    }
}

impl From<&AccountPreconditions> for AccountPreconditionInput {
    fn from(ap: &AccountPreconditions) -> Self {
        AccountPreconditionInput {
            balance: ap.balance.as_ref().map(range_to_interval),
            nonce: ap.nonce.as_ref().map(range_to_interval),
            receipt_chain_hash: ap.receipt_chain_hash.as_ref().map(field_to_string),
            delegate: ap.delegate.as_ref().map(|pk| pk.0.into_address()),
            state: ap
                .state
                .iter()
                .map(|opt| opt.as_ref().map(field_to_string))
                .collect(),
            action_state: ap.action_state.as_ref().map(|a| field_to_string(&a.0)),
            proved_state: ap.proved_state,
            is_new: ap.is_new,
        }
    }
}

impl From<&EpochData> for EpochDataPreconditionInput {
    fn from(ed: &EpochData) -> Self {
        EpochDataPreconditionInput {
            ledger: EpochLedgerPreconditionInput::from(&ed.ledger),
            seed: ed.seed.as_ref().map(field_to_string),
            start_checkpoint: ed.start_checkpoint.as_ref().map(field_to_string),
            lock_checkpoint: ed.lock_checkpoint.as_ref().map(field_to_string),
            epoch_length: ed.epoch_length.as_ref().map(range_to_interval),
        }
    }
}

impl From<&EpochLedger> for EpochLedgerPreconditionInput {
    fn from(el: &EpochLedger) -> Self {
        EpochLedgerPreconditionInput {
            hash: el.hash.as_ref().map(field_to_string),
            total_currency: el.total_currency.as_ref().map(range_to_interval),
        }
    }
}

// -------------------------------------------------------------------------------------------------
// -------------------------------- GraphQL Mutation Builder ---------------------------------------
// -------------------------------------------------------------------------------------------------

/// Build a sendZkapp mutation request for submitting zkApp transactions
///
/// # Example
/// ```ignore
/// use frost_bluepallas::transactions::zkapp_tx::{ZKAppCommand, zkapp_graphql::build_send_zkapp_mutation};
///
/// let zkapp_command: ZKAppCommand = /* ... */;
/// let request = build_send_zkapp_mutation(&zkapp_command);
/// // Serialize and send to Mina GraphQL endpoint
/// let json = serde_json::to_string(&request)?;
/// ```
pub fn build_send_zkapp_mutation(cmd: &ZKAppCommand) -> GraphqlRequest<SendZkappVariables> {
    let query = r#"
mutation SendZkapp($input: SendZkappInput!) {
  sendZkapp(input: $input) {
    zkapp {
      id
      hash
      failureReason {
        index
        failures
      }
    }
  }
}
"#
    .trim()
    .to_string();

    GraphqlRequest {
        operation_name: Some("SendZkapp".to_string()),
        query,
        variables: Some(SendZkappVariables {
            input: SendZkappInput::from(cmd),
        }),
    }
}

// -------------------------------------------------------------------------------------------------
// -------------------------------- Tests ----------------------------------------------------------
// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        base58::TOKEN_ID_VERSION_BYTE,
        transactions::zkapp_tx::{
            AccountUpdate, AccountUpdateBody, Actions, Authorization, AuthorizationKind,
            BalanceChange, Events, FeePayer, FeePayerBody, MayUseToken, Preconditions, PublicKey,
            TokenId, Update, ZKAppCommand,
        },
    };
    use ark_ff::{BigInteger, PrimeField};
    use mina_signer::CompressedPubKey;

    /// Encode a TokenId to base58check string
    fn encode_token_id(token_id: &TokenId) -> String {
        let bytes: [u8; 32] = token_id
            .0
             .0
            .into_bigint()
            .to_bytes_le()
            .try_into()
            .expect("TokenId should be 32 bytes");
        to_base58_check(&bytes, TOKEN_ID_VERSION_BYTE)
    }

    fn create_test_public_key() -> PublicKey {
        let test_address = "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg";
        let compressed_pk =
            CompressedPubKey::from_address(test_address).expect("Valid test address");
        PublicKey(compressed_pk)
    }

    fn create_minimal_zkapp_command() -> ZKAppCommand {
        let pk = create_test_public_key();

        let fee_payer = FeePayer {
            body: FeePayerBody {
                public_key: pk.clone(),
                fee: 100_000_000, // 0.1 MINA
                valid_until: Some(1000),
                nonce: 5,
            },
            authorization: "test_signature".to_string(),
        };

        let account_update = AccountUpdate {
            body: AccountUpdateBody {
                public_key: pk,
                token_id: TokenId::default(),
                update: Update::default(),
                balance_change: BalanceChange {
                    magnitude: 50_000_000,
                    sgn: -1,
                },
                increment_nonce: true,
                events: Events::default(),
                actions: Actions::default(),
                call_data: Field::default(),
                call_depth: 0,
                preconditions: Preconditions::default(),
                use_full_commitment: true,
                implicit_account_creation_fee: false,
                may_use_token: MayUseToken::default(),
                authorization_kind: AuthorizationKind::default(),
            },
            authorization: Authorization {
                proof: None,
                signature: Some("account_update_sig".to_string()),
            },
        };

        // Create memo: 0x01, length byte, "test"
        let mut memo = [0u8; MEMO_BYTES];
        memo[0] = 0x01;
        memo[1] = 4;
        memo[2..6].copy_from_slice(b"test");

        ZKAppCommand {
            fee_payer,
            account_updates: vec![account_update],
            memo,
        }
    }

    #[test]
    fn test_auth_required_to_string() {
        assert_eq!(auth_required_to_string(&AuthRequired::None), "None");
        assert_eq!(auth_required_to_string(&AuthRequired::Either), "Either");
        assert_eq!(auth_required_to_string(&AuthRequired::Proof), "Proof");
        assert_eq!(
            auth_required_to_string(&AuthRequired::Signature),
            "Signature"
        );
        assert_eq!(
            auth_required_to_string(&AuthRequired::Impossible),
            "Impossible"
        );
        assert_eq!(auth_required_to_string(&AuthRequired::Both), "Signature");
    }

    #[test]
    fn test_send_zkapp_input_serialization() {
        let cmd = create_minimal_zkapp_command();
        let input = SendZkappInput::from(&cmd);

        let json = serde_json::to_string(&input).expect("Should serialize");

        // Check key fields are present with camelCase
        assert!(json.contains("\"zkappCommand\""));
        assert!(json.contains("\"feePayer\""));
        assert!(json.contains("\"accountUpdates\""));
        assert!(json.contains("\"memo\""));
    }

    #[test]
    fn test_zkapp_command_input_structure() {
        let cmd = create_minimal_zkapp_command();
        let input = ZkappCommandInput::from(&cmd);

        assert_eq!(input.account_updates.len(), 1);
        assert!(!input.memo.is_empty());
    }

    #[test]
    fn test_build_send_zkapp_mutation() {
        let cmd = create_minimal_zkapp_command();
        let request = build_send_zkapp_mutation(&cmd);

        assert_eq!(request.operation_name, Some("SendZkapp".to_string()));
        assert!(request.query.contains("mutation SendZkapp"));
        assert!(request.query.contains("sendZkapp(input: $input)"));
        assert!(request.query.contains("failureReason"));
        assert!(request.variables.is_some());
    }

    #[test]
    fn test_full_request_serialization() {
        let cmd = create_minimal_zkapp_command();
        let request = build_send_zkapp_mutation(&cmd);

        let json = serde_json::to_string(&request).expect("Should serialize full request");

        assert!(json.contains("\"operationName\":\"SendZkapp\""));
        assert!(json.contains("\"query\""));
        assert!(json.contains("\"variables\""));
        assert!(json.contains("\"input\""));
    }

    #[test]
    fn test_token_id_encoding() {
        let token_id = TokenId::default();
        let encoded = encode_token_id(&token_id);

        // Default token ID should encode to a valid base58check string
        assert!(!encoded.is_empty());
        // Should be reasonably long for base58check encoding of 32 bytes + version + checksum
        assert!(encoded.len() > 40);
    }

    #[test]
    fn test_memo_encoding() {
        let mut memo = [0u8; MEMO_BYTES];
        memo[0] = 0x01;
        memo[1] = 4;
        memo[2..6].copy_from_slice(b"test");

        let encoded = encode_memo(&memo);

        // Should produce a valid base58check string
        assert!(!encoded.is_empty());
    }

    #[test]
    fn test_optional_fields_skip_serializing() {
        let input = AccountUpdateModificationInput {
            app_state: vec![None; 8],
            delegate: None,
            verification_key: None,
            permissions: None,
            zkapp_uri: None,
            token_symbol: None,
            timing: None,
            voting_for: None,
        };

        let json = serde_json::to_string(&input).expect("Should serialize");

        // Optional None fields should be skipped
        assert!(!json.contains("\"delegate\""));
        assert!(!json.contains("\"verificationKey\""));
        assert!(!json.contains("\"permissions\""));
        assert!(!json.contains("\"zkappUri\""));
        assert!(!json.contains("\"tokenSymbol\""));
        assert!(!json.contains("\"timing\""));
        assert!(!json.contains("\"votingFor\""));

        // Required field should still be present
        assert!(json.contains("\"appState\""));
    }

    #[test]
    fn test_preconditions_input_structure() {
        let pre = Preconditions::default();
        let input = PreconditionsInput::from(&pre);

        let json = serde_json::to_string(&input).expect("Should serialize");

        // Required nested structures should be present
        assert!(json.contains("\"network\""));
        assert!(json.contains("\"account\""));
        // Optional valid_while should be skipped when None
        assert!(!json.contains("\"validWhile\""));
    }

    #[test]
    fn test_epoch_data_precondition_input() {
        let ed = EpochData::default();
        let input = EpochDataPreconditionInput::from(&ed);

        let json = serde_json::to_string(&input).expect("Should serialize");

        // Required ledger should be present
        assert!(json.contains("\"ledger\""));
    }

    #[test]
    fn test_interval_input() {
        let range = RangeCondition {
            lower: 10u32,
            upper: 100u32,
        };

        let interval = range_to_interval(&range);

        assert_eq!(interval.lower, "10");
        assert_eq!(interval.upper, "100");
    }
}
