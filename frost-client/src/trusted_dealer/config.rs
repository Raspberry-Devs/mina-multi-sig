use eyre::{eyre, Result};
use frost_core::Ciphersuite;

/// Configuration for trusted dealer key generation
///
/// This structure defines the parameters for FROST threshold signature scheme
/// key generation using a trusted dealer approach.
/// # FROST Parameters
///
/// - `min_signers` (threshold): The minimum number of participants required to create a valid signature
/// - `max_signers`: The total number of participants who will receive key shares
///
/// The scheme allows any subset of `min_signers` participants from the total `max_signers`
/// to collaborate and produce a valid group signature.
#[derive(Debug, PartialEq, Clone)]
pub struct Config {
    pub min_signers: u16,
    pub max_signers: u16,
}

impl Config {
    pub fn new<C: Ciphersuite>(threshold: u16, num_signers: u16) -> Result<Self> {
        let config = Self {
            min_signers: threshold,
            max_signers: num_signers,
        };

        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<()> {
        if self.min_signers < 2 {
            return Err(eyre!("Minimum signers must be at least 2"));
        }
        if self.max_signers < 2 {
            return Err(eyre!("Maximum signers must be at least 2"));
        }
        if self.min_signers > self.max_signers {
            return Err(eyre!("Minimum signers cannot exceed maximum signers"));
        }
        Ok(())
    }
}
