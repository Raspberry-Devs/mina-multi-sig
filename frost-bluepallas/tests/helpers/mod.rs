// Required since each integration test is compiled as a separated crate,
// and each one uses only part of the module.
#![allow(dead_code)]

pub mod samples;
pub mod types;

use frost_bluepallas::BluePallas;
use mina_signer::{NetworkId, Signer};
use mina_tx::pallas_message::{translate_pk, translate_sig, PallasMessage};

type Suite = BluePallas<PallasMessage>;

// #[cfg(test)]
pub fn verify_signature(
    msg: &[u8],
    group_signature: frost_core::Signature<Suite>,
    group_pubkey: frost_core::VerifyingKey<Suite>,
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
