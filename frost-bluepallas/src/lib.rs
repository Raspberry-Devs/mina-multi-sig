//! Information sharing:
//! https://github.com/o1-labs/proof-systems defines tools for interfacing with the mina blockchain
//! (pretty sure that) the actual internals of the mina blockchain such as signature verification for
//! contracts with the `signature` permission happens through the OCaml implementation.
//!
//! There are 3 relevant crates in the proof-systems, `signer` which uses `hasher` and `curves`
//! Do not use the `pasta-curves` from crates.io. That's different implementation of pasta by the
//! ZCash Foundation (the won't match up nicely). The above 3 crates are not on crates.io and are
//! used directly from github.
//!
//! The goal is to replace the functionality of `signer` with the implementation of `frost-core`
//! found in this file! So the tests will generate a signature with our implementation and try to
//! verify it with the `signer`'s verify method. We do not use `signer` at all in our
//! implementation. We do use `hasher` which provides the hash functions used by `signer` and our
//! implementation of `frost-core`.

extern crate alloc;

use alloc::collections::BTreeMap;

use ark_ec::{models::CurveConfig, CurveGroup, Group as ArkGroup};

use ark_ff::{fields::Field as ArkField, UniformRand};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
pub use frost_core::{self as frost, Ciphersuite, Field, FieldError, Group, GroupError};
use mina_curves::pasta::{PallasParameters, ProjectivePallas};

use num_traits::identities::Zero;
use rand_core::{CryptoRng, RngCore};

pub type Error = frost_core::Error<PallasPoseidon>;

use crate::{
    hasher::{hash_to_array, hash_to_scalar, message_hash, PallasMessage},
    translate::translate_pk,
};

pub mod hasher;
pub mod keys;
pub mod translate;

#[derive(Clone, Copy)]
pub struct PallasScalarField;

impl Field for PallasScalarField {
    // Equivalent to Fq in mina::curves::pasta i.e. the scalar field of the Pallas curve
    type Scalar = <PallasParameters as CurveConfig>::ScalarField;
    type Serialization = [u8; 32];
    fn zero() -> Self::Scalar {
        <Self::Scalar as ArkField>::ZERO
    }
    fn one() -> Self::Scalar {
        <Self::Scalar as ArkField>::ONE
    }
    fn invert(scalar: &Self::Scalar) -> Result<Self::Scalar, FieldError> {
        <Self::Scalar as ArkField>::inverse(scalar).ok_or(FieldError::InvalidZeroScalar)
    }
    fn random<R: RngCore + CryptoRng>(rng: &mut R) -> Self::Scalar {
        Self::Scalar::rand(rng)
    }

    fn serialize(scalar: &Self::Scalar) -> Self::Serialization {
        // Serialize the scalar in compressed form
        let mut buf = [0u8; 32];
        scalar
            .serialize_compressed(&mut buf[..])
            .map_err(|_| FieldError::MalformedScalar)
            .expect("Serialization should not fail for valid scalars");

        buf
    }

    fn little_endian_serialize(scalar: &Self::Scalar) -> Self::Serialization {
        Self::serialize(scalar)
    }

    // Parse the canonical 32-byte big-endian form back into a field element,
    fn deserialize(buf: &Self::Serialization) -> Result<Self::Scalar, FieldError> {
        let scalar = <Self::Scalar as CanonicalDeserialize>::deserialize_compressed(&buf[..])
            .map_err(|_| FieldError::MalformedScalar)?;
        Ok(scalar)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PallasGroup {}

impl Group for PallasGroup {
    type Element = ProjectivePallas;
    type Field = PallasScalarField;
    type Serialization = [u8; 32 * 3]; // Projective Pallas is a struct with 3 of PallasBaseField

    fn cofactor() -> <Self::Field as Field>::Scalar {
        Self::Field::one()
    }
    fn identity() -> Self::Element {
        Self::Element::zero()
    }
    fn generator() -> Self::Element {
        <Self::Element as ArkGroup>::generator()
    }
    fn serialize(element: &Self::Element) -> Result<Self::Serialization, GroupError> {
        // Ensure that the element is not the identity element
        // The FROST protocol requires that the identity element is never serialized or used in computations
        if element.is_zero() {
            return Err(GroupError::InvalidIdentityElement);
        }

        let mut buf: Self::Serialization = [0u8; 96];
        // Does the size reduce below 96 bytes for compressed serialize, though that's probably
        // fine? Could try switching to compressed (de)serialize
        element
            .serialize_compressed(&mut buf[..])
            .map_err(|_| GroupError::MalformedElement)?;

        Ok(buf)
    }
    fn deserialize(buf: &Self::Serialization) -> Result<Self::Element, GroupError> {
        let point = <Self::Element as CanonicalDeserialize>::deserialize_compressed(&buf[..])
            .map_err(|_| GroupError::MalformedElement);

        // Ensure that the deserialized point is not the identity element
        match point {
            Ok(p) if p.is_zero() => Err(GroupError::InvalidIdentityElement),
            Ok(p) => Ok(p),
            Err(_) => Err(GroupError::MalformedElement),
        }
    }
}

// Define the ciphersuite for Pallas with Poseidon as the hash function
// https://github.com/MinaProtocol/mina/blob/master/docs/specs/signatures/description.md
const CONTEXT_STRING: &str = "bluepallas";
const HASH_SIZE: usize = 32; // Posiedon hash output size

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PallasPoseidon {}

impl Ciphersuite for PallasPoseidon {
    const ID: &'static str = CONTEXT_STRING;

    type Group = PallasGroup;
    type HashOutput = [u8; HASH_SIZE];

    type SignatureSerialization = [u8; HASH_SIZE];
    fn H1(m: &[u8]) -> <<Self::Group as Group>::Field as Field>::Scalar {
        hash_to_scalar(&[CONTEXT_STRING.as_bytes(), b"rho", m])
    }
    fn H2(_m: &[u8]) -> <<Self::Group as Group>::Field as Field>::Scalar {
        unimplemented!("H2 is not implemented on purpose, please see the `challenge` function");
    }
    fn H3(m: &[u8]) -> <<Self::Group as Group>::Field as Field>::Scalar {
        hash_to_scalar(&[CONTEXT_STRING.as_bytes(), b"nonce", m])
    }
    fn H4(m: &[u8]) -> Self::HashOutput {
        hash_to_array(&[CONTEXT_STRING.as_bytes(), b"msg", m])
    }
    fn H5(m: &[u8]) -> Self::HashOutput {
        hash_to_array(&[CONTEXT_STRING.as_bytes(), b"com", m])
    }

    fn HDKG(m: &[u8]) -> Option<<<Self::Group as Group>::Field as Field>::Scalar> {
        Some(hash_to_scalar(&[CONTEXT_STRING.as_bytes(), b"dkg", m]))
    }

    fn HID(m: &[u8]) -> Option<<<Self::Group as Group>::Field as Field>::Scalar> {
        Some(hash_to_scalar(&[CONTEXT_STRING.as_bytes(), b"id", m]))
    }

    #[allow(non_snake_case)]
    fn challenge(
        R: &frost_core::Element<Self>,
        verifying_key: &frost_core::VerifyingKey<Self>,
        message: &[u8],
    ) -> Result<frost_core::Challenge<Self>, frost_core::Error<Self>> {
        // Convert public key and R to the Mina format
        let mina_pk = translate_pk(verifying_key).unwrap();
        let rx = R.into_affine().x;
        let mina_msg = PallasMessage(message.to_vec());

        let scalar = message_hash(&mina_pk, rx, &mina_msg);

        Ok(frost_core::Challenge::from_scalar(scalar))
    }
}

// Simply type alias for the FROST ciphersuite using Pallas with Poseidon
pub type P = PallasPoseidon;

// A PallasPoseidon identifier
pub type Identifier = frost::Identifier<P>;

/// FROST(Pallas, Posiedon) Round 1 functionality and types.
pub mod round1 {

    use crate::keys::SigningShare;

    use super::*;
    /// Comprised of FROST(Pallas, Posiedon) hiding and binding nonces.
    ///
    /// Note that [`SigningNonces`] must be used *only once* for a signing
    /// operation; re-using nonces will result in leakage of a signer's long-lived
    /// signing key.
    pub type SigningNonces = frost::round1::SigningNonces<P>;

    /// Published by each participant in the first round of the signing protocol.
    ///
    /// This step can be batched if desired by the implementation. Each
    /// SigningCommitment can be used for exactly *one* signature.
    pub type SigningCommitments = frost::round1::SigningCommitments<P>;

    /// A commitment to a signing nonce share.
    pub type NonceCommitment = frost::round1::NonceCommitment<P>;

    /// Performed once by each participant selected for the signing operation.
    ///
    /// Generates the signing nonces and commitments to be used in the signing
    /// operation.
    pub fn commit<RNG>(secret: &SigningShare, rng: &mut RNG) -> (SigningNonces, SigningCommitments)
    where
        RNG: CryptoRng + RngCore,
    {
        frost::round1::commit::<P, RNG>(secret, rng)
    }
}

/// Generated by the coordinator of the signing operation and distributed to
/// each signing party.
pub type SigningPackage = frost::SigningPackage<P>;

/// FROST(Pallas, Posiedon) Round 2 functionality and types, for signature share generation.
pub mod round2 {
    use super::*;

    /// A FROST(Pallas, Posiedon) participant's signature share, which the Coordinator will aggregate with all other signer's
    /// shares into the joint signature.
    pub type SignatureShare = frost::round2::SignatureShare<P>;

    /// Performed once by each participant selected for the signing operation.
    ///
    /// Receives the message to be signed and a set of signing commitments and a set
    /// of randomizing commitments to be used in that signing operation, including
    /// that for this participant.
    ///
    /// Assumes the participant has already determined which nonce corresponds with
    /// the commitment that was assigned by the coordinator in the SigningPackage.
    pub fn sign(
        signing_package: &SigningPackage,
        signer_nonces: &round1::SigningNonces,
        key_package: &keys::KeyPackage,
    ) -> Result<SignatureShare, Error> {
        frost::round2::sign(signing_package, signer_nonces, key_package)
    }
}

/// A Schnorr signature on FROST(Pallas, Posiedon).
pub type Signature = frost::Signature<P>;

/// Verifies each FROST(Pallas, Posiedon) participant's signature share, and if all are valid,
/// aggregates the shares into a signature to publish.
///
/// Resulting signature is compatible with verification of a plain Schnorr
/// signature.
///
/// This operation is performed by a coordinator that can communicate with all
/// the signing participants before publishing the final signature. The
/// coordinator can be one of the participants or a semi-trusted third party
/// (who is trusted to not perform denial of service attacks, but does not learn
/// any secret information). Note that because the coordinator is trusted to
/// report misbehaving parties in order to avoid publishing an invalid
/// signature, if the coordinator themselves is a signer and misbehaves, they
/// can avoid that step. However, at worst, this results in a denial of
/// service attack due to publishing an invalid signature.
pub fn aggregate(
    signing_package: &SigningPackage,
    signature_shares: &BTreeMap<Identifier, round2::SignatureShare>,
    pubkeys: &keys::PublicKeyPackage,
) -> Result<Signature, Error> {
    frost::aggregate(signing_package, signature_shares, pubkeys)
}

/// A signing key for a Schnorr signature on FROST(Pallas, Posiedon).
pub type SigningKey = frost::SigningKey<P>;

/// A valid verifying key for Schnorr signatures on FROST(Pallas, Posiedon).
pub type VerifyingKey = frost::VerifyingKey<P>;
