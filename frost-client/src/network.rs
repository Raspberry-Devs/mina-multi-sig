use clap::ValueEnum;
use serde::{Deserialize, Serialize};

/// Network to use for signing operations.
#[derive(Copy, Clone, Debug, ValueEnum, Serialize, Deserialize)]
pub enum Network {
    Testnet,
    Mainnet,
}

impl From<Network> for u8 {
    fn from(n: Network) -> Self {
        match n {
            Network::Testnet => 0,
            Network::Mainnet => 1,
        }
    }
}

impl TryFrom<u8> for Network {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Network::Testnet),
            1 => Ok(Network::Mainnet),
            _ => Err(()),
        }
    }
}
