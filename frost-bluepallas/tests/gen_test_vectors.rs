/// Generates FROST(Pallas, Poseidon) test vectors and prints them as JSON.
///
/// Reads fixed secrets/randomness from the existing vector files and
/// recomputes every derived value (binding_factor_input, binding_factor,
/// nonce commitments, sig_shares, final sig) using the current serialization.
///
/// Run with:
///   cargo test -p frost-bluepallas gen_test_vectors -- --nocapture
///   cargo test -p frost-bluepallas gen_test_vectors_big_identifier -- --nocapture
mod helpers;

use std::collections::BTreeMap;

use frost_bluepallas::{
    self as frost,
    keys::{KeyPackage, SigningShare},
    round1::{NonceCommitment, SigningCommitments, SigningNonces},
    round2::SignatureShare,
    Ciphersuite, Field, Group, VerifyingKey,
};
use frost_core::round1::Nonce;
use mina_tx::pallas_message::PallasMessage;
use serde_json::{json, Value};

// Use the M-parameterised aliases (M = PallasMessage)
type M = PallasMessage;
type Suite = frost::BluePallas<M>;
type Identifier = frost::Identifier<M>;

fn ser_scalar(s: &frost_core::Scalar<Suite>) -> String {
    hex::encode(<<Suite as Ciphersuite>::Group as Group>::Field::serialize(
        s,
    ))
}

fn regenerate(v: &Value) -> Value {
    let inputs = &v["inputs"];

    // ── fixed inputs ──────────────────────────────────────────────────────────
    let secret_key_bytes = hex::decode(inputs["group_secret_key"].as_str().unwrap()).unwrap();

    let message_bytes = hex::decode(inputs["message"].as_str().unwrap()).unwrap();

    let share_polynomial_coefficients: Vec<frost_core::Scalar<Suite>> = inputs
        ["share_polynomial_coefficients"]
        .as_array()
        .unwrap()
        .iter()
        .map(|val| {
            let bytes = hex::decode(val.as_str().unwrap()).unwrap();
            <<Suite as Ciphersuite>::Group as Group>::Field::deserialize(
                bytes.as_slice().try_into().unwrap(),
            )
            .unwrap()
        })
        .collect();

    let verifying_key = VerifyingKey::<M>::deserialize(
        &hex::decode(inputs["verifying_key_key"].as_str().unwrap()).unwrap(),
    )
    .unwrap();

    let verifying_key_hex = hex::encode(verifying_key.serialize().unwrap());

    let participant_list: Vec<u64> = inputs["participant_list"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_u64().unwrap())
        .collect();

    // ── participant shares ────────────────────────────────────────────────────
    let participant_shares_json = inputs["participant_shares"].as_array().unwrap();
    let min_signers = (share_polynomial_coefficients.len() + 1) as u16;

    let mut key_packages: BTreeMap<Identifier, KeyPackage<M>> = BTreeMap::new();
    let mut all_shares_json = vec![];

    for share in participant_shares_json {
        let i = share["identifier"].as_u64().unwrap() as u16;
        let share_hex = share["participant_share"].as_str().unwrap();
        let share_bytes = hex::decode(share_hex).unwrap();
        let signing_share = SigningShare::<M>::deserialize(share_bytes.as_slice()).unwrap();
        let verifying_share = frost_core::keys::VerifyingShare::from(signing_share);
        let identifier: Identifier = i.try_into().unwrap();
        let kp = KeyPackage::<M>::new(
            identifier,
            signing_share,
            verifying_share,
            verifying_key,
            min_signers,
        );
        key_packages.insert(identifier, kp);
        all_shares_json.push(json!({
            "identifier": i,
            "participant_share": share_hex,
        }));
    }

    // ── round one: reconstruct nonces from stored values ─────────────────────
    let round_one_outputs_json = v["round_one_outputs"]["outputs"].as_array().unwrap();

    struct SignerData {
        identifier: u16,
        hiding_rand_hex: String,
        binding_rand_hex: String,
        hiding_nonce_hex: String,
        binding_nonce_hex: String,
        hiding_commitment_bytes: Vec<u8>,
        binding_commitment_bytes: Vec<u8>,
        nonces: SigningNonces<M>,
        commitments: SigningCommitments<M>,
    }

    let mut signers: Vec<SignerData> = vec![];

    for signer in round_one_outputs_json {
        let i = signer["identifier"].as_u64().unwrap() as u16;

        // Reconstruct nonces from the stored scalar values.
        // (nonce_generate_from_random_bytes is pub(crate) in frost-core)
        let hiding_nonce = Nonce::<Suite>::deserialize(
            &hex::decode(signer["hiding_nonce"].as_str().unwrap()).unwrap(),
        )
        .unwrap();
        let binding_nonce = Nonce::<Suite>::deserialize(
            &hex::decode(signer["binding_nonce"].as_str().unwrap()).unwrap(),
        )
        .unwrap();

        let nonces = SigningNonces::<M>::from_nonces(hiding_nonce, binding_nonce);

        let hiding_comm = NonceCommitment::from(nonces.hiding());
        let binding_comm = NonceCommitment::from(nonces.binding());
        let hiding_commitment_bytes = hiding_comm.serialize().unwrap();
        let binding_commitment_bytes = binding_comm.serialize().unwrap();
        let commitments = SigningCommitments::<M>::new(hiding_comm, binding_comm);

        signers.push(SignerData {
            identifier: i,
            hiding_rand_hex: signer["hiding_nonce_randomness"]
                .as_str()
                .unwrap()
                .to_owned(),
            binding_rand_hex: signer["binding_nonce_randomness"]
                .as_str()
                .unwrap()
                .to_owned(),
            hiding_nonce_hex: signer["hiding_nonce"].as_str().unwrap().to_owned(),
            binding_nonce_hex: signer["binding_nonce"].as_str().unwrap().to_owned(),
            hiding_commitment_bytes,
            binding_commitment_bytes,
            nonces,
            commitments,
        });
    }

    // ── signing package, binding factor inputs/factors ────────────────────────
    let signer_commitments: BTreeMap<Identifier, SigningCommitments<M>> = signers
        .iter()
        .map(|s| {
            let id: Identifier = s.identifier.try_into().unwrap();
            (id, s.commitments)
        })
        .collect();

    let signer_nonces: BTreeMap<Identifier, SigningNonces<M>> = signers
        .iter()
        .map(|s| {
            let id: Identifier = s.identifier.try_into().unwrap();
            (id, s.nonces.clone())
        })
        .collect();

    let signing_package =
        frost_core::SigningPackage::<Suite>::new(signer_commitments, &message_bytes);

    let preimages: BTreeMap<Identifier, Vec<u8>> = signing_package
        .binding_factor_preimages(&verifying_key, &[])
        .unwrap()
        .into_iter()
        .collect();

    let binding_factors: BTreeMap<Identifier, frost_core::Scalar<Suite>> = preimages
        .iter()
        .map(|(id, preimage)| (*id, <Suite as Ciphersuite>::H1(preimage)))
        .collect();

    // ── round two ─────────────────────────────────────────────────────────────
    let mut signature_shares: BTreeMap<Identifier, SignatureShare<M>> = BTreeMap::new();

    for (identifier, nonces) in &signer_nonces {
        let key_package = &key_packages[identifier];
        let sig_share = frost_core::round2::sign(&signing_package, nonces, key_package).unwrap();
        signature_shares.insert(*identifier, sig_share);
    }

    // ── aggregate ─────────────────────────────────────────────────────────────
    let verifying_shares: BTreeMap<Identifier, frost_core::keys::VerifyingShare<Suite>> =
        key_packages
            .iter()
            .map(|(id, kp)| (*id, *kp.verifying_share()))
            .collect();
    let pubkey_package = frost_core::keys::PublicKeyPackage::<Suite>::new(
        verifying_shares,
        verifying_key,
        Some(min_signers),
    );

    let final_sig =
        frost_core::aggregate(&signing_package, &signature_shares, &pubkey_package).unwrap();
    let final_sig_bytes = final_sig.serialize().unwrap();

    // ── output JSON ───────────────────────────────────────────────────────────
    let round_one_json: Vec<Value> = signers
        .iter()
        .map(|s| {
            let id: Identifier = s.identifier.try_into().unwrap();
            json!({
                "identifier": s.identifier,
                "hiding_nonce_randomness": &s.hiding_rand_hex,
                "binding_nonce_randomness": &s.binding_rand_hex,
                "hiding_nonce": &s.hiding_nonce_hex,
                "hiding_nonce_commitment": hex::encode(&s.hiding_commitment_bytes),
                "binding_nonce": &s.binding_nonce_hex,
                "binding_nonce_commitment": hex::encode(&s.binding_commitment_bytes),
                "binding_factor_input": hex::encode(preimages.get(&id).unwrap()),
                "binding_factor": ser_scalar(binding_factors.get(&id).unwrap()),
            })
        })
        .collect();

    let round_two_json: Vec<Value> = signers
        .iter()
        .map(|s| {
            let id: Identifier = s.identifier.try_into().unwrap();
            json!({
                "identifier": s.identifier,
                "sig_share": hex::encode(signature_shares[&id].serialize()),
            })
        })
        .collect();

    json!({
        "config": v["config"],
        "inputs": {
            "group_secret_key": hex::encode(&secret_key_bytes),
            "message": hex::encode(&message_bytes),
            "participant_list": participant_list,
            "participant_shares": all_shares_json,
            "share_polynomial_coefficients": share_polynomial_coefficients
                .iter()
                .map(ser_scalar)
                .collect::<Vec<_>>(),
            "verifying_key_key": verifying_key_hex,
        },
        "round_one_outputs": { "outputs": round_one_json },
        "round_two_outputs": { "outputs": round_two_json },
        "final_output": { "sig": hex::encode(&final_sig_bytes) },
    })
}

#[test]
fn gen_test_vectors() {
    let v: Value = serde_json::from_str(include_str!("helpers/vectors.json").trim()).unwrap();
    let result = regenerate(&v);
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}

#[test]
fn gen_test_vectors_big_identifier() {
    let v: Value =
        serde_json::from_str(include_str!("helpers/vectors-big-identifier.json").trim()).unwrap();
    let result = regenerate(&v);
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
