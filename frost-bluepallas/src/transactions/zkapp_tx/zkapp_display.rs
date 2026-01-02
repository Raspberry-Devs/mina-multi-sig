//! This module provides Display implementations for ZkApp transaction structs.
//!
//! Composite structs use `serde_json::to_string_pretty` for consistent JSON output.
//! Simple wrapper types (PublicKey, Field, ActionState) display their inner value directly.

use super::*;
use core::fmt;
use serde::Serialize;

/// Helper to create a JSON-based Display implementation.
/// Falls back to Debug formatting if serialization fails.
pub(crate) fn json_display<T: Serialize + fmt::Debug>(
    value: &T,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    match serde_json::to_string_pretty(value) {
        Ok(json) => write!(f, "{}", json),
        Err(_) => write!(f, "{:?}", value),
    }
}

/// Macro to implement `Display` for types using JSON serialization.
macro_rules! impl_json_display {
    ($($ty:ty),* $(,)?) => {
        $(
            impl fmt::Display for $ty {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    json_display(self, f)
                }
            }
        )*
    };
}

// -------------------------------------------------------------------------------------------------
// --------------------------------- Composite Structs (JSON) --------------------------------------
// -------------------------------------------------------------------------------------------------

impl_json_display!(
    ZKAppCommand,
    FeePayer,
    FeePayerBody,
    AccountUpdate,
    AccountUpdateBody,
    Update,
    Permissions,
    SetVerificationKey,
    Preconditions,
    AccountPreconditions,
    NetworkPreconditions,
    Events,
    Actions,
    Authorization,
    VerificationKeyData,
    TimingData,
    EpochData,
    EpochLedger,
    AuthRequired,
    MayUseToken,
    BalanceChange,
    AuthorizationKind,
    ZkappUri,
    TokenSymbol,
);

// Generic impl for RangeCondition (cannot use macro due to generic bounds)
impl<T: Serialize + fmt::Debug> fmt::Display for RangeCondition<T> {
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
