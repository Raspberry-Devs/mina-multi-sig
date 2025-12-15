use crate::transactions::{
    legacy_tx,
    network_id_serde::NetworkIdSerde,
    zkapp_tx::{zkapp_hashable::ZKAppCommandHashable, ZKAppCommand},
};
use mina_hasher::Hashable;
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
