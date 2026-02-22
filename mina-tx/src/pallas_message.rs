//! Mina transaction challenge message representation for BluePallas signing.

use alloc::{string::String, vec::Vec};
#[cfg(feature = "frost-bluepallas-compat")]
use ark_ec::CurveGroup;
use ark_ff::PrimeField;
#[cfg(feature = "frost-bluepallas-compat")]
use frost_core::{Scalar, Signature as FrSig, VerifyingKey};
use mina_hasher::{Hashable, Hasher, ROInput};
#[cfg(feature = "frost-bluepallas-compat")]
use mina_signer::Keypair;
use mina_signer::{
    pubkey::PubKey, signature::Signature as MinaSig, BaseField, NetworkId, ScalarField,
};

const PALLAS_MESSAGE_VERSION: u8 = 1;

#[cfg(feature = "frost-bluepallas-compat")]
type BluePallasSuite = frost_bluepallas::BluePallas<PallasMessage>;

/// Hashing payload used by BluePallas challenge computation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PallasMessage {
    /// The ROInput to be hashed.
    pub(crate) input: ROInput,
    /// The network ID for domain string selection during hashing.
    network_id: NetworkId,
    /// Whether legacy hashing mode should be used.
    is_legacy: bool,
}

impl PallasMessage {
    /// Create a new `PallasMessage` from hash input metadata.
    pub fn from_parts(input: ROInput, network_id: NetworkId, is_legacy: bool) -> Self {
        Self {
            input,
            network_id,
            is_legacy,
        }
    }

    /// Build a fallback message from raw bytes when explicit message encoding is unavailable.
    pub fn from_raw_bytes_default(input: &[u8]) -> Self {
        Self {
            input: ROInput::new().append_bytes(input),
            network_id: NetworkId::TESTNET,
            is_legacy: true,
        }
    }

    /// Serialize this message to bytes for transport/signing.
    pub fn serialize(&self) -> Vec<u8> {
        let roi_bytes = self.input.serialize();
        let mut out = Vec::with_capacity(7 + roi_bytes.len());
        out.push(PALLAS_MESSAGE_VERSION);
        out.push(self.network_id.clone() as u8);
        out.push(u8::from(self.is_legacy));
        out.extend_from_slice(&(roi_bytes.len() as u32).to_le_bytes());
        out.extend_from_slice(&roi_bytes);
        out
    }

    /// Deserialize a message from the bytes produced by [`Self::serialize`].
    pub fn deserialize(input: &[u8]) -> Result<Self, crate::errors::MinaTxError> {
        if input.len() < 7 {
            return Err(crate::errors::MinaTxError::DeSerializationError(
                "PallasMessage bytes too short".into(),
            ));
        }

        if input[0] != PALLAS_MESSAGE_VERSION {
            return Err(crate::errors::MinaTxError::DeSerializationError(
                "Unsupported PallasMessage version".into(),
            ));
        }

        let network_id = match input[1] {
            0 => NetworkId::TESTNET,
            1 => NetworkId::MAINNET,
            _ => {
                return Err(crate::errors::MinaTxError::DeSerializationError(
                    "Invalid network id in PallasMessage".into(),
                ))
            }
        };

        let is_legacy = match input[2] {
            0 => false,
            1 => true,
            _ => {
                return Err(crate::errors::MinaTxError::DeSerializationError(
                    "Invalid legacy flag in PallasMessage".into(),
                ))
            }
        };

        let roi_len = u32::from_le_bytes([input[3], input[4], input[5], input[6]]) as usize;
        if input.len() != 7 + roi_len {
            return Err(crate::errors::MinaTxError::DeSerializationError(
                "Malformed PallasMessage length".into(),
            ));
        }

        let roi = ROInput::deserialize(&input[7..]).map_err(|_| {
            crate::errors::MinaTxError::DeSerializationError("Failed to deserialize ROInput".into())
        })?;

        Ok(Self {
            input: roi,
            network_id,
            is_legacy,
        })
    }

    pub fn is_legacy(&self) -> bool {
        self.is_legacy
    }

    pub fn network_id(&self) -> NetworkId {
        self.network_id.clone()
    }
}

impl Hashable for PallasMessage {
    type D = NetworkId;

    fn to_roinput(&self) -> ROInput {
        self.input.clone()
    }

    fn domain_string(network_id: NetworkId) -> Option<String> {
        network_id.into_domain_string().into()
    }
}

/// Convert FROST public key to Mina public key.
#[cfg(feature = "frost-bluepallas-compat")]
pub fn translate_pk(
    fr_pk: &VerifyingKey<BluePallasSuite>,
) -> Result<PubKey, crate::errors::MinaTxError> {
    Ok(PubKey::from_point_unsafe(fr_pk.to_element().into_affine()))
}

/// Convert FROST signature to Mina signature.
#[cfg(feature = "frost-bluepallas-compat")]
pub fn translate_sig(
    fr_sig: &FrSig<BluePallasSuite>,
) -> Result<MinaSig, crate::errors::MinaTxError> {
    let rx = fr_sig.R().into_affine().x;
    let z: Scalar<BluePallasSuite> = *fr_sig.z();

    Ok(MinaSig { rx, s: z })
}

/// Convert Mina keypair to FROST signing key.
#[cfg(feature = "frost-bluepallas-compat")]
pub fn translate_minask(
    msg: &Keypair,
) -> Result<frost_core::SigningKey<BluePallasSuite>, crate::errors::MinaTxError> {
    let scalar = msg.secret.scalar();
    frost_core::SigningKey::from_scalar(*scalar).map_err(|_| {
        crate::errors::MinaTxError::DeSerializationError("Failed to convert keypair scalar".into())
    })
}

/// Hashes message using Mina hasher, selecting legacy/kimchi mode by transaction kind.
pub fn message_hash<H>(
    pub_key: &PubKey,
    rx: BaseField,
    input: H,
    network_id: NetworkId,
    is_legacy: bool,
) -> ScalarField
where
    H: Hashable<D = NetworkId>,
{
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

    let schnorr_input = Message::<H> {
        input,
        pub_key_x: pub_key.point().x,
        pub_key_y: pub_key.point().y,
        rx,
    };

    let scalar_output = if is_legacy {
        let mut hasher = mina_hasher::create_legacy::<Message<H>>(network_id);
        hasher.hash(&schnorr_input)
    } else {
        let mut hasher = mina_hasher::create_kimchi::<Message<H>>(network_id);
        hasher.hash(&schnorr_input)
    };

    ScalarField::from(scalar_output.into_bigint())
}

#[cfg(feature = "frost-bluepallas-compat")]
impl frost_bluepallas::ChallengeMessage for PallasMessage {
    fn challenge(
        r: &frost_core::Element<BluePallasSuite>,
        verifying_key: &frost_core::VerifyingKey<BluePallasSuite>,
        message: &[u8],
    ) -> Result<frost_core::Challenge<BluePallasSuite>, frost_core::Error<BluePallasSuite>> {
        let mina_pk =
            translate_pk(verifying_key).map_err(|_| frost_core::FieldError::MalformedScalar)?;
        let rx = r.into_affine().x;

        let msg =
            Self::deserialize(message).unwrap_or_else(|_| Self::from_raw_bytes_default(message));
        let network_id = msg.network_id();
        let is_legacy = msg.is_legacy();

        let scalar = message_hash(&mina_pk, rx, msg, network_id, is_legacy);
        Ok(frost_core::Challenge::from_scalar(scalar))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pallas_message_round_trip() {
        let message = PallasMessage::from_parts(
            ROInput::new().append_bytes(b"hello"),
            NetworkId::MAINNET,
            false,
        );

        let bytes = message.serialize();
        let decoded = PallasMessage::deserialize(&bytes).expect("deserialize should succeed");

        assert_eq!(decoded.network_id(), NetworkId::MAINNET);
        assert!(!decoded.is_legacy());
        assert_eq!(decoded.input, message.input);
    }
}
