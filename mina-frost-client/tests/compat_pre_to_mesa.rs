//! Compatibility test:
//! - create participants and group config with pre-Mesa binary
//! - sign a Mesa transaction with Mesa binary using those same config files

mod helpers;

use helpers::{
    build_client_binary, form_group_with_dkg, greet_participants, group_keys_from_config,
    introduce_participant, network_to_cli_arg, parse_and_verify_signature, sign_with_binary,
    start_frostd, write_json_str_pretty, ChildGuard, CliParticipant, SigningParticipant,
};
use mina_tx::zkapp_tx::test_vectors::{get_zkapp_test_vectors, ZkAppTestVector};
use mina_tx::{TransactionEnvelope, TransactionKind};
use std::{fs, path::Path};
use tempfile::TempDir;

const SERVER_URL: &str = "localhost:2744";
const SIG_FILE: &str = "signature.json";
const THRESHOLD: usize = 2;
const NUM_PARTICIPANTS: usize = 3;

fn write_vector_message(message_path: &Path, vector: &ZkAppTestVector) -> String {
    let envelope: TransactionEnvelope = vector.clone().into();
    let network_cli_arg = network_to_cli_arg(&envelope.network_id());

    let zkapp = match envelope.inner() {
        TransactionKind::ZkApp(zkapp) => zkapp,
        _ => panic!("compatibility vector must be a zkApp transaction"),
    };
    let json = serde_json::to_string(&zkapp).expect("failed serializing compatibility payload");

    write_json_str_pretty(message_path, &json)
        .expect("failed writing compatibility transaction payload");

    network_cli_arg
}

fn run_compat_case(
    case_name: &str,
    group_binary: &Path,
    sign_binary: &Path,
    root: &Path,
    vector: &ZkAppTestVector,
) {
    let workdir = root.join(case_name);
    fs::create_dir_all(&workdir)
        .unwrap_or_else(|_| panic!("failed to create case directory {}", workdir.display()));

    let participants = (0..NUM_PARTICIPANTS)
        .map(|i| introduce_participant(group_binary, &workdir, &format!("p{}", i)))
        .collect::<Vec<CliParticipant>>();

    greet_participants(group_binary, &workdir, &participants);

    form_group_with_dkg(
        group_binary,
        &workdir,
        &participants,
        THRESHOLD,
        SERVER_URL,
        &format!("compat-{case_name}"),
    )
    .unwrap_or_else(|_| panic!("failed to form group for case {case_name}"));

    let (group_pk_hex, group_pk_mina) =
        group_keys_from_config(group_binary, &workdir, &participants[0].toml);

    let message_path = workdir.join(format!("{}.json", vector.name));
    let network_id = write_vector_message(&message_path, vector);

    let signing_participants = participants
        .iter()
        .map(|p| SigningParticipant {
            config_path: p.toml.clone(),
            pubkey_hex: p.pubkey_hex.clone(),
        })
        .collect::<Vec<_>>();

    sign_with_binary(
        sign_binary,
        &workdir,
        &group_pk_hex,
        &message_path,
        SIG_FILE,
        &network_id,
        SERVER_URL,
        THRESHOLD,
        &signing_participants,
    );

    let signature_path = workdir.join(SIG_FILE);
    assert!(
        signature_path.exists(),
        "[{}] signature file was not produced at {}",
        case_name,
        signature_path.display()
    );
    parse_and_verify_signature(
        &group_pk_mina,
        &message_path,
        &signature_path,
        &network_id,
        false,
    );
}

#[test]
fn compatibility_matrix_between_pre_and_mesa_binaries() {
    let pre_binary = build_client_binary(env!("CARGO_MANIFEST_DIR"), Some("pre-compat"), false);
    let mesa_binary = build_client_binary(env!("CARGO_MANIFEST_DIR"), Some("mesa-compat"), true);

    let temp_dir = TempDir::new().expect("failed to create temp compatibility test directory");
    let root = temp_dir.path();

    let _frostd = ChildGuard(start_frostd(root).expect("failed to start frostd"));

    let vectors = get_zkapp_test_vectors();
    assert!(
        !vectors.is_empty(),
        "expected at least one zkapp test vector for active feature"
    );
    let vector = vectors[0].clone();

    let mesa_active = mina_tx::zkapp_tx::IS_MESA_HARDFORK;
    let (case_name, group_binary, sign_binary) = if mesa_active {
        (
            format!("pre_to_mesa__{}", vector.name),
            pre_binary.as_path(),
            mesa_binary.as_path(),
        )
    } else {
        (
            format!("mesa_to_pre__{}", vector.name),
            mesa_binary.as_path(),
            pre_binary.as_path(),
        )
    };

    run_compat_case(case_name.as_str(), group_binary, sign_binary, root, &vector);
}
