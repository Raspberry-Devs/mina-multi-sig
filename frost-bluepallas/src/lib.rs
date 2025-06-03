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

use ark_ec::{models::CurveConfig, Group as ArkGroup};

use ark_ff::{fields::Field as ArkField, BigInteger, PrimeField, UniformRand};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use frost_core::{self as frost, Ciphersuite, Field, FieldError, Group, GroupError};
use mina_curves::pasta::{PallasParameters, ProjectivePallas};
use num_traits::identities::Zero;
use rand_core::{CryptoRng, RngCore};

pub type Error = frost_core::Error<PallasPoseidonBlake2b>;

use blake2::{
    digest::{Update, VariableOutput},
    Blake2bVar,
};

#[derive(Clone, Copy)]
pub struct PallasScalarField;

impl Field for PallasScalarField {
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
        // Convert the field element to its big integer representation …
        let bytes_be = scalar.into_bigint().to_bytes_be();
        // … and left-pad to a full 32-byte array.
        let mut out = [0u8; 32];
        out[32 - bytes_be.len()..].copy_from_slice(&bytes_be);
        out
    }

    fn little_endian_serialize(scalar: &Self::Scalar) -> Self::Serialization {
        let bytes_le = scalar.into_bigint().to_bytes_le();
        let mut out = [0u8; 32];
        out[..bytes_le.len()].copy_from_slice(&bytes_le);
        out
    }

    // Parse the canonical 32-byte big-endian form back into a field element,
    fn deserialize(buf: &Self::Serialization) -> Result<Self::Scalar, FieldError> {
        let scalar = <Self::Scalar as PrimeField>::from_be_bytes_mod_order(buf);
        Ok(scalar)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PallasGroup {}

impl Group for PallasGroup {
    type Element = ProjectivePallas;
    type Field = PallasScalarField;
    type Serialization = [u8; 32 * 3]; // Projective Pallas is a struct with 3 of PallasScalarField

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

// Define the ciphersuite for Pallas with Poseidon and Blake2b as the hash function
// https://github.com/MinaProtocol/mina/blob/master/docs/specs/signatures/description.md
const CONTEXT_STRING: &str = "FROST-PALLAS-POSEIDON-BLAKE2b-v1";
const HASH_SIZE: usize = 32; // Blake2b output size

fn blake2b_hash_to_array(input: &[&[u8]]) -> [u8; HASH_SIZE] {
    let mut hasher =
        Blake2bVar::new(HASH_SIZE).expect("Blake2bVar should be initialized with a valid size");
    for i in input {
        hasher.update(i);
    }
    let mut output = [0u8; HASH_SIZE];
    hasher
        .finalize_variable(&mut output)
        .expect("Blake2bVar should finalize without error");
    output
}

fn blake2b_hash_to_scalar(input: &[&[u8]]) -> <<PallasGroup as Group>::Field as Field>::Scalar {
    let mut output = blake2b_hash_to_array(input);
    // Copied from https://github.com/o1-labs/proof-systems/blob/55219b0fc6ec589041545ae9470dd1edb29e3e02/signer/src/schnorr.rs#L131C9-L135C14
    output[output.len() - 1] &= 0b0011_1111;

    // Deserialize the output into a scalar field element
    PallasScalarField::deserialize(&output).expect("Blake2b output should be a valid scalar")
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PallasPoseidonBlake2b;

impl Ciphersuite for PallasPoseidonBlake2b {
    const ID: &'static str = CONTEXT_STRING;

    type Group = PallasGroup;
    type HashOutput = [u8; HASH_SIZE];

    type SignatureSerialization = [u8; HASH_SIZE];
    fn H1(m: &[u8]) -> <<Self::Group as Group>::Field as Field>::Scalar {
        blake2b_hash_to_scalar(&[CONTEXT_STRING.as_bytes(), b"rho", m])
    }
    fn H2(m: &[u8]) -> <<Self::Group as Group>::Field as Field>::Scalar {
        // THIS WILL NEED TO BE CHANGED
        // to use Poseidon hash function
        blake2b_hash_to_scalar(&[CONTEXT_STRING.as_bytes(), b"chal", m])
    }
    fn H3(m: &[u8]) -> <<Self::Group as Group>::Field as Field>::Scalar {
        blake2b_hash_to_scalar(&[CONTEXT_STRING.as_bytes(), b"nonce", m])
    }
    fn H4(m: &[u8]) -> Self::HashOutput {
        blake2b_hash_to_array(&[CONTEXT_STRING.as_bytes(), b"msg", m])
    }
    fn H5(m: &[u8]) -> Self::HashOutput {
        blake2b_hash_to_array(&[CONTEXT_STRING.as_bytes(), b"com", m])
    }

    fn HDKG(m: &[u8]) -> Option<<<Self::Group as Group>::Field as Field>::Scalar> {
        Some(blake2b_hash_to_scalar(&[
            CONTEXT_STRING.as_bytes(),
            b"dkg",
            m,
        ]))
    }

    fn HID(m: &[u8]) -> Option<<<Self::Group as Group>::Field as Field>::Scalar> {
        Some(blake2b_hash_to_scalar(&[
            CONTEXT_STRING.as_bytes(),
            b"id",
            m,
        ]))
    }
}

// Simply type alias for the FROST ciphersuite using Pallas with Poseidon and Blake2b
pub type P = PallasPoseidonBlake2b;

// A PallasPoseidonBlake2b identifier
pub type Identifier = frost::Identifier<P>;

pub mod keys {
    use super::*;

    pub type IdentifierList<'a> = frost::keys::IdentifierList<'a, P>;

    /// Secret and public key material generated by a dealer performing
    /// [`generate_with_dealer`].
    ///
    /// # Security
    ///
    /// To derive a FROST keypair, the receiver of the [`SecretShare`] *must* call
    /// .into(), which under the hood also performs validation.
    pub type SecretShare = frost::keys::SecretShare<P>;

    /// A secret scalar value representing a signer's share of the group secret.
    pub type SigningShare = frost::keys::SigningShare<P>;

    /// A public group element that represents a single signer's public verification share.
    pub type VerifyingShare = frost::keys::VerifyingShare<P>;

    /// A FROST(ristretto255, SHA-512) keypair, which can be generated either by a trusted dealer or using
    /// a DKG.
    ///
    /// When using a central dealer, [`SecretShare`]s are distributed to
    /// participants, who then perform verification, before deriving
    /// [`KeyPackage`]s, which they store to later use during signing.
    pub type KeyPackage = frost::keys::KeyPackage<P>;

    /// Public data that contains all the signers' public keys as well as the
    /// group public key.
    ///
    /// Used for verification purposes before publishing a signature.
    pub type PublicKeyPackage = frost::keys::PublicKeyPackage<P>;

    /// Contains the commitments to the coefficients for our secret polynomial _f_,
    /// used to generate participants' key shares.
    ///
    /// [`VerifiableSecretSharingCommitment`] contains a set of commitments to the coefficients (which
    /// themselves are scalars) for a secret polynomial f, where f is used to
    /// generate each ith participant's key share f(i). Participants use this set of
    /// commitments to perform verifiable secret sharing.
    ///
    /// Note that participants MUST be assured that they have the *same*
    /// [`VerifiableSecretSharingCommitment`], either by performing pairwise comparison, or by using
    /// some agreed-upon public location for publication, where each participant can
    /// ensure that they received the correct (and same) value.
    pub type VerifiableSecretSharingCommitment = frost::keys::VerifiableSecretSharingCommitment<P>;

    pub fn generate_with_dealer<RNG: RngCore + CryptoRng>(
        max_signers: u16,
        min_signers: u16,
        identifiers: IdentifierList,
        mut rng: RNG,
    ) -> Result<(BTreeMap<Identifier, SecretShare>, PublicKeyPackage), Error> {
        frost::keys::generate_with_dealer(max_signers, min_signers, identifiers, &mut rng)
    }
}
