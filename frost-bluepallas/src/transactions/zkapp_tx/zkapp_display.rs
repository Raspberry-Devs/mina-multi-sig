//! This module provides Display implementations for ZkApp transaction structs
use crate::transactions::MEMO_HEADER_BYTES;

use super::*;
use core::fmt;

pub struct DisplayableOption<'a, T>(pub Option<&'a T>);

impl<'a, T: fmt::Display> fmt::Display for DisplayableOption<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(value) => write!(f, "{}", value),
            None => write!(f, "null"),
        }
    }
}

// prefer borrowing: &Option<T> -> DisplayableOption<'_, T>
impl<'a, T> From<&'a Option<T>> for DisplayableOption<'a, T> {
    fn from(opt: &'a Option<T>) -> Self {
        DisplayableOption(opt.as_ref())
    }
}

// also allow Option<&T> directly
impl<'a, T> From<Option<&'a T>> for DisplayableOption<'a, T> {
    fn from(opt: Option<&'a T>) -> Self {
        DisplayableOption(opt)
    }
}

impl fmt::Display for ZKAppCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\n  \"fee_payer\": {},\n  \"account_updates\": [\n",
            self.fee_payer
        )?;

        for (i, update) in self.account_updates.iter().enumerate() {
            if i > 0 {
                writeln!(f, ",")?;
            }
            write!(f, "    {}", update)?;
        }

        let memo_str = match self.memo.len() {
            len if len < MEMO_HEADER_BYTES => String::new(),
            _ => self.memo[MEMO_HEADER_BYTES..]
                .iter()
                .take_while(|&&b| b != 0)
                .map(|&b| b as char)
                .collect::<String>(),
        };

        write!(f, "\n  ],\n  \"memo\": \"{}\"\n}}", memo_str)
    }
}

impl fmt::Display for FeePayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\n    \"body\": {},\n    \"authorization\": \"{}\"\n  }}",
            self.body, self.authorization
        )
    }
}

impl fmt::Display for FeePayerBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\n      \"public_key\": \"{}\",\n      \"fee\": \"{}\",\n      \"valid_until\": {},\n      \"nonce\": \"{}\"\n    }}",
            self.public_key,
            self.fee,
            match self.valid_until {
                Some(val) => format!("\"{}\"", val),
                None => "null".to_string(),
            },
            self.nonce
        )
    }
}

impl fmt::Display for AccountUpdate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\n      \"body\": {},\n      \"authorization\": {}\n    }}",
            self.body, self.authorization
        )
    }
}

impl fmt::Display for AccountUpdateBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\n        \"public_key\": \"{}\",\n        \"token_id\": \"{}\",\n        \"update\": {},\n        \"balance_change\": {},\n        \"increment_nonce\": {},\n        \"events\": {},\n        \"actions\": {},\n        \"call_data\": \"{}\",\n        \"call_depth\": {},\n        \"preconditions\": {},\n        \"use_full_commitment\": {},\n        \"implicit_account_creation_fee\": {},\n        \"may_use_token\": {},\n        \"authorization_kind\": {}\n      }}",
            self.public_key,
            self.token_id.0,
            self.update,
            self.balance_change,
            self.increment_nonce,
            self.events,
            self.actions,
            self.call_data,
            self.call_depth,
            self.preconditions,
            self.use_full_commitment,
            self.implicit_account_creation_fee,
            self.may_use_token,
            self.authorization_kind
        )
    }
}

impl fmt::Display for Update {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\n          \"app_state\": [")?;
        for (i, state) in self.app_state.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }

            write!(f, "{}", DisplayableOption::from(state))?;
        }
        write!(
            f,
            "],\n          \"delegate\": {},\n          \"verification_key\": {},\n          \"permissions\": {},\n          \"zkapp_uri\": {},\n          \"token_symbol\": {},\n          \"timing\": {},\n          \"voting_for\": {}\n        }}",
            DisplayableOption::from(&self.delegate),
            DisplayableOption::from(&self.verification_key),
            DisplayableOption::from(&self.permissions),
            DisplayableOption::from(&self.zkapp_uri),
            DisplayableOption::from(&self.token_symbol),
            DisplayableOption::from(&self.timing),
            DisplayableOption::from(&self.voting_for)
        )
    }
}

impl fmt::Display for Permissions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\n            \"edit_state\": {},\n            \"access\": {},\n            \"send\": {},\n            \"receive\": {},\n            \"set_delegate\": {},\n            \"set_permissions\": {},\n            \"set_verification_key\": {},\n            \"set_zkapp_uri\": {},\n            \"edit_action_state\": {},\n            \"set_token_symbol\": {},\n            \"increment_nonce\": {},\n            \"set_voting_for\": {},\n            \"set_timing\": {}\n          }}",
            self.edit_state,
            self.access,
            self.send,
            self.receive,
            self.set_delegate,
            self.set_permissions,
            self.set_verification_key,
            self.set_zkapp_uri,
            self.edit_action_state,
            self.set_token_symbol,
            self.increment_nonce,
            self.set_voting_for,
            self.set_timing
        )
    }
}

impl fmt::Display for SetVerificationKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\n              \"auth\": {},\n              \"txn_version\": \"{}\"\n            }}",
            self.auth, self.txn_version
        )
    }
}

impl fmt::Display for Preconditions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\n            \"network\": {},\n            \"account\": {},\n            \"valid_while\": {}\n          }}",
            self.network, self.account, DisplayableOption::from(&self.valid_while)
        )
    }
}

impl fmt::Display for AccountPreconditions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\n              \"balance\": {},\n              \"nonce\": {},\n              \"receipt_chain_hash\": {},\n              \"delegate\": {},\n              \"state\": [",
               DisplayableOption::from(&self.balance), DisplayableOption::from(&self.nonce), DisplayableOption::from(&self.receipt_chain_hash), DisplayableOption::from(&self.delegate))?;

        for (i, state) in self.state.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", DisplayableOption::from(state))?;
        }

        write!(
            f,
            "],\n              \"action_state\": {},\n              \"proved_state\": {},\n              \"is_new\": {}\n            }}",
            DisplayableOption::from(&self.action_state),
            DisplayableOption::from(&self.proved_state),
            DisplayableOption::from(&self.is_new)
        )
    }
}

impl fmt::Display for ActionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for NetworkPreconditions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\n              \"snarked_ledger_hash\": {},\n              \"blockchain_length\": {},\n              \"min_window_density\": {},\n              \"total_currency\": {},\n              \"global_slot_since_genesis\": {},\n              \"staking_epoch_data\": {},\n              \"next_epoch_data\": {}\n            }}",
            DisplayableOption::from(&self.snarked_ledger_hash),
            DisplayableOption::from(&self.blockchain_length),
            DisplayableOption::from(&self.min_window_density),
            DisplayableOption::from(&self.total_currency),
            DisplayableOption::from(&self.global_slot_since_genesis),
            self.staking_epoch_data,
            self.next_epoch_data
        )
    }
}

impl fmt::Display for Events {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\n            \"data\": [")?;
        for (i, event_row) in self.data.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "[")?;
            for (j, field) in event_row.iter().enumerate() {
                if j > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "\"{}\"", field)?;
            }
            write!(f, "]")?;
        }
        write!(f, "]\n}}",)
    }
}

impl fmt::Display for Actions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\n            \"data\": [")?;
        for (i, action_row) in self.data.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "[")?;
            for (j, field) in action_row.iter().enumerate() {
                if j > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "\"{}\"", field)?;
            }
            write!(f, "]")?;
        }
        write!(f, "]\n          }}",)
    }
}

impl fmt::Display for Authorization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\n        \"proof\": {},\n        \"signature\": {}\n      }}",
            match &self.proof {
                Some(p) => format!("\"{}\"", p),
                None => "null".to_string(),
            },
            match &self.signature {
                Some(s) => format!("\"{}\"", s),
                None => "null".to_string(),
            }
        )
    }
}

impl<T: fmt::Display> fmt::Display for RangeCondition<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\n                \"lower\": \"{}\",\n                \"upper\": \"{}\"\n              }}",
            self.lower, self.upper
        )
    }
}

impl fmt::Display for VerificationKeyData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\n            \"data\": \"{}\",\n            \"hash\": \"{}\"\n          }}",
            self.data, self.hash
        )
    }
}

impl fmt::Display for TimingData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\n            \"initial_minimum_balance\": \"{}\",\n            \"cliff_time\": \"{}\",\n            \"cliff_amount\": \"{}\",\n            \"vesting_period\": \"{}\",\n            \"vesting_increment\": \"{}\"\n          }}",
            self.initial_minimum_balance,
            self.cliff_time,
            self.cliff_amount,
            self.vesting_period,
            self.vesting_increment
        )
    }
}

impl fmt::Display for EpochData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\n              \"ledger\": {},\n              \"seed\": {},\n              \"start_checkpoint\": {},\n              \"lock_checkpoint\": {},\n              \"epoch_length\": {}\n            }}",
            self.ledger,
            DisplayableOption::from(&self.seed),
            DisplayableOption::from(&self.start_checkpoint),
            DisplayableOption::from(&self.lock_checkpoint),
            DisplayableOption::from(&self.epoch_length)
        )
    }
}

impl fmt::Display for EpochLedger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\n                \"hash\": {},\n                \"total_currency\": {}\n              }}",
            DisplayableOption::from(&self.hash),
            DisplayableOption::from(&self.total_currency)
        )
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.into_address())
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for AuthRequired {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let AuthRequiredEncoded {
            constant,
            signature_necessary,
            signature_sufficient,
        } = self.clone().encode();
        let type_name = match self {
            AuthRequired::None => "None",
            AuthRequired::Proof => "Proof",
            AuthRequired::Signature => "Signature",
            AuthRequired::Either => "Either",
            AuthRequired::Impossible => "Impossible",
            AuthRequired::Both => "Both",
        };

        write!(
            f,
            "{{\n              \"type\": {},\n              \"constant\": {},\n              \"signature_necessary\": {},\n              \"signature_sufficient\": {}\n            }}",
            type_name, constant, signature_necessary, signature_sufficient
        )
    }
}

impl fmt::Display for AuthRequiredEncoded<bool> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\n              \"constant\": {},\n              \"signature_necessary\": {},\n              \"signature_sufficient\": {}\n            }}",
            self.constant, self.signature_necessary, self.signature_sufficient
        )
    }
}

impl fmt::Display for MayUseToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\n            \"parents_own_token\": {},\n            \"inherit_from_parent\": {}\n          }}",
            self.parents_own_token, self.inherit_from_parent
        )
    }
}

impl fmt::Display for BalanceChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\n          \"magnitude\": \"{}\",\n          \"sgn\": {}\n        }}",
            self.magnitude, self.sgn
        )
    }
}

impl fmt::Display for AuthorizationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\n          \"is_signed\": {},\n          \"is_proved\": {},\n          \"verification_key_hash\": \"{}\"\n        }}",
            self.is_signed, self.is_proved, self.verification_key_hash
        )
    }
}

impl fmt::Display for ZkappUri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\n            \"zkapp_uri_data\": \"{}\"\n          }}",
            String::from_utf8_lossy(&self.0)
        )
    }
}

impl fmt::Display for TokenSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\n            \"token_symbol_data\": \"{}\"\n          }}",
            String::from_utf8_lossy(&self.0)
        )
    }
}
