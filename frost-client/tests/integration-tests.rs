use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::io::Result;
use std::path::PathBuf;
use std::process;
use std::process::Command;
use std::process::Stdio;

lazy_static! {
    static ref binary_path: PathBuf = PathBuf::from(format!(
        "{}/../target/release/{}",
        env!("CARGO_MANIFEST_DIR"),
        if cfg!(windows) {
            "frost-client.exe"
        } else {
            "frost-client"
        }
    ));
    static ref working_dir: PathBuf =
        PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/assets"));
}

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
    (
        args = [ $( $arg:expr ),* $(,)? ],
        regexes = [ $( $regex:expr ),* $(,)? ]
    ) => {{

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

        let stderr_str = String::from_utf8_lossy(&output.stderr);

        (
            $({
                let re = Regex::new($regex).unwrap();
                let caps = re.captures(&stderr_str).unwrap();
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

/// Create a participant by making them a .toml file
fn introduce(name: &str) -> Result<Pid> {
    let toml = format!("{}.toml", name);
    run_cli!(["init", "-c", &toml]);

    let (contact, pk) = run_cli_extract!(
        args = ["export", "--name", name, "-c", &toml],
        regexes = [r"(?m)^minafrost[^\r\n]*$", r"(?m)^Public Key: ([0-9a-f]+)$"]
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

fn cross_package_test(threshold: usize, max_signers: usize) -> Result<()> {
    let mut server_process = setup()?;

    let pids = (0..max_signers)
        .map(|x| x.to_string())
        .map(|x| introduce(&x))
        .collect::<Result<Vec<Pid>>>()?;

    greet(&pids)?;

    form_group(&pids, threshold)?;

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
