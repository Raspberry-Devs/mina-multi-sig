use std::collections::HashMap;

use crate::cipher::{PrivateKey, PublicKey};
use crate::mina_network::Network;
use frost_core::{keys::PublicKeyPackage, Ciphersuite, Identifier};

#[derive(Clone)]
pub struct Config<C: Ciphersuite> {
    /// Use Web Socket communication if true. Otherwise, use HTTP.
    pub socket: bool,

    /// Signers to use in HTTP mode, as a map of public keys to identifiers.
    pub signers: HashMap<PublicKey, Identifier<C>>,

    /// The number of participants.
    pub num_signers: u16,

    /// Public key package to use.
    pub public_key_package: PublicKeyPackage<C>,

    /// The messages to sign.
    pub messages: Vec<Vec<u8>>,

    /// IP to bind to, if using socket comms.
    /// IP to connect to, if using HTTP mode.
    pub ip: String,

    /// Port to bind to, if using socket comms.
    /// Port to connect to, if using HTTP mode.
    pub port: u16,

    /// The coordinator's communication private key for HTTP mode.
    pub comm_privkey: Option<PrivateKey>,

    /// The coordinator's communication public key for HTTP mode.
    pub comm_pubkey: Option<PublicKey>,

    /// Network to use for signing.
    pub network: Network,
}
