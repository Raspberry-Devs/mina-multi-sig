use alloc::{string::String, vec::Vec};
use ark_ff::BigInt;
use mina_signer::pubkey::PubKey;
use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Deserializer, Serialize,
};

use crate::{
    base58::{to_base58_check, SIGNATURE_VERSION_BYTE, SIGNATURE_VERSION_NUMBER},
    transactions::{TransactionEnvelope, TransactionKind},
    zkapp_tx::SignatureInjectionResult,
};

#[derive(Clone, Debug)]
pub struct Sig {
    pub field: BigInt<4>,
    pub scalar: BigInt<4>,
}

impl Sig {
    fn bigint_to_bytes(value: &BigInt<4>) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        for (i, limb) in value.0.iter().enumerate() {
            let limb_bytes = limb.to_le_bytes();
            bytes[i * 8..(i + 1) * 8].copy_from_slice(&limb_bytes);
        }
        bytes
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(65);
        bytes.push(SIGNATURE_VERSION_NUMBER);
        bytes.extend_from_slice(&Self::bigint_to_bytes(&self.field));
        bytes.extend_from_slice(&Self::bigint_to_bytes(&self.scalar));
        bytes
    }

    pub fn to_base58(&self) -> String {
        let bytes = self.to_bytes();
        to_base58_check(&bytes, SIGNATURE_VERSION_BYTE)
    }
}

impl Serialize for Sig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("signature", 3)?;
        state.serialize_field("field", &self.field.to_string())?;
        state.serialize_field("scalar", &self.scalar.to_string())?;
        state.serialize_field("base58", self.to_base58().as_str())?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Sig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct SigHelper {
            field: String,
            scalar: String,
            #[allow(dead_code)]
            base58: Option<String>,
        }

        let helper = SigHelper::deserialize(deserializer)?;

        let field = helper
            .field
            .parse::<BigInt<4>>()
            .map_err(|_| serde::de::Error::custom("Failed to parse 'field' as BigInt<4>"))?;
        let scalar = helper
            .scalar
            .parse::<BigInt<4>>()
            .map_err(|_| serde::de::Error::custom("Failed to parse 'scalar' as BigInt<4>"))?;

        Ok(Sig { field, scalar })
    }
}

#[allow(non_snake_case)]
pub struct PubKeySer {
    pub pubKey: PubKey,
}

impl From<PubKey> for PubKeySer {
    #[allow(non_snake_case)]
    fn from(pubKey: PubKey) -> Self {
        PubKeySer { pubKey }
    }
}

impl Serialize for PubKeySer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("publicKey", 1)?;
        state.serialize_field("address", &self.pubKey.into_address())?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for PubKeySer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct PubKeyHelper {
            address: String,
        }

        let helper = PubKeyHelper::deserialize(deserializer)?;

        let pub_key = PubKey::from_address(&helper.address)
            .map_err(|_| serde::de::Error::custom("Failed to parse 'address' as PubKey"))?;

        Ok(PubKeySer { pubKey: pub_key })
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TransactionSignature {
    pub publicKey: PubKeySer,
    pub signature: Sig,
    pub payload: TransactionEnvelope,
}

impl TransactionSignature {
    pub fn new_with_zkapp_injection(
        public_key: PubKeySer,
        signature: Sig,
        mut payload: TransactionEnvelope,
    ) -> (Self, Option<SignatureInjectionResult>) {
        let injection_result = match payload.inner_mut() {
            TransactionKind::ZkApp(zkapp) => {
                Some(zkapp.inject_signature(&public_key.pubKey, &signature))
            }
            TransactionKind::Legacy(_) => None,
        };

        let tx_sig = Self {
            publicKey: public_key,
            signature,
            payload,
        };

        (tx_sig, injection_result)
    }

    pub fn to_graphql_query_json(&self) -> Result<String, serde_json::Error> {
        self.payload.to_graphql_query_json(self.signature.clone())
    }
}
