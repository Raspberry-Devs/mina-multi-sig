pub mod http;
pub mod socket;

use frost_core::{self as frost, Ciphersuite};

use std::{
    collections::BTreeMap,
    error::Error,
    io::{BufRead, Write},
};

use async_trait::async_trait;

use frost::{
    keys::PublicKeyPackage,
    round1::SigningCommitments,
    round2::SignatureShare,
    serde::{self, Deserialize, Serialize},
    Identifier, SigningPackage,
};

/// Types of messages exchanged between coordinator and participants
#[derive(Serialize, Deserialize)]
#[serde(crate = "self::serde")]
#[serde(bound = "C: Ciphersuite")]
#[allow(clippy::large_enum_variant)]
pub enum Message<C: Ciphersuite> {
    IdentifiedCommitments {
        identifier: Identifier<C>,
        commitments: SigningCommitments<C>,
    },
    SigningPackage {
        signing_package: SigningPackage<C>,
    },
    SignatureShare(SignatureShare<C>),
}

/// Trait for communication between coordinator and participants
/// Implementations will most likely use forstd server
#[async_trait(?Send)]
pub trait Comms<C: Ciphersuite> {
    async fn get_signing_commitments(
        &mut self,
        input: &mut dyn BufRead,
        output: &mut dyn Write,
        pub_key_package: &PublicKeyPackage<C>,
        num_of_participants: u16,
    ) -> Result<BTreeMap<Identifier<C>, SigningCommitments<C>>, Box<dyn Error>>;

    async fn send_signing_package_and_get_signature_shares(
        &mut self,
        input: &mut dyn BufRead,
        output: &mut dyn Write,
        signing_package: &SigningPackage<C>,
    ) -> Result<BTreeMap<Identifier<C>, SignatureShare<C>>, Box<dyn Error>>;

    /// Do any cleanups in case an error occurs during the protocol run.
    async fn cleanup_on_error(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
