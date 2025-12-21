// ! Module for packing zkApp transaction structures into a format suitable for hashing.
use alloc::vec::Vec;
use ark_ff::{AdditiveGroup, BigInt};
use mina_hasher::Fp;

use crate::transactions::zkapp_tx::{
    commit::{hash_noinput, hash_with_prefix},
    constants::{
        MINA_ZKAPP_URI, ZK_ACTION_STATE_EMPTY, ZK_APP_ACTIONS_EMPTY, ZK_APP_ACTIONS_PREFIX,
        ZK_APP_EVENTS_EMPTY, ZK_APP_EVENTS_PREFIX, ZK_APP_EVENT_PREFIX,
    },
    *,
};

// ------------------------------------------------------------------------------------------------
// -----------------------------STRUCTS -----------------------------------------------------------
// ------------------------------------------------------------------------------------------------

// Represents a random oracle input (ROInput) from mina-hasher but with a different structure
// that is easier to debug and work with
#[derive(Default)]
pub struct PackedInput {
    pub bits: Vec<BitData>,
    pub fields: Vec<Fp>,
}

// Represents bits as tuples simillarly as o1js in Typescript
// Similar structure exists in mina-haher but it is very restrictive and we prefer to have our own
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

    pub fn pack_to_fields(self) -> PackedInput {
        let fields = self.fields;
        let bits = self.bits;

        if bits.is_empty() {
            return PackedInput { bits, fields };
        }

        let mut packed_bits = Vec::new();
        let mut current_packed_field = Fp::ZERO;
        let mut current_size = 0;
        for bit_data in bits {
            let size = bit_data.bit_data_size();
            let field = bit_data.to_field();

            current_size += size;
            if current_size < 255 {
                current_packed_field =
                    current_packed_field * Fp::from(BigInt::from(1u64) << size as u32) + field;
            } else {
                packed_bits.push(current_packed_field);
                current_size = size;
                current_packed_field = field;
            }
        }
        packed_bits.push(current_packed_field);
        PackedInput {
            bits: vec![],
            fields: [fields, packed_bits].concat(),
        }
    }
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

// ------------------------------------------------------------------------------------------------
// ----------------------------- TRAITS -----------------------------------------------------------
// ------------------------------------------------------------------------------------------------

// This trait is implemented for all structures in zkApp_tx - specifically ZkAppCommand and its substructures
pub trait Packable {
    fn pack(&self) -> PackedInput;
}

// Used for packing empty values for Option<T>
pub trait Emptiable {
    fn empty_roi() -> PackedInput;
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
        let fields = field_inputs.pack_to_fields().fields;
        let hash = hash_with_prefix(MINA_ZKAPP_URI, &fields).unwrap();
        PackedInput::new().append_field(hash)
    }
}

// ------------------------------------------------------------------------------------------------
// ----------------------------- EMPTIABLE IMPLEMENTATIONS ----------------------------------------
// ------------------------------------------------------------------------------------------------

impl Emptiable for Field {
    fn empty_roi() -> PackedInput {
        Self(mina_hasher::Fp::from(0)).pack()
    }
}

impl Emptiable for ActionState {
    fn empty_roi() -> PackedInput {
        let field = hash_noinput(ZK_ACTION_STATE_EMPTY).unwrap();

        PackedInput::new().append_field(field)
    }
}

impl Emptiable for VerificationKeyData {
    fn empty_roi() -> PackedInput {
        Field::empty_roi()
    }
}

impl Emptiable for PublicKey {
    fn empty_roi() -> PackedInput {
        PublicKey(CompressedPubKey::empty()).pack()
    }
}

impl Emptiable for Permissions {
    fn empty_roi() -> PackedInput {
        use crate::transactions::zkapp_tx::constants::TXN_VERSION_CURRENT;
        use crate::transactions::zkapp_tx::AuthRequired::*;
        Self {
            edit_state: None,
            send: None,
            receive: None,
            access: None,
            set_delegate: None,
            set_permissions: None,
            set_verification_key: SetVerificationKey {
                auth: None,
                txn_version: TXN_VERSION_CURRENT,
            },
            set_zkapp_uri: None,
            edit_action_state: None,
            set_token_symbol: None,
            increment_nonce: None,
            set_voting_for: None,
            set_timing: None,
        }
        .pack()
    }
}

impl Emptiable for TokenSymbol {
    fn empty_roi() -> PackedInput {
        let mut roi = PackedInput::new();
        roi = roi.append_packedinput(TokenSymbol::default().pack());
        roi
    }
}

impl Emptiable for TimingData {
    fn empty_roi() -> PackedInput {
        Self {
            initial_minimum_balance: 0,
            cliff_time: 0,
            cliff_amount: 0,
            vesting_period: 0,
            vesting_increment: 0,
        }
        .pack()
    }
}

impl Emptiable for RangeCondition<UInt32> {
    fn empty_roi() -> PackedInput {
        Self {
            lower: UInt32::MIN,
            upper: UInt32::MAX,
        }
        .pack()
    }
}

impl Emptiable for RangeCondition<UInt64> {
    fn empty_roi() -> PackedInput {
        Self {
            lower: UInt64::MIN,
            upper: UInt64::MAX,
        }
        .pack()
    }
}

impl Emptiable for ZkappUri {
    fn empty_roi() -> PackedInput {
        let mut roi = mina_hasher::ROInput::new();
        roi = roi.append_field(Fp::ZERO);
        roi = roi.append_field(Fp::ZERO);
        let hash = hash_with_prefix(MINA_ZKAPP_URI, &roi.to_fields()).unwrap();
        PackedInput::new().append_field(hash)
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
    fn test_pub_key_packing() {
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
    fn test_auth_required_packing() {
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
    fn test_balance_change_positive_packing() {
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
    fn test_balance_change_negative_packing() {
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
    fn test_may_use_token_packing() {
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
    fn test_token_symbol_packing() {
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
