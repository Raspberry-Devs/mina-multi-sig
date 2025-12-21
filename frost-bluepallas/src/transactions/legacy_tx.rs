//! This module defines the legacy transaction structure and related functionality (mainly serde and hashing/commitment).

use alloc::string::{String, ToString};
use core::fmt;
use mina_hasher::{Hashable, ROInput};
use mina_signer::{CompressedPubKey, NetworkId, PubKey};

use serde::{
    ser::{Serialize, SerializeStruct, Serializer},
    Deserialize,
};

use crate::{
    errors::BluePallasError,
    transactions::{MEMO_BYTES, MEMO_HEADER_BYTES},
};

/// Copied from https://github.com/o1-labs/proof-systems/blob/master/signer/tests/transaction.rs
const TAG_BITS: usize = 3;
const PAYMENT_TX_TAG: [bool; TAG_BITS] = [false, false, false];
const DELEGATION_TX_TAG: [bool; TAG_BITS] = [false, false, true];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transaction {
    // Common
    pub fee: u64,
    pub fee_token: u64,
    pub fee_payer_pk: CompressedPubKey,
    pub nonce: u32,
    pub valid_until: u32,
    pub memo: [u8; MEMO_BYTES],
    // Body
    pub tag: [bool; TAG_BITS],
    pub source_pk: CompressedPubKey,
    pub receiver_pk: CompressedPubKey,
    pub token_id: u64,
    pub amount: u64,
    pub token_locked: bool,
}

impl Serialize for Transaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Transaction", 7)?;
        state.serialize_field("to", &self.receiver_pk.into_address())?;
        state.serialize_field("from", &self.source_pk.into_address())?;
        state.serialize_field("fee", &self.fee.to_string())?;
        match self.tag {
            DELEGATION_TX_TAG => {} // Noop
            _ => state.serialize_field("amount", &self.amount.to_string())?,
        }
        state.serialize_field("nonce", &self.nonce.to_string())?;

        // Read the length parameter
        let memo_len = self.memo[1] as usize;
        // Serialize memo as a string, dropping the header bytes
        let memo_str =
            String::from_utf8(self.memo[MEMO_HEADER_BYTES..MEMO_HEADER_BYTES + memo_len].to_vec())
                .map_err(serde::ser::Error::custom)?;
        state.serialize_field("memo", &memo_str)?;

        state.serialize_field("valid_until", &self.valid_until.to_string())?;
        state.serialize_field("tag", &self.tag)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Transaction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct TransactionData {
            to: String,
            from: String,
            fee: String,
            #[serde(default)]
            amount: Option<String>,
            nonce: String,
            memo: String,
            valid_until: String,
            tag: [bool; TAG_BITS],
        }

        let data = TransactionData::deserialize(deserializer)?;

        let from = PubKey::from_address(&data.from).map_err(serde::de::Error::custom)?;
        let to = PubKey::from_address(&data.to).map_err(serde::de::Error::custom)?;
        let fee = data.fee.parse().map_err(serde::de::Error::custom)?;
        let nonce = data.nonce.parse().map_err(serde::de::Error::custom)?;
        let valid_until = data.valid_until.parse().map_err(serde::de::Error::custom)?;

        // Match transaction tag to determine whether we have a payment or delegation transaction
        let tx = match data.tag {
            PAYMENT_TX_TAG => {
                // Expect data.amount to exist
                let ser_amount = data.amount.ok_or(serde::de::Error::custom(
                    "Missing amount for payment transaction",
                ))?;
                let amount = ser_amount.parse().map_err(serde::de::Error::custom)?;
                Transaction::new_payment(from, to, amount, fee, nonce)
                    .set_memo_str(&data.memo)
                    .map_err(serde::de::Error::custom)?
                    .set_valid_until(valid_until)
            }
            DELEGATION_TX_TAG => {
                if data.amount.is_some() {
                    return Err(serde::de::Error::custom(
                        "Unexpected amount for delegation transaction",
                    ));
                }
                Transaction::new_delegation(from, to, fee, nonce)
                    .set_memo_str(&data.memo)
                    .map_err(serde::de::Error::custom)?
                    .set_valid_until(valid_until)
            }
            _ => return Err(serde::de::Error::custom("Invalid transaction tag")),
        };

        Ok(tx)
    }
}

impl Hashable for Transaction {
    type D = NetworkId;

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new()
            .append_field(self.fee_payer_pk.x)
            .append_field(self.source_pk.x)
            .append_field(self.receiver_pk.x)
            .append_u64(self.fee)
            .append_u64(self.fee_token)
            .append_bool(self.fee_payer_pk.is_odd)
            .append_u32(self.nonce)
            .append_u32(self.valid_until)
            .append_bytes(&self.memo);

        for tag_bit in self.tag {
            roi = roi.append_bool(tag_bit);
        }

        roi.append_bool(self.source_pk.is_odd)
            .append_bool(self.receiver_pk.is_odd)
            .append_u64(self.token_id)
            .append_u64(self.amount)
            .append_bool(self.token_locked)
    }

    fn domain_string(network_id: NetworkId) -> Option<String> {
        // Domain strings must have length <= 20
        match network_id {
            NetworkId::MAINNET => "MinaSignatureMainnet",
            NetworkId::TESTNET => "CodaSignature",
        }
        .to_string()
        .into()
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let memo_str = match self.memo.len() {
            len if len < MEMO_HEADER_BYTES => String::new(),
            _ => self.memo[MEMO_HEADER_BYTES..]
                .iter()
                .take_while(|&&b| b != 0)
                .map(|&b| b as char)
                .collect::<String>(),
        };

        let tx_type = match self.tag {
            PAYMENT_TX_TAG => "payment",
            DELEGATION_TX_TAG => "delegation",
            _ => "unknown",
        };

        write!(
            f,
            "{{\n  \"type\": \"{}\",\n  \"to\": \"{}\",\n  \"from\": \"{}\",\n  \"fee\": \"{}\",\n  \"fee_token\": \"{}\",\n  \"fee_payer_pk\": \"{}\",\n  \"amount\": \"{}\",\n  \"nonce\": \"{}\",\n  \"valid_until\": \"{}\",\n  \"memo\": \"{}\",\n  \"tag\": {:?},\n  \"source_pk\": \"{}\",\n  \"receiver_pk\": \"{}\",\n  \"token_id\": \"{}\",\n  \"token_locked\": {}\n}}",
            tx_type,
            self.receiver_pk.into_address(),
            self.source_pk.into_address(),
            self.fee,
            self.fee_token,
            self.fee_payer_pk.into_address(),
            self.amount,
            self.nonce,
            self.valid_until,
            memo_str,
            self.tag,
            self.source_pk.into_address(),
            self.receiver_pk.into_address(),
            self.token_id,
            self.token_locked
        )
    }
}

impl Transaction {
    pub fn new_payment(from: PubKey, to: PubKey, amount: u64, fee: u64, nonce: u32) -> Self {
        Transaction {
            fee,
            fee_token: 1,
            fee_payer_pk: from.into_compressed(),
            nonce,
            valid_until: u32::MAX,
            memo: core::array::from_fn(|i| (i == 0) as u8),
            tag: PAYMENT_TX_TAG,
            source_pk: from.into_compressed(),
            receiver_pk: to.into_compressed(),
            token_id: 1,
            amount,
            token_locked: false,
        }
    }

    pub fn new_delegation(from: PubKey, to: PubKey, fee: u64, nonce: u32) -> Self {
        Transaction {
            fee,
            fee_token: 1,
            fee_payer_pk: from.into_compressed(),
            nonce,
            valid_until: u32::MAX,
            memo: core::array::from_fn(|i| (i == 0) as u8),
            tag: DELEGATION_TX_TAG,
            source_pk: from.into_compressed(),
            receiver_pk: to.into_compressed(),
            token_id: 1,
            amount: 0,
            token_locked: false,
        }
    }

    pub fn set_valid_until(mut self, global_slot: u32) -> Self {
        self.valid_until = global_slot;

        self
    }

    pub fn set_memo(mut self, memo: [u8; MEMO_BYTES - 2]) -> Self {
        self.memo[0] = 0x01;
        self.memo[1] = (MEMO_BYTES - 2) as u8;
        self.memo[2..].copy_from_slice(&memo[..]);

        self
    }

    pub fn set_memo_str(mut self, memo: &str) -> Result<Self, BluePallasError> {
        // Prevent overflow
        if memo.len() > MEMO_BYTES - MEMO_HEADER_BYTES || memo.len() > u8::MAX as usize {
            return Err(BluePallasError::invalid_memo("Memo exceeds maximum length"));
        }

        self.memo[0] = 0x01;
        self.memo[1] = memo.len() as u8;
        let memo = format!("{memo:\0<32}"); // Pad user-supplied memo with zeros
        self.memo[2..].copy_from_slice(memo.as_bytes());

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mina_signer::{PubKey, SecKey};
    use rand_core::SeedableRng;

    fn create_test_pubkey(seed: [u8; 32]) -> PubKey {
        // Create a deterministic test pubkey
        let mut rng = rand_chacha::ChaCha12Rng::from_seed(seed);
        let sec = SecKey::rand(&mut rng);
        PubKey::from_secret_key(sec).unwrap()
    }

    #[test]
    fn test_payment_serialization_roundtrip() {
        let from = create_test_pubkey([1; 32]);
        let to = create_test_pubkey([2; 32]);
        let original = Transaction::new_payment(from, to, 1000000, 10000, 42)
            .set_memo_str("test memo")
            .unwrap()
            .set_valid_until(12345);

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Transaction = serde_json::from_str(&json).unwrap();

        // Compare key fields
        assert_eq!(original.source_pk.x, deserialized.source_pk.x);
        assert_eq!(original.receiver_pk.x, deserialized.receiver_pk.x);
        assert_eq!(original.amount, deserialized.amount);
        assert_eq!(original.fee, deserialized.fee);
        assert_eq!(original.nonce, deserialized.nonce);
        assert_eq!(original.valid_until, deserialized.valid_until);
        assert_eq!(original.memo, deserialized.memo);
        assert_eq!(original.tag, deserialized.tag);
    }

    #[test]
    fn test_delegation_serialization_roundtrip() {
        let from = create_test_pubkey([3; 32]);
        let to = create_test_pubkey([4; 32]);
        let original = Transaction::new_delegation(from, to, 5000, 100).set_valid_until(54321);

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Transaction = serde_json::from_str(&json).unwrap();

        assert_eq!(original.source_pk.x, deserialized.source_pk.x);
        assert_eq!(original.receiver_pk.x, deserialized.receiver_pk.x);
        assert_eq!(original.amount, deserialized.amount);
        assert_eq!(deserialized.amount, 0);
        assert_eq!(original.fee, deserialized.fee);
        assert_eq!(original.nonce, deserialized.nonce);
        assert_eq!(original.valid_until, deserialized.valid_until);
        assert_eq!(original.tag, deserialized.tag);

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_bytes_roundtrip_payment() {
        let from = create_test_pubkey([5; 32]);
        let to = create_test_pubkey([6; 32]);
        let original = Transaction::new_payment(from, to, 2500000, 15000, 123)
            .set_memo_str("roundtrip test")
            .unwrap()
            .set_valid_until(98765);

        // Convert to bytes and back
        let bytes = serde_json::to_vec(&original).expect("Should serialize successfully");
        let reconstructed =
            serde_json::from_slice::<Transaction>(&bytes).expect("Should reconstruct successfully");

        // Verify all fields are identical
        assert_eq!(original.fee, reconstructed.fee);
        assert_eq!(original.fee_token, reconstructed.fee_token);
        assert_eq!(original.fee_payer_pk.x, reconstructed.fee_payer_pk.x);
        assert_eq!(
            original.fee_payer_pk.is_odd,
            reconstructed.fee_payer_pk.is_odd
        );
        assert_eq!(original.nonce, reconstructed.nonce);
        assert_eq!(original.valid_until, reconstructed.valid_until);
        assert_eq!(original.memo, reconstructed.memo);
        assert_eq!(original.tag, reconstructed.tag);
        assert_eq!(original.source_pk.x, reconstructed.source_pk.x);
        assert_eq!(original.source_pk.is_odd, reconstructed.source_pk.is_odd);
        assert_eq!(original.receiver_pk.x, reconstructed.receiver_pk.x);
        assert_eq!(
            original.receiver_pk.is_odd,
            reconstructed.receiver_pk.is_odd
        );
        assert_eq!(original.token_id, reconstructed.token_id);
        assert_eq!(original.amount, reconstructed.amount);
        assert_eq!(original.token_locked, reconstructed.token_locked);

        assert_eq!(original, reconstructed);
    }

    #[test]
    fn test_bytes_roundtrip_delegation() {
        let from = create_test_pubkey([7; 32]);
        let to = create_test_pubkey([8; 32]);
        let original = Transaction::new_delegation(from, to, 8000, 456)
            .set_memo_str("delegation test")
            .unwrap()
            .set_valid_until(11111);

        // Convert to bytes and back
        let bytes = serde_json::to_vec(&original).expect("Should serialize successfully");
        let reconstructed =
            serde_json::from_slice::<Transaction>(&bytes).expect("Should reconstruct successfully");

        // Verify all fields are identical
        assert_eq!(original.fee, reconstructed.fee);
        assert_eq!(original.fee_token, reconstructed.fee_token);
        assert_eq!(original.fee_payer_pk.x, reconstructed.fee_payer_pk.x);
        assert_eq!(
            original.fee_payer_pk.is_odd,
            reconstructed.fee_payer_pk.is_odd
        );
        assert_eq!(original.nonce, reconstructed.nonce);
        assert_eq!(original.valid_until, reconstructed.valid_until);
        assert_eq!(original.memo, reconstructed.memo);
        assert_eq!(original.tag, reconstructed.tag);
        assert_eq!(original.source_pk.x, reconstructed.source_pk.x);
        assert_eq!(original.source_pk.is_odd, reconstructed.source_pk.is_odd);
        assert_eq!(original.receiver_pk.x, reconstructed.receiver_pk.x);
        assert_eq!(
            original.receiver_pk.is_odd,
            reconstructed.receiver_pk.is_odd
        );
        assert_eq!(original.token_id, reconstructed.token_id);
        assert_eq!(original.amount, reconstructed.amount);
        assert_eq!(reconstructed.amount, 0);
        assert_eq!(original.token_locked, reconstructed.token_locked);

        assert_eq!(original, reconstructed);
    }

    #[test]
    fn test_deserialize_invalid_nonce_string() {
        let json = r#"{
            "to": "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            "from": "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            "fee": "10000",
            "amount": "1000000",
            "nonce": "not_a_number",
            "memo": "test",
            "valid_until": "12345",
            "tag": [
                    false,
                    false,
                    false
                ]
        }"#;

        let result: Result<Transaction, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_invalid_delegation_with_amount_string() {
        let json = r#"{
            "to": "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            "from": "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            "fee": "10000",
            "amount": "1000000",
            "nonce": "0",
            "memo": "test",
            "valid_until": "12345",
            "tag": [
                    false,
                    false,
                    true
                ]
        }"#;

        let result: Result<Transaction, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_valid_delegation_string() {
        let json = r#"{
            "to": "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            "from": "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            "fee": "10000",
            "nonce": "0",
            "memo": "test",
            "valid_until": "12345",
            "tag": [
                    false,
                    false,
                    true
                ]
        }"#;

        let result: Result<Transaction, _> = serde_json::from_str(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_deserialize_invalid_fee_negative() {
        let json = r#"{
            "to": "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            "from": "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            "fee": "-10000",
            "amount": "1000000",
            "nonce": "42",
            "memo": "test",
            "valid_until": "12345",
            "tag": [
                    false,
                    false,
                    false
                ]
        }"#;

        let result: Result<Transaction, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_invalid_amount_overflow() {
        let json = r#"{
            "to": "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            "from": "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            "fee": "10000",
            "amount": "18446744073709551616",
            "nonce": "42",
            "memo": "test",
            "valid_until": "12345",
            "tag": [
                    false,
                    false,
                    false
                ]
        }"#;

        let result: Result<Transaction, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_invalid_nonce_overflow() {
        let json = r#"{
            "to": "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            "from": "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            "fee": "10000",
            "amount": "1000000",
            "nonce": "4294967296",
            "memo": "test",
            "valid_until": "12345",
            "tag": [
                    false,
                    false,
                    false
                ]
        }"#;

        let result: Result<Transaction, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_invalid_address() {
        let json = r#"{
            "to": "invalid_address",
            "from": "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            "fee": "10000",
            "amount": "1000000",
            "nonce": "42",
            "memo": "test",
            "valid_until": "12345",
            "tag": [
                    false,
                    false,
                    false
                ]
        }"#;

        let result: Result<Transaction, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_missing_field() {
        let json = r#"{
            "to": "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            "from": "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            "fee": "10000",
            "nonce": "42",
            "memo": "test",
            "valid_until": "12345"
        }"#;

        let result: Result<Transaction, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_memo_length_parameter_correctness() {
        // This test specifically targets the length parameter usage
        // The old implementation would fail because it didn't respect the length byte
        let from = create_test_pubkey([15; 32]);
        let to = create_test_pubkey([16; 32]);

        let mut tx = Transaction::new_payment(from, to, 1000000, 10000, 42);

        // Set memo with length=5 but put more data after those 5 bytes
        tx.memo[0] = 0x01; // Format marker
        tx.memo[1] = 0x05; // Length = 5 bytes only
        tx.memo[2] = b'h';
        tx.memo[3] = b'e';
        tx.memo[4] = b'l';
        tx.memo[5] = b'l';
        tx.memo[6] = b'o';
        tx.memo[7] = b'j'; // This should be ignored (beyond length)
        tx.memo[8] = b'u'; // This should be ignored (beyond length)
        tx.memo[9] = b'n'; // This should be ignored (beyond length)
        tx.memo[10] = b'k'; // This should be ignored (beyond length)
                            // Rest zeros

        // Serialize to JSON
        let json = serde_json::to_string(&tx).unwrap();

        // The memo should only include the first 5 characters: "hello"
        // The old implementation would have included the junk data or failed
        assert!(json.contains("\"memo\":\"hello\""));
        assert!(!json.contains("junk"));

        // Roundtrip test
        let deserialized: Transaction = serde_json::from_str(&json).unwrap();
        assert_ne!(tx.memo, deserialized.memo);
    }

    #[test]
    fn test_deserialize_invalid_memo_too_long() {
        // Construct a memo longer than the allowed 32 bytes
        let long_memo = "A".repeat(33);

        let json = format!(
            r#"{{
            "to": "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            "from": "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
            "fee": "10000",
            "amount": "1000000",
            "nonce": "42",
            "memo": "{long_memo}",
            "valid_until": "12345",
            "tag": [false, false, false]
        }}"#
        );

        let result: Result<Transaction, _> = serde_json::from_str(&json);
        assert!(
            result.is_err(),
            "Deserialization should fail for too-long memo"
        );
    }

    #[test]
    fn test_set_memo_str_rejects_too_long() {
        let from = create_test_pubkey([21; 32]);
        let to = create_test_pubkey([22; 32]);
        let base = Transaction::new_payment(from, to, 1_000_000, 1_000, 1);

        let long_memo = "B".repeat(33);
        let res = base.set_memo_str(&long_memo);
        assert!(
            res.is_err(),
            "set_memo_str should return an error for too-long memo"
        );
        if let Err(e) = res {
            match e {
                BluePallasError::InvalidMemo(_) => {}
                other => panic!("Unexpected error variant: {:?}", other),
            }
        }
    }
}
