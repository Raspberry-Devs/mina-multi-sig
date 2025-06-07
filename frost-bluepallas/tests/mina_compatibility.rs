use frost_core as frost;
use frost_bluepallas::{PallasPoseidon, 
    translate::{translate_pk, translate_sig, PallasMessage},
};

use mina_signer::{NetworkId, Signer};

#[test]
fn frost_sign_mina_verify() -> Result<(), Box<dyn std::error::Error>> {
    let rng = rand_core::OsRng;

    let (fr_msg, fr_sig, fr_pk) = frost::tests::ciphersuite_generic::check_sign_with_dealer::<PallasPoseidon, _>(
        rng
    );
    let mina_pk = translate_pk(&fr_pk)?;
    let mina_sig = translate_sig(&fr_sig)?;
    let mina_msg = PallasMessage(fr_msg.clone());

    let mut ctx = mina_signer::create_legacy::<PallasMessage>(NetworkId::TESTNET);
    assert!(ctx.verify(&mina_sig, &mina_pk, &mina_msg));
    Ok(())
}