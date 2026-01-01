//! This module provides Display implementations for ZkApp transaction structs.
//!
//! Composite structs use `serde_json::to_string_pretty` for consistent JSON output.
//! Simple wrapper types (PublicKey, Field, ActionState) display their inner value directly.

use super::*;
use core::fmt;
use serde::Serialize;

/// Helper to create a JSON-based Display implementation.
/// Falls back to Debug formatting if serialization fails.
pub fn json_display<T: Serialize + fmt::Debug>(
    value: &T,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    match serde_json::to_string_pretty(value) {
        Ok(json) => write!(f, "{}", json),
        Err(_) => write!(f, "{:?}", value),
    }
}

// -------------------------------------------------------------------------------------------------
// --------------------------------- Composite Structs (JSON) --------------------------------------
// -------------------------------------------------------------------------------------------------

impl fmt::Display for ZKAppCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        json_display(self, f)
    }
}

impl fmt::Display for FeePayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        json_display(self, f)
    }
}

impl fmt::Display for FeePayerBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        json_display(self, f)
    }
}

impl fmt::Display for AccountUpdate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        json_display(self, f)
    }
}

impl fmt::Display for AccountUpdateBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        json_display(self, f)
    }
}

impl fmt::Display for Update {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        json_display(self, f)
    }
}

impl fmt::Display for Permissions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        json_display(self, f)
    }
}

impl fmt::Display for SetVerificationKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        json_display(self, f)
    }
}

impl fmt::Display for Preconditions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        json_display(self, f)
    }
}

impl fmt::Display for AccountPreconditions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        json_display(self, f)
    }
}

impl fmt::Display for NetworkPreconditions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        json_display(self, f)
    }
}

impl fmt::Display for Events {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        json_display(self, f)
    }
}

impl fmt::Display for Actions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        json_display(self, f)
    }
}

impl fmt::Display for Authorization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        json_display(self, f)
    }
}

impl<T: Serialize + fmt::Debug> fmt::Display for RangeCondition<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        json_display(self, f)
    }
}

impl fmt::Display for VerificationKeyData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        json_display(self, f)
    }
}

impl fmt::Display for TimingData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        json_display(self, f)
    }
}

impl fmt::Display for EpochData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        json_display(self, f)
    }
}

impl fmt::Display for EpochLedger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        json_display(self, f)
    }
}

impl fmt::Display for AuthRequired {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        json_display(self, f)
    }
}

impl fmt::Display for MayUseToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        json_display(self, f)
    }
}

impl fmt::Display for BalanceChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        json_display(self, f)
    }
}

impl fmt::Display for AuthorizationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        json_display(self, f)
    }
}

impl fmt::Display for ZkappUri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        json_display(self, f)
    }
}

impl fmt::Display for TokenSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        json_display(self, f)
    }
}

// -------------------------------------------------------------------------------------------------
// --------------------------------- Simple Types (Value Display) ----------------------------------
// -------------------------------------------------------------------------------------------------

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

impl fmt::Display for ActionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for AuthRequiredEncoded<bool> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ \"constant\": {}, \"signature_necessary\": {}, \"signature_sufficient\": {} }}",
            self.constant, self.signature_necessary, self.signature_sufficient
        )
    }
}
