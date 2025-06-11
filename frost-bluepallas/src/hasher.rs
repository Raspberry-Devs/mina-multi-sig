use ark_ff::PrimeField;
use frost_core::{Ciphersuite, Field, Group};
use mina_hasher::{create_legacy, Hashable, Hasher, ROInput};
use mina_signer::NetworkId;

use crate::{PallasGroup, PallasPoseidon, PallasScalarField, VerifyingKey};

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

// This s temporary: it's easier to test that u8 slices as messages work correctly first
// We can implmement (or reuse) the Hashable trait for transaction as in this example afterwards:
// https://github.com/o1-labs/proof-systems/blob/master/signer/README.md?plain=1#L19-L40

#[derive(Clone, Debug)]
pub struct PallasMessage(pub Vec<u8>);

// Implement a hashable trait for a u8 slice
impl Hashable for PallasMessage {
    type D = NetworkId;

    fn to_roinput(&self) -> ROInput {
        ROInput::new().append_bytes(self.0.as_ref())
    }

    // copied from
    // https://github.com/o1-labs/proof-systems/blob/0.1.0/signer/tests/transaction.rs#L53-L61
    fn domain_string(network_id: NetworkId) -> Option<String> {
        // Domain strings must have length <= 20
        match network_id {
            NetworkId::MAINNET => "MinaSignatureMainnet",
            NetworkId::TESTNET => "CodaSignature", //"FROST-PALLAS-POSEIDON",
        }
        .to_string()
        .into()
    }
}

#[allow(non_snake_case)]
#[derive(Clone, Debug)]
pub struct Challenge<'a> {
    // The nonce commitment R is a point on the Pallas curve
    pub(crate) R: &'a <<PallasPoseidon as Ciphersuite>::Group as Group>::Element,
    // The public key
    pub(crate) P: &'a <PallasGroup as Group>::Element,
    pub(crate) message: &'a [u8],
}

impl Hashable for Challenge<'_> {
    type D = NetworkId;

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();

        roi = roi.append_bytes(self.message);
        roi = roi.append_field(self.P.x);
        roi = roi.append_field(self.P.y);
        roi = roi.append_field(self.R.x);

        roi
    }

    fn domain_string(network_id: NetworkId) -> Option<String> {
        match network_id {
            NetworkId::MAINNET => "MinaSignatureMainnet",
            NetworkId::TESTNET => "CodaSignature", //"FROST-PALLAS-POSEIDON",
        }
        .to_string()
        .into()
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

#[allow(non_snake_case)]
pub fn hash_challenge(
    R: &<<PallasPoseidon as Ciphersuite>::Group as Group>::Element,
    verifying_key: &VerifyingKey,
    message: &[u8],
) -> Fq {
    // TODO: Make this generic over NetworkId
    let mut hasher = mina_hasher::create_legacy::<Challenge>(NetworkId::TESTNET);
    let challenge = Challenge {
        R,
        P: &verifying_key.to_element(),
        message,
    };

    Fq::from(hasher.hash(&challenge).into_bigint())
}
