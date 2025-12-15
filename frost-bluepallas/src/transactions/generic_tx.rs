use crate::transactions::{legacy_tx, network_id_serde::NetworkIdSerde, zkapp_tx::ZKAppCommand};
use mina_signer::NetworkId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
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

#[derive(Clone, Serialize, Deserialize)]
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
}
