use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
    marker::PhantomData,
    rc::Rc,
};

use eyre::{Context as _, OptionExt};

use frost_core::Ciphersuite;
use reqwest::Url;
use zeroize::Zeroizing;

use super::{
    args::Command,
    config::{Config, Group, Participant},
};

use crate::api;
use crate::dkg;

/// CLI entry point for distributed key generation
///
/// Generates FROST key shares using distributed key generation protocol
/// and updates the participant config file with group information.
pub async fn run<C: Ciphersuite>(args: &Command) -> Result<(), Box<dyn Error>> {
    let Command::Dkg {
        config: config_path,
        description,
        server_url,
        threshold,
        participants,
    } = (*args).clone()
    else {
        panic!("invalid Command");
    };

    let mut input = Box::new(std::io::stdin().lock());
    let mut output = std::io::stdout();

    // Setup DKG configuration
    let dkg_config =
        setup_dkg_config::<C>(config_path.clone(), &server_url, threshold, &participants)?;

    // Generate key shares through DKG
    let (key_package, public_key_package, pubkey_map) =
        dkg::keygen::<C>(dkg_config, &mut input, &mut output).await?;
    let key_package = Zeroizing::new(key_package);

    // Create participants map from DKG results
    let participants_map = create_participants_map(&public_key_package, pubkey_map)?;

    // Update config file with group information
    update_config_with_group::<C>(
        config_path,
        &description,
        &server_url,
        &key_package,
        &public_key_package,
        &participants_map,
    )?;

    Ok(())
}

/// Setup DKG configuration from command line arguments and config file
///
/// This function reads the participant's config file, parses the server URL,
/// and constructs the DKG configuration needed for key generation.
fn setup_dkg_config<C: Ciphersuite>(
    config_path: Option<String>,
    server_url: &str,
    threshold: u16,
    participants: &[String],
) -> Result<dkg::Config, Box<dyn Error>> {
    let config = Config::<C>::read(config_path)?;

    let server_url_parsed =
        Url::parse(&format!("https://{}", server_url)).wrap_err("error parsing server-url")?;

    let comm_pubkey = config
        .communication_key
        .clone()
        .ok_or_eyre("user not initialized")?
        .pubkey
        .clone();

    let mut participants = participants
        .iter()
        .map(|s| Ok(api::PublicKey(hex::decode(s)?.to_vec())))
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
    // Add ourselves if not already in the list
    if !participants.is_empty() && !participants.contains(&comm_pubkey) {
        participants.push(comm_pubkey.clone());
    }

    let dkg_config = dkg::Config {
        ip: server_url_parsed
            .host_str()
            .ok_or_eyre("host missing in URL")?
            .to_owned(),
        port: server_url_parsed
            .port_or_known_default()
            .expect("always works for https"),
        comm_privkey: Some(
            config
                .communication_key
                .clone()
                .ok_or_eyre("user not initialized")?
                .privkey
                .clone(),
        ),
        comm_pubkey: Some(comm_pubkey),
        comm_participant_pubkey_getter: Some(Rc::new(move |participant_pubkey| {
            config
                .contact_by_pubkey(participant_pubkey)
                .map(|p| p.pubkey.clone())
                .ok()
        })),
        min_signers: threshold,
        participants,
    };

    Ok(dkg_config)
}

/// Create participants map from DKG results
///
/// This function processes the DKG output to create a map of participants
/// with their identifiers and public keys.
fn create_participants_map<C: Ciphersuite>(
    public_key_package: &frost_core::keys::PublicKeyPackage<C>,
    pubkey_map: HashMap<api::PublicKey, frost_core::Identifier<C>>,
) -> Result<BTreeMap<String, Participant>, Box<dyn Error>> {
    // Reverse pubkey_map to get identifier -> pubkey mapping
    let identifier_to_pubkey = pubkey_map
        .into_iter()
        .map(|(pubkey, identifier)| (identifier, pubkey))
        .collect::<HashMap<_, _>>();

    // Create participants map
    let mut participants = BTreeMap::new();
    for identifier in public_key_package.verifying_shares().keys() {
        let pubkey = identifier_to_pubkey
            .get(identifier)
            .ok_or_eyre("missing pubkey")?;
        let participant = Participant {
            identifier: identifier.serialize(),
            pubkey: pubkey.clone(),
        };
        participants.insert(hex::encode(identifier.serialize()), participant);
    }

    Ok(participants)
}

/// Update config file with group information
///
/// This function takes the generated key package and updates the participant's config
/// file with the group information.
fn update_config_with_group<C: Ciphersuite>(
    config_path: Option<String>,
    description: &str,
    server_url: &str,
    key_package: &Zeroizing<frost_core::keys::KeyPackage<C>>,
    public_key_package: &frost_core::keys::PublicKeyPackage<C>,
    participants: &BTreeMap<String, Participant>,
) -> Result<(), Box<dyn Error>> {
    let group = Group::<C> {
        _phantom: PhantomData,
        description: description.to_string(),
        key_package: postcard::to_allocvec(key_package)?,
        public_key_package: postcard::to_allocvec(public_key_package)?,
        participant: participants.clone(),
        server_url: Some(server_url.to_string()),
    };

    // Re-read the config because the old instance is tied to the
    // `comm_participant_pubkey_getter` callback.
    // TODO: is this an issue?
    let mut config = Config::read(config_path)?;
    config.group.insert(
        hex::encode(public_key_package.verifying_key().serialize()?),
        group,
    );
    config.write()?;

    eprintln!(
        "Group created; information written to {}",
        config.path().expect("should not be None").display()
    );

    Ok(())
}
