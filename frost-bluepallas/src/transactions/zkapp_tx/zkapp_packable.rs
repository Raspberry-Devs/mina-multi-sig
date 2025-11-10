//! ZKApp Packable trait implementations
use crate::transactions::zkapp_tx::commit::hash_with_prefix;
use crate::transactions::zkapp_tx::constants::MINA_ZKAPP_URI;
use crate::transactions::zkapp_tx::zkapp_emptiable::Emptiable;
use crate::transactions::zkapp_tx::AccountUpdate;
use crate::transactions::zkapp_tx::*;
use mina_hasher::ROInput;

pub trait Packable {
    fn pack(&self) -> ROInput;
}

impl Packable for AccountUpdate {
    fn pack(&self) -> ROInput {
        // AccountUpdate only uses the body for inputs
        self.body.pack()
    }
}

impl Packable for AccountUpdateBody {
    fn pack(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi = roi.append_roinput(self.public_key.pack());
        roi = roi.append_field(self.token_id.0 .0);
        roi = roi.append_roinput(self.update.pack()); // Update
        roi = roi.append_roinput(self.balance_change.pack()); // BalanceChange
        roi = roi.append_bool(self.increment_nonce);
        roi = roi.append_roinput(self.events.pack()); // Events
        roi = roi.append_roinput(self.actions.pack()); // Actions
        roi = roi.append_field(self.call_data.0);
        roi = roi.append_u32(self.call_depth);
        roi = roi.append_roinput(self.preconditions.pack()); // Preconditions
        roi = roi.append_bool(self.use_full_commitment);
        roi = roi.append_bool(self.implicit_account_creation_fee);
        roi = roi.append_roinput(self.may_use_token.pack()); // MayUseToken
        roi = roi.append_roinput(self.authorization_kind.pack()); // AuthorizationKind
        roi
    }
}

impl Packable for Update {
    fn pack(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi = roi.append_roinput(self.app_state.pack());
        roi = roi.append_roinput(self.delegate.pack());
        roi = roi.append_roinput(self.verification_key.pack());
        roi = roi.append_roinput(self.permissions.pack());
        roi = roi.append_roinput(self.zkapp_uri.pack());
        roi = roi.append_roinput(self.token_symbol.pack());
        roi = roi.append_roinput(self.timing.pack());
        roi = roi.append_roinput(self.voting_for.pack());
        roi
    }
}

impl Packable for Permissions {
    fn pack(&self) -> ROInput {
        ROInput::new()
            .append_roinput(self.edit_state.pack())
            .append_roinput(self.access.pack())
            .append_roinput(self.send.pack())
            .append_roinput(self.receive.pack())
            .append_roinput(self.set_delegate.pack())
            .append_roinput(self.set_permissions.pack())
            .append_roinput(self.set_verification_key.pack())
            .append_roinput(self.set_zkapp_uri.pack())
            .append_roinput(self.edit_action_state.pack())
            .append_roinput(self.set_token_symbol.pack())
            .append_roinput(self.increment_nonce.pack())
            .append_roinput(self.set_voting_for.pack())
            .append_roinput(self.set_timing.pack())
    }
}

impl Packable for Events {
    fn pack(&self) -> ROInput {
        ROInput::new().append_field(self.hash.0)
    }
}

impl Packable for Actions {
    fn pack(&self) -> ROInput {
        ROInput::new().append_field(self.hash.0)
    }
}

impl Packable for TimingData {
    fn pack(&self) -> ROInput {
        ROInput::new()
            .append_u64(self.initial_minimum_balance)
            .append_u32(self.cliff_time)
            .append_u64(self.cliff_amount)
            .append_u32(self.vesting_period)
            .append_u64(self.vesting_increment)
    }
}

impl Packable for AuthRequired {
    fn pack(&self) -> ROInput {
        let encoded_auth = self.clone().encode();
        ROInput::new()
            .append_bool(encoded_auth.constant)
            .append_bool(encoded_auth.signature_necessary)
            .append_bool(encoded_auth.signature_sufficient)
    }
}

impl Packable for SetVerificationKey {
    fn pack(&self) -> ROInput {
        ROInput::new()
            .append_roinput(self.auth.pack())
            .append_u32(self.txn_version)
    }
}

impl Packable for VerificationKeyData {
    fn pack(&self) -> ROInput {
        // Skip the actual data, only pack the hash
        ROInput::new().append_field(self.hash.0)
    }
}

impl Packable for BalanceChange {
    fn pack(&self) -> ROInput {
        let sgn = self.sgn == 1;
        ROInput::new().append_u64(self.magnitude).append_bool(sgn)
    }
}

impl Packable for MayUseToken {
    fn pack(&self) -> ROInput {
        // two bits
        ROInput::new()
            .append_bool(self.parents_own_token)
            .append_bool(self.inherit_from_parent)
    }
}

impl Packable for AuthorizationKind {
    fn pack(&self) -> ROInput {
        ROInput::new()
            .append_bool(self.is_signed)
            .append_bool(self.is_proved)
            .append_field(self.verification_key_hash.0)
    }
}

impl Packable for EpochLedger {
    fn pack(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi = roi.append_roinput(self.hash.pack());
        roi = roi.append_roinput(self.total_currency.pack());
        roi
    }
}

impl Packable for EpochData {
    fn pack(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi = roi.append_roinput(self.ledger.pack());
        roi = roi.append_roinput(self.seed.pack());
        roi = roi.append_roinput(self.start_checkpoint.pack());
        roi = roi.append_roinput(self.lock_checkpoint.pack());
        roi = roi.append_roinput(self.epoch_length.pack());
        roi
    }
}

impl Packable for NetworkPreconditions {
    fn pack(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi = roi.append_roinput(self.snarked_ledger_hash.pack());
        roi = roi.append_roinput(self.blockchain_length.pack());
        roi = roi.append_roinput(self.min_window_density.pack());
        roi = roi.append_roinput(self.total_currency.pack());
        roi = roi.append_roinput(self.global_slot_since_genesis.pack());
        roi = roi.append_roinput(self.staking_epoch_data.pack());
        roi = roi.append_roinput(self.next_epoch_data.pack());
        roi
    }
}

impl Packable for AccountPreconditions {
    fn pack(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi = roi.append_roinput(self.balance.pack());
        roi = roi.append_roinput(self.nonce.pack());
        roi = roi.append_roinput(self.receipt_chain_hash.pack());
        roi = roi.append_roinput(self.delegate.pack());
        roi = roi.append_roinput(self.state.pack());
        roi = roi.append_roinput(self.action_state.pack());
        roi = roi.append_roinput(self.proved_state.pack());
        roi = roi.append_roinput(self.is_new.pack());
        roi
    }
}

impl Packable for Preconditions {
    fn pack(&self) -> ROInput {
        ROInput::new()
            .append_roinput(self.network.pack())
            .append_roinput(self.account.pack())
            .append_roinput(self.valid_while.pack())
    }
}

impl Packable for TokenSymbol {
    fn pack(&self) -> ROInput {
        let mut roi = ROInput::new();
        let mut s = <[u8; 6]>::default();
        self.to_bytes(&mut s);
        roi = roi.append_bytes(&s);
        roi
    }
}

impl Packable for ZkappUri {
    fn pack(&self) -> ROInput {
        let mut field_inputs = ROInput::new();
        for c in self.0.as_slice() {
            for j in 0..8 {
                field_inputs = field_inputs.append_bool((c & (1 << j)) != 0);
            }
        }
        field_inputs = field_inputs.append_bool(true);
        let fields = field_inputs.to_fields();
        let hash = hash_with_prefix(MINA_ZKAPP_URI, &fields).unwrap();
        ROInput::new().append_field(hash)
    }
}

// ------------------------------------------------------------------------------------------------
// ----------------------------- PACKABLE IMPLEMENTATIONS FOR BASIC TYPES -------------------------
// ------------------------------------------------------------------------------------------------

impl Packable for PublicKey {
    fn pack(&self) -> ROInput {
        // PublicKey wraps CompressedPubKey which exposes x and is_odd via conversion
        let pk: mina_signer::CompressedPubKey = self.clone().into();
        ROInput::new().append_field(pk.x).append_bool(pk.is_odd)
    }
}

impl Packable for Field {
    fn pack(&self) -> ROInput {
        ROInput::new().append_field(self.0)
    }
}

impl Packable for RangeCondition<UInt32> {
    fn pack(&self) -> ROInput {
        ROInput::new().append_u32(self.lower).append_u32(self.upper)
    }
}

impl Packable for RangeCondition<UInt64> {
    fn pack(&self) -> ROInput {
        ROInput::new().append_u64(self.lower).append_u64(self.upper)
    }
}

impl<T> Packable for RangeCondition<T>
where
    T: Packable,
{
    fn pack(&self) -> ROInput {
        ROInput::new()
            .append_roinput(self.lower.pack())
            .append_roinput(self.upper.pack())
    }
}

impl<T> Packable for Option<T>
where
    T: Packable + Emptiable,
{
    fn pack(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi = roi.append_bool(self.is_some());
        if self.is_some() {
            roi = roi.append_roinput(self.as_ref().unwrap().pack());
        } else {
            roi = roi.append_roinput(T::empty_roi());
        }
        roi
    }
}

impl Packable for Bool {
    fn pack(&self) -> ROInput {
        ROInput::new().append_bool(*self)
    }
}

impl Packable for Option<Bool> {
    fn pack(&self) -> ROInput {
        let mut roi = ROInput::new();
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
    fn pack(&self) -> ROInput {
        let mut roi = ROInput::new();
        for item in self {
            roi = roi.append_roinput(item.pack());
        }
        roi
    }
}

// ------------------------------------------------------------------------------------------------
// ----------------------------- TESTS ------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::Packable;
    use mina_hasher::{Fp, ROInput};
    use mina_signer::CompressedPubKey;
    use std::str::FromStr;

    #[derive(Clone)]
    enum ROValue {
        Field(String),
        Bool(bool),
        U32(u32),
        U64(u64),
        Bytes(Vec<u8>),
    }

    fn build_roi(values: Vec<ROValue>) -> ROInput {
        let mut roi = ROInput::new();
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

    fn assert_roi_equal(roi: ROInput, expected: ROInput) {
        // Using an unsafe method to access private fields for testing purposes
        assert!(
            roi.to_bytes() == expected.to_bytes(),
            "ROInput values are not equal. Expected \n {:?}, \n but got {:?}",
            expected.to_bytes(),
            roi.to_bytes()
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
    fn test_events() {
        // Data: 2 events - [1, 2, 3] and [100]
        // Only hash is included in toInput (not the actual data)
        let events = super::Events {
            data: vec![
                vec![
                    super::Field(Fp::from(1)),
                    super::Field(Fp::from(2)),
                    super::Field(Fp::from(3)),
                ],
                vec![super::Field(Fp::from(100))],
            ],
            hash: super::Field(Fp::from(999)),
        };
        let roi = events.pack();
        let expected_roi = build_roi(vec![ROValue::Field("999".to_string())]);

        assert_roi_equal(roi, expected_roi);
    }

    #[test]
    fn test_actions() {
        // Data: 1 action - [42, 43]
        // Only hash is included in toInput (not the actual data)
        let actions = super::Actions {
            data: vec![vec![super::Field(Fp::from(42)), super::Field(Fp::from(43))]],
            hash: super::Field(Fp::from(888)),
        };
        let roi = actions.pack();
        let expected_roi = build_roi(vec![ROValue::Field("888".to_string())]);

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
