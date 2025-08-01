pub mod http;

use crate::cipher::PublicKey;
use frost_core::{
    self as frost,
    keys::dkg::{round1, round2},
    Ciphersuite,
};

use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
    io::{BufRead, Write},
};

use async_trait::async_trait;

use frost::Identifier;

#[async_trait(?Send)]
/// Communication abstraction for FROST distributed key generation (DKG)
///
/// This trait defines the interface for coordinating DKG protocol messages between
/// participants. It abstracts the underlying communication mechanism (HTTP, P2P, etc.)
/// and provides a consistent API for the DKG protocol implementation.
///
/// # Protocol Flow
///
/// The DKG process follows this communication pattern:
/// 1. **Setup**: Establish participant identifiers and group size
/// 2. **Round 1**: Exchange commitment packages via echo broadcast
/// 3. **Round 2**: Exchange secret share packages securely
/// 4. **Cleanup**: Handle any errors or cleanup tasks
pub trait Comms<C: Ciphersuite> {
    /// Return this participant's identifier (in case it's derived from other
    /// information) and the number of participants in the signing session.
    async fn get_identifier_and_max_signers(
        &mut self,
        input: &mut dyn BufRead,
        output: &mut dyn Write,
    ) -> Result<(Identifier<C>, u16), Box<dyn Error>>;

    /// Send the Round 1 package to other participants (using echo broadcast),
    /// and receive their Round 1 packages.
    async fn get_round1_packages(
        &mut self,
        input: &mut dyn BufRead,
        output: &mut dyn Write,
        round1_package: round1::Package<C>,
    ) -> Result<BTreeMap<Identifier<C>, round1::Package<C>>, Box<dyn Error>>;

    /// Send the Round 2 packages to other participants, and receive their Round
    /// 2 packages.
    async fn get_round2_packages(
        &mut self,
        input: &mut dyn BufRead,
        output: &mut dyn Write,
        round2_packages: BTreeMap<Identifier<C>, round2::Package<C>>,
    ) -> Result<BTreeMap<Identifier<C>, round2::Package<C>>, Box<dyn Error>>;

    /// Return the map of public keys to identifiers for all participants.
    fn get_pubkey_identifier_map(
        &self,
    ) -> Result<HashMap<PublicKey, Identifier<C>>, Box<dyn Error>>;

    /// Do any cleanups in case an error occurs during the protocol run.
    async fn cleanup_on_error(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
