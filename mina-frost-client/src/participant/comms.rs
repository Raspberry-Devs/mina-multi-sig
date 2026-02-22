pub mod http;

use async_trait::async_trait;
use eyre::eyre;
use mina_tx::TransactionEnvelope;

use crate::api::SendSigningPackageArgs;
use frost_core::{self as frost, Ciphersuite};

use std::{
    error::Error,
    io::{BufRead, Write},
};

use frost::{
    round1::SigningCommitments,
    round2::SignatureShare,
    serde::{self, Deserialize, Serialize},
    Identifier,
};

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
        signing_package: frost::SigningPackage<C>,
        network_id: u8,
    },
    SignatureShare(SignatureShare<C>),
}

/// Trait for communication with the server in the FROST protocol.
#[async_trait(?Send)]
pub trait Comms<C: Ciphersuite> {
    /// Get the signing package from the server.
    async fn get_signing_package(
        &mut self,
        input: &mut dyn BufRead,
        output: &mut dyn Write,
        commitments: SigningCommitments<C>,
        identifier: Identifier<C>,
    ) -> Result<SendSigningPackageArgs<C>, Box<dyn Error>>;

    /// Ask the user if they want to sign the message.
    ///
    /// Implementations should show the message to the user (or auxiliary data
    /// that maps to the message) and ask for confirmation.
    ///
    /// The default implementation prints the message to output and reads
    /// confirmation from input.
    async fn confirm_message(
        &mut self,
        input: &mut dyn BufRead,
        output: &mut dyn Write,
        signing_package: &SendSigningPackageArgs<C>,
        yes: bool,
    ) -> Result<(), Box<dyn Error>> {
        if yes {
            return Ok(());
        }

        let payload_bytes = signing_package
            .signing_package
            .first()
            .ok_or_else(|| eyre!("No signing package found"))?
            .message();

        let transaction = TransactionEnvelope::deserialize(payload_bytes).map_err(|err| {
            eyre!(
                "failed to decode signing payload as TransactionEnvelope: {err}. \
                 expected coordinator payload format is serialized TransactionEnvelope JSON bytes"
            )
        })?;
        writeln!(
            output,
            "Message to be signed (json):\n{}\nDo you want to sign it? (y/n)\n",
            transaction
        )?;

        let mut sign_it = String::new();
        input.read_line(&mut sign_it)?;
        if sign_it.trim() != "y" {
            return Err(eyre!("signing cancelled").into());
        }
        Ok(())
    }

    /// Send the signature share to the server.
    async fn send_signature_share(
        &mut self,
        identifier: Identifier<C>,
        signature_share: SignatureShare<C>,
    ) -> Result<(), Box<dyn Error>>;
}
