use std::rc::Rc;

use crate::cipher::{PrivateKey, PublicKey};
use frost_core::{keys::KeyPackage, Ciphersuite};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Configuration for the participant in the FROST signing protocol.
#[derive(Clone, Zeroize)]
pub struct Config<C: Ciphersuite> {
    // Web Sockets mode. If enabled, it will use Web Sockets communication with a
    // FROST server. Otherwise http mode is used.
    pub socket: bool,

    /// Key package to use.
    pub key_package: KeyPackage<C>,

    /// IP to bind to, if using socket comms.
    /// IP to connect to, if using HTTP mode.
    pub ip: String,

    /// Port to bind to, if using socket comms.
    /// Port to connect to, if using HTTP mode.
    pub port: u16,

    /// Optional Session ID
    pub session_id: String,

    /// The participant's communication private key for HTTP mode.
    pub comm_privkey: Option<PrivateKey>,

    /// The participant's communication public key for HTTP mode.
    pub comm_pubkey: Option<PublicKey>,

    /// A function that confirms that a public key from the server is trusted by
    /// the user; returns the same public key. For HTTP mode.
    // It is a `Rc<dyn Fn>` to make it easier to use;
    // using `fn()` would preclude using closures and using generics would
    // require a lot of code change for something simple.
    #[allow(clippy::type_complexity)]
    #[zeroize(skip)]
    pub comm_coordinator_pubkey_getter: Option<Rc<dyn Fn(&PublicKey) -> Option<PublicKey>>>,
}

impl<C> ZeroizeOnDrop for Config<C> where C: Ciphersuite {}
