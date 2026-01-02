use rand_core::SeedableRng;

mod helpers;

#[test]
fn check_interoperability_in_sign_with_dealer() {
    // Test with multiple keys/signatures to better exercise the key generation
    // and the interoperability check.
    for i in 0..256 {
        let rng = rand_chacha::ChaChaRng::seed_from_u64(i);
        let msg = "Hello from the Raspberry Devs".as_bytes();
        let (sig, pk) =
            frost_bluepallas::signing_utilities::generate_signature_random(msg, rng).unwrap();

        helpers::verify_signature(msg, sig, pk);
    }
}
