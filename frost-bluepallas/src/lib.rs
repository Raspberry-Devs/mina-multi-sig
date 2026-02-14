//! Information sharing:
//! <https://github.com/o1-labs/proof-systems> defines tools for interfacing with the mina blockchain
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
#![no_std]

#[macro_use]
extern crate alloc;

use alloc::{borrow::Cow, collections::BTreeMap};

use ark_ec::{models::CurveConfig, CurveGroup, PrimeGroup};

use ark_ff::{fields::Field as ArkField, UniformRand};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
pub use frost_core::{self as frost, Ciphersuite, Field, FieldError, Group, GroupError};
use frost_core::{compute_group_commitment, BindingFactorList};
use mina_curves::pasta::{PallasParameters, ProjectivePallas};

use num_traits::identities::Zero;
use rand_core::{CryptoRng, RngCore};

pub type Error = frost_core::Error<BluePallas>;

use crate::{
    hasher::{hash_to_array, hash_to_scalar, message_hash},
    mina_compat::{translate_pk, PallasMessage},
    negate::NegateY,
    round1::SigningNonces,
};

mod base58;
pub mod errors;
pub mod graphql;
pub mod hasher;
pub mod keys;
pub mod mina_compat;
mod negate;
pub mod signing_utilities;
pub mod transactions;

/// PallasScalarField implements the FROST field interface for the Pallas scalar field
#[derive(Clone, Copy)]
pub struct PallasScalarField;

impl Field for PallasScalarField {
    // Equivalent to Fq in mina::curves::pasta i.e. the scalar field of the Pallas curve
    type Scalar = <PallasParameters as CurveConfig>::ScalarField;
    type Serialization = [u8; 32];
    fn zero() -> Self::Scalar {
        Self::Scalar::zero()
    }
    fn one() -> Self::Scalar {
        Self::Scalar::ONE
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

/// PallasGroup implements the FROST group interface for the Pallas curve
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
        Self::Element::generator()
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
pub const CONTEXT_STRING: &str = "bluepallas";
const HASH_SIZE: usize = 32; // Posiedon hash output size

/// The BluePallas ciphersuite, which uses the Pallas curve and Poseidon hash function.
///
/// Note that this ciphersuite is used for FROST signatures in the Mina protocol and has a lot of Mina-specific logic
/// This library SHOULD not be treated as a general-purpose BluePallas ciphersuite, but rather as a Mina-specific implementation.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BluePallas {}

impl Ciphersuite for BluePallas {
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
        let mina_pk =
            translate_pk(verifying_key).map_err(|_| frost_core::FieldError::MalformedScalar)?;
        let rx = R.into_affine().x;

        // Attempt to derive the message as a TransactionEnvelope first, if that fails treat it as raw bytes
        let msg = PallasMessage::new(message.to_vec());
        let network_id = msg.network_id();
        let is_legacy = msg.is_legacy();

        let scalar = message_hash::<PallasMessage>(&mina_pk, rx, msg, network_id, is_legacy)
            .map_err(|_| frost_core::FieldError::MalformedScalar)?;

        Ok(frost_core::Challenge::from_scalar(scalar))
    }

    /// This performs the same functionality as [`Self::pre_commitment_sign`], but instead only
    /// negates commitments because the coordinator is not able to receive any nonces.
    /// Naturally, this is called by the coordinator in the [`crate::aggregate`] function.
    fn pre_commitment_aggregate<'a>(
        signing_package: &'a frost_core::SigningPackage<Self>,
        binding_factor_list: &'a BindingFactorList<Self>,
    ) -> Result<Cow<'a, frost_core::SigningPackage<Self>>, frost_core::Error<Self>> {
        use ark_ff::{BigInteger, PrimeField};
        // Compute the group commitment from signing commitments produced in round one.
        let commit = compute_group_commitment(signing_package, binding_factor_list)?;

        if commit.to_element().into_affine().y.into_bigint().is_even() {
            return Ok(Cow::Borrowed(signing_package));
        }

        // Otherwise negate the commitments
        let negated_commitments =
            <frost_core::SigningPackage<Self> as NegateY>::negate_y(signing_package);

        Ok(Cow::Owned(negated_commitments))
    }

    /// This functions computes the group commitment and checks whether the y-coordinate of the
    /// group commitment is even, as required by the Mina protocol.
    /// If the group commitment is not even, it negates the nonces and commitments
    /// This will be called by each individual signer during [`round2::sign`]
    fn pre_commitment_sign<'a>(
        signing_package: &'a frost_core::SigningPackage<Self>,
        signing_nonces: &'a frost_core::round1::SigningNonces<Self>,
        binding_factor_list: &'a BindingFactorList<Self>,
    ) -> Result<
        (
            Cow<'a, frost_core::SigningPackage<Self>>,
            Cow<'a, frost_core::round1::SigningNonces<Self>>,
        ),
        frost_core::Error<Self>,
    > {
        use ark_ff::{BigInteger, PrimeField};
        // Compute the group commitment from signing commitments produced in round one.
        let commit = compute_group_commitment(signing_package, binding_factor_list)?;

        if commit.to_element().into_affine().y.into_bigint().is_even() {
            return Ok((
                Cow::Borrowed(signing_package),
                Cow::Borrowed(signing_nonces),
            ));
        }

        // Otherwise negate the nonce that we know and all the commitments
        let negated_nonce =
            <frost_core::round1::SigningNonces<Self> as NegateY>::negate_y(signing_nonces);
        let negated_commitments =
            <frost_core::SigningPackage<Self> as NegateY>::negate_y(signing_package);

        Ok((Cow::Owned(negated_commitments), Cow::Owned(negated_nonce)))
    }
}

// Simply type alias for the FROST ciphersuite using Pallas with Poseidon
pub type P = BluePallas;

// A BluePallas identifier
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

    pub fn sign(
        signing_package: &SigningPackage,
        signer_nonces: &SigningNonces,
        key_package: &frost::keys::KeyPackage<P>,
    ) -> Result<SignatureShare, Error> {
        frost::round2::sign::<P>(signing_package, signer_nonces, key_package)
    }
}

/// A Schnorr signature on FROST(Pallas, Posiedon).
pub type Signature = frost::Signature<P>;

/// A signing key for a Schnorr signature on FROST(Pallas, Posiedon).
pub type SigningKey = frost::SigningKey<P>;

/// A valid verifying key for Schnorr signatures on FROST(Pallas, Posiedon).
pub type VerifyingKey = frost::VerifyingKey<P>;

pub fn aggregate(
    signing_package: &SigningPackage,
    signature_shares: &BTreeMap<Identifier, frost::round2::SignatureShare<P>>,
    pubkey_package: &frost::keys::PublicKeyPackage<P>,
) -> Result<Signature, Error> {
    frost::aggregate(signing_package, signature_shares, pubkey_package)
}
