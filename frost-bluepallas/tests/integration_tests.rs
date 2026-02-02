use frost_bluepallas::*;
use lazy_static::lazy_static;
use rand_core::SeedableRng;
use serde_json::Value;

#[test]
fn check_zero_key_fails() {
    frost_core::tests::ciphersuite_generic::check_zero_key_fails::<BluePallas>();
}

#[ignore = "upstream frost-core v3.0.0-rc.0 issue #1015: signature share verification bug"]
#[test]
fn check_sign_with_dkg() {
    let rng = rand_chacha::ChaChaRng::seed_from_u64(0);
    frost_core::tests::ciphersuite_generic::check_sign_with_dkg::<BluePallas, _>(rng);
}

#[test]
fn check_dkg_part1_fails_with_invalid_signers_min_signers() {
    let rng = rand_chacha::ChaChaRng::seed_from_u64(0);

    let min_signers = 1;
    let max_signers = 3;
    let error = Error::InvalidMinSigners;

    frost_core::tests::ciphersuite_generic::check_sign_with_dealer_fails_with_invalid_signers::<
        BluePallas,
        _,
    >(min_signers, max_signers, error, rng);
}

#[test]
fn check_dkg_part1_fails_with_min_signers_greater_than_max() {
    let rng = rand_chacha::ChaChaRng::seed_from_u64(0);

    let min_signers = 3;
    let max_signers = 2;
    let error: frost_core::Error<BluePallas> = Error::InvalidMinSigners;

    frost_core::tests::ciphersuite_generic::check_sign_with_dealer_fails_with_invalid_signers::<
        BluePallas,
        _,
    >(min_signers, max_signers, error, rng);
}

#[test]
fn check_dkg_part1_fails_with_invalid_signers_max_signers() {
    let rng = rand_chacha::ChaChaRng::seed_from_u64(0);

    let min_signers = 3;
    let max_signers = 1;
    let error = Error::InvalidMaxSigners;

    frost_core::tests::ciphersuite_generic::check_sign_with_dealer_fails_with_invalid_signers::<
        BluePallas,
        _,
    >(min_signers, max_signers, error, rng);
}

#[test]
fn check_rts() {
    let rng = rand_chacha::ChaChaRng::seed_from_u64(0);

    frost_core::tests::repairable::check_rts::<BluePallas, _>(rng);
}

#[ignore = "upstream frost-core v3.0.0-rc.0 issue #1015: signature share verification bug"]
#[test]
fn check_refresh_shares_with_dealer() {
    let rng = rand_chacha::ChaChaRng::seed_from_u64(0);
    frost_core::tests::refresh::check_refresh_shares_with_dealer::<BluePallas, _>(rng);
}

#[test]
fn check_refresh_shares_with_dealer_serialisation() {
    let rng = rand_chacha::ChaChaRng::seed_from_u64(0);

    frost_core::tests::refresh::check_refresh_shares_with_dealer_serialisation::<BluePallas, _>(
        rng,
    );
}

#[test]
fn check_refresh_shares_with_dealer_fails_with_invalid_public_key_package() {
    let rng = rand_chacha::ChaChaRng::seed_from_u64(0);

    frost_core::tests::refresh::check_refresh_shares_with_dealer_fails_with_invalid_public_key_package::<
        BluePallas,
        _,
    >(rng);
}

// Tests for invalid min_signers, unequal num identifiers/max_signers, and
// min_signers > max_signers were removed because frost-core 3.0.0 no longer
// exposes min_signers/max_signers parameters in the refresh test helper.

#[test]
fn check_refresh_shares_with_dealer_fails_with_invalid_max_signers() {
    let rng = rand_chacha::ChaChaRng::seed_from_u64(0);
    let identifiers = vec![Identifier::try_from(1).unwrap()];
    let error = Error::InvalidMaxSigners;

    frost_core::tests::refresh::check_refresh_shares_with_dealer_fails_with_invalid_signers::<
        BluePallas,
        _,
    >(&identifiers, error, rng);
}

#[test]
fn check_refresh_shares_with_dealer_fails_with_invalid_identifier() {
    let rng = rand_chacha::ChaChaRng::seed_from_u64(0);
    let identifiers = vec![
        Identifier::try_from(8).unwrap(),
        Identifier::try_from(3).unwrap(),
        Identifier::try_from(4).unwrap(),
        Identifier::try_from(6).unwrap(),
    ];
    let error = Error::UnknownIdentifier;

    frost_core::tests::refresh::check_refresh_shares_with_dealer_fails_with_invalid_signers::<
        BluePallas,
        _,
    >(&identifiers, error, rng);
}

#[ignore = "upstream frost-core v3.0.0-rc.0 issue #1015: signature share verification bug"]
#[test]
fn check_refresh_shares_with_dkg() {
    let rng = rand_chacha::ChaChaRng::seed_from_u64(0);
    frost_core::tests::refresh::check_refresh_shares_with_dkg::<BluePallas, _>(rng);
}

#[ignore = "upstream frost-core v3.0.0-rc.0 issue #1015: signature share verification bug"]
#[test]
fn check_sign_with_dealer() {
    let rng = rand_chacha::ChaChaRng::seed_from_u64(0);
    frost_core::tests::ciphersuite_generic::check_sign_with_dealer::<BluePallas, _>(rng);
}

#[test]
fn check_sign_with_dealer_fails_with_invalid_min_signers() {
    let rng = rand_chacha::ChaChaRng::seed_from_u64(0);

    let min_signers = 1;
    let max_signers = 3;
    let error = Error::InvalidMinSigners;

    frost_core::tests::ciphersuite_generic::check_sign_with_dealer_fails_with_invalid_signers::<
        BluePallas,
        _,
    >(min_signers, max_signers, error, rng);
}

#[test]
fn check_sign_with_dealer_fails_with_min_signers_greater_than_max() {
    let rng = rand_chacha::ChaChaRng::seed_from_u64(0);

    let min_signers = 3;
    let max_signers = 2;
    let error: frost_core::Error<BluePallas> = Error::InvalidMinSigners;

    frost_core::tests::ciphersuite_generic::check_sign_with_dealer_fails_with_invalid_signers::<
        BluePallas,
        _,
    >(min_signers, max_signers, error, rng);
}

#[test]
fn check_sign_with_dealer_fails_with_invalid_max_signers() {
    let rng = rand_chacha::ChaChaRng::seed_from_u64(0);

    let min_signers = 3;
    let max_signers = 1;
    let error = Error::InvalidMaxSigners;

    frost_core::tests::ciphersuite_generic::check_sign_with_dealer_fails_with_invalid_signers::<
        BluePallas,
        _,
    >(min_signers, max_signers, error, rng);
}

/// This is testing that Shamir's secret sharing to compute and arbitrary
/// value is working.
#[test]
fn check_share_generation_pallas_poseidon() {
    let rng = rand_chacha::ChaChaRng::seed_from_u64(0);
    frost_core::tests::ciphersuite_generic::check_share_generation::<BluePallas, _>(rng);
}

#[test]
fn check_share_generation_fails_with_invalid_min_signers() {
    let rng = rand_chacha::ChaChaRng::seed_from_u64(0);

    let min_signers = 0;
    let max_signers = 3;
    let error = Error::InvalidMinSigners;

    frost_core::tests::ciphersuite_generic::check_share_generation_fails_with_invalid_signers::<
        BluePallas,
        _,
    >(min_signers, max_signers, error, rng);
}

#[test]
fn check_share_generation_fails_with_min_signers_greater_than_max() {
    let rng = rand_chacha::ChaChaRng::seed_from_u64(0);

    let min_signers = 3;
    let max_signers = 2;
    let error: frost_core::Error<BluePallas> = Error::InvalidMinSigners;

    frost_core::tests::ciphersuite_generic::check_share_generation_fails_with_invalid_signers::<
        BluePallas,
        _,
    >(min_signers, max_signers, error, rng);
}

#[test]
fn check_share_generation_fails_with_invalid_max_signers() {
    let rng = rand_chacha::ChaChaRng::seed_from_u64(0);

    let min_signers = 3;
    let max_signers = 0;
    let error = Error::InvalidMaxSigners;

    frost_core::tests::ciphersuite_generic::check_share_generation_fails_with_invalid_signers::<
        BluePallas,
        _,
    >(min_signers, max_signers, error, rng);
}

lazy_static! {
    pub static ref VECTORS: Value =
        serde_json::from_str(include_str!("../tests/helpers/vectors.json").trim())
            .expect("Test vector is valid JSON");
    pub static ref VECTORS_BIG_IDENTIFIER: Value =
        serde_json::from_str(include_str!("../tests/helpers/vectors-big-identifier.json").trim())
            .expect("Test vector is valid JSON");
}

#[test]
fn check_sign_with_test_vectors() {
    frost_core::tests::vectors::check_sign_with_test_vectors::<BluePallas>(&VECTORS);
}

#[test]
fn check_sign_with_test_vectors_with_big_identifiers() {
    frost_core::tests::vectors::check_sign_with_test_vectors::<BluePallas>(&VECTORS_BIG_IDENTIFIER);
}

#[test]
fn check_error_culprit() {
    frost_core::tests::ciphersuite_generic::check_error_culprit::<BluePallas>();
}

#[test]
fn check_identifier_derivation() {
    frost_core::tests::ciphersuite_generic::check_identifier_derivation::<BluePallas>();
}

// Explicit test which is used in a documentation snippet
#[test]
#[allow(unused_variables)]
fn check_identifier_generation() -> Result<(), Error> {
    // ANCHOR: dkg_identifier
    let participant_identifier = Identifier::try_from(7u16)?;
    let participant_identifier = Identifier::derive("alice@example.com".as_bytes())?;
    // ANCHOR_END: dkg_identifier
    Ok(())
}

#[test]
fn check_sign_with_dealer_and_identifiers() {
    let rng = rand_chacha::ChaChaRng::seed_from_u64(0);

    frost_core::tests::ciphersuite_generic::check_sign_with_dealer_and_identifiers::<BluePallas, _>(
        rng,
    );
}

#[test]
fn check_sign_with_missing_identifier() {
    let rng = rand_chacha::ChaChaRng::seed_from_u64(0);
    frost_core::tests::ciphersuite_generic::check_sign_with_missing_identifier::<BluePallas, _>(
        rng,
    );
}

#[test]
fn check_sign_with_incorrect_commitments() {
    let rng = rand_chacha::ChaChaRng::seed_from_u64(0);
    frost_core::tests::ciphersuite_generic::check_sign_with_incorrect_commitments::<BluePallas, _>(
        rng,
    );
}
