use mina_hasher::{Hashable, ROInput};
use mina_signer::{CompressedPubKey, NetworkId, PubKey};

use serde::{
    ser::{Serialize, SerializeStruct, Serializer},
    Deserialize,
};

/// Copied from https://github.com/o1-labs/proof-systems/blob/master/signer/tests/transaction.rs
const MEMO_BYTES: usize = 34;
const MEMO_HEADER_BYTES: usize = 2; // 0x01 + length byte
const TAG_BITS: usize = 3;
const PAYMENT_TX_TAG: [bool; TAG_BITS] = [false, false, false];
const DELEGATION_TX_TAG: [bool; TAG_BITS] = [false, false, true];

#[derive(Clone)]
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
        state.serialize_field("amount", &self.amount.to_string())?;
        state.serialize_field("nonce", &self.nonce.to_string())?;
        // Memo: drop trailing zeros, or replace with empty string if all zero?
        let memo_str = String::from_utf8(self.memo.to_vec())
            .unwrap_or_default()
            .trim_end_matches(char::from(0))
            .to_string();

        // Serialize memo as a string, dropping the header bytes
        // If length of memo is less than MEMO_HEADER_BYTES, it means it's empty ---
        if memo_str.len() < MEMO_HEADER_BYTES {
            state.serialize_field("memo", "")?;
        } else {
            state.serialize_field("memo", &memo_str[MEMO_HEADER_BYTES..])?;
        }

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
            amount: String,
            nonce: String,
            memo: String,
            valid_until: String,
            tag: [bool; TAG_BITS],
        }

        let data = TransactionData::deserialize(deserializer)?;

        let from = PubKey::from_address(&data.from).map_err(serde::de::Error::custom)?;
        let to = PubKey::from_address(&data.to).map_err(serde::de::Error::custom)?;
        let fee = data.fee.parse().map_err(serde::de::Error::custom)?;
        let amount = data.amount.parse().map_err(serde::de::Error::custom)?;
        let nonce = data.nonce.parse().map_err(serde::de::Error::custom)?;
        let valid_until = data.valid_until.parse().map_err(serde::de::Error::custom)?;

        // Switch case statement
        let tx = match data.tag {
            PAYMENT_TX_TAG => Transaction::new_payment(from, to, amount, fee, nonce)
                .set_memo_str(&data.memo)
                .set_valid_until(valid_until),
            DELEGATION_TX_TAG => Transaction::new_delegation(from, to, fee, nonce)
                .set_memo_str(&data.memo)
                .set_valid_until(valid_until),
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

    pub fn set_memo_str(mut self, memo: &str) -> Self {
        self.memo[0] = 0x01;
        self.memo[1] = core::cmp::min(memo.len(), MEMO_BYTES - 2) as u8;
        let memo = format!("{memo:\0<32}"); // Pad user-supplied memo with zeros
        self.memo[2..]
            .copy_from_slice(&memo.as_bytes()[..core::cmp::min(memo.len(), MEMO_BYTES - 2)]);
        // Anything beyond MEMO_BYTES is truncated

        self
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
        assert_eq!(original.fee, deserialized.fee);
        assert_eq!(original.nonce, deserialized.nonce);
        assert_eq!(original.valid_until, deserialized.valid_until);
        assert_eq!(original.tag, deserialized.tag);
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
            "valid_until": "12345"
        }"#;

        let result: Result<Transaction, _> = serde_json::from_str(json);
        assert!(result.is_err());
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
            "valid_until": "12345"
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
            "valid_until": "12345"
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
            "valid_until": "12345"
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
            "valid_until": "12345"
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
}
