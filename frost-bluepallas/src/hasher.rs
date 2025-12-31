use alloc::string::{String, ToString};
use ark_ff::PrimeField;
use frost_core::Field;
use mina_hasher::{create_legacy, Hashable, Hasher, ROInput};
use mina_signer::{BaseField, NetworkId, PubKey, ScalarField};

use crate::{errors::BluePallasError, mina_compat::PallasMessage, PallasScalarField};

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

/// This allows us to hash a Mina/FROST signature
/// Follows the Mina signing specification at https://github.com/MinaProtocol/mina/blob/develop/docs/specs/signatures/description.md
#[derive(Clone)]
struct Message<H: Hashable> {
    input: H,
    pub_key_x: BaseField,
    pub_key_y: BaseField,
    rx: BaseField,
}

impl<H> Hashable for Message<H>
where
    H: Hashable<D = NetworkId>,
{
    type D = H::D;

    fn to_roinput(&self) -> ROInput {
        self.input
            .to_roinput()
            .append_field(self.pub_key_x)
            .append_field(self.pub_key_y)
            .append_field(self.rx)
    }

    fn domain_string(domain_param: Self::D) -> Option<String> {
        H::domain_string(domain_param)
    }
}

/// Hashable implementation for PallasMessage.
///
/// This allows PallasMessage to be hashed using Mina's Poseidon-based hasher.
impl Hashable for PallasMessage {
    type D = NetworkId;

    fn to_roinput(&self) -> ROInput {
        self.input.clone()
    }

    // Domain string specification from:
    // https://github.com/o1-labs/proof-systems/blob/0.1.0/signer/tests/transaction.rs#L53-L61
    fn domain_string(network_id: NetworkId) -> Option<String> {
        // Domain strings must have length <= 20
        match network_id {
            NetworkId::MAINNET => "MinaSignatureMainnet",
            NetworkId::TESTNET => "CodaSignature",
        }
        .to_string()
        .into()
    }
}

/// Hashes the message using the Mina hasher, given a hashable message and a NetworkId
/// Uses either the legacy or kimchi hasher based on the is_legacy flag
/// Legacy transactions use the legacy hasher, while ZKApp transactions use the kimchi hasher
pub fn message_hash<H>(
    pub_key: &PubKey,
    rx: BaseField,
    input: H,
    network_id: NetworkId,
    is_legacy: bool,
) -> Result<ScalarField, BluePallasError>
where
    H: Hashable<D = NetworkId>,
{
    let schnorr_input = Message::<H> {
        input,
        pub_key_x: pub_key.point().x,
        pub_key_y: pub_key.point().y,
        rx,
    };

    // Use the correct hasher depending on whether we have a legacy transaction or not
    let scalar_output = match is_legacy {
        true => {
            let mut hasher = create_legacy::<Message<H>>(network_id);
            hasher.hash(&schnorr_input)
        }
        false => {
            let mut hasher = mina_hasher::create_kimchi::<Message<H>>(network_id);
            hasher.hash(&schnorr_input)
        }
    };

    // Squeeze and convert from base field element to scalar field element
    // Since the difference in modulus between the two fields is < 2^125, w.h.p., a
    // random value from one field will fit in the other field.
    Ok(ScalarField::from(scalar_output.into_bigint()))
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
    use mina_hasher::Fp;

    use super::*;

    /// A wrapper around `ROInput` that implements `Hashable`.
    /// Useful for hashing pre-constructed `ROInput` values directly.
    #[derive(Clone)]
    pub struct ROInputWrapper {
        pub inner: ROInput,
    }

    impl ROInputWrapper {
        pub fn new(inner: ROInput) -> Self {
            Self { inner }
        }
    }

    impl Hashable for ROInputWrapper {
        type D = NetworkId;

        fn to_roinput(&self) -> ROInput {
            self.inner.clone()
        }

        fn domain_string(network_id: NetworkId) -> Option<String> {
            match network_id {
                NetworkId::MAINNET => "MinaSignatureMainnet",
                NetworkId::TESTNET => "CodaSignature",
            }
            .to_string()
            .into()
        }
    }

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
    fn test_hash_pallas_message_kimchi() {
        use alloc::str::FromStr;
        let field = "11846000834259235905958753603813777773459101710265500737400417221141603138177";
        let fp = Fp::from_str(field).unwrap();
        let msg = ROInput::new().append_field(fp);

        // Wrap ROInput in ROInputWrapper
        let wrapper = ROInputWrapper::new(msg);

        let output = message_hash(
            &PubKey::from_address("B62qrmyUJNTuoaC1pMYUETGjKX4Mn3pk2MRUBPS6bwP6ZDZ7JfKxwVA")
                .unwrap(),
            BaseField::from_str(
                "6455615646068099396871307223841815355688864790843622831931071323550014187712",
            )
            .unwrap(),
            wrapper,
            NetworkId::TESTNET,
            false,
        );

        assert_eq!(
            output.unwrap().to_string(),
            "28034153875204620953456376624553972171235671073234199167504854061926717353316"
        );
    }
}
