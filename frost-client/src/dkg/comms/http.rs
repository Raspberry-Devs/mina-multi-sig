//! HTTP-based communication implementation for FROST DKG
//!
//! This module provides an HTTP client implementation of the `Comms` trait,
//! enabling FROST distributed key generation over HTTPS connections through
//! a coordination server.
//!
//! # Architecture
//!
//! The HTTP implementation uses a client-server model where:
//! - A coordination server facilitates message exchange between participants
//! - Each participant connects as an HTTP client to the server
//! - The server handles session management, message routing, and consistency
//!
//! # Security Features
//!
//! - **Authentication**: Participants authenticate using cryptographic signatures
//! - **Encryption**: Round 2 messages (secret shares) are encrypted end-to-end
//! - **Integrity**: All messages are signed and verified
//! - **Echo Broadcast**: Round 1 uses echo broadcast for consistency guarantees

use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
    io::{BufRead, Write},
    marker::PhantomData,
    time::Duration,
    vec,
};

use async_trait::async_trait;
use eyre::{eyre, OptionExt};
use frost_core::{
    keys::dkg::{round1, round2},
    Ciphersuite, Identifier,
};

use super::Comms;
use crate::client::Client;
use crate::{
    api::{self, PublicKey, Uuid},
    session::DKGSessionState,
};
use crate::{cipher::Cipher, dkg::config::Config};
use rand::thread_rng;

/// This struct manages HTTP client connections to a coordination server,
/// handling authentication, session management, and secure message exchange
/// between DKG participants.
pub struct HTTPComms<C: Ciphersuite> {
    client: Client,
    session_id: Option<Uuid>,
    config: Config,
    state: DKGSessionState<C>,
    identifier: Option<Identifier<C>>,
    pubkeys: HashMap<PublicKey, Identifier<C>>,
    cipher: Option<Cipher>,
    _phantom: PhantomData<C>,
}

impl<C: Ciphersuite> HTTPComms<C> {
    pub fn new(config: Config) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            client: Client::new(format!("https://{}:{}", config.ip, config.port)),
            session_id: None,
            config: config,
            state: DKGSessionState::default(),
            identifier: None,
            pubkeys: Default::default(),
            cipher: None,
            _phantom: Default::default(),
        })
    }
}

/// Implementation of the `Comms` trait for HTTP-based DKG communication
///
/// This implementation handles the HTTP-specific details of:
/// - Server authentication using challenge-response
/// - Session management and participant coordination
/// - Message encryption/decryption for sensitive data
/// - Error handling and cleanup for network failures
#[async_trait(?Send)]
impl<C: Ciphersuite + 'static> Comms<C> for HTTPComms<C> {
    /// Authenticate with server and establish DKG session participation
    async fn get_identifier_and_max_signers(
        &mut self,
        _input: &mut dyn BufRead,
        _output: &mut dyn Write,
    ) -> Result<(Identifier<C>, u16), Box<dyn Error>> {
        // Initialize cryptographically secure random number generator
        let mut rng = thread_rng();

        // --------------- Authentication with Challenge-Response-----------------
        eprintln!("Logging in...");
        // Request authentication challenge from server
        let challenge = self.client.challenge().await?.challenge;

        // Sign the challenge with our private communication key
        let signature: [u8; 64] = self
            .config
            .comm_privkey
            .clone()
            .ok_or_eyre("comm_privkey must be specified")?
            .sign(challenge.as_bytes(), &mut rng)?;

        // Get our public key for server verification
        let comm_pubkey = self
            .config
            .comm_pubkey
            .clone()
            .ok_or_eyre("comm_pubkey must be specified")?;

        // Submit signed challenge to complete authentication
        self.client
            .login(&api::LoginArgs {
                challenge,
                pubkey: comm_pubkey.clone(),
                signature: signature.to_vec(),
            })
            .await?;

        // -------------- Session Management --------------------
        let session_id = if !self.config.participants.is_empty() {
            // Coordinator role: Create new DKG session with specified participants
            eprintln!("Creating DKG session...");
            let r = self
                .client
                .create_new_session(&api::CreateNewSessionArgs {
                    pubkeys: self.config.participants.clone(),
                    message_count: 1, // DKG requires 1 message exchange per round
                })
                .await?;
            r.session_id
        } else {
            // Participant role: Join existing DKG session
            eprintln!("Joining DKG session...");
            match self.session_id {
                // Use explicitly provided session ID
                Some(s) => s,
                None => {
                    // Auto-discover session ID from server
                    let r = self.client.list_sessions().await?;
                    if r.session_ids.len() > 1 {
                        return Err(eyre!("user has more than one FROST session active; use `frost-client sessions` to list them and specify the session ID with `-S`").into());
                    } else if r.session_ids.is_empty() {
                        return Err(eyre!("User has no current sessions active").into());
                    }
                    r.session_ids[0]
                }
            }
        };
        // Store session ID for future API calls
        self.session_id = Some(session_id);

        // --------------- Participant Discovery and Identifier Derivation -----------------
        eprintln!("Getting session info...");
        // Retrieve the complete list of participants in this session
        let session_info = self
            .client
            .get_session_info(&api::GetSessionInfoArgs { session_id })
            .await?;

        // Derive deterministic FROST identifiers for all participants
        // Each identifier is computed as: derive(session_id || participant_pubkey)
        // This ensures all participants compute the same identifiers in the same order
        self.pubkeys = session_info
            .pubkeys
            .iter()
            .map(|p| {
                Ok((
                    p.clone(),
                    Identifier::<C>::derive(&[session_id.as_bytes(), &p.0[..]].concat())?,
                ))
            })
            .collect::<Result<_, frost_core::Error<C>>>()?;

        // Validate minimum participants for DKG security
        if self.pubkeys.len() < 2 {
            return Err(eyre!("DKG session must have at least 2 participants").into());
        }

        // Update internal protocol state with participant information
        match self.state {
            DKGSessionState::WaitingForRound1Packages {
                ref mut pubkeys, ..
            } => {
                *pubkeys = self.pubkeys.clone();
            }
            _ => unreachable!("wrong state"),
        }

        // -------------- Compute This Participant's Identifier -----------------
        // Use the same derivation method to compute our own identifier
        // This ensures consistency with how other participants compute our identifier
        let input = [session_id.as_bytes(), &comm_pubkey.0[..]].concat();
        let identifier = Identifier::<C>::derive(&input)?;
        self.identifier = Some(identifier);

        // Return our identifier and the total number of participants
        Ok((identifier, self.pubkeys.len() as u16))
    }

    /// Perform Round 1 message exchange using echo broadcast
    async fn get_round1_packages(
        &mut self,
        _input: &mut dyn BufRead,
        _output: &mut dyn Write,
        round1_package: round1::Package<C>,
    ) -> Result<BTreeMap<Identifier<C>, round1::Package<C>>, Box<dyn Error>> {
        let (Some(comm_privkey), Some(comm_participant_pubkey_getter)) = (
            &self.config.comm_privkey,
            &self.config.comm_participant_pubkey_getter,
        ) else {
            return Err(
                eyre!("comm_privkey and comm_participant_pubkey_getter must be specified").into(),
            );
        };

        let cipher = Cipher::new(
            comm_privkey.clone(),
            self.pubkeys.keys().map(|pubkey| comm_participant_pubkey_getter(pubkey).ok_or_eyre(
                "A participant in specified FROST session is not registered in the user's address book"
            )).collect::<Result<_,_>>()?,
        )?;
        self.cipher = Some(cipher);
        let cipher = self.cipher.as_mut().expect("was just set");

        // Send Round 1 Package to all other participants
        for pubkey in self.pubkeys.clone().keys() {
            if Some(pubkey) == self.config.comm_pubkey.as_ref() {
                continue;
            }
            let msg = cipher.encrypt(Some(pubkey), serde_json::to_vec(&round1_package)?)?;
            self.client
                .send(&api::SendArgs {
                    session_id: self.session_id.expect("set before"),
                    recipients: vec![pubkey.clone()],
                    msg,
                })
                .await?;
        }

        eprint!("Waiting for other participants to send their Round 1 Packages...");

        loop {
            let r = self
                .client
                .receive(&api::ReceiveArgs {
                    session_id: self.session_id.unwrap(),
                    as_coordinator: false,
                })
                .await?;
            for msg in r.msgs {
                let msg = cipher.decrypt(msg)?;
                self.state
                    .recv(msg, self.identifier.expect("must have been set"))?;
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
            eprint!(".");
            if self.state.has_round1_packages() {
                break;
            }
        }
        eprintln!();

        // We might need to skip the echo broadcast if its not needed (e.g.
        // only 2 participants)
        if !self.state.has_round1_broadcast_packages() {
            // Broadcast received Round 1 Packages to all other participants
            for (recipient_pubkey, recipient_identifier) in self.pubkeys.clone().iter() {
                // No need to broadcast to oneself
                if Some(recipient_pubkey) == self.config.comm_pubkey.as_ref() {
                    continue;
                }
                for (sender_identifier, package) in self.state.round1_packages()?.iter() {
                    // No need to broadcast back to the sender
                    if *sender_identifier == *recipient_identifier {
                        continue;
                    }
                    let msg = cipher.encrypt(
                        Some(recipient_pubkey),
                        serde_json::to_vec(&(*sender_identifier, package))?,
                    )?;
                    self.client
                        .send(&api::SendArgs {
                            session_id: self.session_id.expect("set before"),
                            recipients: vec![recipient_pubkey.clone()],
                            msg,
                        })
                        .await?;
                }
            }

            eprint!("Waiting for other participants to send their broadcasted Round 1 Packages...");

            loop {
                let r = self
                    .client
                    .receive(&api::ReceiveArgs {
                        session_id: self.session_id.unwrap(),
                        as_coordinator: false,
                    })
                    .await?;
                for msg in r.msgs {
                    let msg = cipher.decrypt(msg)?;
                    self.state
                        .recv(msg, self.identifier.expect("must have been set"))?;
                }
                tokio::time::sleep(Duration::from_secs(2)).await;
                eprint!(".");
                if self.state.has_round1_broadcast_packages() {
                    break;
                }
            }
            eprintln!();
        }

        self.state.round1_packages()
    }

    /// Perform Round 2 message exchange with secure delivery
    async fn get_round2_packages(
        &mut self,
        _input: &mut dyn BufRead,
        _output: &mut dyn Write,
        round2_packages: BTreeMap<Identifier<C>, round2::Package<C>>,
    ) -> Result<BTreeMap<Identifier<C>, round2::Package<C>>, Box<dyn Error>> {
        let cipher = self.cipher.as_mut().expect("was just set");
        // Send Round 2 Packages to all other participants
        for (pubkey, identifier) in self.pubkeys.clone().into_iter() {
            if Some(&pubkey) == self.config.comm_pubkey.as_ref() {
                continue;
            }
            let msg = cipher.encrypt(
                Some(&pubkey),
                serde_json::to_vec(
                    &round2_packages
                        .get(&identifier)
                        .ok_or_eyre("must have Round 2 Package for the given identifier")?,
                )?,
            )?;
            self.client
                .send(&api::SendArgs {
                    session_id: self.session_id.expect("set before"),
                    recipients: vec![pubkey.clone()],
                    msg,
                })
                .await?;
        }

        eprint!("Waiting for other participants to send their Round 2 Packages...");

        loop {
            let r = self
                .client
                .receive(&api::ReceiveArgs {
                    session_id: self.session_id.unwrap(),
                    as_coordinator: false,
                })
                .await?;
            for msg in r.msgs {
                let msg = cipher.decrypt(msg)?;
                self.state
                    .recv(msg, self.identifier.expect("must have been set"))?;
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
            eprint!(".");
            if self.state.has_round2_packages() {
                break;
            }
        }
        eprintln!();

        if !self.config.participants.is_empty() {
            let _r = self
                .client
                .close_session(&api::CloseSessionArgs {
                    session_id: self.session_id.unwrap(),
                })
                .await?;
        }

        let _r = self.client.logout().await?;

        self.state.round2_packages()
    }

    /// Retrieve the mapping between communication keys and FROST identifiers
    fn get_pubkey_identifier_map(
        &self,
    ) -> Result<HashMap<PublicKey, Identifier<C>>, Box<dyn Error>> {
        match &self.state {
            DKGSessionState::Round2PackagesReady { pubkeys, .. } => Ok(pubkeys.clone()),
            _ => Err(eyre!("wrong state").into()),
        }
    }

    /// Cleanup on error: close the session if it was created
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
