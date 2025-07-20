pub mod config;
pub mod trusted_dealer_keygen;

pub use config::Config;
pub use trusted_dealer_keygen::{trusted_dealer_keygen};

#[cfg(test)]
mod tests;