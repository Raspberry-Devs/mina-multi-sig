use std::collections::BTreeMap;

use crate::{self as frost};
use rand_core::{CryptoRng, RngCore};

/// Helper function to sign a message using existing key packages
pub fn sign_from_packages<R: RngCore + CryptoRng>(
    message: &[u8],
    shares: BTreeMap<frost::Identifier, frost::keys::SecretShare>,
    pubkey_package: frost::keys::PublicKeyPackage,
    mut rng: R,
) -> Result<(frost::Signature, frost::VerifyingKey), frost::Error> {
    let min_signers = pubkey_package.verifying_shares().len().min(3);

    // Verifies the secret shares from the dealer and store them in a BTreeMap.
    // In practice, the KeyPackages must be sent to its respective participants
    // through a confidential and authenticated channel.
    let mut key_packages: BTreeMap<_, _> = BTreeMap::new();

    for (identifier, secret_share) in shares {
        let key_package = frost::keys::KeyPackage::try_from(secret_share)?;
        key_packages.insert(identifier, key_package);
    }

    let mut nonces_map = BTreeMap::new();
    let mut commitments_map = BTreeMap::new();

    ////////////////////////////////////////////////////////////////////////////
    // Round 1: generating nonces and signing commitments for each participant
    ////////////////////////////////////////////////////////////////////////////

    // In practice, each iteration of this loop will be executed by its respective participant.
    for participant_index in 1..=min_signers {
        let participant_identifier = frost::Identifier::try_from(participant_index as u16)
            .map_err(|_| frost::Error::MalformedIdentifier)?;
        let key_package = &key_packages[&participant_identifier];
        // Generate one (1) nonce and one SigningCommitments instance for each
        // participant, up to _threshold_.
        let (nonces, commitments) = frost::round1::commit(key_package.signing_share(), &mut rng);
        // In practice, the nonces must be kept by the participant to use in the
        // next round, while the commitment must be sent to the coordinator
        // (or to every other participant if there is no coordinator) using
        // an authenticated channel.
        nonces_map.insert(participant_identifier, nonces);
        commitments_map.insert(participant_identifier, commitments);
    }

    // This is what the signature aggregator / coordinator needs to do:
    // - decide what message to sign
    // - take one (unused) commitment per signing participant
    let mut signature_shares = BTreeMap::new();
    let signing_package = frost::SigningPackage::new(commitments_map, message);

    ////////////////////////////////////////////////////////////////////////////
    // Round 2: each participant generates their signature share
    ////////////////////////////////////////////////////////////////////////////

    // In practice, each iteration of this loop will be executed by its respective participant.
    for participant_identifier in nonces_map.keys() {
        let key_package = &key_packages[participant_identifier];

        let nonces = &nonces_map[participant_identifier];

        // Each participant generates their signature share.
        let signature_share = frost::round2::sign(&signing_package, nonces, key_package)?;

        // In practice, the signature share must be sent to the Coordinator
        // using an authenticated channel.
        signature_shares.insert(*participant_identifier, signature_share);
    }

    ////////////////////////////////////////////////////////////////////////////
    // Aggregation: collects the signing shares from all participants,
    // generates the final signature.
    ////////////////////////////////////////////////////////////////////////////

    // Aggregate (also verifies the signature shares)
    let group_signature = frost::aggregate(&signing_package, &signature_shares, &pubkey_package)?;
    let pk = pubkey_package.verifying_key();

    Ok((group_signature, *pk))
}

/// Helper function which uses FROST to generate a signature, message and verifying key to use in tests.
/// This uses trusted dealer rather than DKG
#[allow(dead_code)]
pub fn generate_signature_random<R: RngCore + CryptoRng>(
    message: &[u8],
    mut rng: R,
) -> Result<(frost::Signature, frost::VerifyingKey), frost::Error> {
    let max_signers = 5;
    let min_signers = 3;
    let (shares, pubkey_package) = frost::keys::generate_with_dealer(
        max_signers,
        min_signers,
        frost::keys::IdentifierList::Default,
        &mut rng,
    )?;

    sign_from_packages(message, shares, pubkey_package, rng)
}

/// Helper function which splits an existing signing key into FROST shares and generates a signature.
/// This uses the split function to create shares from a single signing key.
#[allow(dead_code)]
pub fn generate_signature_from_sk<R: RngCore + CryptoRng>(
    message: &[u8],
    signing_key: &frost::SigningKey,
    mut rng: R,
) -> Result<(frost::Signature, frost::VerifyingKey), frost::Error> {
    let max_signers = 5;
    let min_signers = 3;
    let (shares, pubkey_package) = frost::keys::split(
        signing_key,
        max_signers,
        min_signers,
        frost::keys::IdentifierList::Default,
        &mut rng,
    )?;

    sign_from_packages(message, shares, pubkey_package, rng)
}
