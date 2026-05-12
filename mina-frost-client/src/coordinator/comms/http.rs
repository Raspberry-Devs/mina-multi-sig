//! HTTP implementation of the Comms trait.

use std::{
    collections::{BTreeMap, HashMap, HashSet},
    error::Error,
    io::{BufRead, Write},
    marker::PhantomData,
    time::Duration,
    vec,
};

use async_trait::async_trait;
use eyre::{eyre, OptionExt};
use frost_core::{
    keys::PublicKeyPackage, round1::SigningCommitments, round2::SignatureShare, Ciphersuite,
    Identifier, SigningPackage,
};
use rand::thread_rng;
use serde_json;

use crate::cipher::Cipher;
use crate::client::Client;
use crate::{
    api::{self, PublicKey, SendSigningPackageArgs, Uuid},
    session::CoordinatorSessionState,
};

use super::super::config::Config;
use super::Comms;

pub struct HTTPComms<C: Ciphersuite> {
    client: Client,
    session_id: Option<Uuid>,
    config: Config<C>,
    state: CoordinatorSessionState<C>,
    pubkeys: HashMap<PublicKey, Identifier<C>>,
    cipher: Option<Cipher>,
    /// Pubkeys that have already submitted a commitment in round 1. Used in
    /// round 2 to detect and skip duplicate Noise handshakes from participants
    /// that re-joined the session (e.g. from a second process), which would
    /// otherwise cause SnowError(Decrypt) because the transport-mode Noise
    /// state cannot process a new handshake message.
    commitment_senders: HashSet<PublicKey>,
    _phantom: PhantomData<C>,
}

impl<C: Ciphersuite> HTTPComms<C> {
    pub fn new(config: &Config<C>) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            client: Client::new(format!("https://{}:{}", config.ip, config.port)),
            session_id: None,
            config: config.clone(),
            state: CoordinatorSessionState::new(
                1, // Supporting just one message
                config.num_signers as usize,
                config.signers.clone(),
            ),
            pubkeys: Default::default(),
            cipher: None,
            commitment_senders: Default::default(),
            _phantom: Default::default(),
        })
    }
}

#[async_trait(?Send)]
impl<C: Ciphersuite + 'static> Comms<C> for HTTPComms<C> {
    async fn get_signing_commitments(
        &mut self,
        _input: &mut dyn BufRead,
        _output: &mut dyn Write,
        _pub_key_package: &PublicKeyPackage<C>,
        _num_signers: u16,
    ) -> Result<BTreeMap<Identifier<C>, SigningCommitments<C>>, Box<dyn Error>> {
        let mut rng = thread_rng();

        eprintln!("Logging in...");
        let challenge = self.client.challenge().await?.challenge;

        let signature: [u8; 64] = self
            .config
            .comm_privkey
            .clone()
            .ok_or_eyre("comm_privkey must be specified")?
            .sign(challenge.as_bytes(), &mut rng)?;

        self.client
            .login(&api::LoginArgs {
                challenge,
                pubkey: self
                    .config
                    .comm_pubkey
                    .clone()
                    .ok_or_eyre("comm_pubkey must be specified")?,
                signature: signature.to_vec(),
            })
            .await?;

        eprintln!("Creating signing session...");
        let r = self
            .client
            .create_new_session(&api::CreateNewSessionArgs {
                pubkeys: self.config.signers.keys().cloned().collect(),
                message_count: 1,
            })
            .await?;

        if self.config.signers.is_empty() {
            eprintln!(
                "Send the following session ID to participants: {}",
                r.session_id
            );
        }
        self.session_id = Some(r.session_id);

        let Some(comm_privkey) = &self.config.comm_privkey else {
            return Err(eyre!("comm_privkey must be specified").into());
        };

        // If encryption is enabled, create the Noise objects

        let mut cipher = Cipher::new(
            comm_privkey.clone(),
            self.config.signers.keys().cloned().collect(),
        )?;

        eprint!("Waiting for participants to send their commitments...");

        loop {
            let r = self
                .client
                .receive(&api::ReceiveArgs {
                    session_id: r.session_id,
                    as_coordinator: true,
                })
                .await?;
            for msg in r.msgs {
                if self.commitment_senders.contains(&msg.sender) {
                    continue;
                }
                let sender = msg.sender.clone();
                let msg = cipher.decrypt(msg)?;
                self.state.recv(msg)?;
                self.commitment_senders.insert(sender);
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
            eprint!(".");
            if self.state.has_commitments() {
                break;
            }
        }
        eprintln!();

        self.cipher = Some(cipher);

        let (commitments, pubkeys) = self.state.commitments()?;
        self.pubkeys = pubkeys;

        // TODO: support more than 1
        Ok(commitments[0].clone())
    }

    async fn send_signing_package_and_get_signature_shares(
        &mut self,
        _input: &mut dyn BufRead,
        _output: &mut dyn Write,
        signing_package: &SigningPackage<C>,
    ) -> Result<BTreeMap<Identifier<C>, SignatureShare<C>>, Box<dyn Error>> {
        eprintln!("Sending SigningPackage to participants...");
        let cipher = self
            .cipher
            .as_mut()
            .expect("cipher must have been set before");
        let send_signing_package_config = SendSigningPackageArgs {
            signing_package: vec![signing_package.clone()],
            aux_msg: Default::default(),
        };

        // We need to send a message separately for each recipient even if the
        // message is the same, because they are (possibly) encrypted
        // individually for each recipient.
        let pubkeys: Vec<_> = self.pubkeys.keys().cloned().collect();
        for recipient in pubkeys {
            let msg = cipher.encrypt(
                Some(&recipient),
                serde_json::to_vec(&send_signing_package_config)?,
            )?;
            let _r = self
                .client
                .send(&api::SendArgs {
                    session_id: self.session_id.unwrap(),
                    recipients: vec![recipient.clone()],
                    msg,
                })
                .await?;
        }

        eprintln!("Waiting for participants to send their SignatureShares...");

        let mut seen_share_senders: HashSet<api::PublicKey> = HashSet::new();
        loop {
            let r = self
                .client
                .receive(&api::ReceiveArgs {
                    session_id: self.session_id.unwrap(),
                    as_coordinator: true,
                })
                .await?;
            for msg in r.msgs {
                if seen_share_senders.contains(&msg.sender) {
                    continue;
                }
                let sender = msg.sender.clone();
                match cipher.decrypt(msg) {
                    Ok(msg) => {
                        self.state.recv(msg)?;
                        seen_share_senders.insert(sender);
                    }
                    Err(_) if self.commitment_senders.contains(&sender) => {
                        // A duplicate participant process re-joined this session
                        // and sent a fresh Noise handshake. The Noise state for
                        // this sender is already in transport mode so decryption
                        // fails. Ignore — the real signature share will arrive
                        // from the original process.
                    }
                    Err(e) => return Err(e.into()),
                }
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
            eprint!(".");
            if self.state.has_signature_shares() {
                break;
            }
        }
        eprintln!();

        let _r = self
            .client
            .close_session(&api::CloseSessionArgs {
                session_id: self.session_id.unwrap(),
            })
            .await?;

        let _r = self.client.logout().await?;

        let signature_shares = self.state.signature_shares()?;

        // TODO: support more than 1
        Ok(signature_shares[0].clone())
    }

    async fn cleanup_on_error(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(session_id) = self.session_id {
            let _r = self
                .client
                .close_session(&api::CloseSessionArgs { session_id })
                .await?;
        }
        Ok(())
    }
}
