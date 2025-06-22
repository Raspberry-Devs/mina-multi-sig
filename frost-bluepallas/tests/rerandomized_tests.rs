use frost_bluepallas::PallasPoseidon;
use rand_core::SeedableRng;

#[test]
fn check_randomized_sign_with_dealer() {
    let rng = rand_chacha::ChaChaRng::seed_from_u64(0);

    let (_msg, _group_signature, _group_pubkey) =
        frost_rerandomized::tests::check_randomized_sign_with_dealer::<PallasPoseidon, _>(rng);
}
