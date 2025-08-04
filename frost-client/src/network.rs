use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt;

use frost_bluepallas::errors::BluePallasError;

/// Network identifiers as transmitted over the wire
pub const TESTNET_ID: u8 = 0;
pub const MAINNET_ID: u8 = 1;

/// Network to use for signing operations.
#[derive(Copy, Clone, Debug, ValueEnum, Serialize, Deserialize, PartialEq, Eq)]
pub enum Network {
    Testnet,
    Mainnet,
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Network::Testnet => write!(f, "TESTNET"),
            Network::Mainnet => write!(f, "MAINNET"),
        }
    }
}

impl Network {
    /// Set the appropriate network configuration for the FROST hasher
    pub fn configure_hasher(self) -> Result<(), BluePallasError> {
        use frost_bluepallas::hasher::{set_network_mainnet, set_network_testnet};
        match self {
            Network::Testnet => set_network_testnet(),
            Network::Mainnet => set_network_mainnet(),
        }
    }
}

impl From<Network> for u8 {
    fn from(n: Network) -> Self {
        match n {
            Network::Testnet => TESTNET_ID,
            Network::Mainnet => MAINNET_ID,
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Invalid network ID: {0}. Expected {TESTNET_ID} (testnet) or {MAINNET_ID} (mainnet)")]
pub struct InvalidNetworkId(pub u8);

impl TryFrom<u8> for Network {
    type Error = InvalidNetworkId;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            TESTNET_ID => Ok(Network::Testnet),
            MAINNET_ID => Ok(Network::Mainnet),
            _ => Err(InvalidNetworkId(value)),
        }
    }
}
