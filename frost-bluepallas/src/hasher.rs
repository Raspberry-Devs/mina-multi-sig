//! Mina-compatible hashing utilities for FROST using the Pallas curve.

use alloc::string::String;
use ark_ff::PrimeField;
use frost_core::Field;
use mina_hasher::{create_legacy, Hashable, Hasher, ROInput};

use crate::PallasScalarField;

/// This is a Hashable interface for an array of bytes
/// This allows us to provide a easy-to-read interface for hashing FROST elements in H1, H3, H4, H5
#[derive(Clone, Debug)]
pub(crate) struct PallasHashElement<'a> {
    value: &'a [&'a [u8]],
}

// Implement a hashable trait for a u8 slice
impl Hashable for PallasHashElement<'_> {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();

        for val in self.value {
            roi = roi.append_bytes(val);
        }

        roi
    }

    // As of right now, assume domain string is included in the input
    fn domain_string(_domain_param: Self::D) -> Option<String> {
        None
    }
}

type Fq = <PallasScalarField as Field>::Scalar;

// Maps poseidon hash of input to a scalar field element
pub fn hash_to_scalar(input: &[&[u8]]) -> Fq {
    let wrap = PallasHashElement { value: input };
    let mut hasher = create_legacy::<PallasHashElement>(());

    // Convert from base field to scalar field
    // This is performed in the mina-signer crate
    // https://github.com/o1-labs/proof-systems/blob/6d2ac796205456d314d7ea2a3db6e0e816d60a99/signer/src/schnorr.rs#L145-L158
    Fq::from(hasher.hash(&wrap).into_bigint())
}

// Maps poseidon hash of input to a 32-byte array
pub fn hash_to_array(input: &[&[u8]]) -> <PallasScalarField as frost_core::Field>::Serialization {
    let scalar = hash_to_scalar(input);

    PallasScalarField::serialize(&scalar)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_to_scalar_is_deterministic_and_differs() {
        let input = &[&b"abc"[..]];
        let s1 = hash_to_scalar(input);
        let s2 = hash_to_scalar(input);
        assert_eq!(s1, s2, "same input must yield same scalar");

        let other = &[&b"def"[..]];
        let s3 = hash_to_scalar(other);
        assert_ne!(s1, s3, "different input must yield a different scalar");
    }

    #[test]
    fn test_hash_to_array_length() {
        let arr = hash_to_array(&[&b"hello"[..]]);
        // Serialization for PallasScalarField is 32 bytes
        assert_eq!(arr.len(), 32);
    }
}
