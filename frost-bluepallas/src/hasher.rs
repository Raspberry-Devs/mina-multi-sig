use ark_ff::{BigInteger, PrimeField};
use frost_core::Field;
use mina_hasher::{create_legacy, Hashable, Hasher, ROInput};

use crate::PallasScalarField;

#[derive(Clone, Debug)]
struct PallasHashElement<'a> {
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
pub fn hash_to_array(input: &[&[u8]]) -> [u8; 32] {
    let scalar = hash_to_scalar(input);

    let bytes_be = scalar.into_bigint().to_bytes_be();
    let mut out = [0u8; 32];
    out[32 - bytes_be.len()..].copy_from_slice(&bytes_be);
    out
}
