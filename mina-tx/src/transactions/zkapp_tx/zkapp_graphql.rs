//! GraphQL input types for zkApp transactions.
//!
//! This module provides minimal wrapper types for submitting zkApp transactions
//! via the Mina GraphQL API. It follows the same pattern established in
//! [`crate::graphql`] for legacy transactions.
//!
//! The internal [`ZKAppCommand`] type's serde serialization is assumed to be
//! correct for GraphQL submission, so this module only provides the necessary
//! GraphQL wrapper structure.

use crate::{graphql::GraphqlRequest, transactions::zkapp_tx::ZKAppCommand};
use alloc::string::ToString;
use serde::Serialize;

// -------------------------------------------------------------------------------------------------
// -------------------------------- GraphQL Variables Wrapper --------------------------------------
// -------------------------------------------------------------------------------------------------

/// Variables for the sendZkapp mutation
#[derive(Serialize)]
pub struct SendZkappVariables<'a> {
    pub input: SendZkappInput<'a>,
}

// -------------------------------------------------------------------------------------------------
// -------------------------------- Top-Level Input Types ------------------------------------------
// -------------------------------------------------------------------------------------------------

/// Input type for sendZkapp mutation.
///
/// This wraps a reference to [`ZKAppCommand`] and serializes it with the
/// correct GraphQL field name (`zkappCommand`).
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendZkappInput<'a> {
    pub zkapp_command: &'a ZKAppCommand,
}

// -------------------------------------------------------------------------------------------------
// -------------------------------- GraphQL Mutation Builder ---------------------------------------
// -------------------------------------------------------------------------------------------------

/// Build a sendZkapp mutation request for submitting zkApp transactions.
///
/// This function creates a complete GraphQL request that can be serialized to JSON
/// and sent to a Mina node's GraphQL endpoint.
///
/// # Example
/// ```ignore
/// use mina_tx::zkapp_tx::{ZKAppCommand, zkapp_graphql::build_send_zkapp_mutation};
///
/// let zkapp_command: ZKAppCommand = /* ... */;
/// let request = build_send_zkapp_mutation(&zkapp_command);
/// // Serialize and send to Mina GraphQL endpoint
/// let json = serde_json::to_string(&request)?;
/// ```
pub fn build_send_zkapp_mutation(cmd: &ZKAppCommand) -> GraphqlRequest<SendZkappVariables<'_>> {
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
            input: SendZkappInput { zkapp_command: cmd },
        }),
    }
}

// -------------------------------------------------------------------------------------------------
// -------------------------------- Tests ----------------------------------------------------------
// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transactions::zkapp_tx::{
        AccountUpdate, AccountUpdateBody, Actions, Authorization, AuthorizationKind, BalanceChange,
        Events, FeePayer, FeePayerBody, MayUseToken, Preconditions, PublicKey, TokenId, Update,
        ZKAppCommand, MEMO_BYTES,
    };
    use mina_signer::CompressedPubKey;

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
                call_data: Default::default(),
                call_depth: 0,
                preconditions: Preconditions::default(),
                use_full_commitment: true,
                implicit_account_creation_fee: false,
                may_use_token: MayUseToken::default(),
                authorization_kind: AuthorizationKind::default(),
            },
            authorization: Authorization {
                proof: None,
                signature: Some("test_sig".to_string()),
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
    fn test_send_zkapp_input_serialization() {
        let cmd = create_minimal_zkapp_command();
        let input = SendZkappInput {
            zkapp_command: &cmd,
        };

        let json = serde_json::to_string(&input).expect("Should serialize");

        // Check key fields are present with camelCase
        assert!(json.contains("\"zkappCommand\""));
        assert!(json.contains("\"feePayer\""));
        assert!(json.contains("\"accountUpdates\""));
        assert!(json.contains("\"memo\""));
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
    fn test_zkapp_command_fields_serialized_correctly() {
        let cmd = create_minimal_zkapp_command();
        let request = build_send_zkapp_mutation(&cmd);

        let json = serde_json::to_string(&request).expect("Should serialize");

        // Fee payer fields
        assert!(json.contains("\"publicKey\""));
        assert!(json.contains("\"fee\":\"100000000\"")); // Serialized as string
        assert!(json.contains("\"nonce\":\"5\"")); // Serialized as string
        assert!(json.contains("\"validUntil\":\"1000\"")); // Serialized as string

        // Balance change
        assert!(json.contains("\"magnitude\":\"50000000\"")); // Serialized as string
        assert!(json.contains("\"sgn\":\"Negative\"")); // Serialized as Positive/Negative

        // Memo should be base58check encoded (not raw bytes)
        assert!(!json.contains("[1,4,")); // Not raw array
    }

    #[test]
    fn test_memo_is_base58check_encoded() {
        let cmd = create_minimal_zkapp_command();
        let json = serde_json::to_string(&cmd).expect("Should serialize");

        // Memo should be a string, not an array
        // The base58check encoding of a memo starting with [0x01, 0x04, ...] should be a string
        assert!(json.contains("\"memo\":\""));
        // Should not contain array notation for memo
        let memo_check = json.find("\"memo\":");
        assert!(memo_check.is_some());
    }
}
