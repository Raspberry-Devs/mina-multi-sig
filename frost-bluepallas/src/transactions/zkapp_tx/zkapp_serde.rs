//! This module provides serde implementations for some structs used in ZkApp transactions
//! for which custom serialization is required.

use crate::{
    base58::{from_base58_check, to_base58_check, Base58Error, MEMO_VERSION_BYTE},
    transactions::{
        zkapp_tx::{Field, PublicKey},
        MEMO_BYTES,
    },
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
    // Serialize memo as base58check string with the memo version byte
    let encoded = to_base58_check(memo, MEMO_VERSION_BYTE);
    serializer.serialize_str(&encoded)
}

pub(crate) fn memo_deser<'de, D>(deserializer: D) -> Result<[u8; MEMO_BYTES], D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let decoded = from_base58_check(&s, MEMO_VERSION_BYTE).map_err(|e| match e {
        Base58Error::InvalidBase58 => serde::de::Error::custom("Invalid base58 encoding"),
        Base58Error::TooShort => serde::de::Error::custom("Memo too short for base58check"),
        Base58Error::InvalidVersionByte { expected, actual } => serde::de::Error::custom(format!(
            "Invalid memo version byte: expected {}, got {}",
            expected, actual
        )),
        Base58Error::InvalidChecksum => serde::de::Error::custom("Invalid memo checksum"),
        Base58Error::InvalidLength { expected, actual } => serde::de::Error::custom(format!(
            "Invalid memo length: expected {}, got {}",
            expected, actual
        )),
    })?;

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
    use crate::base58::{to_base58_check, MEMO_VERSION_BYTE};
    use crate::transactions::zkapp_tx::*;
    use mina_signer::CompressedPubKey;
    use serde_json;

    const TEST_MEMO: [u8; MEMO_BYTES] = [
        0x01, 0x04, b'T', b'e', b's', b't', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    /// Helper to get the base58check encoded test memo
    fn test_memo_base58() -> String {
        to_base58_check(&TEST_MEMO, MEMO_VERSION_BYTE)
    }

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
        let memo_base58 = test_memo_base58();
        let json_str = format!(
            r#"{{
            "fee_payer": {{
                "body": {{
                    "public_key": "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
                    "fee": 1000000,
                    "valid_until": null,
                    "nonce": 42
                }},
                "authorization": "test_auth"
            }},
            "account_updates": [],
            "memo": "{}"
        }}"#,
            memo_base58
        );

        let result: ZKAppCommand = serde_json::from_str(&json_str).unwrap();
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

    #[test]
    fn test_example_file() {}

    #[test]
    fn test_deserialize_real_zkapp_transaction() {
        // Read the JSON file from test fixtures
        let json_str = include_str!("../../../tests/data/tx-2026-01-06T18-06-16-703Z.json");

        // Deserialize into ZKAppCommand
        let result: Result<ZKAppCommand, _> = serde_json::from_str(json_str);
        assert!(
            result.is_ok(),
            "Failed to deserialize ZKAppCommand: {:?}",
            result.err()
        );

        let zkapp_command = result.unwrap();

        // Validate fee payer
        assert_eq!(
            zkapp_command.fee_payer.body.public_key.0.into_address(),
            "B62qn2fycPwxZJNZUdG2h11Muf4JEZmarctiyQnJy9dzBQj9kwzyoU5",
            "Fee payer public key mismatch"
        );
        assert_eq!(
            zkapp_command.fee_payer.body.fee, 100000000,
            "Fee amount mismatch"
        );
        assert_eq!(zkapp_command.fee_payer.body.nonce, 1, "Nonce mismatch");
        assert_eq!(
            zkapp_command.fee_payer.body.valid_until, None,
            "Valid until should be None"
        );
        assert_eq!(
            zkapp_command.fee_payer.authorization,
            "7mXW37UR8WQauBnQv4TFBKzm4ArBsa4x8E6MgmrcK4mcTc5EDnW1ef5abBhBw9rw5ESaK4gpbeeUa54kRjmXgWbnSVBNTZNE",
            "Fee payer authorization mismatch"
        );

        // Validate memo
        assert_eq!(
            zkapp_command.memo[0], 0x01,
            "Memo should start with 0x01 header"
        );

        // Validate account updates count
        assert_eq!(
            zkapp_command.account_updates.len(),
            2,
            "Expected 2 account updates"
        );

        // Validate first account update (zkApp)
        let first_update = &zkapp_command.account_updates[0];
        assert_eq!(
            first_update.body.public_key.0.into_address(),
            "B62qicDGrJPbycVahfk4NDt5VFwd2VjQba3R3hXARcRFkPmAUhWT6Ct",
            "First update public key mismatch"
        );
        assert_eq!(
            first_update.body.balance_change.magnitude, 0,
            "First update balance change magnitude should be 0"
        );
        assert_eq!(
            first_update.body.balance_change.sgn,
            1, // Positive
            "First update balance change should be positive"
        );
        assert!(
            first_update.body.increment_nonce,
            "First update should not increment nonce"
        );
        assert_eq!(
            first_update.body.call_depth, 0,
            "First update call depth should be 0"
        );
        assert!(
            !first_update.body.use_full_commitment,
            "First update should not use full commitment"
        );
        assert!(
            !first_update.body.authorization_kind.is_signed,
            "First update should not be signed"
        );
        assert!(
            first_update.body.authorization_kind.is_proved,
            "First update should be proved"
        );
        assert!(
            first_update.authorization.proof.is_some(),
            "First update should have a proof"
        );
        assert!(
            first_update.authorization.signature.is_none(),
            "First update should not have a signature"
        );

        // Validate second account update (signature)
        let second_update = &zkapp_command.account_updates[1];
        assert_eq!(
            second_update.body.public_key.0.into_address(),
            "B62qn2fycPwxZJNZUdG2h11Muf4JEZmarctiyQnJy9dzBQj9kwzyoU5",
            "Second update public key mismatch"
        );
        assert_eq!(
            second_update.body.balance_change.magnitude, 0,
            "Second update balance change magnitude should be 0"
        );
        assert_eq!(
            second_update.body.call_depth, 1,
            "Second update call depth should be 1"
        );
        assert!(
            second_update.body.use_full_commitment,
            "Second update should use full commitment"
        );
        assert!(
            second_update.body.authorization_kind.is_signed,
            "Second update should be signed"
        );
        assert!(
            !second_update.body.authorization_kind.is_proved,
            "Second update should not be proved"
        );
        assert!(
            second_update.authorization.proof.is_none(),
            "Second update should not have a proof"
        );
        assert!(
            second_update.authorization.signature.is_some(),
            "Second update should have a signature"
        );
        assert_eq!(
            second_update.authorization.signature.as_ref().unwrap(),
            "7mXW37UR8WQauBnQv4TFBKzm4ArBsa4x8E6MgmrcK4mcTc5EDnW1ef5abBhBw9rw5ESaK4gpbeeUa54kRjmXgWbnSVBNTZNE",
            "Second update signature mismatch"
        );

        // Validate verification key in first update
        assert!(
            first_update.body.update.verification_key.is_some(),
            "First update should have a verification key"
        );
        let vk = first_update.body.update.verification_key.as_ref().unwrap();
        assert_eq!(
            vk.hash.0.to_string(),
            "11637459424629262763516613417942459725081885818445034343602400886490139666450",
            "Verification key hash mismatch"
        );

        // Validate preconditions for first update
        let first_preconditions = &first_update.body.preconditions;
        assert!(
            first_preconditions.account.state[0].is_some(),
            "First state element should be set"
        );
        assert_eq!(
            first_preconditions.account.state[0]
                .as_ref()
                .unwrap()
                .0
                .to_string(),
            "14342720814455193863288733022440466111280056178648241250920008914963913484906",
            "First state element mismatch"
        );
        assert!(
            first_preconditions.account.state[1].is_some(),
            "Second state element should be set"
        );
        assert_eq!(
            first_preconditions.account.state[1]
                .as_ref()
                .unwrap()
                .0
                .to_string(),
            "0",
            "Second state element should be 0"
        );

        // Round-trip test: serialize back to JSON and deserialize again
        let serialized =
            serde_json::to_string(&zkapp_command).expect("Failed to serialize ZKAppCommand");
        let round_trip: ZKAppCommand =
            serde_json::from_str(&serialized).expect("Failed to deserialize round-trip");

        assert_eq!(
            zkapp_command, round_trip,
            "Round-trip serialization should produce identical result"
        );
    }
}
