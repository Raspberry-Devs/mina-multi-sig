use crate::transactions::legacy_tx::LegacyTransaction;
use alloc::string::{String, ToString};
use serde::Serialize;

// ------------------------------- GraphQL Request Structs --------------------------------
#[derive(Serialize)]
pub struct GraphqlRequest<V> {
    #[serde(rename = "operationName", skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<V>,
}

#[derive(Serialize)]
pub struct SendPaymentSignedVariables {
    pub input: SendPaymentInput,
    pub signature: SignatureInput,
}

#[derive(Serialize)]
pub struct SendDelegationSignedVariables {
    pub input: SendDelegationInput,
    pub signature: SignatureInput,
}

#[derive(Serialize)]
pub struct SendPaymentInput {
    pub from: String,
    pub to: String,
    pub amount: String,
    pub fee: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[serde(rename = "validUntil", skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<u32>,
}

impl From<&LegacyTransaction> for SendPaymentInput {
    fn from(tx: &LegacyTransaction) -> Self {
        SendPaymentInput {
            from: tx.source_pk.clone().into_address(),
            to: tx.receiver_pk.clone().into_address(),
            amount: tx.amount.to_string(),
            fee: tx.fee.to_string(),
            nonce: Option::Some(tx.nonce),
            memo: Option::Some(tx.get_memo_string().unwrap_or_default()),
            valid_until: Option::Some(tx.valid_until),
        }
    }
}

#[derive(Serialize)]
pub struct SendDelegationInput {
    pub from: String,
    pub to: String,
    pub fee: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[serde(rename = "validUntil", skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<u32>,
}

impl From<&LegacyTransaction> for SendDelegationInput {
    fn from(tx: &LegacyTransaction) -> Self {
        SendDelegationInput {
            from: tx.source_pk.clone().into_address(),
            to: tx.receiver_pk.clone().into_address(),
            fee: tx.fee.to_string(),
            nonce: Option::Some(tx.nonce),
            memo: Option::Some(tx.get_memo_string().unwrap_or_default()),
            valid_until: Option::Some(tx.valid_until),
        }
    }
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum SignatureInput {
    Raw {
        #[serde(rename = "rawSignature")]
        raw_signature: String,
    },
    FieldScalar {
        field: String,
        scalar: String,
    },
}

// ------------------------------- GraphQL Request Builders --------------------------------
pub fn build_send_payment_mutation(
    input: SendPaymentInput,
    signature: SignatureInput,
) -> GraphqlRequest<SendPaymentSignedVariables> {
    let query = r#"
mutation SendPaymentSigned($input: SendPaymentInput!, $signature: SignatureInput!) {
  sendPayment(input: $input, signature: $signature) {
    payment {
      id
      hash
      kind
      nonce
      from
      to
      amount
      fee
      memo
      failureReason
    }
  }
}
"#
    .trim()
    .to_string();

    GraphqlRequest {
        operation_name: Some("SendPaymentSigned".to_string()),
        query,
        variables: Some(SendPaymentSignedVariables { input, signature }),
    }
}

pub fn build_send_delegation_mutation(
    input: SendDelegationInput,
    signature: SignatureInput,
) -> GraphqlRequest<SendDelegationSignedVariables> {
    let query = r#"
mutation SendDelegationSigned($input: SendDelegationInput!, $signature: SignatureInput!) {
  sendDelegation(input: $input, signature: $signature) {
    delegation {
      id
      hash
      kind
      nonce
      from
      to
      fee
      memo
      failureReason
      ... on UserCommandDelegation {
        delegator { publicKey }
        delegatee { publicKey }
      }
    }
  }
}
"#
    .trim()
    .to_string();

    GraphqlRequest {
        operation_name: Some("SendDelegationSigned".to_string()),
        query,
        variables: Some(SendDelegationSignedVariables { input, signature }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mina_signer::{PubKey, SecKey};
    use rand_core::SeedableRng;

    fn create_test_pubkey(seed: [u8; 32]) -> PubKey {
        let mut rng = rand_chacha::ChaCha12Rng::from_seed(seed);
        let sec = SecKey::rand(&mut rng);
        PubKey::from_secret_key(sec).unwrap()
    }

    #[test]
    fn test_send_payment_input_from_legacy_transaction() {
        let from = create_test_pubkey([1; 32]);
        let to = create_test_pubkey([2; 32]);
        let tx = LegacyTransaction::new_payment(from, to, 1_000_000, 10_000, 42)
            .set_memo_str("test memo")
            .unwrap()
            .set_valid_until(12345);

        let input = SendPaymentInput::from(&tx);

        assert_eq!(input.from, tx.source_pk.into_address());
        assert_eq!(input.to, tx.receiver_pk.into_address());
        assert_eq!(input.amount, "1000000");
        assert_eq!(input.fee, "10000");
        assert_eq!(input.nonce, Some(42));
        assert_eq!(input.memo, Some("test memo".to_string()));
        assert_eq!(input.valid_until, Some(12345));
    }

    #[test]
    fn test_send_payment_input_serialization() {
        let from = create_test_pubkey([3; 32]);
        let to = create_test_pubkey([4; 32]);
        let tx = LegacyTransaction::new_payment(from, to, 500_000, 5_000, 10)
            .set_memo_str("payment")
            .unwrap();

        let input = SendPaymentInput::from(&tx);
        let json = serde_json::to_string(&input).unwrap();

        assert!(json.contains("\"from\""));
        assert!(json.contains("\"to\""));
        assert!(json.contains("\"amount\":\"500000\""));
        assert!(json.contains("\"fee\":\"5000\""));
        assert!(json.contains("\"nonce\":10"));
        assert!(json.contains("\"memo\":\"payment\""));
    }

    #[test]
    fn test_send_payment_input_omits_none_fields() {
        let input = SendPaymentInput {
            from: "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg".to_string(),
            to: "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg".to_string(),
            amount: "1000".to_string(),
            fee: "100".to_string(),
            nonce: None,
            memo: None,
            valid_until: None,
        };

        let json = serde_json::to_string(&input).unwrap();

        assert!(!json.contains("\"nonce\""));
        assert!(!json.contains("\"memo\""));
        assert!(!json.contains("\"validUntil\""));
    }

    #[test]
    fn test_send_delegation_input_from_legacy_transaction() {
        let from = create_test_pubkey([5; 32]);
        let to = create_test_pubkey([6; 32]);
        let tx = LegacyTransaction::new_delegation(from, to, 8_000, 100)
            .set_memo_str("delegate")
            .unwrap()
            .set_valid_until(54321);

        let input = SendDelegationInput::from(&tx);

        assert_eq!(input.from, tx.source_pk.into_address());
        assert_eq!(input.to, tx.receiver_pk.into_address());
        assert_eq!(input.fee, "8000");
        assert_eq!(input.nonce, Some(100));
        assert_eq!(input.memo, Some("delegate".to_string()));
        assert_eq!(input.valid_until, Some(54321));
    }

    #[test]
    fn test_send_delegation_input_serialization() {
        let from = create_test_pubkey([7; 32]);
        let to = create_test_pubkey([8; 32]);
        let tx = LegacyTransaction::new_delegation(from, to, 3_000, 50);

        let input = SendDelegationInput::from(&tx);
        let json = serde_json::to_string(&input).unwrap();

        assert!(json.contains("\"from\""));
        assert!(json.contains("\"to\""));
        assert!(json.contains("\"fee\":\"3000\""));
        assert!(json.contains("\"nonce\":50"));
        assert!(!json.contains("\"amount\"")); // Should not have amount field
    }

    #[test]
    fn test_signature_input_raw_serialization() {
        let sig = SignatureInput::Raw {
            raw_signature: "7mXGPCbsJJaV5GgGxn3xVqPHfH3V91M5VxmJLBqDzx5g".to_string(),
        };

        let json = serde_json::to_string(&sig).unwrap();
        assert!(json.contains("\"rawSignature\""));
        assert!(json.contains("7mXGPCbsJJaV5GgGxn3xVqPHfH3V91M5VxmJLBqDzx5g"));
    }

    #[test]
    fn test_signature_input_field_scalar_serialization() {
        let sig = SignatureInput::FieldScalar {
            field: "field_value".to_string(),
            scalar: "scalar_value".to_string(),
        };

        let json = serde_json::to_string(&sig).unwrap();
        assert!(json.contains("\"field\":\"field_value\""));
        assert!(json.contains("\"scalar\":\"scalar_value\""));
        assert!(!json.contains("rawSignature"));
    }

    #[test]
    fn test_build_send_payment_mutation_structure() {
        let input = SendPaymentInput {
            from: "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg".to_string(),
            to: "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg".to_string(),
            amount: "1000000".to_string(),
            fee: "10000".to_string(),
            nonce: Some(42),
            memo: Some("test".to_string()),
            valid_until: Some(12345),
        };

        let signature = SignatureInput::Raw {
            raw_signature: "test_sig".to_string(),
        };

        let request = build_send_payment_mutation(input, signature);

        assert_eq!(
            request.operation_name,
            Some("SendPaymentSigned".to_string())
        );
        assert!(request.query.contains("mutation SendPaymentSigned"));
        assert!(request.query.contains("sendPayment"));
        assert!(request.query.contains("$input: SendPaymentInput!"));
        assert!(request.query.contains("$signature: SignatureInput!"));
        assert!(request.variables.is_some());
    }

    #[test]
    fn test_build_send_delegation_mutation_structure() {
        let input = SendDelegationInput {
            from: "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg".to_string(),
            to: "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg".to_string(),
            fee: "5000".to_string(),
            nonce: Some(10),
            memo: None,
            valid_until: None,
        };

        let signature = SignatureInput::FieldScalar {
            field: "field".to_string(),
            scalar: "scalar".to_string(),
        };

        let request = build_send_delegation_mutation(input, signature);

        assert_eq!(
            request.operation_name,
            Some("SendDelegationSigned".to_string())
        );
        assert!(request.query.contains("mutation SendDelegationSigned"));
        assert!(request.query.contains("sendDelegation"));
        assert!(request.query.contains("$input: SendDelegationInput!"));
        assert!(request.query.contains("$signature: SignatureInput!"));
        assert!(request.query.contains("UserCommandDelegation"));
        assert!(request.variables.is_some());
    }

    #[test]
    fn test_graphql_request_serialization_complete() {
        let input = SendPaymentInput {
            from: "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg".to_string(),
            to: "B62qkrpyTw7KhiycGKjKqXeT4Fi8zxMdmzF4dXXq2Sx2bcQVPCCCXkZ".to_string(),
            amount: "2500000".to_string(),
            fee: "15000".to_string(),
            nonce: Some(5),
            memo: Some("full test".to_string()),
            valid_until: Some(99999),
        };

        let signature = SignatureInput::Raw {
            raw_signature: "sig123".to_string(),
        };

        let request = build_send_payment_mutation(input, signature);
        let json = serde_json::to_string(&request).unwrap();

        assert!(json.contains("\"operationName\":\"SendPaymentSigned\""));
        assert!(json.contains("\"query\""));
        assert!(json.contains("\"variables\""));
        assert!(json.contains("\"input\""));
        assert!(json.contains("\"signature\""));
    }

    #[test]
    fn test_empty_memo_handling() {
        let from = create_test_pubkey([9; 32]);
        let to = create_test_pubkey([10; 32]);
        let tx = LegacyTransaction::new_payment(from, to, 100_000, 1_000, 1);
        // No memo set - should use default empty memo

        let input = SendPaymentInput::from(&tx);

        // Empty memo should serialize as empty string
        assert_eq!(input.memo, Some(String::new()));
    }

    #[test]
    fn test_max_values_serialization() {
        let input = SendPaymentInput {
            from: "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg".to_string(),
            to: "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg".to_string(),
            amount: u64::MAX.to_string(),
            fee: u64::MAX.to_string(),
            nonce: Some(u32::MAX),
            memo: Some("x".repeat(32)), // Max memo length
            valid_until: Some(u32::MAX),
        };

        let json = serde_json::to_string(&input).unwrap();
        assert!(json.contains(&u64::MAX.to_string()));
        assert!(json.contains(&u32::MAX.to_string()));
    }

    #[test]
    fn test_graphql_request_omits_none_operation_name() {
        let request: GraphqlRequest<()> = GraphqlRequest {
            operation_name: None,
            query: "query { test }".to_string(),
            variables: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(!json.contains("operationName"));
    }

    #[test]
    fn test_graphql_request_omits_none_variables() {
        let request: GraphqlRequest<()> = GraphqlRequest {
            operation_name: Some("TestOp".to_string()),
            query: "query TestOp { test }".to_string(),
            variables: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(!json.contains("variables"));
        assert!(json.contains("\"operationName\":\"TestOp\""));
    }
}
