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
use ark_ec::{models::CurveConfig, Group as ArkGroup};

use ark_ff::fields::Field as ArkField;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use frost_core::{Ciphersuite, Field, FieldError, Group, GroupError};
use mina_curves::pasta::{PallasParameters, ProjectivePallas};
use num_traits::identities::Zero;
use rand_core::{CryptoRng, RngCore};

pub type Error = frost_core::Error<PallasPoseidon>;

#[derive(Clone, Copy)]
pub struct PallasScalarField;

impl frost_core::Field for PallasScalarField {
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
        unimplemented!()
    }
    fn serialize(scalar: &Self::Scalar) -> Self::Serialization {
        unimplemented!()
    }

    fn little_endian_serialize(scalar: &Self::Scalar) -> Self::Serialization {
        unimplemented!()
    }

    fn deserialize(buf: &Self::Serialization) -> Result<Self::Scalar, FieldError> {
        unimplemented!()
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
        let mut buf: Self::Serialization = [0u8; 96];
        // Does the size reduce below 96 bytes for compressed serialize, though that's probably
        // fine? Could try switching to compressed (de)serialize
        element
            .serialize_uncompressed(&mut buf[..])
            .map_err(|_| GroupError::MalformedElement)?;
        // realistically an error never occurs so I just picked the most sensible variant of the
        // `GroupError` enum

        // TODO for some reason redpallas implmenetation disallows serialization of identity
        // But this is fine in the projective representation?
        return Ok(buf);
    }
    fn deserialize(buf: &Self::Serialization) -> Result<Self::Element, GroupError> {
        <Self::Element as CanonicalDeserialize>::deserialize_compressed(&buf[..])
            .map_err(|_| GroupError::MalformedElement)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PallasPoseidon;

impl Ciphersuite for PallasPoseidon {
    const ID: &'static str = "FROST(Pallas, Poseidon)";

    type Group = PallasGroup;
    type HashOutput = [u8; 64]; // probably wrong

    type SignatureSerialization = [u8; 64]; // probably wrong
    fn H1(m: &[u8]) -> <<Self::Group as Group>::Field as Field>::Scalar {
        unimplemented!()
    }
    fn H2(m: &[u8]) -> <<Self::Group as Group>::Field as Field>::Scalar {
        unimplemented!()
    }
    fn H3(m: &[u8]) -> <<Self::Group as Group>::Field as Field>::Scalar {
        unimplemented!()
    }
    fn H4(m: &[u8]) -> Self::HashOutput {
        unimplemented!()
    }
    fn H5(m: &[u8]) -> Self::HashOutput {
        unimplemented!()
    }
}
