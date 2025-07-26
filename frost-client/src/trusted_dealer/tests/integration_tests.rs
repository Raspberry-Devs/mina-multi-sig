use frost_bluepallas as frost;

use super::helpers::{key_package, round_1, round_2};
use crate::trusted_dealer::config::Config;
use crate::trusted_dealer::keygen::keygen as trusted_dealer_keygen;
use frost::aggregate;
use rand::thread_rng;

#[test]
fn check_keygen_with_dealer() {
    let mut rng = thread_rng();
    let config = Config {
        min_signers: 2,
        max_signers: 3,
    };
    let (shares, pubkeys) = trusted_dealer_keygen(&config, &mut rng).unwrap();

    let key_packages = key_package(&shares);
    let (nonces, commitments) = round_1(config.min_signers, &mut rng, &key_packages);
    let message = "i am a message".as_bytes();
    let (signing_package, signature_shares) = round_2(nonces, &key_packages, commitments, message);
    let group_signature = aggregate(&signing_package, &signature_shares, &pubkeys).unwrap();
    let verify_signature = pubkeys.verifying_key().verify(message, &group_signature);

    assert!(verify_signature.is_ok());
}

#[test]
fn check_keygen_with_dealer_with_large_num_of_signers() {
    let mut rng = thread_rng();
    let config = Config {
        min_signers: 14,
        max_signers: 20,
    };
    let (shares, pubkeys) = trusted_dealer_keygen(&config, &mut rng).unwrap();

    let key_packages = key_package(&shares);
    let (nonces, commitments) = round_1(config.min_signers, &mut rng, &key_packages);
    let message = "i am a message".as_bytes();
    let (signing_package, signature_shares) = round_2(nonces, &key_packages, commitments, message);
    let group_signature = aggregate(&signing_package, &signature_shares, &pubkeys).unwrap();
    let verify_signature = pubkeys.verifying_key().verify(message, &group_signature);

    assert!(verify_signature.is_ok());
}
