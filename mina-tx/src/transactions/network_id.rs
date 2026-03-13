//! Custom NetworkId type supporting Testnet, Mainnet, and custom network identifiers.
//!
//! This replaces direct use of `mina_signer::NetworkId` to allow custom network IDs
//! (e.g., devnets) that the upstream crate does not support.

use alloc::{string::String, vec, vec::Vec};
use mina_hasher::DomainParameter;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
            NetworkId::Custom(_) => {
                unimplemented!("DomainParameter::into_bytes for Custom network")
            }
        }
    }
}

impl NetworkId {
    /// Returns the domain string used for hashing/signing.
    pub fn into_domain_string(self) -> String {
        match self {
            NetworkId::Mainnet => String::from("MinaSignatureMainnet"),
            NetworkId::Testnet => String::from("CodaSignature"),
            NetworkId::Custom(_) => {
                unimplemented!("into_domain_string for Custom network")
            }
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

impl From<NetworkId> for mina_signer::NetworkId {
    fn from(id: NetworkId) -> Self {
        match id {
            NetworkId::Testnet => mina_signer::NetworkId::TESTNET,
            NetworkId::Mainnet => mina_signer::NetworkId::MAINNET,
            NetworkId::Custom(_) => {
                unimplemented!("Cannot convert Custom NetworkId to mina_signer::NetworkId")
            }
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

    #[test]
    fn test_to_mina_signer() {
        let testnet: mina_signer::NetworkId = NetworkId::Testnet.into();
        assert_eq!(testnet, mina_signer::NetworkId::TESTNET);
        let mainnet: mina_signer::NetworkId = NetworkId::Mainnet.into();
        assert_eq!(mainnet, mina_signer::NetworkId::MAINNET);
    }
}
