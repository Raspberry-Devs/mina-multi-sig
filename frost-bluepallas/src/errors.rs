//! Error types for the frost-bluepallas library

use alloc::{boxed::Box, string::String};
use core::{error, fmt, result::Result};

// TODO: Replace with BluePallasError within
pub type BluePallasResult<T> = Result<T, Box<dyn error::Error>>;

/// Error enum for frost-bluepallas operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BluePallasError {
    /// Serialization operation failed
    SerializationError(String),

    /// Deserialization operation failed
    DeSerializationError(String),

    /// No messages have been provided for signing
    NoMessageProvided,

    /// Saving Signature failed
    SaveSignatureError(String),

    /// Invalid Memo provided
    InvalidMemo(String),
}

impl fmt::Display for BluePallasError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BluePallasError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            BluePallasError::DeSerializationError(msg) => {
                write!(f, "Deserialization error: {}", msg)
            }
            BluePallasError::NoMessageProvided => {
                write!(f, "No messages have been provided for signing")
            }
            BluePallasError::SaveSignatureError(msg) => {
                write!(f, "Failed to save signature: {}", msg)
            }
            BluePallasError::InvalidMemo(msg) => write!(f, "Invalid memo: {}", msg),
        }
    }
}

impl error::Error for BluePallasError {}

// Convenience constructors
impl BluePallasError {
    /// Create a serialization error with a custom message
    pub fn serialization_error(message: impl Into<String>) -> Self {
        BluePallasError::SerializationError(message.into())
    }

    /// Create a deserialization error with a custom message
    pub fn deserialization_error(message: impl Into<String>) -> Self {
        BluePallasError::DeSerializationError(message.into())
    }

    /// Create an invalid memo error with a custom message
    pub fn invalid_memo(message: impl Into<String>) -> Self {
        BluePallasError::InvalidMemo(message.into())
    }
}
