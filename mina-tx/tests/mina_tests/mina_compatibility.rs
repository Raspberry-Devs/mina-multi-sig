use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::fields::PrimeField;
use ark_ff::BigInteger;
use frost_bluepallas::{BluePallas, PallasGroup};
use frost_core::{Ciphersuite, Group};
use mina_tx::{
    legacy_tx::LegacyTransaction,
    pallas_message::{message_hash, translate_pk, translate_sig, PallasMessage},
    TransactionEnvelope,
};

use mina_hasher::Hashable;
use mina_signer::{CurvePoint, PubKey, Signer};
use mina_tx::NetworkId;
use rand_core::SeedableRng;

use std::ops::{Add, Neg};

use frost_bluepallas::signing_utilities::generate_signature_random;
type Suite = BluePallas<PallasMessage>;

#[test]
fn frost_sign_mina_verify() -> Result<(), Box<dyn std::error::Error>> {
    // Esnure that the FROST implementation can sign a message and Mina can verify it

    let network_id = NetworkId::Testnet;

    let rng = rand_chacha::ChaChaRng::seed_from_u64(100);
    let tx = TransactionEnvelope::new_legacy(
        network_id.clone(),
        LegacyTransaction::new_payment(
            PubKey::from_address("B62qqM5PCrqATE21oWhkY4UkrzT9XpUjsdgMk5MBbEmuAjPBdjN91mZ")
                .expect("invalid address"),
            PubKey::from_address("B62qqM5PCrqATE21oWhkY4UkrzT9XpUjsdgMk5MBbEmuAjPBdjN91mZ")
                .expect("invalid address"),
            1000000,
            20000,
            16,
        ),
    );
    let fr_msg = tx.to_pallas_message().serialize().unwrap();

    let (fr_sig, fr_pk) = generate_signature_random::<PallasMessage, _>(&fr_msg, rng)?;

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

    let res = BluePallas::<PallasMessage>::verify_signature(&fr_msg, &fr_sig, &fr_pk);
    assert!(res.is_ok(), "FROST correctly verifies signature");

    let mina_pk = translate_pk(&fr_pk)?;
    let mina_sig = translate_sig(&fr_sig)?;
    let mina_msg = tx.to_pallas_message();

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

    let mina_chall = message_hash::<PallasMessage>(
        &mina_pk,
        mina_sig.rx,
        mina_msg.clone(),
        network_id.clone(),
        true,
    )?;
    let chall = BluePallas::<PallasMessage>::challenge(fr_sig.R(), &fr_pk, &fr_msg)?;

    // As of now this should be trivially true because the implementations are the same
    assert_eq!(
        mina_chall,
        chall.to_scalar(),
        "Message Hashes from FROST and Mina do not match"
    );

    let mut ctx = mina_signer::create_legacy::<PallasMessage>(NetworkId::Testnet);
    println!(
        "Mina verification result: {:?}",
        ctx.verify(&mina_sig, &mina_pk, &mina_msg)
    );

    let ev = message_hash(&mina_pk, mina_sig.rx, mina_msg.clone(), network_id, true)?;

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
    let (_shares, pubkey_package) =
        frost_bluepallas::keys::generate_with_dealer::<PallasMessage, _>(
            max_signers,
            min_signers,
            frost_bluepallas::keys::IdentifierList::Default,
            rng,
        )
        .expect("Failed to generate key shares");

    let tx = LegacyTransaction::new_payment(
        translate_pk(pubkey_package.verifying_key())
            .expect("failed to translate verifying key to Mina public key"),
        PubKey::from_address("B62qicipYxyEHu7QjUqS7QvBipTs5CzgkYZZZkPoKVYBu6tnDUcE9Zt")
            .expect("invalid address"),
        1729000000000,
        2000000000,
        16,
    )
    .set_valid_until(271828)
    .set_memo_str("Hello Mina!")
    .unwrap();

    let tx_env = TransactionEnvelope::new_legacy(NetworkId::Testnet, tx.clone());

    let msg = tx_env.to_pallas_message();
    assert_eq!(
        msg.to_roinput(),
        tx.to_roinput(),
        "ROI Input does not match after translation"
    );
}

#[test]
fn frost_even_commitment() {
    // Iterate 32 times to ensure we have at least one even commitment
    for i in 0..256 {
        let rng = rand_chacha::ChaChaRng::seed_from_u64(i);
        let fr_msg = b"Test message for FROST even commitment".to_vec();
        let (fr_sig, _fr_pk): (
            frost_core::Signature<Suite>,
            frost_core::VerifyingKey<Suite>,
        ) = generate_signature_random::<PallasMessage, _>(&fr_msg, rng)
            .expect("Failed to generate signature");

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

#[test]
fn delegation_mina_compatibility() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{
        "to": "B62qkcvM4DZE7k23ZHMLt1uaMVcixuxxuyz1XNJNCLkFbitDdUHxWs1",
        "from": "B62qrzao6tj1TsWcUwvYRbzCkiaqQ5wNJo3zfH37T8b5w9EmEt1jXoV",
        "fee": "10000000",
        "nonce": "0",
        "memo": "Hello Mina x FROST NETWORK",
        "valid_until": "4294967295",
        "tag": [
            false,
            false,
            true
        ]
    }"#;
    // We want to now deserialize into a transaction
    let tx: LegacyTransaction = serde_json::from_str(json).unwrap();
    let tx_env = TransactionEnvelope::new_legacy(NetworkId::Testnet, tx);
    let msg = tx_env.to_pallas_message().serialize().unwrap();

    for _ in 0..64 {
        let rng = rand_core::OsRng;

        let (fr_sig, fr_pk) =
            generate_signature_random(&msg, rng).expect("Failed to generate signature");

        let mina_sig = translate_sig(&fr_sig)?;
        let mina_pk = translate_pk(&fr_pk)?;

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

        let mut ctx = mina_signer::create_legacy::<TransactionEnvelope>(NetworkId::Testnet);
        assert!(ctx.verify(&mina_sig, &mina_pk, &tx_env));
    }

    Ok(())
}

#[test]
fn translate_sig_rejects_identity_commitment() {
    use mina_tx::errors::MinaTxError;
    use num_traits::Zero;

    // A signature whose commitment R is the point at infinity must be rejected rather than
    // silently reading R.x = 0 from the affine coordinates. The scalar `z` is irrelevant
    // to this check, so any value works.
    let identity_sig = frost_core::Signature::<Suite>::new(
        frost_bluepallas::PallasGroup::identity(),
        mina_signer::ScalarField::zero(),
    );

    assert!(
        matches!(
            translate_sig(&identity_sig),
            Err(MinaTxError::MalformedGroupElement)
        ),
        "translate_sig must reject the identity commitment"
    );
}

/// Builds a curve point whose affine y-coordinate has the requested parity.
///
/// For any non-identity point `P = (x, y)` over the Pallas base field, `-P = (x, -y)`,
/// and exactly one of `y` / `-y` is odd (the base field modulus is odd, so `y` and
/// `p - y` differ in parity). We therefore start from the generator and negate it when
/// its y-coordinate does not already have the parity we want.
fn generator_with_y_parity(want_odd: bool) -> frost_core::Element<Suite> {
    let g = PallasGroup::generator();
    let y_is_odd = g
        .into_affine()
        .y()
        .expect("generator must have a y-coordinate")
        .into_bigint()
        .is_odd();

    if y_is_odd == want_odd {
        g
    } else {
        g.neg()
    }
}

#[test]
fn translate_sig_rejects_odd_y_commitment() {
    use mina_tx::errors::MinaTxError;
    use num_traits::Zero;

    // Mina stores only R.x and reconstructs R with an even y-coordinate. A FROST signature
    // whose R has an odd y-coordinate would silently convert into a different (invalid) Mina
    // signature if we only kept R.x. Such a signature must be rejected instead.
    let odd_r = generator_with_y_parity(true);
    assert!(
        odd_r
            .into_affine()
            .y()
            .expect("R must have a y-coordinate")
            .into_bigint()
            .is_odd(),
        "test fixture must have an odd y-coordinate"
    );

    let odd_sig = frost_core::Signature::<Suite>::new(odd_r, mina_signer::ScalarField::zero());

    assert!(
        matches!(
            translate_sig(&odd_sig),
            Err(MinaTxError::MalformedGroupElement)
        ),
        "translate_sig must reject a commitment R with an odd y-coordinate"
    );

    // The `Sig: TryFrom<FrSig>` conversion in bluepallas_compat.rs must reject it too.
    assert!(
        matches!(
            mina_tx::Sig::try_from(odd_sig),
            Err(MinaTxError::MalformedGroupElement)
        ),
        "Sig::try_from must reject a commitment R with an odd y-coordinate"
    );
}

#[test]
fn translate_sig_accepts_even_y_commitment() {
    use num_traits::Zero;

    // Sanity check that the parity guard is specific to odd y-coordinates and does not
    // blanket-reject otherwise valid commitments with an even y-coordinate.
    let even_r = generator_with_y_parity(false);
    assert!(
        even_r
            .into_affine()
            .y()
            .expect("R must have a y-coordinate")
            .into_bigint()
            .is_even(),
        "test fixture must have an even y-coordinate"
    );

    let even_sig = frost_core::Signature::<Suite>::new(even_r, mina_signer::ScalarField::zero());

    let mina_sig = translate_sig(&even_sig).expect("even y-coordinate commitment must be accepted");
    assert_eq!(
        mina_sig.rx,
        even_r.into_affine().x,
        "translate_sig must keep R.x for an even y-coordinate commitment"
    );

    assert!(
        mina_tx::Sig::try_from(even_sig).is_ok(),
        "Sig::try_from must accept a commitment R with an even y-coordinate"
    );
}

#[test]
fn message_hash_rejects_identity_public_key() {
    use mina_tx::errors::MinaTxError;
    use num_traits::Zero;

    // A public key at the point at infinity must be rejected rather than silently reading
    // pub_key_x = pub_key_y = 0.
    let identity_pk = PubKey::from_point_unsafe(CurvePoint::zero());

    let tx = LegacyTransaction::new_payment(
        PubKey::from_address("B62qqM5PCrqATE21oWhkY4UkrzT9XpUjsdgMk5MBbEmuAjPBdjN91mZ")
            .expect("invalid address"),
        PubKey::from_address("B62qqM5PCrqATE21oWhkY4UkrzT9XpUjsdgMk5MBbEmuAjPBdjN91mZ")
            .expect("invalid address"),
        1000000,
        20000,
        16,
    );
    let msg = TransactionEnvelope::new_legacy(NetworkId::Testnet, tx).to_pallas_message();

    let result = message_hash(
        &identity_pk,
        mina_signer::BaseField::zero(),
        msg,
        NetworkId::Testnet,
        true,
    );

    assert!(
        matches!(result, Err(MinaTxError::MalformedGroupElement)),
        "message_hash must reject an identity public key"
    );
}
