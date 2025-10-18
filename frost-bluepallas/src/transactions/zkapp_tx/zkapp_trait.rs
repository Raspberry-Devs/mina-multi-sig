//! ZKApp Hashable trait implementations
use crate::transactions::zkapp_tx::AccountUpdate;
use crate::transactions::zkapp_tx::*;
use mina_hasher::{Hashable, ROInput};

impl Hashable for Field {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new().append_field(self.0)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for PublicKey {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        // PublicKey wraps CompressedPubKey which exposes x and is_odd via conversion
        let pk: mina_signer::CompressedPubKey = self.clone().into();
        ROInput::new().append_field(pk.x).append_bool(pk.is_odd)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for ZkappAccount {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();

        // zkapp_uri as optional string -> include as bytes if non-empty
        if !self.zkapp_uri.is_empty() {
            roi = roi.append_bytes(self.zkapp_uri.as_bytes());
        }

        roi = roi.append_bool(self.proved_state);
        roi = roi.append_u32(self.last_action_slot);

        for fp in &self.action_state {
            roi = roi.append_field(fp.0);
        }

        roi = roi.append_u32(self.zkapp_version);

        let vk_hash = self
            .verification_key
            .as_ref()
            .map(|vk| vk.hash.0)
            .unwrap_or_else(|| Field(mina_hasher::Fp::from(0)).0);

        roi = roi.append_field(vk_hash);

        for fp in &self.app_state {
            roi = roi.append_field(fp.0);
        }

        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for Account {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();

        // zkapp hash field
        let field_zkapp = match &self.zkapp {
            Some(zkapp) => {
                // derive a simple deterministic representation: use first app_state element if present,
                // otherwise use zero. This mirrors including zkapp fields into the ROInput.
                zkapp
                    .app_state
                    .first()
                    .map(|f| f.0)
                    .unwrap_or_else(|| mina_hasher::Fp::from(0))
            }
            None => mina_hasher::Fp::from(0),
        };

        roi = roi.append_field(field_zkapp);

        // permissions: serialize a selection of boolean flags and txn_version
        roi = roi
            .append_bool(self.permissions.edit_state.constant)
            .append_bool(self.permissions.edit_state.signature_necessary)
            .append_bool(self.permissions.edit_state.signature_sufficient)
            .append_bool(self.permissions.access.constant)
            .append_bool(self.permissions.send.constant)
            .append_u32(self.permissions.set_verification_key.txn_version);

        // timing
        roi = roi.append_bool(self.timing.is_timed);
        roi = roi.append_u64(self.timing.initial_minimum_balance);
        roi = roi.append_u32(self.timing.cliff_time);
        roi = roi.append_u64(self.timing.cliff_amount);
        roi = roi.append_u32(self.timing.vesting_period);
        roi = roi.append_u64(self.timing.vesting_increment);

        // voting_for
        roi = roi.append_field(self.voting_for.0);

        // delegate (optional public key)
        let delegate_x = match &self.delegate {
            Some(pk) => {
                let cp: mina_signer::CompressedPubKey = pk.clone().into();
                cp.x
            }
            None => mina_hasher::Fp::from(0),
        };
        roi = roi.append_field(delegate_x);

        // receipt_chain_hash
        roi = roi.append_field(self.receipt_chain_hash.0);

        // nonce
        roi = roi.append_u32(self.nonce);

        // balance
        roi = roi.append_u64(self.balance);

        // token_symbol (max len 6)
        assert!(self.token_symbol.len() <= 6);
        roi = roi.append_bytes(self.token_symbol.as_bytes());

        // token_id
        roi = roi.append_field(self.token_id.0);

        // public_key
        let pk_cp: mina_signer::CompressedPubKey = self.public_key.clone().into();
        roi = roi.append_field(pk_cp.x).append_bool(pk_cp.is_odd);

        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for Events {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new().append_field(self.hash.0)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for Actions {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new().append_field(self.hash.0)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for TimingData {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new()
            .append_u64(self.initial_minimum_balance)
            .append_u32(self.cliff_time)
            .append_u64(self.cliff_amount)
            .append_u32(self.vesting_period)
            .append_u64(self.vesting_increment)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for AuthRequired {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new()
            .append_bool(self.constant)
            .append_bool(self.signature_necessary)
            .append_bool(self.signature_sufficient)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for SetVerificationKey {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new()
            .append_bytes(&self.auth.to_roinput().to_bytes())
            .append_u32(self.txn_version)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for VerificationKeyData {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new().append_field(self.hash.0)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for BalanceChange {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let sgn = self.sgn == 1;
        ROInput::new().append_u64(self.magnitude).append_bool(sgn)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for Authorization {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        if let Some(p) = &self.proof {
            roi = roi.append_bool(true).append_bytes(p.as_bytes());
        } else {
            roi = roi.append_bool(false);
        }
        if let Some(s) = &self.signature {
            roi = roi.append_bool(true).append_bytes(s.as_bytes());
        } else {
            roi = roi.append_bool(false);
        }

        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for MayUseToken {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        // two bits
        ROInput::new()
            .append_bool(self.parents_own_token)
            .append_bool(self.inherit_from_parent)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for AuthorizationKind {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new()
            .append_bool(self.is_signed)
            .append_bool(self.is_proved)
            .append_field(self.verification_key_hash.0)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for AccountTiming {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new()
            .append_bool(self.is_timed)
            .append_u64(self.initial_minimum_balance)
            .append_u32(self.cliff_time)
            .append_u64(self.cliff_amount)
            .append_u32(self.vesting_period)
            .append_u64(self.vesting_increment)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for EpochLedger {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        if self.hash.is_some {
            roi = roi.append_field(self.hash.value.0);
        } else {
            roi = roi.append_field(mina_hasher::Fp::from(0));
        }
        // total_currency is OptionalValue<RangeCondition<UInt64>>
        if self.total_currency.is_some {
            roi = roi
                .append_u64(self.total_currency.value.lower)
                .append_u64(self.total_currency.value.upper);
        } else {
            roi = roi.append_u64(0).append_u64(0);
        }
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for EpochData {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();

        // ledger
        roi = roi.append_bytes(&self.ledger.to_roinput().to_bytes());

        // seed / checkpoints optional values: encode field or zero
        if self.seed.is_some {
            roi = roi.append_field(self.seed.value.0);
        } else {
            roi = roi.append_field(mina_hasher::Fp::from(0));
        }
        if self.start_checkpoint.is_some {
            roi = roi.append_field(self.start_checkpoint.value.0);
        } else {
            roi = roi.append_field(mina_hasher::Fp::from(0));
        }
        if self.lock_checkpoint.is_some {
            roi = roi.append_field(self.lock_checkpoint.value.0);
        } else {
            roi = roi.append_field(mina_hasher::Fp::from(0));
        }

        // epoch_length: OptionalValue<RangeCondition<UInt32>>
        if self.epoch_length.is_some {
            roi = roi
                .append_u32(self.epoch_length.value.lower)
                .append_u32(self.epoch_length.value.upper);
        } else {
            roi = roi.append_u32(0).append_u32(0);
        }

        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for NetworkPreconditions {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();

        if self.snarked_ledger_hash.is_some {
            roi = roi.append_field(self.snarked_ledger_hash.value.0);
        } else {
            roi = roi.append_field(mina_hasher::Fp::from(0));
        }

        // For each optional RangeCondition<UInt32>, encode lower then upper or zeros
        if self.blockchain_length.is_some {
            roi = roi
                .append_u32(self.blockchain_length.value.lower)
                .append_u32(self.blockchain_length.value.upper);
        } else {
            roi = roi.append_u32(0).append_u32(0);
        }

        if self.min_window_density.is_some {
            roi = roi
                .append_u32(self.min_window_density.value.lower)
                .append_u32(self.min_window_density.value.upper);
        } else {
            roi = roi.append_u32(0).append_u32(0);
        }

        if self.total_currency.is_some {
            roi = roi
                .append_u64(self.total_currency.value.lower)
                .append_u64(self.total_currency.value.upper);
        } else {
            roi = roi.append_u64(0).append_u64(0);
        }

        if self.global_slot_since_genesis.is_some {
            roi = roi
                .append_u32(self.global_slot_since_genesis.value.lower)
                .append_u32(self.global_slot_since_genesis.value.upper);
        } else {
            roi = roi.append_u32(0).append_u32(0);
        }

        roi = roi
            .append_bytes(&self.staking_epoch_data.to_roinput().to_bytes())
            .append_bytes(&self.next_epoch_data.to_roinput().to_bytes());

        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for AccountPreconditions {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();

        // balance range
        if self.balance.is_some {
            roi = roi
                .append_u64(self.balance.value.lower)
                .append_u64(self.balance.value.upper);
        } else {
            roi = roi.append_u64(0).append_u64(0);
        }

        // nonce range
        if self.nonce.is_some {
            roi = roi
                .append_u32(self.nonce.value.lower)
                .append_u32(self.nonce.value.upper);
        } else {
            roi = roi.append_u32(0).append_u32(0);
        }

        // receipt chain hash optional
        if self.receipt_chain_hash.is_some {
            roi = roi.append_field(self.receipt_chain_hash.value.0);
        } else {
            roi = roi.append_field(mina_hasher::Fp::from(0));
        }

        // delegate optional
        if self.delegate.is_some {
            let cp: mina_signer::CompressedPubKey = self.delegate.value.clone().into();
            roi = roi.append_field(cp.x).append_bool(cp.is_odd);
        } else {
            roi = roi
                .append_field(mina_hasher::Fp::from(0))
                .append_bool(false);
        }

        // state vector
        for s in &self.state {
            if s.is_some {
                roi = roi.append_field(s.value.0);
            } else {
                roi = roi.append_field(mina_hasher::Fp::from(0));
            }
        }

        // action_state optional
        if self.action_state.is_some {
            roi = roi.append_field(self.action_state.value.0);
        } else {
            roi = roi.append_field(mina_hasher::Fp::from(0));
        }

        // proved_state and is_new
        if self.proved_state.is_some {
            roi = roi.append_bool(self.proved_state.value);
        } else {
            roi = roi.append_bool(false);
        }
        if self.is_new.is_some {
            roi = roi.append_bool(self.is_new.value);
        } else {
            roi = roi.append_bool(false);
        }

        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for Permissions {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new()
            .append_bytes(&self.edit_state.to_roinput().to_bytes())
            .append_bytes(&self.access.to_roinput().to_bytes())
            .append_bytes(&self.send.to_roinput().to_bytes())
            .append_bytes(&self.receive.to_roinput().to_bytes())
            .append_bytes(&self.set_delegate.to_roinput().to_bytes())
            .append_bytes(&self.set_permissions.to_roinput().to_bytes())
            .append_bytes(&self.set_verification_key.to_roinput().to_bytes())
            .append_bytes(&self.set_zkapp_uri.to_roinput().to_bytes())
            .append_bytes(&self.edit_action_state.to_roinput().to_bytes())
            .append_bytes(&self.set_token_symbol.to_roinput().to_bytes())
            .append_bytes(&self.increment_nonce.to_roinput().to_bytes())
            .append_bytes(&self.set_voting_for.to_roinput().to_bytes())
            .append_bytes(&self.set_timing.to_roinput().to_bytes())
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for Preconditions {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new()
            .append_bytes(&self.network.to_roinput().to_bytes())
            .append_bytes(&self.account.to_roinput().to_bytes())
            // valid_while: OptionalValue<RangeCondition<UInt32>> -> encode lower/upper or zeros
            .append_u32(if self.valid_while.is_some {
                self.valid_while.value.lower
            } else {
                0
            })
            .append_u32(if self.valid_while.is_some {
                self.valid_while.value.upper
            } else {
                0
            })
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for Update {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();

        for state in &self.app_state {
            if state.is_some {
                roi = roi.append_field(state.value.0);
            } else {
                roi = roi.append_field(mina_hasher::Fp::from(0));
            }
        }

        // delegate optional
        if self.delegate.is_some {
            let pk: PublicKey = self.delegate.value.clone();
            let cp: mina_signer::CompressedPubKey = pk.into();
            roi = roi.append_field(cp.x).append_bool(cp.is_odd);
        } else {
            roi = roi
                .append_field(mina_hasher::Fp::from(0))
                .append_bool(false);
        }

        // verification_key
        if self.verification_key.is_some {
            roi = roi.append_field(self.verification_key.value.hash.0);
        } else {
            roi = roi.append_field(mina_hasher::Fp::from(0));
        }

        // permissions
        if self.permissions.is_some {
            roi = roi.append_bytes(&self.permissions.value.to_roinput().to_bytes());
        } else {
            // default permissions -> zeros
            roi = roi.append_bool(false).append_bool(false).append_bool(false);
        }

        // zkapp_uri
        if self.zkapp_uri.is_some {
            roi = roi.append_bytes(self.zkapp_uri.value.data.as_bytes());
        } else {
            roi = roi.append_bytes(&[]);
        }

        // token_symbol
        if self.token_symbol.is_some {
            roi = roi.append_bytes(self.token_symbol.value.symbol.as_bytes());
        } else {
            roi = roi.append_bytes(&[]);
        }

        // timing
        if self.timing.is_some {
            roi = roi.append_bytes(&self.timing.value.to_roinput().to_bytes());
        } else {
            roi = roi
                .append_u64(0)
                .append_u32(0)
                .append_u64(0)
                .append_u32(0)
                .append_u64(0);
        }

        // voting_for
        if self.voting_for.is_some {
            roi = roi.append_field(self.voting_for.value.0);
        } else {
            roi = roi.append_field(mina_hasher::Fp::from(0));
        }

        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for AccountUpdateBody {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi = roi.append_bytes(&self.public_key.to_roinput().to_bytes());
        roi = roi.append_field(self.token_id.0);
        roi = roi.append_bytes(&self.update.to_roinput().to_bytes());
        roi = roi.append_bytes(&self.balance_change.to_roinput().to_bytes());
        roi = roi.append_bool(self.increment_nonce);
        roi = roi.append_bytes(&self.events.to_roinput().to_bytes());
        roi = roi.append_bytes(&self.actions.to_roinput().to_bytes());
        roi = roi.append_field(self.call_data.0);
        roi = roi.append_u32(self.call_depth);
        roi = roi.append_bytes(&self.preconditions.to_roinput().to_bytes());
        roi = roi.append_bool(self.use_full_commitment);
        roi = roi.append_bool(self.implicit_account_creation_fee);
        roi = roi.append_bytes(&self.may_use_token.to_roinput().to_bytes());
        roi = roi.append_bytes(&self.authorization_kind.to_roinput().to_bytes());
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for AccountUpdate {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        // AccountUpdate only uses the body for inputs
        self.body.to_roinput()
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for FeePayerBody {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi = roi.append_bytes(&self.public_key.to_roinput().to_bytes());
        roi = roi.append_u64(self.fee);
        match self.valid_until {
            Some(v) => roi = roi.append_u32(v),
            None => roi = roi.append_u32(0),
        }
        roi = roi.append_u32(self.nonce);
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for FeePayer {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi = roi.append_bytes(&self.body.to_roinput().to_bytes());
        roi.append_bytes(self.authorization.as_bytes())
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for ZKAppCommand {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi = roi.append_bytes(&self.fee_payer.to_roinput().to_bytes());
        for au in &self.account_updates {
            roi = roi.append_bytes(&au.to_roinput().to_bytes());
        }
        roi = roi.append_bytes(self.memo.as_bytes());
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for TokenSymbolData {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        assert!(self.symbol.len() <= 6);
        ROInput::new()
            .append_bytes(self.symbol.as_bytes())
            .append_field(self.field.0)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hashable for ZkappUriData {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new()
            .append_bytes(self.data.as_bytes())
            .append_field(self.hash.0)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}
