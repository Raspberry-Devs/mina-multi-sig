use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::fields::PrimeField;
use ark_ff::BigInteger;
use frost_bluepallas::{
    hasher::{message_hash, PallasMessage},
    transactions::Transaction,
    translate::{translate_msg, translate_pk, translate_sig},
    PallasGroup,
};
use frost_core::{Ciphersuite, Group};

use mina_hasher::Hashable;
use mina_signer::{CurvePoint, NetworkId, PubKey, Signer};
use rand_core::SeedableRng;

use std::ops::{Add, Neg};

use frost_bluepallas::helper::generate_signature_random;

#[test]
fn frost_sign_mina_verify() -> Result<(), Box<dyn std::error::Error>> {
    // Esnure that the FROST implementation can sign a message and Mina can verify it

    let rng = rand_chacha::ChaChaRng::seed_from_u64(100);
    let fr_msg = b"Test message for FROST and Mina compatibility".to_vec();

    let (fr_sig, fr_pk) = generate_signature_random(&fr_msg, rng)?;

    assert!(
        fr_sig
            .R()
            .into_affine()
            .y()
            .expect("Failed to extract y-coord from sig")
            .into_bigint()
            .is_even(),
        "Signature commitment y-coordinate must be even"
    );

    let res = frost_bluepallas::PallasPoseidon::verify_signature(&fr_msg, &fr_sig, &fr_pk);
    assert!(res.is_ok(), "FROST correctly verifies signature");

    let mina_pk = translate_pk(&fr_pk)?;
    let mina_sig = translate_sig(&fr_sig)?;
    let mina_msg = PallasMessage::new(fr_msg.clone());

    assert_eq!(
        mina_sig.rx,
        fr_sig.R().into_affine().x,
        "Signature commitment x-coordinate must match"
    );
    assert_eq!(
        CurvePoint::generator(),
        PallasGroup::generator(),
        "Generator point must match"
    );

    let mina_chall = message_hash(&mina_pk, mina_sig.rx, mina_msg.clone());
    let chall = frost_bluepallas::PallasPoseidon::challenge(fr_sig.R(), &fr_pk, &fr_msg)?;

    // As of now this should be trivially true because the implementations are the same
    assert_eq!(
        mina_chall,
        chall.to_scalar(),
        "Message Hashes from FROST and Mina do not match"
    );

    let mut ctx = mina_signer::create_legacy::<PallasMessage>(NetworkId::TESTNET);
    println!(
        "Mina verification result: {:?}",
        ctx.verify(&mina_sig, &mina_pk, &mina_msg)
    );

    let ev = message_hash(&mina_pk, mina_sig.rx, mina_msg.clone());

    let sv = CurvePoint::generator()
        .mul_bigint(mina_sig.s.into_bigint())
        .into_affine();
    // Perform addition and infinity check in projective coordinates for performance
    let rv = mina_pk.point().mul_bigint(ev.into_bigint()).neg().add(sv);

    let rv = rv.into_affine();

    assert_eq!(
        rv.x, mina_sig.rx,
        "Signature commitment x-coordinate must match after verification"
    );

    println!("Signature y-coordinate: {:?}", rv.y.into_bigint());

    assert!(
        rv.y.into_bigint().is_even(),
        "Signature commitment y-coordinate must be even"
    );

    println!("Is rv.x and sig.rx match? {}", rv.x == mina_sig.rx);
    println!("Is rv.y even? {}", rv.y.into_bigint().is_even());

    assert!(ctx.verify(&mina_sig, &mina_pk, &mina_msg));
    Ok(())
}

#[test]
fn roi_mina_tx() {
    let rng = rand_core::OsRng;

    // Use trusted dealer to setup public and packages
    let max_signers = 5;
    let min_signers = 3;
    let (_shares, pubkey_package) = frost_bluepallas::keys::generate_with_dealer(
        max_signers,
        min_signers,
        frost_bluepallas::keys::IdentifierList::Default,
        rng,
    )
    .expect("Failed to generate key shares");

    let tx = Transaction::new_payment(
        translate_pk(pubkey_package.verifying_key())
            .expect("failed to translate verifying key to Mina public key"),
        PubKey::from_address("B62qicipYxyEHu7QjUqS7QvBipTs5CzgkYZZZkPoKVYBu6tnDUcE9Zt")
            .expect("invalid address"),
        1729000000000,
        2000000000,
        16,
    )
    .set_valid_until(271828)
    .set_memo_str("Hello Mina!");

    let msg = PallasMessage::new(translate_msg(&tx));
    assert_eq!(
        msg.to_roinput(),
        tx.to_roinput(),
        "ROI Input does not match after translation"
    );
}

#[test]
fn frost_even_commitment() {
    // Iterate 32 times to ensure we have at least one even commitment
    for i in 0..32 {
        let rng = rand_chacha::ChaChaRng::seed_from_u64(i);
        let fr_msg = b"Test message for FROST even commitment".to_vec();
        let (fr_sig, _fr_pk) =
            generate_signature_random(&fr_msg, rng).expect("Failed to generate signature");

        // Ensure the signature commitment y-coordinate is even
        assert!(
            fr_sig
                .R()
                .into_affine()
                .y()
                .expect("Failed to extract y-coord from sig")
                .into_bigint()
                .is_even(),
            "Signature commitment y-coordinate must be even"
        );
    }
}
