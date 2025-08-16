use frost_core::keys::{KeyPackage, PublicKeyPackage};
use frost_core::{self as frost, Ciphersuite, Identifier};

use crate::cipher::PublicKey;
use rand::thread_rng;
use std::collections::HashMap;
use std::error::Error;
use std::io::{BufRead, Write};
use zeroize::Zeroizing;

use super::comms::http::HTTPComms;
use super::comms::Comms;
use super::config::Config;

/// Performs distributed key generation (DKG) using the FROST protocol
///
/// This function orchestrates the complete 3-round DKG process:
/// 1. **Round 1**: Each participant generates and shares commitments to their secret polynomial
/// 2. **Round 2**: Participants exchange secret shares and verify commitments
/// 3. **Round 3**: Final key package generation and verification
///
/// # Arguments
///
/// * `config` - DKG configuration including network settings, participant info, and threshold
/// * `input` - Input stream for user interaction (e.g., confirmations)
/// * `logger` - Output stream for logging progress and status messages
///
/// # Returns
///
/// A tuple containing:
/// * `KeyPackage<C>` - This participant's private key share and verification data
/// * `PublicKeyPackage<C>` - The group's public key and all participants' public shares
/// * `HashMap<PublicKey, Identifier<C>>` - Mapping of communication public keys to FROST identifiers
///
/// # Errors
///
/// Returns an error if:
/// - Network communication fails
/// - Cryptographic verification fails
/// - Invalid participant responses are received
/// - The DKG protocol is aborted by any participant
pub async fn keygen<C: Ciphersuite + 'static>(
    config: Config,
    input: &mut impl BufRead,
    logger: &mut impl Write,
) -> Result<
    (
        KeyPackage<C>,
        PublicKeyPackage<C>,
        HashMap<PublicKey, Identifier<C>>,
    ),
    Box<dyn Error>,
> {
    let mut comms: Box<dyn Comms<C>> = Box::new(HTTPComms::new(config.clone())?);

    // We put the main logic on a block to be able to cleanup if an error is
    // returned anywhere in it.
    let doit = async {
        let rng = thread_rng();
        let (identifier, max_signers) = comms.get_identifier_and_max_signers(input, logger).await?;
        let (round1_secret_package, round1_package) =
            frost::keys::dkg::part1(identifier, max_signers, config.min_signers, rng)?;
        let received_round1_packages = comms
            .get_round1_packages(input, logger, round1_package)
            .await?;
        let (round2_secret_package, round2_packages) =
            frost::keys::dkg::part2(round1_secret_package, &received_round1_packages)?;
        let round2_secret_package = Zeroizing::new(round2_secret_package);
        let received_round2_packages = comms
            .get_round2_packages(input, logger, round2_packages)
            .await?;
        let (key_package, public_key_package) = frost::keys::dkg::part3(
            &round2_secret_package,
            &received_round1_packages,
            &received_round2_packages,
        )?;
        let pubkey_map = comms.get_pubkey_identifier_map()?;
        Ok((key_package, public_key_package, pubkey_map))
    };

    let result = doit.await;
    if result.is_err() {
        let _ = comms.cleanup_on_error().await;
    }
    result
}
