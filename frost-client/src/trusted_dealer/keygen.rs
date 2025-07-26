use frost_core::{self as frost, Ciphersuite};

use frost::keys::{IdentifierList, PublicKeyPackage, SecretShare};
use frost::{Error, Identifier};
use rand::{CryptoRng, RngCore};
use std::collections::BTreeMap;

use super::config::Config;

/// **TESTING ONLY** - See security warnings in `Command::TrustedDealer`.
/// Generates FROST key shares using a trusted dealer approach
///
/// Creates secret shares for a threshold signature scheme where any `threshold` participants
/// can collaborate to create valid group signatures.
///
/// # Returns
///
/// - Secret shares map for each participant
/// - Public key package for the group
#[allow(clippy::type_complexity)]
pub fn keygen<C: Ciphersuite, R: RngCore + CryptoRng>(
    config: &Config,
    rng: &mut R,
) -> Result<(BTreeMap<Identifier<C>, SecretShare<C>>, PublicKeyPackage<C>), Error<C>> {
    let (shares, pubkeys) = frost::keys::generate_with_dealer(
        config.max_signers,
        config.min_signers,
        IdentifierList::Default,
        rng,
    )?;

    // Validate all shares can be converted to key packages (i.e they are valid)
    for (_k, v) in shares.clone() {
        frost::keys::KeyPackage::try_from(v)?;
    }

    Ok((shares, pubkeys))
}
