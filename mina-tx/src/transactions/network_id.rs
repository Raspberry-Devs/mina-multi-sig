//! Custom NetworkId type supporting Testnet, Mainnet, and custom network identifiers.
//!
//! This replaces direct use of `mina_signer::NetworkId` to allow custom network IDs
//! (e.g., devnets) that the upstream crate does not support.

use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};
use mina_hasher::DomainParameter;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// This should not grow past 255, otherwise we would break compile-time guarantees
pub const MAX_PREFIX_LENGTH: usize = 20;
const PADDING_CHAR: u8 = b'*';

/// Network identifier for Mina transactions.
///
/// Extends the upstream `mina_signer::NetworkId` with a `Custom` variant
/// for devnets and other non-standard networks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkId {
    Testnet,
    Mainnet,
    Custom(String),
}

impl DomainParameter for NetworkId {
    fn into_bytes(self) -> Vec<u8> {
        match self {
            NetworkId::Testnet => vec![0x00],
            NetworkId::Mainnet => vec![0x01],
            NetworkId::Custom(s) => network_id_to_bytes(&s),
        }
    }
}

/// Encode a network ID string as bytes for domain hashing.
///
/// Each character's byte is converted to 8 bits, iterated in reverse character
/// order, and concatenated into a bit string which is packed into bytes.
fn network_id_to_bytes(s: &str) -> Vec<u8> {
    let mut bits = Vec::with_capacity(s.len() * 8);
    for ch in s.bytes().rev() {
        for bit in (0..8).rev() {
            bits.push((ch >> bit) & 1);
        }
    }
    let mut bytes = Vec::with_capacity(bits.len().div_ceil(8));
    for chunk in bits.chunks(8) {
        let mut byte = 0u8;
        for (i, &b) in chunk.iter().enumerate() {
            byte |= b << (7 - i);
        }
        bytes.push(byte);
    }
    bytes
}

impl NetworkId {
    /// Pad or truncate a prefix string to exactly `MAX_PREFIX_LENGTH` characters.
    pub fn create_custom_prefix(prefix: &str) -> String {
        if prefix.len() <= MAX_PREFIX_LENGTH {
            let mut s = String::from(prefix);
            for _ in 0..(MAX_PREFIX_LENGTH - prefix.len()) {
                s.push(PADDING_CHAR as char);
            }
            s
        } else {
            prefix[..MAX_PREFIX_LENGTH].to_string()
        }
    }

    /// Returns the domain string used for hashing/signing.
    pub fn into_domain_string(self) -> String {
        match self {
            NetworkId::Mainnet => String::from("MinaSignatureMainnet"),
            NetworkId::Testnet => String::from("CodaSignature"),
            NetworkId::Custom(s) => Self::create_custom_prefix(&(s + "Signature")),
        }
    }
}

impl From<mina_signer::NetworkId> for NetworkId {
    fn from(id: mina_signer::NetworkId) -> Self {
        match id {
            mina_signer::NetworkId::TESTNET => NetworkId::Testnet,
            mina_signer::NetworkId::MAINNET => NetworkId::Mainnet,
        }
    }
}

impl Serialize for NetworkId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            NetworkId::Testnet => serializer.serialize_str("testnet"),
            NetworkId::Mainnet => serializer.serialize_str("mainnet"),
            NetworkId::Custom(s) => serializer.serialize_str(s),
        }
    }
}

impl<'de> Deserialize<'de> for NetworkId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "testnet" => Ok(NetworkId::Testnet),
            "mainnet" => Ok(NetworkId::Mainnet),
            _ => Ok(NetworkId::Custom(value)),
        }
    }
}

// --- NetworkIdEnvelope ---

#[derive(Debug, Clone)]
pub struct NetworkIdEnvelope(pub NetworkId);

impl Serialize for NetworkIdEnvelope {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for NetworkIdEnvelope {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        NetworkId::deserialize(deserializer).map(NetworkIdEnvelope)
    }
}

impl From<NetworkId> for NetworkIdEnvelope {
    fn from(id: NetworkId) -> Self {
        NetworkIdEnvelope(id)
    }
}

impl core::convert::TryFrom<String> for NetworkIdEnvelope {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "testnet" => Ok(NetworkIdEnvelope(NetworkId::Testnet)),
            "mainnet" => Ok(NetworkIdEnvelope(NetworkId::Mainnet)),
            _ => Ok(NetworkIdEnvelope(NetworkId::Custom(s))),
        }
    }
}

impl PartialEq for NetworkIdEnvelope {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for NetworkIdEnvelope {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_testnet_serialization() {
        let testnet = NetworkIdEnvelope(NetworkId::Testnet);
        let serialized = serde_json::to_string(&testnet).expect("should serialize");
        assert_eq!(serialized, "\"testnet\"");
    }

    #[test]
    fn test_mainnet_serialization() {
        let mainnet = NetworkIdEnvelope(NetworkId::Mainnet);
        let serialized = serde_json::to_string(&mainnet).expect("should serialize");
        assert_eq!(serialized, "\"mainnet\"");
    }

    #[test]
    fn test_testnet_deserialization() {
        let deserialized: NetworkIdEnvelope =
            serde_json::from_str("\"testnet\"").expect("should deserialize");
        assert_eq!(deserialized.0, NetworkId::Testnet);
    }

    #[test]
    fn test_mainnet_deserialization() {
        let deserialized: NetworkIdEnvelope =
            serde_json::from_str("\"mainnet\"").expect("should deserialize");
        assert_eq!(deserialized.0, NetworkId::Mainnet);
    }

    #[test]
    fn test_custom_network_id_deserialization() {
        let deserialized: NetworkIdEnvelope =
            serde_json::from_str("\"devnet\"").expect("should deserialize");
        assert_eq!(deserialized.0, NetworkId::Custom("devnet".into()));
    }

    #[test]
    fn test_custom_network_id_serialization() {
        let custom = NetworkIdEnvelope(NetworkId::Custom("devnet".into()));
        let serialized = serde_json::to_string(&custom).expect("should serialize");
        assert_eq!(serialized, "\"devnet\"");
    }

    #[test]
    fn test_roundtrip() {
        let original = NetworkIdEnvelope(NetworkId::Testnet);
        let json = serde_json::to_string(&original).expect("should serialize");
        let deserialized: NetworkIdEnvelope =
            serde_json::from_str(&json).expect("should deserialize");
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_custom_roundtrip() {
        let original = NetworkIdEnvelope(NetworkId::Custom("mynet".into()));
        let json = serde_json::to_string(&original).expect("should serialize");
        let deserialized: NetworkIdEnvelope =
            serde_json::from_str(&json).expect("should deserialize");
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_from_mina_signer() {
        let testnet: NetworkId = mina_signer::NetworkId::TESTNET.into();
        assert_eq!(testnet, NetworkId::Testnet);
        let mainnet: NetworkId = mina_signer::NetworkId::MAINNET.into();
        assert_eq!(mainnet, NetworkId::Mainnet);
    }
}
