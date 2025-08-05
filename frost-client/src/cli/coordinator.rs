use crate::{
    cipher::PublicKey,
    coordinator::{sign, Config as CoordinatorConfig},
};
use eyre::Context;
use eyre::OptionExt;
use frost_bluepallas::{transactions::Transaction, translate::Translatable, PallasPoseidon};
use frost_core::keys::PublicKeyPackage;
use frost_core::Ciphersuite;
use reqwest::Url;
use std::{
    collections::HashMap,
    error::Error,
    fs,
    io::{BufRead, Write},
    path::Path,
    vec,
};

use super::args::Command;
use super::config::Config as ConfigFile;
use crate::network::Network;

pub async fn run(args: &Command) -> Result<(), Box<dyn Error>> {
    run_for_ciphersuite::<PallasPoseidon>(args).await
}

pub(crate) async fn run_for_ciphersuite<C: Ciphersuite + 'static>(
    args: &Command,
) -> Result<(), Box<dyn Error>> {
    let Command::Coordinator {
        config: config_path,
        server_url,
        group: group_id,
        signers,
        message,
        signature: signature_path,
        network,
    } = (*args).clone()
    else {
        panic!("invalid Command");
    };

    let mut input = Box::new(std::io::stdin().lock());
    let mut output = std::io::stdout();

    // Load and validate configuration
    let (user_config, group_config, public_key_package) =
        load_coordinator_config::<C>(config_path, &group_id)?;

    // Parse signers from command line arguments
    let signers = parse_signers::<C>(&signers, &group_config)?;

    // Setup coordinator configuration
    let params = CoordinatorSetupParams {
        user_config: &user_config,
        group_config: &group_config,
        server_url,
        message_paths: &message,
        output: &mut output,
        input: &mut input,
        network,
    };

    let coordinator_config = setup_coordinator_config::<C>(public_key_package, signers, params)?;

    // Execute signing
    let signature_bytes = sign(&coordinator_config, &mut input, &mut output).await?;

    save_signature(&signature_path, &signature_bytes)?;
    Ok(())
}

// Read message from the provided file or stdin
pub fn read_messages(
    message_paths: &[String],
    output: &mut dyn Write,
    input: &mut dyn BufRead,
) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    // If no message paths are provided, read from stdin
    let messages = if message_paths.is_empty() {
        writeln!(output, "The message to be signed (json string)")?;
        vec![load_transaction_from_stdin(input)?.translate_msg()]
    } else {
        // Otherwise, iterate over the provided paths and attempt to read each message
        message_paths
            .iter()
            .map(|filename| {
                // If the filename is "-" or empty, read from stdin instead
                let msg = if *filename == "-" || filename.is_empty() {
                    writeln!(output, "The message to be signed (json string)")?;
                    load_transaction_from_stdin(input)?
                } else {
                    eprintln!("Reading message from {}...", &filename);
                    load_transaction_from_json(filename)?
                };
                Ok(msg)
            })
            .map(|res| res.map(|msg| msg.translate_msg()))
            .collect::<Result<_, Box<dyn Error>>>()?
    };
    Ok(messages)
}

/// Load and validate coordinator configuration
///
/// This function reads the user config file, extracts the specified group,
/// and deserializes the public key package.
fn load_coordinator_config<C: Ciphersuite>(
    config_path: Option<String>,
    group_id: &str,
) -> Result<(ConfigFile, crate::cli::config::Group, PublicKeyPackage<C>), Box<dyn Error>> {
    let user_config = ConfigFile::read(config_path)?;

    let group_config = user_config
        .group
        .get(group_id)
        .ok_or_eyre("Group not found")?
        .clone();

    let public_key_package: PublicKeyPackage<C> =
        postcard::from_bytes(&group_config.public_key_package)?;

    Ok((user_config, group_config, public_key_package))
}

/// Parse signers from command line arguments
///
/// This function converts hex-encoded public keys to PublicKey objects
/// and maps them to their corresponding identifiers from the group config.
fn parse_signers<C: Ciphersuite>(
    signer_args: &[String],
    group_config: &crate::cli::config::Group,
) -> Result<HashMap<PublicKey, frost_core::Identifier<C>>, Box<dyn Error>> {
    signer_args
        .iter()
        .map(|s| {
            let pubkey = PublicKey(hex::decode(s)?.to_vec());
            let contact = group_config.participant_by_pubkey(&pubkey)?;
            Ok((pubkey, contact.identifier()?))
        })
        .collect::<Result<HashMap<_, _>, Box<dyn Error>>>()
}

/// Parameters for setting up coordinator configuration
///
/// This structure groups related parameters to avoid the Clippy warning about
/// functions with too many arguments.
struct CoordinatorSetupParams<'a> {
    user_config: &'a ConfigFile,
    group_config: &'a crate::cli::config::Group,
    server_url: Option<String>,
    message_paths: &'a [String],
    output: &'a mut dyn Write,
    input: &'a mut dyn BufRead,
    network: Network,
}

/// Setup coordinator configuration for signing
///
/// This function constructs the CoordinatorConfig with all necessary parameters
/// including network settings, keys, signers, and messages.
fn setup_coordinator_config<C: Ciphersuite + 'static>(
    public_key_package: PublicKeyPackage<C>,
    signers: HashMap<PublicKey, frost_core::Identifier<C>>,
    params: CoordinatorSetupParams,
) -> Result<CoordinatorConfig<C>, Box<dyn Error>> {
    // Determine server URL
    let server_url = if let Some(server_url) = params.server_url {
        server_url
    } else {
        params
            .group_config
            .server_url
            .clone()
            .ok_or_eyre("server-url required")?
    };

    // Parse server URL
    let server_url_parsed =
        Url::parse(&format!("https://{}", server_url)).wrap_err("error parsing server-url")?;

    let num_signers = signers.len() as u16;
    let messages = read_messages(params.message_paths, params.output, params.input)?;

    let coordinator_config = CoordinatorConfig {
        socket: false,
        signers,
        num_signers,
        public_key_package,
        messages,
        ip: server_url_parsed
            .host_str()
            .ok_or_eyre("host missing in URL")?
            .to_owned(),
        port: server_url_parsed
            .port_or_known_default()
            .expect("always works for https"),
        comm_privkey: Some(
            params
                .user_config
                .communication_key
                .clone()
                .ok_or_eyre("user not initialized")?
                .privkey
                .clone(),
        ),
        comm_pubkey: Some(
            params
                .user_config
                .communication_key
                .clone()
                .ok_or_eyre("user not initialized")?
                .pubkey
                .clone(),
        ),
        network: params.network,
    };

    Ok(coordinator_config)
}

pub fn save_signature(
    signature_path: &str,
    signature_bytes: &Vec<u8>,
) -> Result<(), Box<dyn Error>> {
    if signature_path == "-" {
        println!("{}", hex::encode(signature_bytes));
    } else {
        fs::write(signature_path, signature_bytes)?;
        println!("Signature saved to {}", signature_path);
    }
    Ok(())
}

fn load_transaction_from_json<P: AsRef<Path>>(
    path: P,
) -> Result<Transaction, Box<dyn std::error::Error>> {
    let json_content = fs::read_to_string(path)?;
    let transaction: Transaction = serde_json::from_str(json_content.trim())
        .map_err(|e| eyre::eyre!("Failed to parse transaction from JSON: {}", e))?;
    Ok(transaction)
}

fn load_transaction_from_stdin(
    input: &mut dyn BufRead,
) -> Result<Transaction, Box<dyn std::error::Error>> {
    let mut json_content = String::new();
    input.read_to_string(&mut json_content)?;
    let transaction: Transaction = serde_json::from_str(json_content.trim())
        .map_err(|e| eyre::eyre!("Failed to parse transaction from JSON: {}", e))?;
    Ok(transaction)
}
