use crate::transactions::{
    legacy_tx,
    network_id_serde::NetworkIdSerde,
    zkapp_tx::{zkapp_hashable::ZKAppCommandHashable, ZKAppCommand},
};
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use mina_hasher::Hashable;
use mina_signer::NetworkId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "tag", content = "transaction")]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TransactionEnvelope {
    network_id: NetworkIdSerde,
    kind: TransactionKind,
}

impl core::fmt::Display for TransactionEnvelope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "TransactionEnvelope(network_id: {:?}, kind: {:?})",
            self.network_id.0, self.kind
        )
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

    pub fn new_legacy(network_id: NetworkId, tx: legacy_tx::Transaction) -> Self {
        Self::new(network_id, TransactionKind::new_legacy(tx))
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

        let legacy_tx = legacy_tx::Transaction::new_payment(
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
}
