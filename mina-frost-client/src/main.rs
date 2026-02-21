use std::error::Error;

use clap::Parser;
use frost_bluepallas::BluePallas;
use mina_frost_client::cli;
use mina_frost_client::cli::args::{Args, Command};
use mina_tx::pallas_message::PallasMessage;

type BluePallasSuite = BluePallas<PallasMessage>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    stable_eyre::install()?;
    let args = Args::parse();

    match args.command {
        Command::Init { .. } => cli::init::init::<BluePallasSuite>(&args.command).await,
        Command::Export { .. } => cli::contact::export::<BluePallasSuite>(&args.command),
        Command::Import { .. } => cli::contact::import::<BluePallasSuite>(&args.command),
        Command::Contacts { .. } => cli::contact::list::<BluePallasSuite>(&args.command),
        Command::RemoveContact { .. } => cli::contact::remove::<BluePallasSuite>(&args.command),
        Command::Groups { .. } => cli::group::list::<BluePallasSuite>(&args.command),
        Command::RemoveGroup { .. } => cli::group::remove::<BluePallasSuite>(&args.command),
        Command::Sessions { .. } => cli::session::list::<BluePallasSuite>(&args.command).await,
        Command::TrustedDealer { .. } => cli::trusted_dealer::run::<BluePallasSuite>(&args.command),
        Command::Dkg { .. } => cli::dkg::run::<BluePallasSuite>(&args.command).await,
        // Coordinator implicitly assumes within the run() function that we use BluePallas
        Command::Coordinator { .. } => cli::coordinator::run_bluepallas(&args.command).await,
        // Participant implicitly assumes within the run() function that we use BluePallas
        Command::Participant { .. } => cli::participant::run_bluepallas(&args.command).await,
        Command::GraphqlBuild { .. } => cli::graphql::graphql_build_command(&args.command),
        Command::GraphqlBroadcast { .. } => {
            cli::graphql::graphql_broadcast_command(&args.command).await
        }
    }?;

    Ok(())
}
