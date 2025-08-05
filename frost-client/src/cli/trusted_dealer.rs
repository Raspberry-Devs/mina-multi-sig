use std::{collections::BTreeMap, error::Error};

use eyre::{eyre, OptionExt};
use itertools::izip;
use rand::thread_rng;

use frost_bluepallas::PallasPoseidon;
use frost_core::{keys::KeyPackage, Ciphersuite};

use super::{
    args::Command,
    config::{Config, Group, Participant},
    contact::Contact,
};

use crate::trusted_dealer;

/// Type alias for participant extraction result
type ParticipantExtractionResult =
    Result<(BTreeMap<String, Participant>, Vec<Contact>), Box<dyn Error>>;

/// CLI entry point for trusted dealer key generation
///
/// Generates FROST key shares using PallasPoseidon ciphersuite and updates
/// participant config files with group information.
///
/// **TESTING ONLY** - See security warnings in `Command::TrustedDealer`.
pub fn run(args: &Command) -> Result<(), Box<dyn Error>> {
    run_for_ciphersuite::<PallasPoseidon>(args)
}

/// Trusted dealer key generation for a specific ciphersuite
pub(crate) fn run_for_ciphersuite<C: Ciphersuite + 'static>(
    args: &Command,
) -> Result<(), Box<dyn Error>> {
    let Command::TrustedDealer {
        config,
        description,
        threshold,
        names,
        server_url,
    } = (*args).clone()
    else {
        panic!("invalid Command");
    };

    let num_signers = names.len() as u16;
    // QUESTION: Should we make the user confirm after that?
    print!(
        "Running Trusted Dealer with {} participants and threshold {}",
        num_signers, threshold
    );

    if config.len() != num_signers as usize {
        return Err(
            eyre!("The `config` option must specify `num_signers` different config files").into(),
        );
    }
    if threshold > num_signers {
        return Err(eyre!("Threshold cannot be greater than the number of signers").into());
    }

    let trusted_dealer_config = trusted_dealer::Config::new::<C>(threshold, num_signers)?;
    let mut rng = thread_rng();

    // Generate key shares
    let (shares, public_key_package) =
        trusted_dealer::keygen::<C, _>(&trusted_dealer_config, &mut rng)?;

    // Extract participant information from config files
    let (participants, contacts) = extract_participant_info(&shares, &config, &names)?;

    // Update config files with group information
    update_config_files::<C>(
        &shares,
        &config,
        &public_key_package,
        &description,
        &participants,
        &contacts,
        &server_url,
    )?;

    Ok(())
}

/// Extract participant information from config files
///
/// This function reads each participant's config file and extracts their communication
/// keys to build both the participants map and contacts list.
fn extract_participant_info<C: Ciphersuite>(
    shares: &BTreeMap<frost_core::Identifier<C>, frost_core::keys::SecretShare<C>>,
    config_paths: &[String],
    names: &[String],
) -> ParticipantExtractionResult {
    let mut participants = BTreeMap::new();
    let mut contacts = Vec::new();

    for (identifier, path, name) in izip!(shares.keys(), config_paths.iter(), names.iter()) {
        let config = Config::read(Some(path.to_string()))?;
        let pubkey = config
            .communication_key
            .ok_or_eyre("config not initialized")?
            .pubkey
            .clone();
        let participant = Participant {
            identifier: identifier.serialize(),
            pubkey: pubkey.clone(),
        };
        participants.insert(hex::encode(identifier.serialize()), participant);
        let contact = Contact {
            version: None,
            name: name.clone(),
            pubkey,
        };
        contacts.push(contact);
    }

    Ok((participants, contacts))
}

/// Update config files with group information
///
/// This function takes the generated key shares and updates each participant's config
/// file with the group information, including their key package and all participants.
fn update_config_files<C: Ciphersuite + 'static>(
    shares: &BTreeMap<frost_core::Identifier<C>, frost_core::keys::SecretShare<C>>,
    config_paths: &[String],
    public_key_package: &frost_core::keys::PublicKeyPackage<C>,
    description: &str,
    participants: &BTreeMap<String, Participant>,
    contacts: &[Contact],
    server_url: &Option<String>,
) -> Result<(), Box<dyn Error>> {
    for (share, path) in shares.values().zip(config_paths.iter()) {
        let mut config = Config::read(Some(path.to_string()))?;
        // IMPORTANT: the TrustedDealer command is intended for tests only, see
        // comment in [`Command::TrustedDealer`]. If you're using this code as a
        // reference, note that participants should not convert a SecretShare
        // into a KeyPackage without first checking if
        // [`SecretShare::commitment()`] is the same for all participants using
        // a broadcast channel.
        let key_package: KeyPackage<C> = share.clone().try_into()?;
        let group = Group {
            ciphersuite: C::ID.to_string(),
            description: description.to_string(),
            key_package: postcard::to_allocvec(&key_package)?,
            public_key_package: postcard::to_allocvec(public_key_package)?,
            participant: participants.clone(),
            server_url: server_url.clone(),
        };
        config.group.insert(
            hex::encode(public_key_package.verifying_key().serialize()?),
            group,
        );
        for c in contacts {
            config.contact.insert(c.name.clone(), c.clone());
        }
        config.write()?;
    }

    Ok(())
}
