//! Module containint various constants used by ZkApp transactions (both for packing and hashing)

use alloc::string::ToString;
use core::str::FromStr;

use ark_ff::{AdditiveGroup, BigInt, PrimeField};
use lazy_static::lazy_static;
use mina_hasher::Fp;
use mina_signer::NetworkId;

use crate::{errors::BluePallasError, transactions::zkapp_tx::Field};

pub const TXN_VERSION_CURRENT: u32 = 3; // Used in Emptiable

// Constant value for a dummy verification key, if an account update is not proved and instead signed
// then we use this constant hash value to indicate that no verification key is associated with the account update.
lazy_static! {
    pub static ref DUMMY_HASH: Field = Field::from(
        Fp::from_bigint(BigInt::from_str(DUMMY_HASH_STR).unwrap())
            .ok_or(BluePallasError::InvalidZkAppCommand(
                "Failed to convert dummy hash to Fp".to_string()
            ),)
            .unwrap()
    );
}
const DUMMY_HASH_STR: &str =
    "3392518251768960475377392625298437850623664973002200885669375116181514017494";

pub const EMPTY_STACK_HASH: Fp = Fp::ZERO;

// Used as prefix for hashing
pub const ZK_APP_BODY_MAINNET: &str = "MainnetZkappBody****";
pub const ZK_APP_BODY_TESTNET: &str = "TestnetZkappBody****";
pub const PREFIX_ACCOUNT_UPDATE_NODE: &str = "MinaAcctUpdateNode**";
pub const PREFIX_ACCOUNT_UPDATE_CONS: &str = "MinaAcctUpdateCons**";
pub const ZK_APP_MEMO: &str = "MinaZkappMemo";

pub const ZK_APP_EVENTS_EMPTY: &str = "MinaZkappEventsEmpty";
pub const ZK_APP_EVENTS_PREFIX: &str = "MinaZkappEvents";
pub const ZK_APP_EVENT_PREFIX: &str = "MinaZkappEvent";

pub const ZK_APP_ACTIONS_EMPTY: &str = "MinaZkappActionsEmpty";
pub const ZK_APP_ACTIONS_PREFIX: &str = "MinaZkappSeqEvents";

pub const ZK_ACTION_STATE_EMPTY: &str = "MinaZkappActionStateEmptyElt";

pub const MINA_ZKAPP_URI: &str = "MinaZkappUri";

pub(crate) const APP_STATE_LENGTH: usize = 8;

// Enum to represent the prefix used for hashing zkapp body based on network
pub enum ZkAppBodyPrefix {
    Mainnet,
    Testnet,
}

impl From<NetworkId> for ZkAppBodyPrefix {
    fn from(network: NetworkId) -> Self {
        match network {
            NetworkId::MAINNET => ZkAppBodyPrefix::Mainnet,
            NetworkId::TESTNET => ZkAppBodyPrefix::Testnet,
        }
    }
}

impl From<ZkAppBodyPrefix> for &'static str {
    fn from(value: ZkAppBodyPrefix) -> Self {
        match value {
            ZkAppBodyPrefix::Mainnet => ZK_APP_BODY_MAINNET,
            ZkAppBodyPrefix::Testnet => ZK_APP_BODY_TESTNET,
        }
    }
}

// Poseidon hashing parameters
pub(crate) const POSEIDON_FULL_ROUNDS: usize = 55;
