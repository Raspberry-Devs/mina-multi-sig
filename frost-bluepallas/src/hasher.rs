//! Mina-compatible hashing utilities for FROST using the Pallas curve.

use alloc::string::{String, ToString};
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

const HASH_ELEMENT_STRING: &str = "PallasHashElement";

// Implement a hashable trait for a u8 slice
impl Hashable for PallasHashElement<'_> {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        let count_bytes = (self.value.len() as u64).to_le_bytes();
        roi = roi.append_bytes(&count_bytes);
        for segment in self.value {
            let len_bytes = (segment.len() as u64).to_le_bytes();
            roi = roi.append_bytes(&len_bytes);
            roi = roi.append_bytes(segment);
        }

        roi
    }

    // Use a fixed domain string for PallasHashElement hashing
    fn domain_string(_domain_param: Self::D) -> Option<String> {
        HASH_ELEMENT_STRING.to_string().into()
    }
}

type Fq = <PallasScalarField as Field>::Scalar;

// Maps poseidon hash of input to a scalar field element
pub fn hash_to_scalar(input: &[&[u8]]) -> Fq {
    // Hash via PallasHashElement, which length-prefixes the segment count and each segment
    // to prevent padding and segmentation-based collision attacks.
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

    #[test]
    fn test_padding_attack_resistance() {
        let base = &[&b"1"[..]];
        let padded = &[&b"1"[..], &[0u8][..]];
        let h_base = hash_to_scalar(base);
        let h_padded = hash_to_scalar(padded);
        assert_ne!(
            h_base, h_padded,
            "trailing zero-byte padding collides for this variable-length encoding",
        );
    }

    #[test]
    fn test_segment_boundary_collision() {
        let a = hash_to_scalar(&[b"ab", b"c"]);
        let b = hash_to_scalar(&[b"a", b"bc"]);
        assert_ne!(a, b, "different segmentation can yield the same hash");
    }
}
