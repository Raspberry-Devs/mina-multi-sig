use std::error::Error;

use clap::Parser;
use frost_bluepallas::BluePallas;
use mina_frost_client::cli;
use mina_frost_client::cli::args::{Args, Command};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    stable_eyre::install()?;
    let args = Args::parse();

    match args.command {
        Command::Init { .. } => cli::init::init::<BluePallas>(&args.command).await,
        Command::Export { .. } => cli::contact::export::<BluePallas>(&args.command),
        Command::Import { .. } => cli::contact::import::<BluePallas>(&args.command),
        Command::Contacts { .. } => cli::contact::list::<BluePallas>(&args.command),
        Command::RemoveContact { .. } => cli::contact::remove::<BluePallas>(&args.command),
        Command::Groups { .. } => cli::group::list::<BluePallas>(&args.command),
        Command::RemoveGroup { .. } => cli::group::remove::<BluePallas>(&args.command),
        Command::Sessions { .. } => cli::session::list::<BluePallas>(&args.command).await,
        Command::TrustedDealer { .. } => cli::trusted_dealer::run::<BluePallas>(&args.command),
        Command::Dkg { .. } => cli::dkg::run::<BluePallas>(&args.command).await,
        // Coordinator implicitly assumes within the run() function that we use BluePallas
        Command::Coordinator { .. } => cli::coordinator::run_bluepallas(&args.command).await,
        // Participant implicitly assumes within the run() function that we use BluePallas
        Command::Participant { .. } => cli::participant::run_bluepallas(&args.command).await,
        Command::Graphql {
            input_path,
            output_path,
        } => cli::graphql::run_graphql_command(&input_path, &output_path),
    }?;

    Ok(())
}
