use ark_ec::AffineRepr;
use frost_bluepallas::{
    hasher::{message_hash, PallasMessage},
    translate::{translate_pk, translate_sig},
    PallasGroup, PallasPoseidon,
};
use frost_core::{self as frost, Ciphersuite, Group};

use mina_signer::{CurvePoint, NetworkId, Signer};
use rand_core::SeedableRng;

#[test]
fn frost_sign_mina_verify() -> Result<(), Box<dyn std::error::Error>> {
    let rng = rand_chacha::ChaChaRng::seed_from_u64(0);

    let (fr_msg, fr_sig, fr_pk) =
        frost::tests::ciphersuite_generic::check_sign_with_dealer::<PallasPoseidon, _>(rng);

    let res = frost_bluepallas::PallasPoseidon::verify_signature(&fr_msg, &fr_sig, &fr_pk);
    assert!(res.is_ok(), "FROST correctly verifies signature");

    let mina_pk = translate_pk(&fr_pk)?;
    let mina_sig = translate_sig(&fr_sig)?;
    let mina_msg = PallasMessage(fr_msg.clone());

    assert_eq!(
        mina_sig.rx,
        fr_sig.R().x,
        "Signature commitment x-coordinate must match"
    );
    assert_eq!(
        CurvePoint::generator(),
        PallasGroup::generator(),
        "Generator point must match"
    );

    let mina_chall = message_hash(&mina_pk, mina_sig.rx, &mina_msg);
    let chall = frost_bluepallas::PallasPoseidon::challenge(fr_sig.R(), &fr_pk, &fr_msg)?;

    // As of now this should be trivially true because the implementations are the same
    assert_eq!(
        mina_chall,
        chall.to_scalar(),
        "Message Hashes from FROST and Mina do not match"
    );

    let mut ctx = mina_signer::create_legacy::<PallasMessage>(NetworkId::TESTNET);
    assert!(ctx.verify(&mina_sig, &mina_pk, &mina_msg));
    Ok(())
}
