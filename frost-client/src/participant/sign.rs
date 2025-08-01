use super::config::Config;

use super::comms::http::HTTPComms;
use super::comms::socket::SocketComms;

use super::comms::Comms;

use crate::network::Network;
use frost_bluepallas::hasher::{set_network_mainnet, set_network_testnet};
use frost_core::Ciphersuite;
use rand::thread_rng;
use std::io::{BufRead, Write};
use zeroize::Zeroizing;

/// Implementation of the participation in the FROST protocol.
/// This function handles the signing process for a participant.
/// The signing process needs to be started by a coordinator first.
pub async fn sign<C: Ciphersuite + 'static>(
    config: Config<C>,
    input: &mut impl BufRead,
    logger: &mut impl Write,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut comms: Box<dyn Comms<C>> = if config.socket {
        Box::new(SocketComms::new(&config))
    } else {
        Box::new(HTTPComms::new(&config)?)
    };

    // Round 1

    let key_package = &config.key_package;

    let mut rng = thread_rng();
    let (ret_nonces, commitments) =
        frost_core::round1::commit(key_package.signing_share(), &mut rng);
    let nonces = Zeroizing::new(ret_nonces);

    // Round 2 - Sign

    let round_2_config = comms
        .get_signing_package(input, logger, commitments, *key_package.identifier())
        .await?;

    comms
        .confirm_message(input, logger, &round_2_config)
        .await?;

    match Network::try_from(round_2_config.network_id).unwrap_or(Network::Testnet) {
        Network::Testnet => set_network_testnet()?,
        Network::Mainnet => set_network_mainnet()?,
    }

    let signing_package = round_2_config.signing_package.first().unwrap();

    let signature = frost_core::round2::sign(signing_package, &nonces, key_package)?;

    comms
        .send_signature_share(*key_package.identifier(), signature)
        .await?;

    writeln!(logger, "Done")?;

    Ok(())
}
