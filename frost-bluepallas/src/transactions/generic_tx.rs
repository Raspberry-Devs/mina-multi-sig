use crate::{
    errors::BluePallasError,
    transactions::{legacy_tx, zkapp_tx::ZKAppCommand},
};

#[derive(Clone)]
pub enum TransactionEnvelope {
    ZkApp(ZKAppCommand),
    Legacy(legacy_tx::Transaction),
}

impl TransactionEnvelope {
    pub fn new_zkapp(tx: ZKAppCommand) -> Self {
        TransactionEnvelope::ZkApp(tx)
    }

    pub fn new_legacy(tx: legacy_tx::Transaction) -> Self {
        TransactionEnvelope::Legacy(tx)
    }

    pub fn deserialize(transaction_str: &str) -> Result<Self, BluePallasError> {
        // Attempt to deserialize as ZKApp first
        if let Ok(zkapp_tx) = serde_json::from_str::<ZKAppCommand>(transaction_str) {
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

    /// Serialize into bytes using binary format
    pub fn serialize(&self) -> Result<Vec<u8>, BluePallasError> {
        match self {
            TransactionEnvelope::ZkApp(tx) => postcard::to_allocvec(tx).map_err(|e| {
                BluePallasError::UnknownTransactionType(format!(
                    "Failed to serialize ZKApp transaction: {}",
                    e
                ))
            }),
            TransactionEnvelope::Legacy(tx) => postcard::to_allocvec(tx).map_err(|e| {
                BluePallasError::UnknownTransactionType(format!(
                    "Failed to serialize Legacy transaction: {}",
                    e
                ))
            }),
        }
    }
}
