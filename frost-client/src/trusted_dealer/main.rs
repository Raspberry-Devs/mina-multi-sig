use std::io;

use clap::Parser;

use frost_client::trusted_dealer::{args::Args, cli::cli};

// TODO: Update to use exit codes
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut reader = Box::new(io::stdin().lock());
    let mut logger = io::stdout();
    if args.ciphersuite == "bluepallas" {
        cli::<frost_bluepallas::PallasPoseidon>(&args, &mut reader, &mut logger)?;
    } else {
        return Err(format!("Unsupported ciphersuite: {}", args.ciphersuite).into());
    }

    Ok(())
}
