use std::cell::RefCell;

use ark_ff::PrimeField;
use frost_core::Field;
use mina_hasher::{create_legacy, Hashable, Hasher, ROInput};
use mina_signer::{BaseField, NetworkId, PubKey, ScalarField};

use crate::PallasScalarField;

thread_local! {
    static NETWORK_ID: RefCell<Option<NetworkId>> = const { RefCell::new(Some(NetworkId::TESTNET)) }
}

/// Set the network ID for the current thread
pub fn set_network_id(network_id: NetworkId) -> Result<(), String> {
    NETWORK_ID.with(|id| {
        *id.borrow_mut() = Some(network_id);
    });
    Ok(())
}

/// Get the network ID for the current thread, returns error if not set
pub fn get_network_id() -> Result<NetworkId, String> {
    NETWORK_ID.with(|id| {
        id.borrow()
            .clone()
            .ok_or_else(|| "NetworkId not set. Call set_network_id() first.".to_string())
    })
}

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

pub fn message_hash<H>(pub_key: &PubKey, rx: BaseField, input: &H) -> ScalarField
where
    H: Hashable<D = NetworkId>,
{
    let network_id = get_network_id().expect("NetworkId must be set before calling message_hash");
    let mut hasher = mina_hasher::create_legacy::<Message<H>>(network_id);

    let schnorr_input = Message::<H> {
        input: input.clone(),
        pub_key_x: pub_key.point().x,
        pub_key_y: pub_key.point().y,
        rx,
    };

    // Squeeze and convert from base field element to scalar field element
    // Since the difference in modulus between the two fields is < 2^125, w.h.p., a
    // random value from one field will fit in the other field.
    ScalarField::from(hasher.hash(&schnorr_input).into_bigint())
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
