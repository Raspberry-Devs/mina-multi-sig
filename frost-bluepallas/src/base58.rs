//! Base58check encoding utilities for Mina-compatible formats.
//!
//! This module provides base58check encoding and decoding functions compatible with
//! the Mina blockchain's encoding format. The encoding follows the standard base58check
//! format: version_byte || payload || checksum (first 4 bytes of double SHA256).

use alloc::{string::String, vec::Vec};
use sha2::{Digest, Sha256};

/// Version byte for Mina signatures in base58check encoding
pub const SIGNATURE_VERSION_BYTE: u8 = 154;

/// Version number prepended to signature bytes before base58check encoding
pub const SIGNATURE_VERSION_NUMBER: u8 = 1;

/// Version byte for user command memos in base58check encoding
pub const MEMO_VERSION_BYTE: u8 = 20;

/// Compute a checksum for base58check encoding (double SHA256, first 4 bytes)
pub fn compute_checksum(input: &[u8]) -> [u8; 4] {
    let hash1 = Sha256::digest(input);
    let hash2 = Sha256::digest(hash1);
    let mut checksum = [0u8; 4];
    checksum.copy_from_slice(&hash2[..4]);
    checksum
}

/// Convert bytes to base58check encoding with a version byte.
///
/// Format: base58(version_byte || input || checksum)
/// where checksum = SHA256(SHA256(version_byte || input))[0..4]
pub fn to_base58_check(input: &[u8], version_byte: u8) -> String {
    let mut with_version = Vec::with_capacity(1 + input.len() + 4);
    with_version.push(version_byte);
    with_version.extend_from_slice(input);

    let checksum = compute_checksum(&with_version);
    with_version.extend_from_slice(&checksum);

    bs58::encode(with_version).into_string()
}

/// Decode a base58check encoded string, verifying the version byte and checksum.
///
/// Returns the payload bytes (without version byte and checksum) on success.
pub fn from_base58_check(input: &str, expected_version_byte: u8) -> Result<Vec<u8>, Base58Error> {
    let bytes = bs58::decode(input)
        .into_vec()
        .map_err(|_| Base58Error::InvalidBase58)?;

    if bytes.len() < 5 {
        return Err(Base58Error::TooShort);
    }

    // Check version byte
    let version_byte = bytes[0];
    if version_byte != expected_version_byte {
        return Err(Base58Error::InvalidVersionByte {
            expected: expected_version_byte,
            actual: version_byte,
        });
    }

    // Verify checksum
    let payload_with_version = &bytes[..bytes.len() - 4];
    let provided_checksum = &bytes[bytes.len() - 4..];
    let computed_checksum = compute_checksum(payload_with_version);

    if provided_checksum != computed_checksum {
        return Err(Base58Error::InvalidChecksum);
    }

    // Return payload (without version byte and checksum)
    Ok(bytes[1..bytes.len() - 4].to_vec())
}

/// Errors that can occur during base58check encoding/decoding
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Base58Error {
    /// The input string is not valid base58
    InvalidBase58,
    /// The input is too short to contain version byte and checksum
    TooShort,
    /// The version byte doesn't match the expected value
    InvalidVersionByte { expected: u8, actual: u8 },
    /// The checksum doesn't match
    InvalidChecksum,
    /// The payload length is invalid
    InvalidLength { expected: usize, actual: usize },
}

impl core::fmt::Display for Base58Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Base58Error::InvalidBase58 => write!(f, "Invalid base58 encoding"),
            Base58Error::TooShort => write!(f, "Input too short for base58check"),
            Base58Error::InvalidVersionByte { expected, actual } => {
                write!(
                    f,
                    "Invalid version byte: expected {}, got {}",
                    expected, actual
                )
            }
            Base58Error::InvalidChecksum => write!(f, "Invalid checksum"),
            Base58Error::InvalidLength { expected, actual } => {
                write!(f, "Invalid length: expected {}, got {}", expected, actual)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_checksum_deterministic() {
        let input = b"test input";
        let checksum1 = compute_checksum(input);
        let checksum2 = compute_checksum(input);
        assert_eq!(checksum1, checksum2);
    }

    #[test]
    fn test_base58_check_roundtrip() {
        let payload = b"hello world";
        let version_byte = 42u8;

        let encoded = to_base58_check(payload, version_byte);
        let decoded = from_base58_check(&encoded, version_byte).unwrap();

        assert_eq!(decoded, payload);
    }

    #[test]
    fn test_base58_check_wrong_version_byte() {
        let payload = b"test";
        let encoded = to_base58_check(payload, 42);

        let result = from_base58_check(&encoded, 43);
        assert!(matches!(
            result,
            Err(Base58Error::InvalidVersionByte {
                expected: 43,
                actual: 42
            })
        ));
    }

    #[test]
    fn test_base58_check_invalid_checksum() {
        let payload = b"test";
        let mut encoded = to_base58_check(payload, 42);

        // Modify the last character to corrupt the checksum
        let mut chars: Vec<char> = encoded.chars().collect();
        if let Some(last) = chars.last_mut() {
            *last = if *last == '1' { '2' } else { '1' };
        }
        encoded = chars.into_iter().collect();

        let result = from_base58_check(&encoded, 42);
        // Could be InvalidBase58 or InvalidChecksum depending on what the corruption does
        assert!(result.is_err());
    }

    #[test]
    fn test_base58_check_too_short() {
        let result = from_base58_check("abc", 42);
        assert!(matches!(result, Err(Base58Error::TooShort)));
    }
}
