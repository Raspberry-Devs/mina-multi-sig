use super::config::Config;

use super::comms::http::HTTPComms;

use super::comms::Comms;

use crate::mina_network::Network;
use frost_bluepallas::PallasPoseidon;
use rand::thread_rng;
use std::io::{BufRead, Write};
use zeroize::Zeroizing;

/// Implementation of the participation in the FROST protocol.
/// This function handles the signing process for a participant.
/// The signing process needs to be started by a coordinator first.
pub async fn sign(
    config: Config<PallasPoseidon>,
    input: &mut impl BufRead,
    logger: &mut impl Write,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut comms: Box<dyn Comms<PallasPoseidon>> = Box::new(HTTPComms::new(&config)?);

    // Round 1

    let key_package = &config.key_package;

    let mut rng = thread_rng();
    let (ret_nonces, commitments) =
        frost_bluepallas::round1::commit(key_package.signing_share(), &mut rng);
    let nonces = Zeroizing::new(ret_nonces);

    // Round 2 - Sign

    let round_2_config = comms
        .get_signing_package(input, logger, commitments, *key_package.identifier())
        .await?;

    comms
        .confirm_message(input, logger, &round_2_config)
        .await?;

    // Set the internal NetworkID based on the configuration
    Network::try_from(round_2_config.network_id)?.configure_hasher()?;

    let signing_package = round_2_config.signing_package.first().unwrap();

    // Use frost_bluepallas modified sign behaviour
    let signature = frost_bluepallas::round2::sign(signing_package, &nonces, key_package)?;

    comms
        .send_signature_share(*key_package.identifier(), signature)
        .await?;

    writeln!(logger, "Done")?;

    Ok(())
}
