use std::io;

use clap::Parser;

use frost_client::dkg::{args::Args, cli::cli};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut reader = Box::new(io::stdin().lock());
    let mut logger = io::stdout();

    if args.ciphersuite == "bluepallas" {
        cli::<frost_bluepallas::PallasPoseidon>(&mut reader, &mut logger).await?;
    } else {
        return Err(eyre::eyre!("unsupported ciphersuite").into());
    }

    Ok(())
}
