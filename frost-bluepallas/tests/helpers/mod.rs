// Required since each integration test is compiled as a separated crate,
// and each one uses only part of the module.
#![allow(dead_code)]

pub mod samples;

use frost_bluepallas::{
    pallas_message::{translate_pk, translate_sig, PallasMessage},
    BluePallas,
};
use mina_signer::{NetworkId, Signer};

// #[cfg(test)]
pub fn verify_signature(
    msg: &[u8],
    group_signature: frost_core::Signature<BluePallas>,
    group_pubkey: frost_core::VerifyingKey<BluePallas>,
) {
    // TODO remove the result type from the translate api. It's always okay
    let sig = translate_sig(&group_signature).unwrap();

    let pub_key = translate_pk(&group_pubkey).unwrap();
    // Check that signature validation has the expected result.

    let mut ctx = mina_signer::create_legacy::<PallasMessage>(NetworkId::TESTNET);
    let pallas_message = PallasMessage::deserialize(msg)
        .unwrap_or_else(|_| PallasMessage::from_raw_bytes_default(msg));
    assert!(ctx.verify(&sig, &pub_key, &pallas_message));
}
