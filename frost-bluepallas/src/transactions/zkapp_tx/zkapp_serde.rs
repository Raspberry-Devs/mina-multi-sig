use crate::transactions::{
    zkapp_tx::{Field, PublicKey},
    MEMO_BYTES,
};
use alloc::string::{String, ToString};
use mina_hasher::Fp;
use mina_signer::CompressedPubKey;
use serde::{ser::Serialize, Deserialize};

// --------------- CompressedPubKey serde wrapper ---------------
impl Serialize for PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let state = serializer.serialize_str(&self.0.into_address())?;
        Ok(state)
    }
}

impl<'de> Deserialize<'de> for PublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let pk = CompressedPubKey::from_address(&s).map_err(serde::de::Error::custom)?;
        Ok(PublicKey(pk))
    }
}

// --------------- Field serde wrapper ---------------
impl Serialize for Field {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Why serialize as string?
        // Field.toJSON() in the o1js library returns a string (not a number)
        // https://docs.minaprotocol.com/zkapps/o1js-reference/classes/Field
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for Field {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let fp = s.parse::<Fp>().map_err(|_| {
            serde::de::Error::custom(format!("Failed to parse Fp from string: {}", s))
        })?;
        Ok(Field(fp))
    }
}

pub(crate) fn memo_serde<S>(memo: &[u8; MEMO_BYTES], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    // Serialize memo as base58 string
    let encoded = bs58::encode(memo).into_string();
    serializer.serialize_str(&encoded)
}

pub(crate) fn memo_deser<'de, D>(deserializer: D) -> Result<[u8; MEMO_BYTES], D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let decoded = bs58::decode(&s)
        .into_vec()
        .map_err(serde::de::Error::custom)?;

    if decoded.len() != MEMO_BYTES {
        return Err(serde::de::Error::custom(format!(
            "Invalid memo length: expected {}, got {}",
            MEMO_BYTES,
            decoded.len()
        )));
    }

    if decoded.first() != Some(&0x01) {
        return Err(serde::de::Error::custom(
            "Invalid memo header: missing 0x01 prefix",
        ));
    }

    let mut memo = [0u8; MEMO_BYTES];
    memo.copy_from_slice(&decoded);
    Ok(memo)
}

#[cfg(test)]
mod tests {
    use crate::transactions::zkapp_tx::*;
    use mina_signer::CompressedPubKey;
    use serde_json;

    const TEST_MEMO: [u8; MEMO_BYTES] = [
        0x01, 0x04, b'T', b'e', b's', b't', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    fn create_test_public_key() -> PublicKey {
        let test_address = "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg";
        let compressed_pk =
            CompressedPubKey::from_address(test_address).expect("Valid test address");
        PublicKey(compressed_pk)
    }

    fn create_test_field() -> Field {
        Field(mina_hasher::Fp::from(12345u64))
    }

    fn create_minimal_zkapp_command() -> ZKAppCommand {
        ZKAppCommand {
            fee_payer: FeePayer {
                body: FeePayerBody {
                    public_key: create_test_public_key(),
                    fee: 1000000u64,
                    valid_until: Some(500u32),
                    nonce: 42u32,
                },
                authorization: "test_auth".to_string(),
            },
            account_updates: vec![],
            memo: TEST_MEMO,
        }
    }

    #[test]
    fn test_zkapp_command_serialization() {
        let command = create_minimal_zkapp_command();
        let json = serde_json::to_string(&command).unwrap();

        assert!(json.contains("fee_payer"));
        assert!(json.contains("account_updates"));
        assert!(json.contains("memo"));
    }

    #[test]
    fn test_public_key_serialization() {
        let public_key = create_test_public_key();

        // Test serialization
        let json = serde_json::to_string(&public_key).unwrap();
        assert!(
            json.starts_with('"') && json.ends_with('"'),
            "PublicKey should serialize as quoted string"
        );

        // Verify it contains a valid Mina address format
        let address = json.trim_matches('"');
        assert!(
            address.starts_with("B62q"),
            "Should be a valid Mina address format"
        );
    }

    #[test]
    fn test_public_key_deserialization() {
        // Test with a known valid Mina address
        let json_str = r#""B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg""#;
        let result: Result<PublicKey, _> = serde_json::from_str(json_str);

        assert!(result.is_ok(), "Should deserialize valid Mina address");
        let public_key = result.unwrap();
        assert_eq!(
            public_key.0.into_address(),
            "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg"
        );
    }

    #[test]
    fn test_field_serialization() {
        let field = create_test_field();

        // Test serialization
        let json = serde_json::to_string(&field).unwrap();
        assert!(
            json.starts_with('"') && json.ends_with('"'),
            "Field should serialize as quoted string"
        );

        // Should contain numeric content
        let content = json.trim_matches('"');
        assert!(
            !content.is_empty(),
            "Field serialization should not be empty"
        );
    }

    #[test]
    fn test_field_deserialization() {
        // Test with a simple numeric string
        let json_str = r#""12345""#;
        let result: Result<Field, _> = serde_json::from_str(json_str);

        assert!(result.is_ok(), "Should deserialize valid numeric string");
        let field = result.unwrap();
        assert_eq!(field.0, mina_hasher::Fp::from(12345u64));
    }

    #[test]
    fn test_field_deserialization_invalid() {
        // Test with invalid input
        let json_str = r#""not_a_number""#;
        let result: Result<Field, _> = serde_json::from_str(json_str);

        assert!(result.is_err(), "Should fail with invalid field format");
    }

    #[test]
    fn test_public_key_round_trip() {
        let original = create_test_public_key();
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: PublicKey = serde_json::from_str(&json).unwrap();

        assert_eq!(original.0.into_address(), deserialized.0.into_address());
    }

    #[test]
    fn test_zkapp_command_round_trip() {
        let original = create_minimal_zkapp_command();
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ZKAppCommand = serde_json::from_str(&json).unwrap();

        assert_eq!(original.memo, deserialized.memo);
        assert_eq!(original.fee_payer.body.fee, deserialized.fee_payer.body.fee);
        assert_eq!(
            original.account_updates.len(),
            deserialized.account_updates.len()
        );
    }

    #[test]
    fn test_deserialize_from_json() {
        let json_str = r#"{
            "fee_payer": {
                "body": {
                    "public_key": "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
                    "fee": 1000000,
                    "valid_until": null,
                    "nonce": 42
                },
                "authorization": "test_auth"
            },
            "account_updates": [],
            "memo": "2LLNoLLTNMVDUcSsdkJXnDByvpXjxSmdy6MWWXSW73QkSK"
        }"#;

        let result: ZKAppCommand = serde_json::from_str(json_str).unwrap();
        assert_eq!(result.memo, TEST_MEMO);
        assert_eq!(result.fee_payer.body.fee, 1000000);
        assert_eq!(result.fee_payer.body.valid_until, None);
    }

    #[test]
    fn test_invalid_public_key_fails() {
        let json_str = r#""invalid_key_format""#;
        let result: Result<PublicKey, _> = serde_json::from_str(json_str);
        assert!(result.is_err());
    }
}
