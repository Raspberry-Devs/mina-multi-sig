// Used to prevent warning on `comm_participant_pubkey_getter` field due to zeroize(skip) macro
#![allow(unused_assignments)]

use std::rc::Rc;

use crate::cipher::{PrivateKey, PublicKey};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Configuration for distributed key generation (DKG) operations
///
/// This struct contains all the necessary parameters and cryptographic material
/// needed to participate in a FROST DKG protocol session. It includes network
/// configuration, participant authentication keys, and protocol parameters.
///
/// # Security Notes
///
/// - The struct implements `Zeroize` to securely clear sensitive data from memory
/// - Private keys are automatically zeroed when the struct is dropped
/// - The `comm_participant_pubkey_getter` function should verify participant authenticity
///
/// # Network Modes
///
/// Currently supports HTTP mode for communication between participants.
/// Future versions may support additional communication protocols.
#[derive(Clone, Zeroize)]
pub struct Config {
    /// IP to connect to (HTTP mode).
    pub ip: String,

    /// Port to connect to (HTTP mode).
    pub port: u16,

    /// The participant's communication private key (HTTP mode)
    pub comm_privkey: Option<PrivateKey>,

    /// The participant's communication public key (HTTP mode)
    pub comm_pubkey: Option<PublicKey>,

    /// A function that confirms that a public key from the server is trusted by
    /// the user; returns the same public key. (HTTP mode)
    // It is a `Rc<dyn Fn>` to make it easier to use;
    // using `fn()` would preclude using closures and using generics would
    // require a lot of code change for something simple.
    #[allow(clippy::type_complexity)]
    #[zeroize(skip)]
    pub comm_participant_pubkey_getter: Option<Rc<dyn Fn(&PublicKey) -> Option<PublicKey>>>,

    /// The threshold to use for the shares
    pub min_signers: u16,

    /// The list of pubkeys for the other participants. This is only required
    /// for the first participant who creates the DKG session.
    pub participants: Vec<PublicKey>,
}

impl ZeroizeOnDrop for Config {}
