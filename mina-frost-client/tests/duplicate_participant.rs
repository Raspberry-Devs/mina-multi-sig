mod helpers;

use helpers::{
    binary_name, build_client_binary, form_group_with_dkg, get_session_id, greet_participants,
    group_keys_from_config, introduce_participant, run_cli_spawn_piped, start_frostd,
    CliParticipant, SigningParticipant,
};
use lazy_static::lazy_static;
use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf};
use std::process::{Child, Output};
use std::thread;
use std::time::Duration;

lazy_static! {
    static ref binary_path: PathBuf = PathBuf::from(format!(
        "{}/../target/release/{}",
        env!("CARGO_MANIFEST_DIR"),
        binary_name()
    ));
    static ref working_dir: PathBuf = PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/assets-duplicate"
    ));
    static ref message_path: PathBuf = PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/signing_example/message.json"
    ));
}

const SIG_FILE: &str = "signature.json";
const NETWORK_ID: &str = "testnet";

fn setup() -> Result<std::process::Child> {
    if working_dir.exists() {
        fs::remove_dir_all(working_dir.clone())?;
    }
    fs::create_dir_all(working_dir.clone())?;

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

    start_frostd(&working_dir)
}

fn participant_args(
    participant: &SigningParticipant,
    server_url: &str,
    group_pk_hex: &str,
    session_id: &str,
) -> Vec<String> {
    vec![
        "participant".to_string(),
        "-c".to_string(),
        participant.config_path.clone(),
        "-s".to_string(),
        server_url.to_string(),
        "--group".to_string(),
        group_pk_hex.to_string(),
        "-S".to_string(),
        session_id.to_string(),
        "-y".to_string(),
    ]
}

fn sign_with_duplicate_participant(
    binary: &Path,
    cwd: &Path,
    group_pk_hex: &str,
    msg_path: &Path,
    sig_file: &str,
    network_id: &str,
    server_url: &str,
    threshold: usize,
    participants: &[SigningParticipant],
) -> Output {
    assert!(participants.len() >= threshold && threshold > 0);

    let mut coord_args = vec![
        "coordinator".to_string(),
        "-c".to_string(),
        participants[0].config_path.clone(),
        "-s".to_string(),
        server_url.to_string(),
        "--group".to_string(),
        group_pk_hex.to_string(),
        "-m".to_string(),
        msg_path.to_string_lossy().to_string(),
        "-o".to_string(),
        sig_file.to_string(),
        "-n".to_string(),
        network_id.to_string(),
    ];
    for participant in participants.iter().take(threshold) {
        coord_args.push("-S".to_string());
        coord_args.push(participant.pubkey_hex.clone());
    }

    let coordinator = run_cli_spawn_piped(binary, cwd, &coord_args);

    let session_id = get_session_id(
        binary,
        cwd,
        &participants[0].config_path,
        server_url,
        group_pk_hex,
    )
    .expect("no signing session appeared after coordinator started");

    let participant_children: Vec<Child> = participants
        .iter()
        .take(threshold)
        .map(|p| {
            let args = participant_args(p, server_url, group_pk_hex, &session_id);
            run_cli_spawn_piped(binary, cwd, &args)
        })
        .collect();

    // Give them time to complete the Noise handshake and send their commitments
    thread::sleep(Duration::from_millis(1500));

    // Spawn a duplicate of the first participant (fresh Noise context, same session)
    let dupe_args = participant_args(&participants[0], server_url, group_pk_hex, &session_id);
    let mut dupe = run_cli_spawn_piped(binary, cwd, &dupe_args);
    thread::sleep(Duration::from_secs(2));
    let _ = dupe.kill();
    let _ = dupe.wait();

    for child in participant_children {
        let _ = child.wait_with_output();
    }

    coordinator
        .wait_with_output()
        .expect("coordinator subprocess did not exit")
}

/// Verifies that when a participant accidentally re-joins a session with a fresh
/// Noise context, the coordinator warns and continues rather than crashing, and
/// the signing session completes successfully.
#[test]
fn duplicate_participant_handled_gracefully() -> Result<()> {
    let mut server_process = setup()?;

    let participants = (0..3)
        .map(|x| introduce_participant(&binary_path, &working_dir, &x.to_string()))
        .collect::<Vec<CliParticipant>>();

    greet_participants(&binary_path, &working_dir, &participants);

    form_group_with_dkg(
        &binary_path,
        &working_dir,
        &participants,
        2,
        "localhost:2744",
        "Duplicate Test Group",
    )?;

    let (group_pk_hex, _) =
        group_keys_from_config(&binary_path, &working_dir, &participants[0].toml);

    let signing_participants: Vec<SigningParticipant> = participants
        .iter()
        .map(|p| SigningParticipant {
            config_path: p.toml.clone(),
            pubkey_hex: p.pubkey_hex.clone(),
        })
        .collect();

    let output = sign_with_duplicate_participant(
        &binary_path,
        &working_dir,
        &group_pk_hex,
        &message_path,
        SIG_FILE,
        NETWORK_ID,
        "localhost:2744",
        2,
        &signing_participants,
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "coordinator failed unexpectedly\nstdout={stdout}\nstderr={stderr}"
    );
    assert!(
        stdout.contains("Signature saved"),
        "coordinator did not save a signature\nstdout={stdout}\nstderr={stderr}"
    );
    let warned = stderr.contains("attempted to rejoin the session; ignoring")
        || stderr.contains("failed to decrypt message from");
    assert!(
        warned,
        "coordinator did not warn about the duplicate participant\nstderr={stderr}"
    );
    assert!(
        !stderr.contains("SnowError"),
        "coordinator crashed with SnowError (regression)\nstderr={stderr}"
    );

    server_process.kill()?;
    Ok(())
}
