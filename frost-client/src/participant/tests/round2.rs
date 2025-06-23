#![cfg(test)]

use std::{collections::BTreeMap, io::BufWriter};

use frost_bluepallas as frost;

use frost::Identifier;
use frost::{
    keys::{KeyPackage, SigningShare, VerifyingShare},
    round1::{self, NonceCommitment, SigningCommitments},
    round2::SignatureShare,
    SigningPackage, VerifyingKey,
};
use crate::api::SendSigningPackageArgs;
// use frostd::SendSigningPackageArgs;
use crate::participant::comms::cli::CLIComms;
use crate::participant::round2::print_values_round_2;
use crate::participant::round2::{generate_signature, round_2_request_inputs, Round2Config};
use rand::thread_rng;

const PUBLIC_KEY: &str = "81646bb7849d7ad5ac12eae2c2b1dc848cfedceed3518a795f5ca09163a3dd2d00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
const GROUP_PUBLIC_KEY: &str = "0d3037389dfcc11f0ece67160d96ea7a0c7fec71cfb93dd11e22a956682e363680000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
const SIGNING_SHARE: &str = "1932bede7d78fc6792031bf82b1985b7a398bd75033748c19bc27f56edabf30a";
const HIDING_COMMITMENT_2: &str = "a6004b8d59349a0b1694203b3f033e6aebf8b8e630691c801800bd81eda1e53980000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
const BINDING_COMMITMENT_2: &str = "22247e68ca705360b2878e7cf1dbce40b34fe82a4c18fb04dc45f016856eb70500000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";

pub fn nonce_commitment(input: &str) -> NonceCommitment {
    NonceCommitment::deserialize(&hex::decode(input).unwrap()).unwrap()
}

#[tokio::test]
async fn check_valid_round_2_inputs() {
    // TODO: refactor

    // Generate commitments

    let mut comms = CLIComms::new();
    let my_signer_commitments = SigningCommitments::new(
        nonce_commitment("9e873f63a9debeb378dc619208d556b5f7896237a89ec83ce2b789c314aa730900000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"),
        nonce_commitment("162475dd0abdbd8da8f7d8a0eb63059bb43d2198da01d8f15760bc32c020d70b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"),
    );

    let signer_commitments_3 = SigningCommitments::new(
        nonce_commitment("08a07c1fa33f276452622bd29c2de5ccedcf9b532e6b23c4dc414251be86050900000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"),
        nonce_commitment("a63b131aa847eb1ccb963617636b52265d115dfe247b5ae964817eb5776b673a80000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"),
    );

    let mut signer_commitments = BTreeMap::new();
    signer_commitments.insert(Identifier::try_from(1).unwrap(), my_signer_commitments);
    signer_commitments.insert(Identifier::try_from(3).unwrap(), signer_commitments_3);

    let message = hex::decode("74657374").unwrap();

    let signing_package = r#"{"header":{"version":0,"ciphersuite":"bluepallas"},"signing_commitments":{"0100000000000000000000000000000000000000000000000000000000000000":{"header":{"version":0,"ciphersuite":"bluepallas"},"hiding":"9e873f63a9debeb378dc619208d556b5f7896237a89ec83ce2b789c314aa730900000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000","binding":"162475dd0abdbd8da8f7d8a0eb63059bb43d2198da01d8f15760bc32c020d70b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"},"0300000000000000000000000000000000000000000000000000000000000000":{"header":{"version":0,"ciphersuite":"bluepallas"},"hiding":"08a07c1fa33f276452622bd29c2de5ccedcf9b532e6b23c4dc414251be86050900000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000","binding":"a63b131aa847eb1ccb963617636b52265d115dfe247b5ae964817eb5776b673a80000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"}},"message":"74657374"}"#;

    let expected = Round2Config {
        signing_package: SigningPackage::new(signer_commitments, &message),
        randomizer: None,
    };

    let mut buf = BufWriter::new(Vec::new());

    let input = format!("{}\n", signing_package);
    let mut valid_input = input.as_bytes();

    let round_2_config = round_2_request_inputs(
        &mut comms,
        &mut valid_input,
        &mut buf,
        my_signer_commitments,
        Identifier::try_from(1).unwrap(),
        false,
    )
    .await;

    assert!(round_2_config.is_ok());
    assert_eq!(
        expected.signing_package,
        round_2_config.unwrap().signing_package[0]
    )
}

// TODO: test for invalid inputs

#[tokio::test]
async fn check_sign() {
    let key_package = KeyPackage::new(
        Identifier::try_from(1).unwrap(),
        SigningShare::deserialize(&hex::decode(SIGNING_SHARE).unwrap()).unwrap(),
        VerifyingShare::deserialize(&hex::decode(PUBLIC_KEY).unwrap()).unwrap(),
        VerifyingKey::deserialize(&hex::decode(GROUP_PUBLIC_KEY).unwrap()).unwrap(),
        2,
    );

    let mut rng = thread_rng();

    // TODO: Nonce doesn't seem to be exported. Look into this to improve these tests
    let (nonces, my_commitments) = round1::commit(
        &SigningShare::deserialize(&hex::decode(SIGNING_SHARE).unwrap()).unwrap(),
        &mut rng,
    );

    let signer_commitments_2 = SigningCommitments::new(
        NonceCommitment::deserialize(&hex::decode(HIDING_COMMITMENT_2).unwrap()).unwrap(),
        NonceCommitment::deserialize(&hex::decode(BINDING_COMMITMENT_2).unwrap()).unwrap(),
    );

    let mut signer_commitments = BTreeMap::new();
    signer_commitments.insert(Identifier::try_from(1).unwrap(), my_commitments);
    signer_commitments.insert(Identifier::try_from(2).unwrap(), signer_commitments_2);

    let message =
        &hex::decode("15d21ccd7ee42959562fc8aa63224c8851fb3ec85a3faf66040d380fb9738673").unwrap();

    let signing_package = SigningPackage::new(signer_commitments, message);

    let config = SendSigningPackageArgs {
        signing_package: vec![signing_package],
        randomizer: vec![],
        aux_msg: vec![],
    };

    let signature = generate_signature(config, &key_package, &nonces);

    assert!(signature.is_ok()) // TODO: Should be able to test this more specifically when I remove randomness from the test
}

#[tokio::test]
async fn check_print_values_round_2() {
    let mut buf = BufWriter::new(Vec::new());

    const SIGNATURE_SHARE: &str =
        "44055c54d0604cbd006f0d1713a22474d7735c5e8816b1878f62ca94bf105900";
    let signature_response =
        SignatureShare::deserialize(&hex::decode(SIGNATURE_SHARE).unwrap()).unwrap();

    print_values_round_2(signature_response, &mut buf).unwrap();

    let log = "Please send the following to the Coordinator\nSignatureShare:\n{\"header\":{\"version\":0,\"ciphersuite\":\"bluepallas\"},\"share\":\"44055c54d0604cbd006f0d1713a22474d7735c5e8816b1878f62ca94bf105900\"}\n";

    let out = String::from_utf8(buf.into_inner().unwrap()).unwrap();

    assert_eq!(out, log);
}
