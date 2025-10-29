use crate::{
    errors::BluePallasError,
    transactions::{legacy_tx, zkapp_tx::ZKAppCommandWithNetwork},
    translate::Translatable,
};

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

impl Translatable for TransactionEnvelope {
    fn translate_msg(&self) -> Vec<u8> {
        match self {
            TransactionEnvelope::ZkApp(tx) => tx.translate_msg(),
            TransactionEnvelope::Legacy(tx) => tx.translate_msg(),
        }
    }
}
