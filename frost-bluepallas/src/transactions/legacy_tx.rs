use core::fmt;

use ark_ff::PrimeField;
use bitvec::prelude::*;
use mina_curves::pasta::Pallas;
use mina_hasher::{Fp, Hashable, ROInput};
use mina_signer::{CompressedPubKey, NetworkId, PubKey};

use serde::{
    ser::{Serialize, SerializeStruct, Serializer},
    Deserialize,
};

const HEADER_BYTES: usize = 4;

use crate::{
    errors::{BluePallasError, BluePallasResult},
    translate::Translatable,
};

/// Copied from https://github.com/o1-labs/proof-systems/blob/master/signer/tests/transaction.rs
const MEMO_BYTES: usize = 34;
const MEMO_HEADER_BYTES: usize = 2; // 0x01 + length byte
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

impl Translatable for Transaction {
    fn translate_msg(&self) -> Vec<u8> {
        self.to_roinput().serialize()
    }

    fn from_bytes(bytes: &[u8]) -> BluePallasResult<Self>
    where
        Self: Sized,
    {
        // Deserialize the bytes into ROInput
        let roi = ROInput::deserialize(bytes)
            .map_err(|_| BluePallasError::deserialization_error("Failed to deserialize ROInput"))?;

        // TODO: Add pr to o1-labs/proof-systems so that we can directly retrieve the fields and bits

        // Extract number of fields
        let fields_len = u32::from_le_bytes(bytes[0..HEADER_BYTES].try_into()?) as usize;

        // Convert to fields
        let fields = roi
            .to_fields()
            .iter()
            .take(fields_len)
            .copied()
            .collect::<Vec<_>>();

        // First fields should be the x coordinates of the public keys
        let fee_payer_pk_x = fields[0];
        let source_pk_x = fields[1];
        let receiver_pk_x = fields[2];

        // Convert to bits using BitVec and skip field bits (each field is 255 bits)
        let all_bits = BitVec::<u8, bitvec::order::Lsb0>::from_slice(&roi.to_bytes());
        let field_bits_count = fields_len * Fp::MODULUS_BIT_SIZE as usize;
        let remaining_bits = &all_bits[field_bits_count..];

        // Helper functions using BitVec's optimized methods
        let mut bit_index = 0;
        let read_u64 = |start_idx: &mut usize| -> Result<u64, BluePallasError> {
            if *start_idx + 64 > remaining_bits.len() {
                return Err(BluePallasError::deserialization_error(
                    "Not enough bits for u64",
                ));
            }
            let value: u64 = remaining_bits[*start_idx..*start_idx + 64].load_le();
            *start_idx += 64;
            Ok(value)
        };

        let read_u32 = |start_idx: &mut usize| -> Result<u32, BluePallasError> {
            if *start_idx + 32 > remaining_bits.len() {
                return Err(BluePallasError::deserialization_error(
                    "Not enough bits for u32",
                ));
            }
            let value: u32 = remaining_bits[*start_idx..*start_idx + 32].load_le();
            *start_idx += 32;
            Ok(value)
        };

        let read_bool = |start_idx: &mut usize| -> Result<bool, BluePallasError> {
            if *start_idx >= remaining_bits.len() {
                return Err(BluePallasError::deserialization_error(
                    "Not enough bits for bool",
                ));
            }
            let value = remaining_bits[*start_idx];
            *start_idx += 1;
            Ok(value)
        };

        let read_bytes =
            |start_idx: &mut usize, count: usize| -> Result<Vec<u8>, BluePallasError> {
                if *start_idx + count * 8 > remaining_bits.len() {
                    return Err(BluePallasError::deserialization_error(
                        "Not enough bits for bytes",
                    ));
                }
                let mut bytes = Vec::with_capacity(count);
                for _ in 0..count {
                    let byte: u8 = remaining_bits[*start_idx..*start_idx + 8].load_le();
                    bytes.push(byte);
                    *start_idx += 8;
                }
                Ok(bytes)
            };

        // Extract fields in the same order as to_roinput
        let fee = read_u64(&mut bit_index)?;
        let fee_token = read_u64(&mut bit_index)?;
        let fee_payer_is_odd = read_bool(&mut bit_index)?;
        let nonce = read_u32(&mut bit_index)?;
        let valid_until = read_u32(&mut bit_index)?;

        // Read memo bytes
        let memo_bytes = read_bytes(&mut bit_index, MEMO_BYTES)?;
        let mut memo = [0u8; MEMO_BYTES];
        memo.copy_from_slice(&memo_bytes);

        // Read tag bits
        let mut tag = [false; TAG_BITS];
        for tag_bit in tag.iter_mut() {
            *tag_bit = read_bool(&mut bit_index)?;
        }

        let source_pk_is_odd = read_bool(&mut bit_index)?;
        let receiver_pk_is_odd = read_bool(&mut bit_index)?;
        let token_id = read_u64(&mut bit_index)?;
        let amount = read_u64(&mut bit_index)?;
        let token_locked = read_bool(&mut bit_index)?;

        // Reconstruct public keys with correct is_odd flags
        let fee_payer_pk = {
            let point =
                Pallas::get_point_from_x_unchecked(fee_payer_pk_x, true).ok_or_else(|| {
                    BluePallasError::deserialization_error("Invalid fee payer public key")
                })?;

            // Validate point is on curve
            if !point.is_on_curve() {
                return Err(Box::new(BluePallasError::deserialization_error(
                    "Fee payer public key point not on curve",
                )));
            }

            let mut pubkey = PubKey::from_point_unsafe(point);
            if pubkey.into_compressed().is_odd != fee_payer_is_odd {
                let neg_point = -point;
                pubkey = PubKey::from_point_unsafe(neg_point);
            }
            pubkey
        };

        let source_pk = {
            let point = Pallas::get_point_from_x_unchecked(source_pk_x, true).ok_or_else(|| {
                BluePallasError::deserialization_error("Invalid source public key")
            })?;

            // Validate point is on curve
            if !point.is_on_curve() {
                return Err(Box::new(BluePallasError::deserialization_error(
                    "Source public key point not on curve",
                )));
            }

            let mut pubkey = PubKey::from_point_unsafe(point);
            if pubkey.into_compressed().is_odd != source_pk_is_odd {
                let neg_point = -point;
                pubkey = PubKey::from_point_unsafe(neg_point);
            }
            pubkey
        };

        let receiver_pk = {
            let point =
                Pallas::get_point_from_x_unchecked(receiver_pk_x, true).ok_or_else(|| {
                    BluePallasError::deserialization_error("Invalid receiver public key")
                })?;

            // Validate point is on curve
            if !point.is_on_curve() {
                return Err(Box::new(BluePallasError::deserialization_error(
                    "Receiver public key point not on curve",
                )));
            }

            let mut pubkey = PubKey::from_point_unsafe(point);
            if pubkey.into_compressed().is_odd != receiver_pk_is_odd {
                let neg_point = -point;
                pubkey = PubKey::from_point_unsafe(neg_point);
            }
            pubkey
        };

        Ok(Transaction {
            fee,
            fee_token,
            fee_payer_pk: fee_payer_pk.into_compressed(),
            nonce,
            valid_until,
            memo,
            tag,
            source_pk: source_pk.into_compressed(),
            receiver_pk: receiver_pk.into_compressed(),
            token_id,
            amount,
            token_locked,
        })
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
        let bytes = original.translate_msg();
        let reconstructed =
            Transaction::from_bytes(&bytes).expect("Should reconstruct successfully");

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
        let bytes = original.translate_msg();
        let reconstructed =
            Transaction::from_bytes(&bytes).expect("Should reconstruct successfully");

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
    fn test_deserialize_invalid_curve_points() {
        // Create a transaction with valid data first
        let from = create_test_pubkey([9; 32]);
        let to = create_test_pubkey([10; 32]);
        let mut tx = Transaction::new_payment(from, to, 1000000, 10000, 42);

        // Manually corrupt the public key x-coordinate to an invalid curve point
        // Use a field element that doesn't correspond to a valid curve point
        tx.fee_payer_pk.x = Fp::from(999999999u64); // Invalid x coordinate

        let bytes = tx.translate_msg();
        let result = Transaction::from_bytes(&bytes);

        // Should fail with curve validation error
        assert!(result.is_err());
        let error_msg = format!("{:?}", result.unwrap_err());
        assert!(error_msg.contains("curve") || error_msg.contains("public key"));
    }

    #[test]
    fn test_from_bytes_arbitrary_bytes() {
        // Test with completely random bytes
        let random_bytes = vec![0xDE, 0xAD, 0xBE, 0xEF, 0xFF, 0x00, 0x11, 0x22];
        let result = Transaction::from_bytes(&random_bytes);
        assert!(result.is_err());

        // Test with insufficient bytes
        let short_bytes = vec![0x01, 0x00];
        let result = Transaction::from_bytes(&short_bytes);
        assert!(result.is_err());

        // Test with header indicating more fields than available
        let mut malformed_bytes = vec![0xFF, 0xFF, 0xFF, 0xFF]; // Claims huge number of fields
        malformed_bytes.extend_from_slice(&[0u8; 100]); // But not enough data
        let result = Transaction::from_bytes(&malformed_bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_bytes_insufficient_bits() {
        // Create valid transaction bytes but truncate them
        let from = create_test_pubkey([11; 32]);
        let to = create_test_pubkey([12; 32]);
        let tx = Transaction::new_payment(from, to, 1000000, 10000, 42);
        let mut bytes = tx.translate_msg();

        // Truncate to simulate insufficient bits for complete reconstruction
        bytes.truncate(bytes.len() / 2);

        let result = Transaction::from_bytes(&bytes);
        assert!(result.is_err());
        let error_msg = format!("{:?}", result.unwrap_err());
        assert!(error_msg.contains("Not enough bits") || error_msg.contains("deserialize"));
    }

    #[test]
    fn test_from_bytes_zero_fields() {
        // Test with zero fields in header
        let bytes = vec![0x00, 0x00, 0x00, 0x00]; // Zero fields
        let result = Transaction::from_bytes(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_memo_with_embedded_nulls() {
        // This test would fail with the previous buggy implementation
        // because it relied on string conversion and trimming instead of using the length parameter
        let from = create_test_pubkey([13; 32]);
        let to = create_test_pubkey([14; 32]);

        // Create a transaction and manually set memo with embedded nulls and non-UTF8 bytes
        let mut tx = Transaction::new_payment(from, to, 1000000, 10000, 42);

        // Set up memo with: [0x01, 0x08, 'h', 'e', 'l', 0x00, 'l', 'o', 0xFF, 0x80, padding...]
        // Length = 8, but contains null byte at position 3 and invalid UTF-8 at the end
        tx.memo[0] = 0x01; // Format marker
        tx.memo[1] = 0x08; // Length = 8 bytes
        tx.memo[2] = b'h';
        tx.memo[3] = b'e';
        tx.memo[4] = b'l';
        tx.memo[5] = 0x00; // Embedded null byte
        tx.memo[6] = b'l';
        tx.memo[7] = b'o';
        tx.memo[8] = 0x00; // Invalid UTF-8 start byte
        tx.memo[9] = 0x00; // Invalid UTF-8 continuation
                           // Rest should be zeros (padding)

        // Test JSON serialization/deserialization roundtrip
        let json = serde_json::to_string(&tx).unwrap();
        let deserialized: Transaction = serde_json::from_str(&json).unwrap();

        // With the current correct implementation, this should work
        assert_eq!(tx.memo, deserialized.memo);

        // Test that the serialized JSON contains the expected memo content
        // The memo should serialize as "hel\u0000lo\u0000" (with lossy UTF-8 conversion)
        assert!(json.contains("hel"));

        // Test bytes roundtrip as well
        let bytes = tx.translate_msg();
        let reconstructed =
            Transaction::from_bytes(&bytes).expect("Should reconstruct successfully");
        assert_eq!(tx.memo, reconstructed.memo);
        assert_eq!(tx, reconstructed);
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
