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
            fee: tx.fee.clone().to_string(),
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
