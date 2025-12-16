use mina_signer::NetworkId;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
pub(crate) struct NetworkIdSerde(pub NetworkId);

impl Serialize for NetworkIdSerde {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // NetworkId uses chain_id field internally
        // TESTNET = 0x00, MAINNET = 0x01
        serializer.serialize_u8(self.0.clone() as u8)
    }
}

impl<'de> Deserialize<'de> for NetworkIdSerde {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        match value {
            0 => Ok(NetworkIdSerde(NetworkId::TESTNET)),
            1 => Ok(NetworkIdSerde(NetworkId::MAINNET)),
            _ => Err(serde::de::Error::custom(format!(
                "invalid NetworkId: expected 0 or 1, got {}",
                value
            ))),
        }
    }
}

impl From<NetworkId> for NetworkIdSerde {
    fn from(id: NetworkId) -> Self {
        NetworkIdSerde(id)
    }
}

impl PartialEq for NetworkIdSerde {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (&self.0, &other.0),
            (NetworkId::TESTNET, NetworkId::TESTNET) | (NetworkId::MAINNET, NetworkId::MAINNET)
        )
    }
}

impl Eq for NetworkIdSerde {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_testnet_serialization() {
        let testnet = NetworkIdSerde(NetworkId::TESTNET);
        let serialized = serde_json::to_string(&testnet).expect("should serialize");
        assert_eq!(serialized, "0");
    }

    #[test]
    fn test_mainnet_serialization() {
        let mainnet = NetworkIdSerde(NetworkId::MAINNET);
        let serialized = serde_json::to_string(&mainnet).expect("should serialize");
        assert_eq!(serialized, "1");
    }

    #[test]
    fn test_testnet_deserialization() {
        let deserialized: NetworkIdSerde = serde_json::from_str("0").expect("should deserialize");
        assert_eq!(deserialized.0 as u8, 0);
    }

    #[test]
    fn test_mainnet_deserialization() {
        let deserialized: NetworkIdSerde = serde_json::from_str("1").expect("should deserialize");
        assert_eq!(deserialized.0 as u8, 1);
    }

    #[test]
    fn test_invalid_network_id() {
        let result: Result<NetworkIdSerde, _> = serde_json::from_str("2");
        assert!(result.is_err());
    }

    #[test]
    fn test_roundtrip() {
        let original = NetworkIdSerde(NetworkId::TESTNET);
        let json = serde_json::to_string(&original).expect("should serialize");
        let deserialized: NetworkIdSerde = serde_json::from_str(&json).expect("should deserialize");
        assert_eq!(original, deserialized);
    }
}
