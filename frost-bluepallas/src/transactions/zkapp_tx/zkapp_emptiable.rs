use mina_hasher::ROInput;
use mina_signer::CompressedPubKey;
use super::zkapp_packable::Packable;
use crate::transactions::zkapp_tx::{Field, Permissions, PublicKey, RangeCondition, SetVerificationKey, TimingData, TokenSymbol, UInt32, UInt64, VerificationKeyData, ZkappUri};

pub trait Emptiable {
    fn empty_roi() -> ROInput;
}

impl Emptiable for Field {
    fn empty_roi() -> ROInput{
        Self(mina_hasher::Fp::from(0)).pack()
    }
}

impl Emptiable for VerificationKeyData {
    fn empty_roi() -> ROInput {
        Field::empty_roi()
    }
}

impl Emptiable for PublicKey {
    fn empty_roi() -> ROInput  {
        PublicKey(CompressedPubKey::empty()).pack()
    }
}

impl Emptiable for Permissions {
    fn empty_roi() -> ROInput {
        use crate::transactions::zkapp_tx::AuthRequired::*;
        use crate::transactions::zkapp_tx::constants::TXN_VERSION_CURRENT;
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
        }.pack()
    }
}

impl Emptiable for ZkappUri {
    fn empty_roi() -> ROInput {
        // TODO: This si a placeholder. Implement this
        Field::empty_roi()
    }
}

impl Emptiable for TokenSymbol {    
    fn empty_roi() -> ROInput {
        let mut roi = ROInput::new();
        roi = roi.append_roinput(TokenSymbol::default().pack());
        roi
    }
}

impl Emptiable for TimingData {
    fn empty_roi() -> ROInput {
        Self {
            initial_minimum_balance: 0,
            cliff_time: 0,
            cliff_amount: 0,
            vesting_period: 0,
            vesting_increment: 0,
        }.pack()
    }
}

impl Emptiable for RangeCondition<UInt32> {
    fn empty_roi() -> ROInput {
        Self {
            lower: UInt32::min_value(),
            upper: UInt32::max_value(),
        }.pack()
    }
}

impl Emptiable for RangeCondition<UInt64> {
    fn empty_roi() -> ROInput {
        Self {
            lower: UInt64::min_value(),
            upper: UInt64::max_value(),
        }.pack()
    }
}
