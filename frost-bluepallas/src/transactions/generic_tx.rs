use crate::transactions::{
    legacy_tx,
    network_id_serde::NetworkIdSerde,
    zkapp_tx::{zkapp_hashable::ZKAppCommandHashable, ZKAppCommand},
};
use mina_hasher::Hashable;
use mina_signer::NetworkId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", content = "transaction")]
pub enum TransactionKind {
    ZkApp(ZKAppCommand),
    Legacy(legacy_tx::Transaction),
}

impl TransactionKind {
    pub fn new_zkapp(tx: ZKAppCommand) -> Self {
        TransactionKind::ZkApp(tx)
    }

    pub fn new_legacy(tx: legacy_tx::Transaction) -> Self {
        TransactionKind::Legacy(tx)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionEnvelope {
    network_id: NetworkIdSerde,
    kind: TransactionKind,
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

    pub fn new_legacy(network_id: NetworkId, tx: legacy_tx::Transaction) -> Self {
        Self::new(network_id, TransactionKind::new_legacy(tx))
    }

    /// Serialize the TransactionEnvelope to a byte vector using postcard.
    pub fn serialize(&self) -> Result<Vec<u8>, postcard::Error> {
        postcard::to_allocvec(self)
    }

    /// Deserialize a TransactionEnvelope from a byte slice using postcard.
    pub fn deserialize(bytes: &[u8]) -> Result<Self, postcard::Error> {
        postcard::from_bytes(bytes)
    }

    pub fn network_id(&self) -> NetworkId {
        self.network_id.0.clone()
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
    use super::*;

    #[test]
    fn test_transaction_envelope_serialization_roundtrip() {
        let legacy_tx = legacy_tx::Transaction::new_payment(
            mina_signer::PubKey::from_hex(
                "B62qj9Y5Z5bY3N6G6u8g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6A7B8C9D",
            )
            .unwrap(),
            mina_signer::PubKey::from_hex(
                "B62qkL1M2N3O4P5Q6R7S8T9U0V1W2X3Y4Z5a6b7c8d9e0f1g2h3i4j5k6l7m8n9o0p",
            )
            .unwrap(),
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
}
