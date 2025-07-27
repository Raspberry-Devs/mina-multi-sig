use std::error::Error;
use std::rc::Rc;

use eyre::Context;
use eyre::OptionExt;
use reqwest::Url;

use frost_bluepallas::PallasPoseidon;
use frost_core::keys::KeyPackage;
use frost_core::Ciphersuite;

use super::{args::Command, config::Config as CliConfig};

use crate::participant::sign;
use crate::participant::Config as ParticipantConfig;

pub async fn run(args: &Command) -> Result<(), Box<dyn Error>> {
    run_for_ciphersuite::<PallasPoseidon>(args).await
}

pub(crate) async fn run_for_ciphersuite<C: Ciphersuite + 'static>(
    args: &Command,
) -> Result<(), Box<dyn Error>> {
    let Command::Participant {
        config: config_path,
        server_url,
        group,
        session,
    } = (*args).clone()
    else {
        panic!("invalid Command");
    };

    let user_config = CliConfig::read(config_path)?;

    let group = user_config
        .group
        .get(&group)
        .ok_or_eyre("Group not found")?;

    let key_package: KeyPackage<C> = postcard::from_bytes(&group.key_package)?;

    let mut input = Box::new(std::io::stdin().lock());
    let mut output = std::io::stdout();

    let server_url = if let Some(server_url) = server_url {
        server_url
    } else {
        group.server_url.clone().ok_or_eyre("server-url required")?
    };
    let server_url_parsed =
        Url::parse(&format!("https://{}", server_url)).wrap_err("error parsing server-url")?;

    let group_participants = group.participant.clone();
    let pargs = ParticipantConfig::<C> {
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
                .clone()
                .ok_or_eyre("user not initialized")?
                .privkey
                .clone(),
        ),
        comm_pubkey: Some(
            user_config
                .communication_key
                .ok_or_eyre("user not initialized")?
                .pubkey
                .clone(),
        ),
        comm_coordinator_pubkey_getter: Some(Rc::new(move |coordinator_pubkey| {
            group_participants
                .values()
                .find(|p| p.pubkey == *coordinator_pubkey)
                .map(|p| p.pubkey.clone())
        })),
    };

    sign(pargs, &mut input, &mut output).await?;

    Ok(())
}
