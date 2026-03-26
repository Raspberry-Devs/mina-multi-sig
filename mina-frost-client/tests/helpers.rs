#![allow(dead_code)]

use mina_signer::{PubKey, Signature, Signer};
use mina_tx::{
    network_id::NetworkIdEnvelope, NetworkId, TransactionEnvelope, TransactionKind,
    TransactionSignature,
};
use regex::Regex;
use std::{
    fs, io,
    path::{Path, PathBuf},
    process::{Child, Command, Output, Stdio},
    thread,
    time::Duration,
};

pub struct ChildGuard(pub Child);

impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

#[derive(Clone, Debug)]
pub struct SigningParticipant {
    pub config_path: String,
    pub pubkey_hex: String,
}

#[derive(Clone, Debug)]
pub struct CliParticipant {
    pub name: String,
    pub toml: String,
    pub contact: String,
    pub pubkey_hex: String,
}

pub fn binary_name() -> &'static str {
    if cfg!(windows) {
        "mina-frost-client.exe"
    } else {
        "mina-frost-client"
    }
}

fn binary_alias_name(alias: &str) -> String {
    if cfg!(windows) {
        format!("mina-frost-client-{}.exe", alias)
    } else {
        format!("mina-frost-client-{}", alias)
    }
}

pub fn repo_root(manifest_dir: &str) -> PathBuf {
    PathBuf::from(format!("{manifest_dir}/.."))
}

pub fn to_owned_args(args: &[&str]) -> Vec<String> {
    args.iter().map(|s| (*s).to_string()).collect()
}

pub fn network_to_cli_arg(network: &NetworkId) -> String {
    match network {
        NetworkId::Mainnet => "mainnet".to_string(),
        NetworkId::Testnet => "testnet".to_string(),
        NetworkId::Custom(id) => id.clone(),
    }
}

pub fn build_client_binary(manifest_dir: &str, alias: Option<&str>, mesa: bool) -> PathBuf {
    let repo_root = repo_root(manifest_dir);
    let mut args = vec!["build", "-p", "mina-frost-client", "--release"];
    if mesa {
        args.push("--features");
        args.push("mina-tx/mesa-hardfork");
    }

    let status = Command::new("cargo")
        .args(&args)
        .current_dir(&repo_root)
        .status()
        .expect("failed to run cargo build for client binary");
    assert!(
        status.success(),
        "cargo build failed for mesa={} args={:?}",
        mesa,
        args
    );

    let target_dir = repo_root.join("target").join("release");
    let source = target_dir.join(binary_name());
    if let Some(alias) = alias {
        let destination = target_dir.join(binary_alias_name(alias));
        fs::copy(&source, &destination).expect("failed to copy built client binary");
        return destination;
    }
    source
}

pub fn run_cli(binary: &Path, cwd: &Path, args: &[String]) {
    let status = Command::new(binary)
        .args(args)
        .current_dir(cwd)
        .status()
        .expect("failed to run CLI command");
    assert!(
        status.success(),
        "CLI command failed: binary={} args={:?}",
        binary.display(),
        args
    );
}

pub fn run_cli_output(binary: &Path, cwd: &Path, args: &[String]) -> Output {
    let output = Command::new(binary)
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("failed to run CLI command");
    assert!(
        output.status.success(),
        "CLI command failed: binary={} args={:?}\nstdout={}\nstderr={}",
        binary.display(),
        args,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

pub fn introduce_participant(binary: &Path, cwd: &Path, name: &str) -> CliParticipant {
    let toml = cwd.join(format!("{name}.toml"));
    let toml_str = toml.to_string_lossy().to_string();

    run_cli(binary, cwd, &to_owned_args(&["init", "-c", &toml_str]));

    let output = run_cli_output(
        binary,
        cwd,
        &to_owned_args(&["export", "--name", name, "-c", &toml_str]),
    );
    let merged = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let contact = Regex::new(r"(?m)^minafrost[^\r\n]*$")
        .unwrap()
        .find(&merged)
        .unwrap_or_else(|| panic!("failed to extract contact from output:\n{}", merged))
        .as_str()
        .to_string();

    let pubkey_hex = Regex::new(r"(?mi)^Public Key: ([0-9a-f]+)$")
        .unwrap()
        .captures(&merged)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
        .unwrap_or_else(|| panic!("failed to extract pubkey from output:\n{}", merged));

    CliParticipant {
        name: name.to_string(),
        toml: toml_str,
        contact,
        pubkey_hex,
    }
}

pub fn greet_participants(binary: &Path, cwd: &Path, participants: &[CliParticipant]) {
    for participant in participants {
        for other in participants {
            if participant.toml != other.toml {
                run_cli(
                    binary,
                    cwd,
                    &to_owned_args(&["import", "-c", &participant.toml, &other.contact]),
                );
            }
        }
    }
}

pub fn form_group_with_dkg(
    binary: &Path,
    cwd: &Path,
    participants: &[CliParticipant],
    threshold: usize,
    server_url: &str,
    description: &str,
) -> io::Result<()> {
    let pks: Vec<&str> = participants.iter().map(|p| p.pubkey_hex.as_str()).collect();
    let mut children: Vec<(Vec<String>, Child)> = Vec::new();

    for (i, participant) in participants.iter().enumerate() {
        let threshold_str = threshold.to_string();
        let mut args = to_owned_args(&[
            "dkg",
            "-d",
            description,
            "-s",
            server_url,
            "-t",
            &threshold_str,
            "-c",
            &participant.toml,
        ]);

        if i == 0 {
            for pk in &pks {
                args.push("-S".to_string());
                args.push((*pk).to_string());
            }
        }

        children.push((args.clone(), run_cli_spawn_piped(binary, cwd, &args)));
        if i == 0 {
            thread::sleep(Duration::from_secs(1));
        }
    }

    for (child_args, child) in children {
        let output = child
            .wait_with_output()
            .expect("participant subprocess didn't stop during group formation");
        assert!(
            output.status.success(),
            "child process failed during group formation\nargs={:?}\nstdout={}\nstderr={}",
            child_args,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

pub fn group_keys_from_config(binary: &Path, cwd: &Path, toml_path: &str) -> (String, String) {
    let output = run_cli_output(binary, cwd, &to_owned_args(&["groups", "-c", toml_path]));
    let merged = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let group_hex = Regex::new(r"(?mi)^Public key \(hex format\): ([0-9a-f]+)$")
        .unwrap()
        .captures(&merged)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
        .unwrap_or_else(|| panic!("failed to extract group hex key from output:\n{}", merged));

    let group_mina = Regex::new(r"(?mi)^Public key \(mina format\): (\S+)$")
        .unwrap()
        .captures(&merged)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
        .unwrap_or_else(|| panic!("failed to extract group mina key from output:\n{}", merged));

    (group_hex, group_mina)
}

pub fn run_cli_spawn_piped(binary: &Path, cwd: &Path, args: &[String]) -> Child {
    Command::new(binary)
        .args(args)
        .current_dir(cwd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn CLI command")
}

pub fn sign_with_binary(
    binary: &Path,
    cwd: &Path,
    group_pk_hex: &str,
    message_path: &Path,
    sig_file: &str,
    network_id: &str,
    server_url: &str,
    threshold: usize,
    participants: &[SigningParticipant],
) {
    assert!(
        !participants.is_empty(),
        "at least one participant is required for signing"
    );
    assert!(
        participants.len() >= threshold,
        "participants ({}) must be >= threshold ({})",
        participants.len(),
        threshold
    );
    assert!(threshold > 0, "threshold must be > 0");

    let mut args = vec![
        "coordinator".to_string(),
        "-c".to_string(),
        participants[0].config_path.clone(),
        "-s".to_string(),
        server_url.to_string(),
        "--group".to_string(),
        group_pk_hex.to_string(),
        "-m".to_string(),
        message_path.to_string_lossy().to_string(),
        "-o".to_string(),
        sig_file.to_string(),
        "-n".to_string(),
        network_id.to_string(),
    ];

    for participant in participants.iter().take(threshold) {
        args.push("-S".to_string());
        args.push(participant.pubkey_hex.clone());
    }

    let mut children: Vec<(Vec<String>, Child)> = Vec::new();
    children.push((args.clone(), run_cli_spawn_piped(binary, cwd, &args)));

    thread::sleep(Duration::from_secs(1));

    for participant in participants.iter().take(threshold) {
        let participant_args = vec![
            "participant".to_string(),
            "-c".to_string(),
            participant.config_path.clone(),
            "-s".to_string(),
            server_url.to_string(),
            "--group".to_string(),
            group_pk_hex.to_string(),
            "-y".to_string(),
        ];

        children.push((
            participant_args.clone(),
            run_cli_spawn_piped(binary, cwd, &participant_args),
        ));
    }

    for (child_args, child) in children {
        let output = child
            .wait_with_output()
            .expect("failed waiting for coordinator/participant child process");
        assert!(
            output.status.success(),
            "child process failed during signing session\nargs={:?}\nstdout={}\nstderr={}",
            child_args,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

pub fn run_cli_spawn_quiet(binary: &Path, cwd: &Path, args: &[String]) -> io::Result<Child> {
    Command::new(binary)
        .args(args)
        .stderr(Stdio::null())
        .current_dir(cwd)
        .spawn()
}

fn status_success_or_err(status: std::process::ExitStatus, op: &str) -> io::Result<()> {
    if status.success() {
        return Ok(());
    }
    Err(io::Error::other(format!(
        "{op} failed with status {status}"
    )))
}

pub fn start_frostd(cwd: &Path) -> io::Result<Child> {
    let install_status = Command::new("mkcert").arg("-install").status()?;
    status_success_or_err(install_status, "mkcert -install")?;

    let cert_status = Command::new("mkcert")
        .args(["localhost", "127.0.0.1", "::1"])
        .current_dir(cwd)
        .status()?;
    status_success_or_err(cert_status, "mkcert localhost certificate generation")?;

    let tls_cert_path = cwd.join("localhost+2.pem");
    let tls_key_path = cwd.join("localhost+2-key.pem");

    let child = Command::new("frostd")
        .arg("--tls-cert")
        .arg(tls_cert_path)
        .arg("--tls-key")
        .arg(tls_key_path)
        .stderr(Stdio::null())
        .spawn()?;

    thread::sleep(Duration::from_secs(1));
    Ok(child)
}

pub fn write_json_str_pretty(path: &Path, json_str: &str) -> io::Result<()> {
    let value: serde_json::Value = serde_json::from_str(json_str)
        .map_err(|e| io::Error::other(format!("failed parsing json: {e}")))?;
    let pretty = serde_json::to_string_pretty(&value)
        .map_err(|e| io::Error::other(format!("failed serializing json: {e}")))?;
    fs::write(path, pretty)
}

pub fn parse_and_verify_signature(
    group_pk_mina: &str,
    message_path: &Path,
    signature_path: &Path,
    network_id: &str,
    is_legacy: bool,
) {
    let msg_json = fs::read_to_string(message_path).expect("failed reading message JSON");
    let network: NetworkIdEnvelope = network_id.to_string().try_into().unwrap();
    let source_tx = TransactionEnvelope::from_str_network(msg_json.trim(), network)
        .expect("failed parsing message JSON into transaction envelope");

    let pk = PubKey::from_address(group_pk_mina).expect("failed parsing group public key");

    let sig_json = fs::read_to_string(signature_path).expect("failed reading signature JSON");
    let tx_sig: TransactionSignature =
        serde_json::from_str(&sig_json).expect("failed parsing transaction signature JSON");

    let output_pk = tx_sig.publicKey.pubKey.into_address();
    assert!(
        output_pk == group_pk_mina,
        "signature output public key {} does not match expected group key {}",
        output_pk,
        group_pk_mina
    );

    let field = tx_sig.signature.field;
    let scalar = tx_sig.signature.scalar;

    let mina_sig: Signature = Signature {
        s: scalar.into(),
        rx: field.into(),
    };

    if !is_legacy {
        let signed_zkapp = match tx_sig.payload.inner() {
            TransactionKind::ZkApp(inner) => inner,
            _ => panic!("Expected ZkApp transaction in payload"),
        };

        assert!(
            !signed_zkapp.fee_payer.authorization.is_empty(),
            "Fee payer authorization should be non-empty in the signed payload"
        );

        for (i, update) in signed_zkapp.account_updates.iter().enumerate() {
            let is_signed = update.body.authorization_kind.is_signed;
            let pk_matches = update.body.public_key.0 == pk.into_compressed();
            let full_commitment = update.body.use_full_commitment;

            if is_signed && pk_matches && full_commitment {
                let sig = update.authorization.signature.as_ref().unwrap_or_else(|| {
                    panic!(
                        "Account update at index {} should have a signature after injection",
                        i
                    )
                });
                assert!(
                    !sig.is_empty(),
                    "Account update at index {} has an empty signature",
                    i
                );
            }
        }
    }

    if is_legacy {
        let mut ctx = mina_signer::create_legacy::<TransactionEnvelope>(source_tx.network_id());
        assert!(ctx.verify(&mina_sig, &pk, &source_tx));
        return;
    }
    let mut ctx = mina_signer::create_kimchi::<TransactionEnvelope>(source_tx.network_id());
    assert!(ctx.verify(&mina_sig, &pk, &source_tx));
}
