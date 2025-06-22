use crate::PallasPoseidon;
use frost_bluepallas::*;
use rand_core::SeedableRng;

mod helpers;

#[test]
fn check_interoperability_in_sign_with_dkg() {
    // Test with multiple keys/signatures to better exercise the key generation
    // and the interoperability check. A smaller number of iterations is used
    // because DKG takes longer and otherwise the test would be too slow.
    for i in 0..32 {
        let rng = rand_chacha::ChaChaRng::seed_from_u64(i);
        let (msg, group_signature, group_pubkey) =
            frost_core::tests::ciphersuite_generic::check_sign_with_dkg::<PallasPoseidon, _>(rng);

        helpers::verify_signature(&msg, group_signature, group_pubkey);
    }
}

#[test]
fn check_interoperability_in_sign_with_dealer() {
    // Test with multiple keys/signatures to better exercise the key generation
    // and the interoperability check.
    for i in 0..256 {
        let rng = rand_chacha::ChaChaRng::seed_from_u64(i);
        let (msg, group_signature, group_pubkey) =
            frost_core::tests::ciphersuite_generic::check_sign_with_dealer::<PallasPoseidon, _>(
                rng,
            );

        // Check that the threshold signature can be verified by the `ed25519_dalek` crate
        // public key (interoperability test)
        helpers::verify_signature(&msg, group_signature, group_pubkey);
    }
}
