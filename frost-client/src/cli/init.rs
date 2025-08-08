use std::error::Error;

use crate::cipher::Cipher;
use frost_core::Ciphersuite;

use super::{
    args::Command,
    config::{CommunicationKey, Config},
};

pub async fn init<C: Ciphersuite>(args: &Command) -> Result<(), Box<dyn Error>> {
    let Command::Init { config } = (*args).clone() else {
        panic!("invalid Command");
    };

    // To make frost client truly ciphersuite agnostic provide the ciphersuite as user flag
    // We don't do this cause it would just be more boilerplate at the moment
    let mut config = Config::<C>::read(config)?;

    if config.communication_key.is_some() {
        eprintln!("Skipping keypair generation; keypair already generated and stored");
    } else {
        eprintln!("Generating keypair... ");
        let (privkey, pubkey) = Cipher::generate_keypair()?;
        config.communication_key = Some(CommunicationKey { privkey, pubkey });
    };

    eprintln!(
        "Writing to config file at {}...",
        config.path().expect("should not be None").display()
    );
    config.write()?;
    eprintln!(
        "Done.\nWARNING: the config file will contain your private FROST shares in clear. \
    Keep it safe and never share it with anyone. Future versions of this tool might encrypt \
    the config file."
    );

    Ok(())
}
