use std::error::Error;

use eyre::OptionExt;
use frost_bluepallas::PallasPoseidon;

use super::{args::Command, config::Config};

pub fn list(args: &Command) -> Result<(), Box<dyn Error>> {
    let Command::Groups { config } = (*args).clone() else {
        panic!("invalid Command");
    };

    let config = Config::<PallasPoseidon>::read(config)?;

    for group in config.group.values() {
        eprint!("{}", group.as_human_readable_summary(&config)?);
        eprintln!();
    }

    Ok(())
}

/// Remove a group from the user's config file.
pub fn remove(args: &Command) -> Result<(), Box<dyn Error>> {
    let Command::RemoveGroup { config, group } = (*args).clone() else {
        panic!("invalid Command");
    };

    let mut config = Config::<PallasPoseidon>::read(config)?;

    config.group.remove(&group).ok_or_eyre("group not found")?;

    config.write()?;

    Ok(())
}
