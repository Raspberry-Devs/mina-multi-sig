use mina_hasher::Hashable;
use serde::{Deserialize, Serialize};

use crate::{
    errors::BluePallasError,
    transactions::{legacy_tx, zkapp_tx::ZKAppCommandWithNetwork},
};

pub trait Transaction: Serialize + for<'de> Deserialize<'de> + Clone + Hashable {}
impl Transaction for ZKAppCommandWithNetwork {}
impl Transaction for legacy_tx::Transaction {}

#[derive(Clone)]
pub enum TransactionEnvelope {
    ZkApp(ZKAppCommandWithNetwork),
    Legacy(legacy_tx::Transaction),
}

impl TransactionEnvelope {
    pub fn new_zkapp(tx: ZKAppCommandWithNetwork) -> Self {
        TransactionEnvelope::ZkApp(tx)
    }

    pub fn new_legacy(tx: legacy_tx::Transaction) -> Self {
        TransactionEnvelope::Legacy(tx)
    }

    pub fn deserialize(transaction_str: &str) -> Result<Self, BluePallasError> {
        // Attempt to deserialize as ZKApp first
        if let Ok(zkapp_tx) = serde_json::from_str::<ZKAppCommandWithNetwork>(transaction_str) {
            return Ok(TransactionEnvelope::ZkApp(zkapp_tx));
        }
        // Fallback to Legacy transaction
        if let Ok(legacy_tx) = serde_json::from_str::<legacy_tx::Transaction>(transaction_str) {
            return Ok(TransactionEnvelope::Legacy(legacy_tx));
        }

        Err(BluePallasError::UnknownTransactionType(
            "Transaction string does not match known formats".to_string(),
        ))
    }
}

impl Hashable for TransactionEnvelope {
    type D = mina_signer::NetworkId;

    fn to_roinput(&self) -> mina_hasher::ROInput {
        match self {
            TransactionEnvelope::ZkApp(tx) => tx.to_roinput(),
            TransactionEnvelope::Legacy(tx) => tx.to_roinput(),
        }
    }

    fn domain_string(domain_param: Self::D) -> Option<String> {
        match domain_param {
            mina_signer::NetworkId::MAINNET => "MinaSignatureMainnet",
            mina_signer::NetworkId::TESTNET => "CodaSignature",
        }
        .to_string()
        .into()
    }
}
