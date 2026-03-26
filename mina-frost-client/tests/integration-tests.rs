//! This file provides end to end tests for the entire project
//! It tests if the results of mina-frost-client agrees with `mina-signer` (cross-package test)
//! It uses mina-frost-client to generate signatures for given pair of (threshold, max-signers),
//! all the way from DKG, spawning subprocesses which interact with each other over the frostd
//! server. through the mina-frost-client. Finally we read the generated signature and verify that it
//! agrees with the group public key according to the `mina-signer` crate

mod helpers;

use helpers::{
    binary_name, build_client_binary, form_group_with_dkg, greet_participants,
    group_keys_from_config, introduce_participant, network_to_cli_arg, parse_and_verify_signature,
    sign_with_binary, start_frostd, write_json_str_pretty, CliParticipant, SigningParticipant,
};
use lazy_static::lazy_static;
use mina_tx::zkapp_tx::test_vectors::get_zkapp_test_vectors;
use mina_tx::{TransactionEnvelope, TransactionKind};
use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf};

lazy_static! {
    static ref binary_path: PathBuf = PathBuf::from(format!(
        "{}/../target/release/{}",
        env!("CARGO_MANIFEST_DIR"),
        binary_name()
    ));
    static ref working_dir: PathBuf =
        PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/assets"));
    static ref message_path: PathBuf = PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/signing_example/message.json"
    ));
}

const SIG_FILE: &str = "signature.json";
const NETWORK_ID: &str = "testnet";

/// Dump ZKApp transaction test vectors to files, return pathnames
fn write_zkapp_tx_files() -> Result<Vec<(String, String)>> {
    let test_vectors = get_zkapp_test_vectors();

    let mut paths: Vec<(String, String)> = Vec::with_capacity(test_vectors.len());
    for (index, tv) in test_vectors.into_iter().enumerate() {
        let path = working_dir.join(format!("zkapp_tx_{}.json", index));
        let tx_env: TransactionEnvelope = tv.into();
        let network_id = network_to_cli_arg(&tx_env.network_id());
        paths.push((path.to_str().unwrap().to_string(), network_id));

        let inner = match tx_env.inner() {
            TransactionKind::ZkApp(inner) => inner,
            _ => panic!("Expected ZKApp transaction"),
        };

        let json = serde_json::to_string(&inner)
            .map_err(|e| std::io::Error::other(format!("failed serializing zkapp tx: {e}")))?;
        write_json_str_pretty(&path, &json)?;
    }
    Ok(paths)
}

/// Sign and obtain the signature
/// We sign the `TransactionEnvelope` as given by `sign_message_path`
fn sign(
    participants: &[CliParticipant],
    group_pk: &str,
    threshold: usize,
    sign_message_path: &str,
    network_id: &str,
) -> Result<()> {
    let signing_participants = participants
        .iter()
        .map(|p| SigningParticipant {
            config_path: p.toml.clone(),
            pubkey_hex: p.pubkey_hex.clone(),
        })
        .collect::<Vec<_>>();

    sign_with_binary(
        &binary_path,
        &working_dir,
        group_pk,
        Path::new(sign_message_path),
        SIG_FILE,
        network_id,
        "localhost:2744",
        threshold,
        &signing_participants,
    );

    Ok(())
}

/// This test will iterate through all zkapp_test_vectors and verify that
/// they will sign correctly
fn cross_package_test(threshold: usize, max_signers: usize) -> Result<()> {
    let mut server_process = setup()?;
    let zkapp_message_paths = write_zkapp_tx_files()?;

    let participants = (0..max_signers)
        .map(|x| introduce_participant(&binary_path, &working_dir, &x.to_string()))
        .collect::<Vec<CliParticipant>>();

    greet_participants(&binary_path, &working_dir, &participants);

    form_group_with_dkg(
        &binary_path,
        &working_dir,
        &participants,
        threshold,
        "localhost:2744",
        "Raspberry Devs",
    )?;

    let (group_pk_hex, group_pk_mina) =
        group_keys_from_config(&binary_path, &working_dir, &participants[0].toml);
    println!("group public key: {}", group_pk_mina);

    // Message paths for signing as (path, is_legacy, network_id)
    let mut message_paths = vec![(
        message_path.to_str().unwrap().to_string(),
        true,
        NETWORK_ID.to_string(),
    )];
    message_paths.extend(
        zkapp_message_paths
            .into_iter()
            .map(|(p, network_id)| (p, false, network_id)),
    );

    // Iterate through each transaction message path, sign+verify
    for (msg, flag, network_id) in message_paths.iter() {
        sign(&participants, &group_pk_hex, threshold, msg, network_id)?;
        parse_and_verify_signature(
            &group_pk_mina,
            Path::new(msg),
            &working_dir.join(SIG_FILE),
            network_id,
            *flag,
        );
    }

    server_process.kill()?;
    Ok(())
}

#[test]
fn permute_cross_package_test() -> Result<()> {
    // TODO for now there's just one cause it's quite slow
    cross_package_test(5, 10)
}

fn setup() -> Result<std::process::Child> {
    // Clean up generated directory if it exists
    if working_dir.exists() {
        println!("Cleaning up existing generated directory...");
        fs::remove_dir_all(working_dir.clone())?;
    }
    // Create directory for generated files
    fs::create_dir_all(working_dir.clone())?;

    // compile release binary
    let built_binary = build_client_binary(
        env!("CARGO_MANIFEST_DIR"),
        None,
        mina_tx::zkapp_tx::IS_MESA_HARDFORK,
    );
    assert!(
        built_binary.exists(),
        "release client binary does not exist at {}",
        built_binary.display()
    );

    // Start frostd server in the background
    start_frostd(&working_dir)
}
