//! Error types for the frost-bluepallas library

use std::fmt;

// TODO: Replace with BluePallasError within
pub type BluePallasResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Error enum for frost-bluepallas operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BluePallasError {
    /// Network ID has not been set for the current thread
    NetworkIdNotSet,

    /// Participant ID must be non-zero
    NonZeroParticipantID,

    /// Serialization operation failed
    SerializationError(String),

    /// Deserialization operation failed
    DeSerializationError(String),

    /// Invalid commitment provided
    InvalidCommitment(String),

    /// Invalid Signature provided
    InvalidSignature(String),

    /// Invalid Public Key
    InvalidPublicKey(String),
}

impl fmt::Display for BluePallasError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BluePallasError::NetworkIdNotSet => {
                write!(f, "NetworkId not set. Call set_network_id() first.")
            }
            BluePallasError::NonZeroParticipantID => write!(f, "Participant ID should be nonzero"),
            BluePallasError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            BluePallasError::DeSerializationError(msg) => {
                write!(f, "Deserialization error: {}", msg)
            }
            BluePallasError::InvalidCommitment(msg) => write!(f, "Invalid commitment: {}", msg),
            BluePallasError::InvalidSignature(msg) => write!(f, "Invalid signature: {}", msg),
            BluePallasError::InvalidPublicKey(msg) => write!(f, "Invalid public key: {}", msg),
        }
    }
}

impl std::error::Error for BluePallasError {}

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

    /// Create an invalid commitment error with a custom message
    pub fn invalid_commitment(message: impl Into<String>) -> Self {
        BluePallasError::InvalidCommitment(message.into())
    }
}
