use std::error::Error;

use clap::Parser;
use frost_bluepallas::PallasPoseidon;
use frost_client::cli;
use frost_client::cli::args::{Args, Command};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    stable_eyre::install()?;
    let args = Args::parse();

    match args.command {
        Command::Init { .. } => cli::init::init::<PallasPoseidon>(&args.command).await,
        Command::Export { .. } => cli::contact::export::<PallasPoseidon>(&args.command),
        Command::Import { .. } => cli::contact::import::<PallasPoseidon>(&args.command),
        Command::Contacts { .. } => cli::contact::list::<PallasPoseidon>(&args.command),
        Command::RemoveContact { .. } => cli::contact::remove::<PallasPoseidon>(&args.command),
        Command::Groups { .. } => cli::group::list::<PallasPoseidon>(&args.command),
        Command::RemoveGroup { .. } => cli::group::remove::<PallasPoseidon>(&args.command),
        Command::Sessions { .. } => cli::session::list::<PallasPoseidon>(&args.command).await,
        Command::TrustedDealer { .. } => cli::trusted_dealer::run::<PallasPoseidon>(&args.command),
        Command::Dkg { .. } => cli::dkg::run::<PallasPoseidon>(&args.command).await,
        // Coordinator implicitly assumes within the run() function that we use PallasPoseidon
        Command::Coordinator { .. } => cli::coordinator::run_bluepallas(&args.command).await,
        // Participant implicitly assumes within the run() function that we use PallasPoseidon
        Command::Participant { .. } => cli::participant::run_bluepallas(&args.command).await,
    }?;

    Ok(())
}
