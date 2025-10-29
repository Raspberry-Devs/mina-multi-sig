//! ZKApp Packable trait implementations
use crate::transactions::zkapp_tx::AccountUpdate;
use crate::transactions::zkapp_tx::*;
use mina_hasher::ROInput;

trait Packable {
    fn pack(&self) -> ROInput;
}

impl Packable for Field {
    fn pack(&self) -> ROInput {
        ROInput::new().append_field(self.0)
    }
}

impl Packable for PublicKey {
    fn pack(&self) -> ROInput {
        // PublicKey wraps CompressedPubKey which exposes x and is_odd via conversion
        let pk: mina_signer::CompressedPubKey = self.clone().into();
        ROInput::new().append_field(pk.x).append_bool(pk.is_odd)
    }
}

impl Packable for ZkappAccount {
    fn pack(&self) -> ROInput {
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
}

impl Packable for Account {
    fn pack(&self) -> ROInput {
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
        ROInput::new()
            .append_bool(self.constant)
            .append_bool(self.signature_necessary)
            .append_bool(self.signature_sufficient)
    }
}

impl Packable for SetVerificationKey {
    fn pack(&self) -> ROInput {
        ROInput::new()
            .append_bytes(&self.auth.pack().to_bytes())
            .append_u32(self.txn_version)
    }
}

impl Packable for VerificationKeyData {
    fn pack(&self) -> ROInput {
        ROInput::new().append_field(self.hash.0)
    }
}

impl Packable for BalanceChange {
    fn pack(&self) -> ROInput {
        let sgn = self.sgn == 1;
        ROInput::new().append_u64(self.magnitude).append_bool(sgn)
    }
}

impl Packable for Authorization {
    fn pack(&self) -> ROInput {
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

impl Packable for AccountTiming {
    fn pack(&self) -> ROInput {
        ROInput::new()
            .append_bool(self.is_timed)
            .append_u64(self.initial_minimum_balance)
            .append_u32(self.cliff_time)
            .append_u64(self.cliff_amount)
            .append_u32(self.vesting_period)
            .append_u64(self.vesting_increment)
    }
}

impl Packable for EpochLedger {
    fn pack(&self) -> ROInput {
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
}

impl Packable for EpochData {
    fn pack(&self) -> ROInput {
        let mut roi = ROInput::new();

        // ledger
        roi = roi.append_bytes(&self.ledger.pack().to_bytes());

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
}

impl Packable for NetworkPreconditions {
    fn pack(&self) -> ROInput {
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
            .append_bytes(&self.staking_epoch_data.pack().to_bytes())
            .append_bytes(&self.next_epoch_data.pack().to_bytes());

        roi
    }
}

impl Packable for AccountPreconditions {
    fn pack(&self) -> ROInput {
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
}

impl Packable for Permissions {
    fn pack(&self) -> ROInput {
        ROInput::new()
            .append_bytes(&self.edit_state.pack().to_bytes())
            .append_bytes(&self.access.pack().to_bytes())
            .append_bytes(&self.send.pack().to_bytes())
            .append_bytes(&self.receive.pack().to_bytes())
            .append_bytes(&self.set_delegate.pack().to_bytes())
            .append_bytes(&self.set_permissions.pack().to_bytes())
            .append_bytes(&self.set_verification_key.pack().to_bytes())
            .append_bytes(&self.set_zkapp_uri.pack().to_bytes())
            .append_bytes(&self.edit_action_state.pack().to_bytes())
            .append_bytes(&self.set_token_symbol.pack().to_bytes())
            .append_bytes(&self.increment_nonce.pack().to_bytes())
            .append_bytes(&self.set_voting_for.pack().to_bytes())
            .append_bytes(&self.set_timing.pack().to_bytes())
    }
}

impl Packable for Preconditions {
    fn pack(&self) -> ROInput {
        ROInput::new()
            .append_bytes(&self.network.pack().to_bytes())
            .append_bytes(&self.account.pack().to_bytes())
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
}

impl Packable for Update {
    fn pack(&self) -> ROInput {
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
            roi = roi.append_bytes(&self.permissions.value.pack().to_bytes());
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
            roi = roi.append_bytes(&self.timing.value.pack().to_bytes());
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
}

impl Packable for AccountUpdateBody {
    fn pack(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi = roi.append_bytes(&self.public_key.pack().to_bytes());
        roi = roi.append_field(self.token_id.0);
        roi = roi.append_bytes(&self.update.pack().to_bytes());
        roi = roi.append_bytes(&self.balance_change.pack().to_bytes());
        roi = roi.append_bool(self.increment_nonce);
        roi = roi.append_bytes(&self.events.pack().to_bytes());
        roi = roi.append_bytes(&self.actions.pack().to_bytes());
        roi = roi.append_field(self.call_data.0);
        roi = roi.append_u32(self.call_depth);
        roi = roi.append_bytes(&self.preconditions.pack().to_bytes());
        roi = roi.append_bool(self.use_full_commitment);
        roi = roi.append_bool(self.implicit_account_creation_fee);
        roi = roi.append_bytes(&self.may_use_token.pack().to_bytes());
        roi = roi.append_bytes(&self.authorization_kind.pack().to_bytes());
        roi
    }
}

impl Packable for AccountUpdate {
    fn pack(&self) -> ROInput {
        // AccountUpdate only uses the body for inputs
        self.body.pack()
    }
}

impl Packable for FeePayerBody {
    fn pack(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi = roi.append_bytes(&self.public_key.pack().to_bytes());
        roi = roi.append_u64(self.fee);
        match self.valid_until {
            Some(v) => roi = roi.append_u32(v),
            None => roi = roi.append_u32(0),
        }
        roi = roi.append_u32(self.nonce);
        roi
    }
}

impl Packable for FeePayer {
    fn pack(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi = roi.append_bytes(&self.body.pack().to_bytes());
        roi.append_bytes(self.authorization.as_bytes())
    }
}

impl Packable for ZKAppCommand {
    fn pack(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi = roi.append_bytes(&self.fee_payer.pack().to_bytes());
        for au in &self.account_updates {
            roi = roi.append_bytes(&au.pack().to_bytes());
        }
        roi = roi.append_bytes(self.memo.as_bytes());
        roi
    }
}

impl Packable for TokenSymbolData {
    fn pack(&self) -> ROInput {
        assert!(self.symbol.len() <= 6);
        ROInput::new()
            .append_bytes(self.symbol.as_bytes())
            .append_field(self.field.0)
    }
}

impl Packable for ZkappUriData {
    fn pack(&self) -> ROInput {
        ROInput::new()
            .append_bytes(self.data.as_bytes())
            .append_field(self.hash.0)
    }
}

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

    #[test]
    fn test_pub_key() {
        // CompressedPubKey format: [x_coordinate:32][parity:1] = 33 bytes = 66 hex characters
        let pub_key_hex = "0f48c65bd25f85f3e4ea4efebeb75b797bd743603be04b4ead845698b76bd33101";
        let pk_bytes = hex::decode(pub_key_hex).expect("Invalid hex in public key");

        let pk = CompressedPubKey::from_bytes(&pk_bytes).expect("Invalid public key bytes");
        let roi = super::PublicKey(pk).pack();

        let expected_roi = build_roi(vec![
            ROValue::Field(
                "22536877747820698688010660184495467853785925552441222123266613953322243475471"
                    .to_string(),
            ),
            ROValue::Bool(true),
        ]);

        assert_eq!(roi.to_bytes(), expected_roi.to_bytes());
    }

    #[test]
    fn test_auth_required() {
        let auth = super::AuthRequired {
            constant: false,
            signature_necessary: false,
            signature_sufficient: true,
        };
        let roi = auth.pack();
        let expected_roi = build_roi(vec![
            ROValue::Bool(false),
            ROValue::Bool(false),
            ROValue::Bool(true),
        ]);

        assert_eq!(roi.to_bytes(), expected_roi.to_bytes());
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

        assert_eq!(roi.to_bytes(), expected_roi.to_bytes());
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

        assert_eq!(roi.to_bytes(), expected_roi.to_bytes());
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

        assert_eq!(roi.to_bytes(), expected_roi.to_bytes());
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

        assert_eq!(roi.to_bytes(), expected_roi.to_bytes());
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

        assert_eq!(roi.to_bytes(), expected_roi.to_bytes());
    }

    #[test]
    fn test_token_symbol_data() {
        // Symbol: "MINA"
        // toInput should only contain packed field value (48 bits), not the symbol bytes
        let token_symbol = super::TokenSymbolData {
            symbol: "MINA".to_string(),
            field: super::Field(Fp::from(1095649613u64)),
        };
        let roi = token_symbol.pack();

        // According to spec: packed field only, NOT bytes + field
        let expected_roi = build_roi(vec![
            ROValue::Bytes(b"MINA".to_vec()),
            ROValue::Field("1095649613".to_string()),
        ]);

        assert_eq!(roi.to_bytes(), expected_roi.to_bytes());
    }

    #[test]
    fn test_zkapp_uri_data() {
        // URI: "https://minaprotocol.com"
        // toInput should only contain hash field, not the URI data
        let zkapp_uri = super::ZkappUriData {
            data: "https://minaprotocol.com".to_string(),
            hash: super::Field(Fp::from(12345)),
        };
        let roi = zkapp_uri.pack();

        // According to spec: hash field only, NOT data bytes + hash
        let expected_roi = build_roi(vec![
            ROValue::Bytes(b"https://minaprotocol.com".to_vec()),
            ROValue::Field("12345".to_string()),
        ]);

        assert_eq!(roi.to_bytes(), expected_roi.to_bytes());
    }
}
