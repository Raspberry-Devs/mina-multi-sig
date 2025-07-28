use std::cell::RefCell;

use ark_ff::PrimeField;
use frost_core::Field;
use mina_hasher::{create_legacy, Hashable, Hasher, ROInput};
use mina_signer::{BaseField, NetworkId, PubKey, ScalarField};

use crate::{errors::BluePallasError, PallasScalarField};

// Currently, the FROST interface only allows for static function calls, which means that passing context-related
// information must be done through global variables or thread-local storage.
// Since we expect FROST to be single-threaded, we use thread-local storage to pass in the NetworkID.
thread_local! {
    // set network id to be the testnet by default
    static NETWORK_ID: RefCell<Option<NetworkId>> = const { RefCell::new(Some(NetworkId::TESTNET)) }
}

/// Set the network ID for the current thread
pub fn set_network_id(network_id: NetworkId) -> Result<(), BluePallasError> {
    NETWORK_ID.with(|id| {
        *id.borrow_mut() = Some(network_id);
    });
    Ok(())
}

/// Get the network ID for the current thread, returns error if not set
pub fn get_network_id() -> Result<NetworkId, BluePallasError> {
    NETWORK_ID.with(|id| id.borrow().clone().ok_or(BluePallasError::NetworkIdNotSet))
}

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

/// This is an adaptor for the Mina Hashable type and allows us to
/// have compatiblility between the Mina and FROST implementations
/// The adaptor will attempt to serialize the input as a ROInput first, if that fails then it will
/// treat the input as raw bytes
#[derive(Clone, Debug)]
pub struct PallasMessage {
    input: ROInput,
}

impl PallasMessage {
    pub fn new(input: Vec<u8>) -> Self {
        // Try to deserialize as ROInput first
        match ROInput::deserialize(&input) {
            Ok(roi) => PallasMessage { input: roi },
            Err(_) => {
                // If deserialization fails, treat input as raw bytes
                let roi = ROInput::new().append_bytes(&input);
                PallasMessage { input: roi }
            }
        }
    }
}

// Implement a hashable trait for a u8 slice
impl Hashable for PallasMessage {
    type D = NetworkId;

    fn to_roinput(&self) -> ROInput {
        self.input.clone()
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

/// Hashes the message using the Mina hasher, given a hashable message and a NetworkId
/// Currently, the FROST Ciphersuite implementation only allows for static function calls
/// This means that any context related information must be passed either through global variables or thread-local values
/// As we ONLY expect FROST to be single-threaded, we opt to use thread-local storage to pass in the NetworkID
pub fn message_hash<H>(
    pub_key: &PubKey,
    rx: BaseField,
    input: H,
) -> Result<ScalarField, BluePallasError>
where
    H: Hashable<D = NetworkId>,
{
    let network_id = get_network_id()?;
    let mut hasher = mina_hasher::create_legacy::<Message<H>>(network_id);

    let schnorr_input = Message::<H> {
        input,
        pub_key_x: pub_key.point().x,
        pub_key_y: pub_key.point().y,
        rx,
    };

    // Squeeze and convert from base field element to scalar field element
    // Since the difference in modulus between the two fields is < 2^125, w.h.p., a
    // random value from one field will fit in the other field.
    Ok(ScalarField::from(hasher.hash(&schnorr_input).into_bigint()))
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
    use mina_signer::Keypair;

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
    fn test_message_hash_is_consistent() {
        let mut rng = rand_core::OsRng;
        let pubkey = Keypair::rand(&mut rng).unwrap().public;
        let rx = BaseField::from(42u64);

        let msg = PallasMessage::new(b"unit test".to_vec());

        // TESTNET
        set_network_id(NetworkId::TESTNET).unwrap();
        let h1 = message_hash(&pubkey, rx, msg.clone()).unwrap();

        // repeat must be same
        let h2 = message_hash(&pubkey, rx, msg.clone()).unwrap();
        assert_eq!(h1, h2);

        // MAINNET should give a different domain, hence a different hash
        set_network_id(NetworkId::MAINNET).unwrap();
        let h3 = message_hash(&pubkey, rx, msg).unwrap();
        assert_ne!(h1, h3);
    }
}
