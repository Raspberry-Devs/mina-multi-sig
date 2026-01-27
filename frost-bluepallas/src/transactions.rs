//! This file defines the generic TransactionEnvelope structure that encapsulates all kinds of transactions that Mina supports. It is the structure that we sign using FROST.

use crate::{
    errors::BluePallasError,
    mina_compat::Sig,
    transactions::{
        legacy_tx::LegacyTransaction,
        network_id::NetworkIdEnvelope,
        zkapp_tx::{
            zkapp_display::json_display, zkapp_graphql, ZKAppCommand, ZKAppCommandHashable,
        },
    },
};
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use mina_hasher::Hashable;
use mina_signer::NetworkId;
use serde::{Deserialize, Serialize};

pub mod legacy_tx;
pub mod network_id;
pub mod zkapp_tx;

const MEMO_BYTES: usize = 34;
const MEMO_HEADER_BYTES: usize = 2; // 0x01 + length byte

// Enum distinguishing between legacy and zkApp transactions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "tag", content = "transaction")]
pub enum TransactionKind {
    ZkApp(ZKAppCommand),
    Legacy(LegacyTransaction),
}

impl TransactionKind {
    pub fn new_zkapp(tx: ZKAppCommand) -> Self {
        TransactionKind::ZkApp(tx)
    }

    pub fn new_legacy(tx: LegacyTransaction) -> Self {
        TransactionKind::Legacy(tx)
    }
    pub fn is_legacy(&self) -> bool {
        match self {
            TransactionKind::Legacy(_) => true,
            TransactionKind::ZkApp(_) => false,
        }
    }
}

// The TransactionEnvelope encapsulates either a legacy transaction or a zkApp transaction along with the network ID.
// Should be the only structure necessary to access when signing transactions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TransactionEnvelope {
    network_id: NetworkIdEnvelope,
    kind: TransactionKind,
}

impl core::fmt::Display for TransactionEnvelope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        json_display(self, f)
    }
}

impl TransactionEnvelope {
    pub fn new(network_id: NetworkId, kind: TransactionKind) -> Self {
        Self {
            network_id: network_id.into(),
            kind,
        }
    }

    pub fn new_zkapp(network_id: NetworkId, tx: ZKAppCommand) -> Self {
        Self::new(network_id, TransactionKind::new_zkapp(tx))
    }

    pub fn new_legacy(network_id: NetworkId, tx: LegacyTransaction) -> Self {
        Self::new(network_id, TransactionKind::new_legacy(tx))
    }

    /// Parse a Legacy or ZkApp transaction from a JSON string.
    /// Auto-detects the transaction type by attempting to parse as each type.
    /// Tries ZkApp first, then Legacy.
    /// Returns an error if parsing fails for both types.
    pub fn from_str_network(
        s: &str,
        network_id: NetworkIdEnvelope,
    ) -> Result<Self, BluePallasError> {
        let s = s.trim();

        // Try parsing as ZkApp transaction first
        if let Ok(zkapp) = serde_json::from_str::<ZKAppCommand>(s) {
            return Ok(Self::new_zkapp(network_id.0, zkapp));
        }

        // Try parsing as Legacy transaction
        if let Ok(legacy) = serde_json::from_str::<LegacyTransaction>(s) {
            return Ok(Self::new_legacy(network_id.0, legacy));
        }

        // Neither worked, return an error
        Err(BluePallasError::UnknownTransactionType(
            "Unable to parse transaction. Expected a valid legacy transaction or ZkApp transaction JSON.".to_string()
        ))
    }

    pub fn inner(&self) -> &TransactionKind {
        &self.kind
    }

    /// Get a mutable reference to the inner transaction kind.
    pub fn inner_mut(&mut self) -> &mut TransactionKind {
        &mut self.kind
    }

    /// Serialize the TransactionEnvelope to a byte vector using serde.
    pub fn serialize(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }

    /// Deserialize a TransactionEnvelope from a byte slice using serde.
    pub fn deserialize(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(bytes)
    }

    pub fn network_id(&self) -> NetworkId {
        self.network_id.0.clone()
    }

    pub fn is_legacy(&self) -> bool {
        self.kind.is_legacy()
    }

    pub fn to_graphql_query_json(&self, signature: Sig) -> Result<String, serde_json::Error> {
        match &self.kind {
            TransactionKind::ZkApp(zkapp) => {
                let mutation = zkapp_graphql::build_send_zkapp_mutation(zkapp);
                serde_json::to_string_pretty(&mutation)
            }
            TransactionKind::Legacy(legacy_tx) => {
                let sig_input = crate::graphql::SignatureInput::FieldScalar {
                    field: signature.field.to_string(),
                    scalar: signature.scalar.to_string(),
                };

                if legacy_tx.is_delegation() {
                    let input = crate::graphql::SendDelegationInput::from(legacy_tx);
                    let mutation = crate::graphql::build_send_delegation_mutation(input, sig_input);
                    serde_json::to_string_pretty(&mutation)
                } else {
                    let input = crate::graphql::SendPaymentInput::from(legacy_tx);
                    let mutation = crate::graphql::build_send_payment_mutation(input, sig_input);
                    serde_json::to_string_pretty(&mutation)
                }
            }
        }
    }
}

impl Hashable for TransactionEnvelope {
    type D = NetworkId;

    fn domain_string(domain_param: Self::D) -> Option<String> {
        match domain_param {
            NetworkId::MAINNET => "MinaSignatureMainnet",
            NetworkId::TESTNET => "CodaSignature",
        }
        .to_string()
        .into()
    }

    fn to_roinput(&self) -> mina_hasher::ROInput {
        match &self.kind {
            TransactionKind::ZkApp(zkapp_tx) => {
                let zkapp_hashable = ZKAppCommandHashable::new(zkapp_tx, self.network_id.0.clone());
                zkapp_hashable.to_roinput()
            }
            TransactionKind::Legacy(legacy_tx) => legacy_tx.to_roinput(),
        }
    }
}

#[cfg(test)]
mod tests {
    use mina_signer::Keypair;

    use crate::{
        errors::BluePallasError, transactions::zkapp_tx::zkapp_test_vectors::get_zkapp_test_vectors,
    };

    use super::*;

    #[test]
    fn test_transaction_envelope_serialization_roundtrip() {
        let private_key_hex = "35dcca7620128d240cc3319c83dc6402ad439038361ba853af538a4cea3ddabc";
        let mina_keypair = Keypair::from_hex(private_key_hex)
            .map_err(|_| BluePallasError::InvalidSignature("Failed to parse keypair".into()))
            .unwrap();

        let legacy_tx = LegacyTransaction::new_payment(
            mina_keypair.public.clone(),
            mina_keypair.public.clone(),
            1000,
            1,
            0,
        );

        let envelope = TransactionEnvelope::new_legacy(NetworkId::TESTNET, legacy_tx);

        let serialized = envelope.serialize().expect("Serialization failed");
        let deserialized =
            TransactionEnvelope::deserialize(&serialized).expect("Deserialization failed");

        assert_eq!(deserialized, envelope);
    }

    #[test]
    fn test_transaction_envelope_zkapp_roundtrip() {
        // Iterate through each test vector in test_vectors
        let test_vectors = get_zkapp_test_vectors();

        for tv in test_vectors {
            let network_id = tv.network;
            let zkapp_tx = tv.zkapp_command;

            let envelope = TransactionEnvelope::new_zkapp(network_id, zkapp_tx.clone());

            let serialized = envelope.serialize().expect("Serialization failed");
            let deserialized =
                TransactionEnvelope::deserialize(&serialized).expect("Deserialization failed");

            assert_eq!(
                deserialized, envelope,
                "ZkApp TransactionEnvelope roundtrip failed for test vector {}",
                tv.name
            );
        }
    }

    #[test]
    fn test_from_str_network_legacy_payment() {
        let json = r#"{
            "to": "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            "from": "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            "fee": "10000",
            "amount": "1000000",
            "nonce": "42",
            "memo": "test",
            "valid_until": "12345",
            "tag": [false, false, false]
        }"#;

        let result = TransactionEnvelope::from_str_network(
            json,
            NetworkIdEnvelope::from(NetworkId::TESTNET),
        );
        assert!(result.is_ok());
        let envelope = result.unwrap();
        assert!(envelope.is_legacy());
        assert_eq!(envelope.network_id() as u8, 0);
    }

    #[test]
    fn test_from_str_network_zkapp() {
        let json = include_str!("../tests/data/payment-zkapp.json");
        let result = TransactionEnvelope::from_str_network(
            json,
            NetworkIdEnvelope::from(NetworkId::MAINNET),
        );
        assert!(result.is_ok());
        let envelope = result.unwrap();
        assert!(!envelope.is_legacy());
        assert_eq!(envelope.network_id() as u8, 1);
    }

    #[test]
    fn test_from_str_network_invalid_json() {
        let result = TransactionEnvelope::from_str_network(
            "not json",
            NetworkIdEnvelope::from(NetworkId::TESTNET),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_from_str_network_unrecognized_type() {
        let json = r#"{"unknown": "field"}"#;
        let result = TransactionEnvelope::from_str_network(
            json,
            NetworkIdEnvelope::from(NetworkId::TESTNET),
        );
        assert!(result.is_err());
    }
}
