//! ZKApp Packable trait implementations
use crate::transactions::zkapp_tx::commit::{hash_noinput, hash_with_prefix};
use crate::transactions::zkapp_tx::constants::{
    MINA_ZKAPP_URI, ZK_APP_ACTIONS_EMPTY, ZK_APP_ACTIONS_PREFIX, ZK_APP_EVENTS_EMPTY,
    ZK_APP_EVENTS_PREFIX, ZK_APP_EVENT_PREFIX,
};
use crate::transactions::zkapp_tx::hash::pack_to_fields;
use crate::transactions::zkapp_tx::zkapp_emptiable::Emptiable;
use crate::transactions::zkapp_tx::AccountUpdate;
use crate::transactions::zkapp_tx::*;
use ark_ec::AdditiveGroup;
use mina_hasher::{Fp, ROInput};

// ------------------------------------------------------------------------------------------------
// --------------------------------- PACKABLE TRAIT------------------------------------------------
// ------------------------------------------------------------------------------------------------

pub trait Packable {
    fn pack(&self) -> PackedInput;
}

#[derive(PartialEq, Debug)]
pub enum BitData {
    U32 { val: u32 },
    U64 { val: u64 },
    BOOL { val: bool },
    BYTES { val: Vec<u8> },
}

impl BitData {
    pub fn bit_data_size(&self) -> usize {
        match self {
            BitData::U32 { .. } => 32,
            BitData::U64 { .. } => 64,
            BitData::BOOL { .. } => 1,
            BitData::BYTES { val } => val.len() * 8,
        }
    }

    pub fn to_field(&self) -> Fp {
        match self {
            BitData::U32 { val } => Fp::from(*val as u64),
            BitData::U64 { val } => Fp::from(*val),
            BitData::BOOL { val } => {
                if *val {
                    Fp::ONE
                } else {
                    Fp::ZERO
                }
            }
            BitData::BYTES { val } => {
                let mut bytes = [0u8; 32];
                let len = val.len().min(32);
                bytes[..len].copy_from_slice(&val[..len]);
                Fp::from_random_bytes(&bytes).expect("Failed to convert bytes to field")
            }
        }
    }
}

// Represents a random oracle input (ROInput) from mina-hasher but with a different structure
// that is easier to debug and work with
#[derive(Default)]
pub struct PackedInput {
    pub bits: Vec<BitData>,
    pub fields: Vec<Fp>,
}

// Represents bits as tuples simillarly as o1js in Typescript
impl PackedInput {
    /// Create a new empty random oracle input
    pub fn new() -> Self {
        PackedInput {
            fields: vec![],
            bits: Vec::new(),
        }
    }

    pub fn append_packedinput(mut self, mut roi: PackedInput) -> Self {
        self.fields.append(&mut roi.fields);
        self.bits.extend(roi.bits);
        self
    }

    pub fn append_field(mut self, f: Fp) -> Self {
        self.fields.push(f);
        self
    }

    pub fn append_bool(mut self, b: bool) -> Self {
        self.bits.push(BitData::BOOL { val: b });
        self
    }

    pub fn append_u32(mut self, x: u32) -> Self {
        self.bits.push(BitData::U32 { val: x });
        self
    }

    pub fn append_u64(mut self, x: u64) -> Self {
        self.bits.push(BitData::U64 { val: x });
        self
    }

    pub fn append_bytes(mut self, bytes: &[u8]) -> Self {
        self.bits.push(BitData::BYTES {
            val: bytes.to_vec(),
        });
        self
    }

    pub fn to_mina_hasher_roi(self) -> ROInput {
        let mut inputs = ROInput::new();

        for field in self.fields {
            inputs = inputs.append_field(field)
        }

        for bit_data in self.bits {
            match bit_data {
                BitData::U32 { val } => {
                    inputs = inputs.append_u32(val);
                }
                BitData::U64 { val } => {
                    inputs = inputs.append_u64(val);
                }
                BitData::BOOL { val } => {
                    inputs = inputs.append_bool(val);
                }
                BitData::BYTES { val } => {
                    inputs = inputs.append_bytes(val.as_slice());
                }
            }
        }

        inputs
    }
}

// ------------------------------------------------------------------------------------------------
// ----------------------------- PACKABLE FOR COMPOSITE TYPES -------------------------------------
// ------------------------------------------------------------------------------------------------

impl Packable for AccountUpdate {
    fn pack(&self) -> PackedInput {
        // AccountUpdate only uses the body for inputs
        self.body.pack()
    }
}

impl Packable for AccountUpdateBody {
    fn pack(&self) -> PackedInput {
        let mut roi = PackedInput::new();
        roi = roi.append_packedinput(self.public_key.pack());
        roi = roi.append_field(self.token_id.0 .0);
        roi = roi.append_packedinput(self.update.pack()); // Update
        roi = roi.append_packedinput(self.balance_change.pack()); // BalanceChange
        roi = roi.append_bool(self.increment_nonce);
        roi = roi.append_packedinput(self.events.pack()); // Events
        roi = roi.append_packedinput(self.actions.pack()); // Actions
        roi = roi.append_field(self.call_data.0);
        roi = roi.append_packedinput(self.preconditions.pack()); // Preconditions
        roi = roi.append_bool(self.use_full_commitment);
        roi = roi.append_bool(self.implicit_account_creation_fee);
        roi = roi.append_packedinput(self.may_use_token.pack()); // MayUseToken
        roi = roi.append_packedinput(self.authorization_kind.pack()); // AuthorizationKind
        roi
    }
}

impl Packable for Update {
    fn pack(&self) -> PackedInput {
        let mut roi = PackedInput::new();

        roi = roi.append_packedinput(self.app_state.pack());
        roi = roi.append_packedinput(self.delegate.pack());
        roi = roi.append_packedinput(self.verification_key.pack());
        roi = roi.append_packedinput(self.permissions.pack());
        roi = roi.append_packedinput(self.zkapp_uri.pack());
        roi = roi.append_packedinput(self.token_symbol.pack());
        roi = roi.append_packedinput(self.timing.pack());
        roi = roi.append_packedinput(self.voting_for.pack());
        roi
    }
}

impl Packable for Permissions {
    fn pack(&self) -> PackedInput {
        PackedInput::new()
            .append_packedinput(self.edit_state.pack())
            .append_packedinput(self.access.pack())
            .append_packedinput(self.send.pack())
            .append_packedinput(self.receive.pack())
            .append_packedinput(self.set_delegate.pack())
            .append_packedinput(self.set_permissions.pack())
            .append_packedinput(self.set_verification_key.pack())
            .append_packedinput(self.set_zkapp_uri.pack())
            .append_packedinput(self.edit_action_state.pack())
            .append_packedinput(self.set_token_symbol.pack())
            .append_packedinput(self.increment_nonce.pack())
            .append_packedinput(self.set_voting_for.pack())
            .append_packedinput(self.set_timing.pack())
    }
}

impl Packable for Events {
    fn pack(&self) -> PackedInput {
        let init = hash_noinput(ZK_APP_EVENTS_EMPTY).unwrap();

        let out: Fp = self.data.iter().rfold(init, |acc: Fp, event: &Vec<Field>| {
            let event_hash = hash_with_prefix(
                ZK_APP_EVENT_PREFIX,
                event.iter().map(|f| f.0).collect::<Vec<Fp>>().as_slice(),
            )
            .unwrap();
            hash_with_prefix(ZK_APP_EVENTS_PREFIX, &[acc, event_hash]).unwrap()
        });
        PackedInput::new().append_field(out)
    }
}

impl Packable for Actions {
    fn pack(&self) -> PackedInput {
        let init = hash_noinput(ZK_APP_ACTIONS_EMPTY).unwrap();

        let out: Fp = self.data.iter().rfold(init, |acc: Fp, event: &Vec<Field>| {
            let event_hash = hash_with_prefix(
                ZK_APP_EVENT_PREFIX,
                event.iter().map(|f| f.0).collect::<Vec<Fp>>().as_slice(),
            )
            .unwrap();
            hash_with_prefix(ZK_APP_ACTIONS_PREFIX, &[acc, event_hash]).unwrap()
        });
        PackedInput::new().append_field(out)
    }
}

impl Packable for TimingData {
    fn pack(&self) -> PackedInput {
        PackedInput::new()
            .append_u64(self.initial_minimum_balance)
            .append_u32(self.cliff_time)
            .append_u64(self.cliff_amount)
            .append_u32(self.vesting_period)
            .append_u64(self.vesting_increment)
    }
}

impl Packable for AuthRequired {
    fn pack(&self) -> PackedInput {
        let encoded_auth = self.clone().encode();
        PackedInput::new()
            .append_bool(encoded_auth.constant)
            .append_bool(encoded_auth.signature_necessary)
            .append_bool(encoded_auth.signature_sufficient)
    }
}

impl Packable for SetVerificationKey {
    fn pack(&self) -> PackedInput {
        PackedInput::new()
            .append_packedinput(self.auth.pack())
            .append_u32(self.txn_version)
    }
}

impl Packable for VerificationKeyData {
    fn pack(&self) -> PackedInput {
        // Skip the actual data, only pack the hash
        PackedInput::new().append_field(self.hash.0)
    }
}

impl Packable for BalanceChange {
    fn pack(&self) -> PackedInput {
        let sgn = self.sgn == 1;
        PackedInput::new()
            .append_u64(self.magnitude)
            .append_bool(sgn)
    }
}

impl Packable for MayUseToken {
    fn pack(&self) -> PackedInput {
        // two bits
        PackedInput::new()
            .append_bool(self.parents_own_token)
            .append_bool(self.inherit_from_parent)
    }
}

impl Packable for AuthorizationKind {
    fn pack(&self) -> PackedInput {
        PackedInput::new()
            .append_bool(self.is_signed)
            .append_bool(self.is_proved)
            .append_field(self.verification_key_hash.0)
    }
}

impl Packable for EpochLedger {
    fn pack(&self) -> PackedInput {
        let mut roi = PackedInput::new();
        roi = roi.append_packedinput(self.hash.pack());
        roi = roi.append_packedinput(self.total_currency.pack());
        roi
    }
}

impl Packable for EpochData {
    fn pack(&self) -> PackedInput {
        let mut roi = PackedInput::new();
        roi = roi.append_packedinput(self.ledger.pack());
        roi = roi.append_packedinput(self.seed.pack());
        roi = roi.append_packedinput(self.start_checkpoint.pack());
        roi = roi.append_packedinput(self.lock_checkpoint.pack());
        roi = roi.append_packedinput(self.epoch_length.pack());
        roi
    }
}

impl Packable for NetworkPreconditions {
    fn pack(&self) -> PackedInput {
        let mut roi = PackedInput::new();
        roi = roi.append_packedinput(self.snarked_ledger_hash.pack());
        roi = roi.append_packedinput(self.blockchain_length.pack());
        roi = roi.append_packedinput(self.min_window_density.pack());
        roi = roi.append_packedinput(self.total_currency.pack());
        roi = roi.append_packedinput(self.global_slot_since_genesis.pack());
        roi = roi.append_packedinput(self.staking_epoch_data.pack());
        roi = roi.append_packedinput(self.next_epoch_data.pack());
        roi
    }
}

impl Packable for AccountPreconditions {
    fn pack(&self) -> PackedInput {
        let mut roi = PackedInput::new();
        roi = roi.append_packedinput(self.balance.pack());
        roi = roi.append_packedinput(self.nonce.pack());
        roi = roi.append_packedinput(self.receipt_chain_hash.pack());
        roi = roi.append_packedinput(self.delegate.pack());
        roi = roi.append_packedinput(self.state.pack());
        roi = roi.append_packedinput(self.action_state.pack());
        roi = roi.append_packedinput(self.proved_state.pack());
        roi = roi.append_packedinput(self.is_new.pack());
        roi
    }
}

impl Packable for Preconditions {
    fn pack(&self) -> PackedInput {
        PackedInput::new()
            .append_packedinput(self.network.pack())
            .append_packedinput(self.account.pack())
            .append_packedinput(self.valid_while.pack())
    }
}

impl Packable for TokenSymbol {
    fn pack(&self) -> PackedInput {
        let mut roi = PackedInput::new();
        let mut s = <[u8; 6]>::default();
        self.to_bytes(&mut s);
        roi = roi.append_bytes(&s);
        roi
    }
}

impl Packable for ZkappUri {
    fn pack(&self) -> PackedInput {
        let mut field_inputs = PackedInput::new();
        for c in self.0.as_slice() {
            for j in 0..8 {
                field_inputs = field_inputs.append_bool((c & (1 << j)) != 0);
            }
        }
        field_inputs = field_inputs.append_bool(true);
        let fields = pack_to_fields(field_inputs).fields;
        let hash = hash_with_prefix(MINA_ZKAPP_URI, &fields).unwrap();
        PackedInput::new().append_field(hash)
    }
}

// ------------------------------------------------------------------------------------------------
// ----------------------------- PACKABLE IMPLEMENTATIONS FOR BASIC TYPES -------------------------
// ------------------------------------------------------------------------------------------------

impl Packable for PublicKey {
    fn pack(&self) -> PackedInput {
        // PublicKey wraps CompressedPubKey which exposes x and is_odd via conversion
        let pk: mina_signer::CompressedPubKey = self.clone().into();
        PackedInput::new().append_field(pk.x).append_bool(pk.is_odd)
    }
}

impl Packable for Field {
    fn pack(&self) -> PackedInput {
        PackedInput::new().append_field(self.0)
    }
}

impl Packable for ActionState {
    fn pack(&self) -> PackedInput {
        PackedInput::new().append_field(self.0 .0)
    }
}

impl Packable for RangeCondition<UInt32> {
    fn pack(&self) -> PackedInput {
        PackedInput::new()
            .append_u32(self.lower)
            .append_u32(self.upper)
    }
}

impl Packable for RangeCondition<UInt64> {
    fn pack(&self) -> PackedInput {
        PackedInput::new()
            .append_u64(self.lower)
            .append_u64(self.upper)
    }
}

impl<T> Packable for RangeCondition<T>
where
    T: Packable,
{
    fn pack(&self) -> PackedInput {
        PackedInput::new()
            .append_packedinput(self.lower.pack())
            .append_packedinput(self.upper.pack())
    }
}

impl<T> Packable for Option<T>
where
    T: Packable + Emptiable,
{
    fn pack(&self) -> PackedInput {
        let mut roi = PackedInput::new();
        roi = roi.append_bool(self.is_some());
        if self.is_some() {
            roi = roi.append_packedinput(self.as_ref().unwrap().pack());
        } else {
            roi = roi.append_packedinput(T::empty_roi());
        }
        roi
    }
}

impl Packable for Bool {
    fn pack(&self) -> PackedInput {
        PackedInput::new().append_bool(*self)
    }
}

impl Packable for Option<Bool> {
    fn pack(&self) -> PackedInput {
        let mut roi = PackedInput::new();
        roi = roi.append_bool(self.is_some());
        if self.is_some() {
            roi = roi.append_bool(self.unwrap());
        } else {
            roi = roi.append_bool(false);
        }
        roi
    }
}

impl<T: Packable> Packable for Vec<T> {
    fn pack(&self) -> PackedInput {
        let mut roi = PackedInput::new();
        for item in self {
            roi = roi.append_packedinput(item.pack());
        }
        roi
    }
}

impl<T: Packable, const N: usize> Packable for [T; N] {
    fn pack(&self) -> PackedInput {
        let mut roi = PackedInput::new();
        for item in self {
            roi = roi.append_packedinput(item.pack());
        }
        roi
    }
}

// ------------------------------------------------------------------------------------------------
// ----------------------------- TESTS ------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;
    use alloc::{string::String, vec::Vec};
    use core::str::FromStr;
    use mina_hasher::Fp;
    use mina_signer::CompressedPubKey;

    #[derive(Clone)]
    enum ROValue {
        Field(String),
        Bool(bool),
        #[allow(dead_code)]
        U32(u32),
        U64(u64),
        Bytes(Vec<u8>),
    }

    fn build_roi(values: Vec<ROValue>) -> PackedInput {
        let mut roi = PackedInput::new();
        for value in values {
            match value {
                ROValue::Field(fs) => {
                    let f = Fp::from_str(&fs).expect("Invalid field string");
                    roi = roi.append_field(f);
                }
                ROValue::Bool(b) => {
                    roi = roi.append_bool(b);
                }
                ROValue::U32(n) => {
                    roi = roi.append_u32(n);
                }
                ROValue::U64(n) => {
                    roi = roi.append_u64(n);
                }
                ROValue::Bytes(bytes) => {
                    roi = roi.append_bytes(&bytes);
                }
            }
        }
        roi
    }

    fn assert_roi_equal(roi: PackedInput, expected: PackedInput) {
        // Using an unsafe method to access private fields for testing purposes
        assert!(
            roi.fields == expected.fields,
            "Fields do not match. Expected \n{:?}\n but got\n{:?}\n",
            expected.fields,
            roi.fields
        );
        assert!(
            roi.bits == expected.bits,
            "Fields do not match. Expected \n{:?}\n but got\n{:?}\n",
            expected.bits,
            roi.bits
        );
    }

    // Helper function to get the test public key
    // This corresponds to B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg in base58
    fn get_test_public_key() -> CompressedPubKey {
        // CompressedPubKey format: [x_coordinate:32][parity:1] = 33 bytes = 66 hex characters
        let pub_key_hex = "0f48c65bd25f85f3e4ea4efebeb75b797bd743603be04b4ead845698b76bd33101";
        let pk_bytes = hex::decode(pub_key_hex).expect("Invalid hex in public key");
        CompressedPubKey::from_bytes(&pk_bytes).expect("Invalid public key bytes")
    }

    #[test]
    fn test_pub_key() {
        let pk = get_test_public_key();
        let roi = super::PublicKey(pk).pack();

        let expected_roi = build_roi(vec![
            ROValue::Field(
                "22536877747820698688010660184495467853785925552441222123266613953322243475471"
                    .to_string(),
            ),
            ROValue::Bool(true),
        ]);

        assert_roi_equal(roi, expected_roi);
    }

    #[test]
    fn test_auth_required() {
        let auth = super::AuthRequired::Either;
        let roi = auth.pack();
        let expected_roi = build_roi(vec![
            ROValue::Bool(false),
            ROValue::Bool(false),
            ROValue::Bool(true),
        ]);

        assert_roi_equal(roi, expected_roi);
    }

    #[test]
    fn test_balance_change_positive() {
        // Amount: +1 MINA (1000000000 nanomina)
        let balance_change = super::BalanceChange {
            magnitude: 1000000000,
            sgn: 1,
        };
        let roi = balance_change.pack();

        let expected_roi = build_roi(vec![ROValue::U64(1000000000), ROValue::Bool(true)]);

        assert_roi_equal(roi, expected_roi);
    }

    #[test]
    fn test_balance_change_negative() {
        // Amount: -0.5 MINA (500000000 nanomina)
        // Negative sign is represented as -1 in sgn field
        let balance_change = super::BalanceChange {
            magnitude: 500000000,
            sgn: -1,
        };
        let roi = balance_change.pack();

        let expected_roi = build_roi(vec![ROValue::U64(500000000), ROValue::Bool(false)]);

        assert_roi_equal(roi, expected_roi);
    }

    #[test]
    fn test_may_use_token() {
        // Configuration: parentsOwnToken=false, inheritFromParent=true
        let may_use_token = super::MayUseToken {
            parents_own_token: false,
            inherit_from_parent: true,
        };
        let roi = may_use_token.pack();
        let expected_roi = build_roi(vec![ROValue::Bool(false), ROValue::Bool(true)]);

        assert_roi_equal(roi, expected_roi);
    }

    #[test]
    fn test_token_symbol_data() {
        // Symbol: "MINA"
        // toInput should only contain packed field value (48 bits), not the symbol bytes
        let token_symbol = super::TokenSymbol::from_str("MINA").unwrap();
        let roi = token_symbol.pack();

        // According to spec: packed field only, NOT bytes + field
        let expected_roi = build_roi(vec![ROValue::Bytes(Vec::<u8>::from(&[
            0x4d, 0x49, 0x4e, 0x41, 0x00, 0x00,
        ]))]);

        assert_roi_equal(roi, expected_roi);
    }
}
