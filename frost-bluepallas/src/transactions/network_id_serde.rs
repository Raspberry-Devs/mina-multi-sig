use mina_signer::NetworkId;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
pub(crate) struct NetworkIdSerde(pub NetworkId);

impl Serialize for NetworkIdSerde {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = match self.0 {
            NetworkId::TESTNET => 0u8,
            NetworkId::MAINNET => 1u8,
        };
        serializer.serialize_u8(value)
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
            _ => Err(serde::de::Error::custom("invalid NetworkId")),
        }
    }
}

impl From<NetworkId> for NetworkIdSerde {
    fn from(id: NetworkId) -> Self {
        NetworkIdSerde(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::de::value::{Error as DeError, U8Deserializer};

    #[test]
    fn deser_0_is_testnet() {
        let deser = U8Deserializer::<DeError>::new(0);
        let got = NetworkIdSerde::deserialize(deser).expect("0 should deserialize");
        assert!(matches!(got.0, NetworkId::TESTNET));
    }

    #[test]
    fn deser_1_is_mainnet() {
        let deser = U8Deserializer::<DeError>::new(1);
        let got = NetworkIdSerde::deserialize(deser).expect("1 should deserialize");
        assert!(matches!(got.0, NetworkId::MAINNET));
    }

    #[test]
    fn deser_other_values_error() {
        let deser = U8Deserializer::<DeError>::new(2);
        let err = NetworkIdSerde::deserialize(deser).unwrap_err();
        assert!(err.to_string().contains("invalid NetworkId"));
    }
}
