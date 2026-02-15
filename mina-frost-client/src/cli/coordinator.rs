use crate::{
    cipher::PublicKey,
    coordinator::{coordinate_signing, Config as CoordinatorConfig},
};
use eyre::Context;
use eyre::OptionExt;
use frost_bluepallas::{errors::BluePallasError, BluePallas};
use frost_core::{keys::PublicKeyPackage, Ciphersuite, VerifyingKey};
use mina_tx::{network_id::NetworkIdEnvelope, TransactionEnvelope, TransactionSignature};
use reqwest::Url;
use std::{
    collections::HashMap,
    error::Error,
    fs,
    io::{BufRead, Write},
    path::Path,
};

use super::args::Command;
use super::config::Config as ConfigFile;

/// This is the BluePallas/BluePallas specific run command for the coordinator which will save the output
/// of the signing session into a Mina-specific transaction.
pub async fn run_bluepallas(args: &Command) -> Result<(), Box<dyn Error>> {
    // Match on command type early to ensure we are running the coordinator command, panic otherwise
    let Command::Coordinator {
        signature: signature_path,
        ..
    } = args
    else {
        panic!("invalid Command");
    };

    let (bytes, transaction, vk) = run(args).await?;

    // Save signature to the specified path or stdout
    save_signature(signature_path, bytes, transaction, vk)
        .map_err(|e| BluePallasError::SaveSignatureError(e.to_string()))?;

    Ok(())
}

pub(crate) async fn run(
    args: &Command,
) -> Result<(Vec<u8>, TransactionEnvelope, VerifyingKey<BluePallas>), Box<dyn Error>> {
    // Note, we duplicate pattern matching code here and in run(), but given that there is no way to pass a Command::Coordinator type
    // to this function, we must instead repeat the check again
    // The alternative is to create a struct which contains the same parameters, not worth it for only one use
    let Command::Coordinator {
        config: config_path,
        server_url,
        group: group_id,
        signers,
        message,
        network,
        signature: _,
    } = (*args).clone()
    else {
        panic!("invalid Command");
    };

    let mut input = Box::new(std::io::stdin().lock());
    let mut output = std::io::stdout();

    // Load and validate configuration
    let (user_config, group_config, public_key_package) =
        load_coordinator_config::<BluePallas>(config_path, &group_id)?;

    // Parse signers from command line arguments
    let signers = parse_signers::<BluePallas>(&signers, &group_config)?;

    let network_id: NetworkIdEnvelope = network.try_into()?;
    let transaction = load_transaction(&message, network_id, &mut output, &mut input)?;
    let message_bytes = transaction.to_pallas_message().serialize();

    // Setup coordinator configuration
    let params = CoordinatorSetupParams {
        user_config: &user_config,
        group_config: &group_config,
        server_url,
        message: message_bytes,
    };

    let coordinator_config =
        setup_coordinator_config::<BluePallas>(public_key_package.clone(), signers, params)?;

    // Execute signing
    let signature_bytes = coordinate_signing(&coordinator_config, &mut input, &mut output).await?;

    Ok((
        signature_bytes,
        transaction,
        *public_key_package.verifying_key(),
    ))
}

fn load_transaction(
    message_path: &str,
    network_id: NetworkIdEnvelope,
    output: &mut dyn Write,
    input: &mut dyn BufRead,
) -> Result<TransactionEnvelope, Box<dyn Error>> {
    // If no message paths are provided, read from stdin
    if message_path == "-" || message_path.is_empty() {
        writeln!(output, "The message to be signed (json string)")?;
        load_transaction_from_stdin(input, network_id)
    } else {
        eprintln!("Reading message from {}...", message_path);
        load_transaction_from_json(message_path, network_id)
    }
}

// Avoid clippy warnings about complex return types
type LoadCoordinatorConfigResult<C> = Result<
    (
        ConfigFile<C>,
        crate::cli::config::Group<C>,
        PublicKeyPackage<C>,
    ),
    Box<dyn Error>,
>;

/// Load and validate coordinator configuration
///
/// This function reads the user config file, extracts the specified group,
/// and deserializes the public key package.
fn load_coordinator_config<C: Ciphersuite>(
    config_path: Option<String>,
    group_id: &str,
) -> LoadCoordinatorConfigResult<C> {
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
    group_config: &crate::cli::config::Group<C>,
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
struct CoordinatorSetupParams<'a, C: Ciphersuite> {
    user_config: &'a ConfigFile<C>,
    group_config: &'a crate::cli::config::Group<C>,
    server_url: Option<String>,
    message: Vec<u8>,
}

/// Setup coordinator configuration for signing
///
/// This function constructs the CoordinatorConfig with all necessary parameters
/// including network settings, keys, signers, and messages.
fn setup_coordinator_config<C: Ciphersuite>(
    public_key_package: PublicKeyPackage<C>,
    signers: HashMap<PublicKey, frost_core::Identifier<C>>,
    params: CoordinatorSetupParams<C>,
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

    let coordinator_config = CoordinatorConfig {
        signers,
        num_signers,
        public_key_package,
        message: params.message,
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
    };

    Ok(coordinator_config)
}

/// Combine the signature with the message and public key to generate the final signed output in json
/// This is BluePallas specific, and so is called in the run() function which specifically uses the PallasPosiedon ciphersuite.
pub fn save_signature(
    signature_path: &str,
    signature_bytes: Vec<u8>,
    transaction: TransactionEnvelope,
    vk: VerifyingKey<BluePallas>,
) -> Result<(), Box<dyn Error>> {
    let (transaction_signature, warnings_opt) =
        TransactionSignature::from_frost_signature_bytes(vk, &signature_bytes, transaction)
            .map_err(|e| BluePallasError::DeSerializationError(e.to_string()))?;

    // If there are any warnings during the creation of the transaction signature, print them out
    if let Some(warnings) = warnings_opt {
        eprintln!("Warnings during transaction signature creation:");
        for warning in warnings.warnings {
            eprintln!("- {}", warning);
        }
    }

    let output_str = serde_json::to_string_pretty(&transaction_signature)
        .map_err(|e| BluePallasError::DeSerializationError(e.to_string()))?;

    if signature_path == "-" {
        println!("{}", output_str);
    } else {
        fs::write(signature_path, output_str)?;
        println!("Signature saved to {}", signature_path);
    }
    Ok(())
}

fn load_transaction_from_json<P: AsRef<Path>>(
    path: P,
    network_id: NetworkIdEnvelope,
) -> Result<TransactionEnvelope, Box<dyn std::error::Error>> {
    let json_content = fs::read_to_string(path)?;

    load_transaction_from_str(&json_content, network_id)
}

fn load_transaction_from_stdin(
    input: &mut dyn BufRead,
    network_id: NetworkIdEnvelope,
) -> Result<TransactionEnvelope, Box<dyn std::error::Error>> {
    let mut json_content = String::new();
    input.read_to_string(&mut json_content)?;

    load_transaction_from_str(&json_content, network_id)
}

fn load_transaction_from_str(
    transaction_str: &str,
    network_id: NetworkIdEnvelope,
) -> Result<TransactionEnvelope, Box<dyn std::error::Error>> {
    let transaction = TransactionEnvelope::from_str_network(transaction_str, network_id)
        .map_err(|e| eyre::eyre!("Failed to parse transaction from JSON: {}", e))?;
    Ok(transaction)
}
