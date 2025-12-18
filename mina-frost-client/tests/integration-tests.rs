//! This file provides end to end tests for the entire project
//! It tests if the results of mina-frost-client agrees with `mina-signer` (cross-package test)
//! It uses mina-frost-client to generate signatures for given pair of (threshold, max-signers),
//! all the way from DKG, spawning subprocesses which interact with each other over the frostd
//! server. through the mina-frost-client. Finally we read the generated signature and verify that it
//! agrees with the group public key according to the `mina-signer` crate

use ark_ff::BigInt;
use frost_bluepallas::transactions::generic_tx::TransactionEnvelope;
use lazy_static::lazy_static;
use mina_signer::PubKey;
use mina_signer::Signature;
use mina_signer::Signer;
use regex::Regex;
use std::fs;
use std::io::Result;
use std::path::PathBuf;
use std::process;
use std::process::Command;
use std::process::Stdio;
use std::str::FromStr;

lazy_static! {
    static ref binary_path: PathBuf = PathBuf::from(format!(
        "{}/../target/release/{}",
        env!("CARGO_MANIFEST_DIR"),
        if cfg!(windows) {
            "mina-frost-client.exe"
        } else {
            "mina-frost-client"
        }
    ));
    static ref working_dir: PathBuf =
        PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/assets"));
    static ref message_path: PathBuf = PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/signing_example/message.json"
    ));
}

const SIG_FILE: &str = "signature.json";

#[derive(Debug)]
struct Pid {
    toml: String,
    contact: String,
    pk: String,
}

macro_rules! run_cli {
    ( $args:expr ) => {{
        let status = Command::new(binary_path.clone())
            .args($args)
            .stderr(Stdio::null()) // control verbosity by commenting line
            .current_dir(working_dir.clone())
            .status()
            .expect("subprocess failed");

        assert!(status.success(), "CLI command failed: {:?}", $args);
    }};
}

macro_rules! run_cli_wait {
    ( $args:expr ) => {{
        Command::new(binary_path.clone())
            .args($args)
            .stderr(Stdio::null()) // control verbosity by commenting line
            .current_dir(working_dir.clone())
            .spawn()?
    }};
}

macro_rules! run_cli_extract {
    ($( $arg:expr ),* $(,)?) => {{
        let output = Command::new(binary_path.clone())
            .args([ $( $arg ),* ])
            .current_dir(working_dir.clone())
            .output()
            .expect("subprocess failed");

        assert!(
            output.status.success(),
            "CLI command failed: {:?}",
            [ $( $arg ),* ]
        );

        String::from_utf8_lossy(&output.stderr).to_string()
    }};
}

macro_rules! regex_match {
    ($str:expr, $( $regex:expr ),* $(,)?) => {{
        (
            $({
                let re = Regex::new($regex).unwrap();
                let caps = re.captures($str).unwrap();
                // if the regex has a capture group, return group 1; otherwise full match
                if caps.len() > 1 {
                    caps[1].to_string()
                } else {
                    caps[0].to_string()
                }
            }),*
        )
    }};
}

/// Dump ZKApp transaction test vectors to files, return pathnames
fn write_zkapp_tx_files() -> Result<Vec<String>> {
    let test_vectors =
        frost_bluepallas::transactions::zkapp_tx::zkapp_test_vectors::get_zkapp_test_vectors();

    let mut paths: Vec<String> = Vec::with_capacity(test_vectors.len());
    for (index, tv) in test_vectors.into_iter().enumerate() {
        let path = working_dir.join(format!("zkapp_tx_{}.json", index));
        paths.push(path.to_str().unwrap().to_string());

        let tx_env: TransactionEnvelope = tv.into();

        let json = serde_json::to_string_pretty(&tx_env).unwrap();
        fs::write(path, json)?;
    }
    Ok(paths)
}

/// Create a participant by making them a .toml file
fn introduce(name: &str) -> Result<Pid> {
    let toml = format!("{}.toml", name);
    run_cli!(["init", "-c", &toml]);

    let export_str = run_cli_extract!("export", "--name", name, "-c", &toml);
    let (contact, pk) = regex_match!(
        &export_str,
        r"(?m)^minafrost[^\r\n]*$",
        r"(?mi)^Public Key: ([0-9a-f]+)$"
    );

    Ok(Pid { toml, contact, pk })
}

// All participants exchange contact info
fn greet(pids: &[Pid]) -> Result<()> {
    for Pid { toml, .. } in pids {
        for Pid {
            contact,
            toml: b_toml,
            ..
        } in pids
        {
            if toml != b_toml {
                run_cli!(["import", "-c", toml, contact]);
            }
        }
    }
    Ok(())
}

/// Errors if pids is empty
/// The first participant acts as coordinator and the rest join the session
/// t is the threshold
fn form_group(pids: &[Pid], threshold: usize) -> Result<()> {
    let len = threshold.to_string();
    let pks: Vec<&str> = pids.iter().map(|Pid { pk, .. }| pk.as_str()).collect();

    let mut children = Vec::new();

    for (i, Pid { toml, .. }) in pids.iter().enumerate() {
        let mut args = vec![
            "dkg",
            "-d",
            "Raspberry Devs",
            "-s",
            "localhost:2744",
            "-t",
            &len,
            "-c",
            toml,
        ];
        if i == 0 {
            // first participant plays coordinator, and also needs to know all public keys
            for pk in &pks {
                args.push("-S");
                args.push(pk);
            }
        }
        children.push(run_cli_wait!(args));

        // Have to make sure that the coordinator properly starts before the participant
        // Otherwise the test will fail. So we hope 1 second is more than enough...
        if i == 0 {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
    for child in &mut children {
        assert!(child
            .wait()
            .expect("participant subprocess didn't stop during group formation")
            .success());
    }
    Ok(())
}

/// Sign and obtain the signature
/// We sign the `TransactionEnvelope` as given by `sign_message_path`
fn sign(pids: &[Pid], group_pk: &str, threshold: usize, sign_message_path: &str) -> Result<()> {
    let mut children = Vec::new();

    // constructing the coordinator command
    let mut args = vec![
        "coordinator",
        "-c",
        &pids[0].toml,
        "-s",
        "localhost:2744",
        "--group",
        group_pk,
        "-m",
        sign_message_path,
        "-o",
        SIG_FILE,
    ];
    let pks: Vec<&str> = pids
        .iter()
        .map(|Pid { pk, .. }| pk.as_str())
        .take(threshold)
        .collect();
    for pk in &pks {
        args.push("-S");
        args.push(pk);
    }

    // Running the coordinator
    children.push(run_cli_wait!(args));
    // sleep to make sure coordinator starts up before the participants
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Spawn the participants
    for Pid { toml, .. } in &pids[..threshold] {
        children.push(run_cli_wait!([
            "participant",
            "-c",
            toml,
            "-s",
            "localhost:2744",
            "--group",
            group_pk,
            "-y",
        ]));
    }
    for child in &mut children {
        assert!(child
            .wait()
            .expect("subprocess didn't stop during signing")
            .success());
    }
    Ok(())
}

/// We're done using the cli. This is the cross-package part of the test
/// now verify the signature generated by mina-frost-client using the `mina-signer` crate
fn parse_and_verify(pk_str: &str, verify_message_path: &str) {
    let msg_json = fs::read_to_string(verify_message_path).unwrap();

    let msg: TransactionEnvelope =
        serde_json::from_str(msg_json.trim()).expect("Failed to parse transaction from JSON");
    let pk = PubKey::from_address(pk_str).unwrap();

    let sig_json = fs::read_to_string(working_dir.join(SIG_FILE).clone()).unwrap();

    // instead of implementing deserialize for frost_bluepallas::signature::TransactionSignature
    // which would be the right way to do this. I cheat by just taking what I want using regexes
    // out of the json, as we already have quite a bit of regex machinery
    let (field_str, scalar_str) = regex_match!(
        &sig_json,
        r#""field"\s*:\s*"([0-9]+)""#,
        r#""scalar"\s*:\s*"([0-9]+)""#
    );
    println!("field: {}", field_str);
    println!("scalar: {}", scalar_str);
    let field = BigInt::<4>::from_str(&field_str).unwrap();
    let scalar = BigInt::<4>::from_str(&scalar_str).unwrap();

    let mina_sig: Signature = Signature {
        s: scalar.into(),
        rx: field.into(),
    };

    let mut ctx = mina_signer::create_legacy::<TransactionEnvelope>(msg.network_id());
    assert!(ctx.verify(&mina_sig, &pk, &msg));
}

/// This test will iterate through all zkapp_test_vectors and verify that
/// they will sign correctly
fn cross_package_test(threshold: usize, max_signers: usize) -> Result<()> {
    let mut server_process = setup()?;
    let zkapp_message_paths = write_zkapp_tx_files()?;

    let pids = (0..max_signers)
        .map(|x| x.to_string())
        .map(|x| introduce(&x))
        .collect::<Result<Vec<Pid>>>()?;

    greet(&pids)?;

    form_group(&pids, threshold)?;

    let pk_str = run_cli_extract!("groups", "-c", &pids[0].toml);
    let (group_pk_hex, group_pk_mina) = regex_match!(
        &pk_str,
        r"(?mi)^Public key \(hex format\): ([0-9a-f]+)$",
        r"(?mi)^Public key \(mina format\): (\S+)$",
    );
    println!("group public key: {}", group_pk_mina);

    // Message paths for signing
    let mut message_paths = vec![message_path.to_str().unwrap().to_string()];
    message_paths.extend(zkapp_message_paths);

    // Iterate through each transction message path, sign+verify
    for msg in message_paths.iter() {
        sign(&pids, &group_pk_hex, threshold, msg)?;
        parse_and_verify(&group_pk_mina, msg);
    }

    server_process.kill()?;
    Ok(())
}

#[test]
fn permute_cross_package_test() -> Result<()> {
    // TODO for now there's just one cause it's quite slow
    cross_package_test(5, 10)
}

fn setup() -> Result<process::Child> {
    // Clean up generated directory if it exists
    if working_dir.exists() {
        println!("Cleaning up existing generated directory...");
        fs::remove_dir_all(working_dir.clone())?;
    }
    // Create directory for generated files
    fs::create_dir_all(working_dir.clone())?;

    // compile release binaries
    let repo_root_path: PathBuf = PathBuf::from(format!("{}/..", env!("CARGO_MANIFEST_DIR")));
    let status = Command::new("cargo")
        .args(["build", "--release"])
        .current_dir(repo_root_path)
        .status()
        .expect("Failed to build release binary");
    assert!(status.success(), "Release build failed");

    // ensure mkcert certificates are installed everytime (required for docker)
    Command::new("mkcert")
        .arg("-install")
        .output()
        .expect("failed to run mkcert -install");

    Command::new("mkcert")
        .args(["localhost", "127.0.0.1", "::1"])
        .stderr(Stdio::null()) // discard stderr
        .current_dir(working_dir.clone())
        .status()?;

    let tls_cert_path = working_dir.join("localhost+2.pem");
    let tls_key_path = working_dir.join("localhost+2-key.pem");

    // Start frostd server in the background
    Command::new("frostd")
        .arg("--tls-cert")
        .arg(&tls_cert_path)
        .arg("--tls-key")
        .arg(&tls_key_path)
        .stderr(Stdio::null()) // discard stderr
        .spawn()
}
