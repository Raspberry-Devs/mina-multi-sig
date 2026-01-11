//! This module provides serde implementations for the NetworkIdEnvelope enum used in transactions.
use alloc::string::String;
use mina_signer::NetworkId;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
pub struct NetworkIdEnvelope(pub NetworkId);

impl Serialize for NetworkIdEnvelope {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Convert NetworkId into string
        match self.0 {
            NetworkId::TESTNET => serializer.serialize_str("testnet"),
            NetworkId::MAINNET => serializer.serialize_str("mainnet"),
        }
    }
}

impl<'de> Deserialize<'de> for NetworkIdEnvelope {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "testnet" => Ok(NetworkIdEnvelope(NetworkId::TESTNET)),
            "mainnet" => Ok(NetworkIdEnvelope(NetworkId::MAINNET)),
            _ => Err(serde::de::Error::custom(format!(
                "invalid NetworkId: expected \"testnet\" or \"mainnet\", got {}",
                value
            ))),
        }
    }
}

impl From<NetworkId> for NetworkIdEnvelope {
    fn from(id: NetworkId) -> Self {
        NetworkIdEnvelope(id)
    }
}

impl From<String> for NetworkIdEnvelope {
    fn from(s: String) -> Self {
        match s.as_str() {
            "testnet" => NetworkIdEnvelope(NetworkId::TESTNET),
            "mainnet" => NetworkIdEnvelope(NetworkId::MAINNET),
            _ => panic!("invalid NetworkId string: {}", s),
        }
    }
}

impl PartialEq for NetworkIdEnvelope {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (&self.0, &other.0),
            (NetworkId::TESTNET, NetworkId::TESTNET) | (NetworkId::MAINNET, NetworkId::MAINNET)
        )
    }
}

impl Eq for NetworkIdEnvelope {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_testnet_serialization() {
        let testnet = NetworkIdEnvelope(NetworkId::TESTNET);
        let serialized = serde_json::to_string(&testnet).expect("should serialize");
        assert_eq!(serialized, "\"testnet\"");
    }

    #[test]
    fn test_mainnet_serialization() {
        let mainnet = NetworkIdEnvelope(NetworkId::MAINNET);
        let serialized = serde_json::to_string(&mainnet).expect("should serialize");
        assert_eq!(serialized, "\"mainnet\"");
    }

    #[test]
    fn test_testnet_deserialization() {
        let deserialized: NetworkIdEnvelope =
            serde_json::from_str("\"testnet\"").expect("should deserialize");
        assert_eq!(deserialized.0 as u8, 0);
    }

    #[test]
    fn test_mainnet_deserialization() {
        let deserialized: NetworkIdEnvelope =
            serde_json::from_str("\"mainnet\"").expect("should deserialize");
        assert_eq!(deserialized.0 as u8, 1);
    }

    #[test]
    fn test_invalid_network_id() {
        let result: Result<NetworkIdEnvelope, _> = serde_json::from_str("\"invalidnet\"");
        assert!(result.is_err());
    }

    #[test]
    fn test_roundtrip() {
        let original = NetworkIdEnvelope(NetworkId::TESTNET);
        let json = serde_json::to_string(&original).expect("should serialize");
        let deserialized: NetworkIdEnvelope =
            serde_json::from_str(&json).expect("should deserialize");
        assert_eq!(original, deserialized);
    }
}
