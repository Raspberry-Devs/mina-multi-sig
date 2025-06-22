// Required since each integration test is compiled as a separated crate,
// and each one uses only part of the module.
#![allow(dead_code)]

pub mod samples;

use frost_bluepallas::{
    hasher::PallasMessage,
    translate::{translate_pk, translate_sig},
    PallasPoseidon,
};
use mina_signer::{NetworkId, Signer};

// #[cfg(test)]
pub fn verify_signature(
    msg: &[u8],
    group_signature: frost_core::Signature<PallasPoseidon>,
    group_pubkey: frost_core::VerifyingKey<PallasPoseidon>,
) {
    // TODO remove the result type from the translate api. It's always okay
    let sig = translate_sig(&group_signature).unwrap();

    let pub_key = translate_pk(&group_pubkey).unwrap();
    // Check that signature validation has the expected result.

    let mut ctx = mina_signer::create_legacy::<PallasMessage>(NetworkId::TESTNET);
    assert!(ctx.verify(&sig, &pub_key, &PallasMessage::new(msg.into())));
}
