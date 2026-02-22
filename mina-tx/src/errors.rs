//! Error types for the frost-bluepallas library

use alloc::string::String;
use core::{error, fmt};

pub type MinaTxResult<T> = Result<T, Box<dyn error::Error>>;

/// Error enum for frost-bluepallas operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MinaTxError {
    /// Serialization operation failed
    SerializationError(String),

    /// Deserialization operation failed
    DeSerializationError(String),

    /// Invalid Signature provided
    InvalidSignature(String),

    /// Invalid Public Key
    InvalidPublicKey(String),

    /// Invalid Memo provided
    InvalidMemo(String),

    /// Memo serialization failed
    MemoSerializationError(String),

    /// Invalid ZKApp Command structure
    InvalidZkAppCommand(String),

    /// Unable to save signature to output file or stdout
    SaveSignatureError(String),

    /// Unknown transaction type during deserialization
    UnknownTransactionType(String),
}

impl fmt::Display for MinaTxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MinaTxError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            MinaTxError::DeSerializationError(msg) => {
                write!(f, "Deserialization error: {}", msg)
            }
            MinaTxError::InvalidSignature(msg) => write!(f, "Invalid signature: {}", msg),
            MinaTxError::InvalidPublicKey(msg) => write!(f, "Invalid public key: {}", msg),
            MinaTxError::InvalidMemo(msg) => write!(f, "Invalid memo: {}", msg),
            MinaTxError::MemoSerializationError(msg) => {
                write!(f, "Memo serialization error: {}", msg)
            }
            MinaTxError::InvalidZkAppCommand(msg) => {
                write!(f, "Invalid ZKApp command: {}", msg)
            }
            MinaTxError::SaveSignatureError(msg) => {
                write!(f, "Failed to save signature: {}", msg)
            }
            MinaTxError::UnknownTransactionType(msg) => {
                write!(f, "Unknown transaction type: {}", msg)
            }
        }
    }
}

impl error::Error for MinaTxError {}

// Convenience constructors
impl MinaTxError {
    /// Create a serialization error with a custom message
    pub fn serialization_error(message: impl Into<String>) -> Self {
        MinaTxError::SerializationError(message.into())
    }

    /// Create a deserialization error with a custom message
    pub fn deserialization_error(message: impl Into<String>) -> Self {
        MinaTxError::DeSerializationError(message.into())
    }

    /// Create an invalid memo error with a custom message
    pub fn invalid_memo(message: impl Into<String>) -> Self {
        MinaTxError::InvalidMemo(message.into())
    }
}
