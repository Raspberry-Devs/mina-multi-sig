/* The Emptiable trait is used when implementing Packable for an Option<T> */

use super::zkapp_packable::{Packable, PackedInput};
use crate::transactions::zkapp_tx::{
    commit::hash_noinput, constants::ZK_ACTION_STATE_EMPTY, ActionState, Field, Permissions,
    PublicKey, RangeCondition, SetVerificationKey, TimingData, TokenSymbol, UInt32, UInt64,
    VerificationKeyData, ZkappUri,
};
use mina_signer::CompressedPubKey;

pub trait Emptiable {
    fn empty_roi() -> PackedInput;
}

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
        use crate::transactions::zkapp_tx::constants::default_zkapp_uri_hash;
        PackedInput::new().append_field(default_zkapp_uri_hash())
    }
}
