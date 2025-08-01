use std::collections::BTreeMap;
use std::io::{BufRead, Write};

use frost_core::{
    self, keys::PublicKeyPackage, round1::SigningCommitments, Ciphersuite, Identifier,
    SigningPackage,
};

use super::comms::http::HTTPComms;
use super::comms::socket::SocketComms;
use super::comms::Comms;
use super::config::Config;
use frost_bluepallas::hasher::{set_network_mainnet, set_network_testnet};

#[derive(Debug, PartialEq)]
pub struct ParticipantsConfig<C: Ciphersuite> {
    pub commitments: BTreeMap<Identifier<C>, SigningCommitments<C>>,
    pub pub_key_package: PublicKeyPackage<C>,
}

pub async fn sign<C: Ciphersuite + 'static>(
    config: &Config<C>,
    reader: &mut impl BufRead,
    logger: &mut impl Write,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    match config.network {
        crate::network::Network::Testnet => set_network_testnet()?,
        crate::network::Network::Mainnet => set_network_mainnet()?,
    }
    let mut comms: Box<dyn Comms<C>> = if config.socket {
        Box::new(SocketComms::new(config))
    } else {
        Box::new(HTTPComms::new(config)?)
    };

    // Round 1 - Get commitments
    let commitments_list = comms
        .get_signing_commitments(
            reader,
            logger,
            &config.public_key_package,
            config.num_signers,
        )
        .await;

    let commitments = match commitments_list {
        Ok(commitments) => commitments,
        Err(e) => {
            let _ = comms.cleanup_on_error().await;
            return Err(e);
        }
    };

    // Round 2 - Create signing package and get signature shares
    let signing_package = SigningPackage::new(commitments.clone(), &config.messages[0]);

    let signatures_list = comms
        .send_signing_package_and_get_signature_shares(reader, logger, &signing_package)
        .await;

    let signatures = match signatures_list {
        Ok(signatures) => signatures,
        Err(e) => {
            let _ = comms.cleanup_on_error().await;
            return Err(e);
        }
    };

    // Aggregate signatures
    let group_signature =
        frost_core::aggregate::<C>(&signing_package, &signatures, &config.public_key_package);

    let signature_bytes_result = match group_signature {
        Ok(signature) => signature.serialize(),
        Err(e) => {
            let _ = comms.cleanup_on_error().await;
            return Err(e.into());
        }
    };

    let signature_bytes = match signature_bytes_result {
        Ok(bytes) => bytes,
        Err(e) => {
            let _ = comms.cleanup_on_error().await;
            return Err(e.into());
        }
    };

    Ok(signature_bytes)
}
