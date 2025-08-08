use std::error::Error;
use std::rc::Rc;

use eyre::Context;
use eyre::OptionExt;
use reqwest::Url;

use frost_core::keys::KeyPackage;
use frost_core::Ciphersuite;

use super::{args::Command, config::Config as ConfigFile};

use crate::cli::config::{Group, Participant};
use crate::participant::sign;
use crate::participant::Config as ParticipantConfig;

/// CLI entry point for participant signing
pub async fn run<C: Ciphersuite>(args: &Command) -> Result<(), Box<dyn Error>> {
    let Command::Participant {
        config: config_path,
        server_url,
        group,
        session,
    } = (*args).clone()
    else {
        panic!("invalid Command");
    };

    let mut input = Box::new(std::io::stdin().lock());
    let mut output = std::io::stdout();

    // Load and validate configuration
    let (user_config, group_config, key_package) =
        load_participant_config::<C>(config_path, &group)?;

    // Setup participant configuration
    let participant_config = setup_participant_config::<C>(
        &user_config,
        &group_config,
        key_package,
        server_url,
        session,
    )?;

    // Execute signing
    sign(participant_config, &mut input, &mut output).await?;

    Ok(())
}

// Avoid clippy warnings about complex return types
type LoadParticipantConfigResult<C> =
    Result<(ConfigFile<C>, Group<C>, KeyPackage<C>), Box<dyn Error>>;

/// Load and validate participant configuration
///
/// This function reads the user config file, extracts the specified group,
/// and deserializes the key package.
fn load_participant_config<C: Ciphersuite>(
    config_path: Option<String>,
    group_id: &str,
) -> LoadParticipantConfigResult<C> {
    let user_config = ConfigFile::read(config_path)?;

    let group_config = user_config
        .group
        .get(group_id)
        .ok_or_eyre("Group not found")?
        .clone();

    let key_package: KeyPackage<C> = postcard::from_bytes(&group_config.key_package)?;

    Ok((user_config, group_config, key_package))
}

/// Setup participant configuration for signing
///
/// This function constructs the ParticipantConfig with all necessary parameters
/// including network settings, keys, and coordinator lookup functionality.
fn setup_participant_config<C: Ciphersuite>(
    user_config: &ConfigFile<C>,
    group_config: &Group<C>,
    key_package: KeyPackage<C>,
    server_url: Option<String>,
    session: Option<String>,
) -> Result<ParticipantConfig<C>, Box<dyn Error>> {
    // Determine server URL
    let server_url = if let Some(server_url) = server_url {
        server_url
    } else {
        group_config
            .server_url
            .clone()
            .ok_or_eyre("server-url required")?
    };

    // Parse server URL
    let server_url_parsed =
        Url::parse(&format!("https://{}", server_url)).wrap_err("error parsing server-url")?;

    // Setup coordinator pubkey lookup
    let group_participants = group_config.participant.clone();
    let coordinator_pubkey_getter = create_coordinator_pubkey_getter(group_participants);

    let participant_config = ParticipantConfig::<C> {
        socket: false,
        key_package,
        ip: server_url_parsed
            .host_str()
            .ok_or_eyre("host missing in URL")?
            .to_owned(),
        port: server_url_parsed
            .port_or_known_default()
            .expect("always works for https"),
        session_id: session.unwrap_or_default(),
        comm_privkey: Some(
            user_config
                .communication_key
                .as_ref()
                .ok_or_eyre("user not initialized")?
                .privkey
                .clone(),
        ),
        comm_pubkey: Some(
            user_config
                .communication_key
                .as_ref()
                .ok_or_eyre("user not initialized")?
                .pubkey
                .clone(),
        ),
        comm_coordinator_pubkey_getter: Some(coordinator_pubkey_getter),
    };

    Ok(participant_config)
}

/// Type alias for coordinator public key getter function
type CoordinatorPubkeyGetter = Rc<dyn Fn(&crate::api::PublicKey) -> Option<crate::api::PublicKey>>;

/// Create coordinator public key getter function
///
/// This function creates a closure that can look up coordinator public keys
/// from the group participants.
fn create_coordinator_pubkey_getter(
    group_participants: std::collections::BTreeMap<String, Participant>,
) -> CoordinatorPubkeyGetter {
    Rc::new(move |coordinator_pubkey| {
        group_participants
            .values()
            .find(|p| p.pubkey == *coordinator_pubkey)
            .map(|p| p.pubkey.clone())
    })
}
