//! Mina transaction challenge message representation for BluePallas signing.

use alloc::{string::String, vec::Vec};
#[cfg(feature = "frost-bluepallas-compat")]
use ark_ec::CurveGroup;
use ark_ff::PrimeField;
#[cfg(feature = "frost-bluepallas-compat")]
use frost_core::{Scalar, Signature as FrSig, VerifyingKey};
use mina_hasher::{Hashable, Hasher, ROInput};
#[cfg(feature = "frost-bluepallas-compat")]
use mina_signer::signature::Signature as MinaSig;
#[cfg(feature = "frost-bluepallas-compat")]
use mina_signer::Keypair;
use mina_signer::{pubkey::PubKey, BaseField, ScalarField};

use crate::{
    errors::MinaTxError,
    transactions::network_id::{NetworkId, MAX_PREFIX_LENGTH},
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
            network_id: NetworkId::Testnet,
            is_legacy: true,
        }
    }

    /// Serialize this message to bytes for transport/signing.
    pub fn serialize(&self) -> Result<Vec<u8>, MinaTxError> {
        let roi_bytes = self.input.serialize();
        let mut out = Vec::with_capacity(7 + roi_bytes.len());
        out.push(PALLAS_MESSAGE_VERSION);
        match &self.network_id {
            NetworkId::Testnet => {
                out.push(0u8);
            }
            NetworkId::Mainnet => {
                out.push(1u8);
            }
            NetworkId::Custom(s) => {
                if s.len() > MAX_PREFIX_LENGTH {
                    return Err(crate::errors::MinaTxError::SerializationError(format!(
                        "Custom network ID exceeds maximum length of {MAX_PREFIX_LENGTH}"
                    )));
                }
                out.push(2u8);
                let name_bytes = s.as_bytes();
                out.push(name_bytes.len() as u8);
                out.extend_from_slice(name_bytes);
            }
        };
        out.push(u8::from(self.is_legacy));
        out.extend_from_slice(&(roi_bytes.len() as u32).to_le_bytes());
        out.extend_from_slice(&roi_bytes);
        Ok(out)
    }

    /// Deserialize a message from the bytes produced by [`Self::serialize`].
    pub fn deserialize(input: &[u8]) -> Result<Self, crate::errors::MinaTxError> {
        if input.len() < 3 {
            return Err(crate::errors::MinaTxError::DeSerializationError(
                "PallasMessage bytes too short".into(),
            ));
        }

        if input[0] != PALLAS_MESSAGE_VERSION {
            return Err(crate::errors::MinaTxError::DeSerializationError(
                "Unsupported PallasMessage version".into(),
            ));
        }

        let (network_id, offset) = match input[1] {
            0 => (NetworkId::Testnet, 2),
            1 => (NetworkId::Mainnet, 2),
            2 => {
                let name_len = input[2] as usize;
                if name_len > MAX_PREFIX_LENGTH {
                    return Err(crate::errors::MinaTxError::DeSerializationError(
                        "Custom network ID exceeds maximum length".into(),
                    ));
                }
                if input.len() < 3 + name_len {
                    return Err(crate::errors::MinaTxError::DeSerializationError(
                        "PallasMessage too short for custom network ID name".into(),
                    ));
                }
                let name = core::str::from_utf8(&input[3..3 + name_len])
                    .map_err(|_| {
                        crate::errors::MinaTxError::DeSerializationError(
                            "Invalid UTF-8 in custom network ID".into(),
                        )
                    })?
                    .into();
                (NetworkId::Custom(name), 3 + name_len)
            }
            _ => {
                return Err(crate::errors::MinaTxError::DeSerializationError(
                    "Invalid network id in PallasMessage".into(),
                ))
            }
        };

        let is_legacy = match input[offset] {
            0 => false,
            1 => true,
            _ => {
                return Err(crate::errors::MinaTxError::DeSerializationError(
                    "Invalid legacy flag in PallasMessage".into(),
                ))
            }
        };

        let header_len = offset + 1 + 4; // version + network + legacy + roi_len(u32)
        if input.len() < header_len {
            return Err(crate::errors::MinaTxError::DeSerializationError(
                "PallasMessage bytes too short".into(),
            ));
        }
        let roi_start = offset + 1;
        let roi_len = u32::from_le_bytes([
            input[roi_start],
            input[roi_start + 1],
            input[roi_start + 2],
            input[roi_start + 3],
        ]) as usize;
        let data_start = roi_start + 4;
        if input.len() != data_start + roi_len {
            return Err(crate::errors::MinaTxError::DeSerializationError(
                "Malformed PallasMessage length".into(),
            ));
        }

        let roi = ROInput::deserialize(&input[data_start..]).map_err(|_| {
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
        Some(network_id.into_domain_string())
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

        // This fall-through into from_raw_bytes_default allows us to pass FROST tests which use arbitrary byte messages
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

    fn round_trip(message: &PallasMessage) -> PallasMessage {
        let bytes = message.serialize().expect("serialize should succeed");
        PallasMessage::deserialize(&bytes).expect("deserialize should succeed")
    }

    // --- Round-trip tests for all NetworkId variants ---

    #[test]
    fn test_round_trip_mainnet() {
        let message = PallasMessage::from_parts(
            ROInput::new().append_bytes(b"hello"),
            NetworkId::Mainnet,
            false,
        );
        let decoded = round_trip(&message);
        assert_eq!(decoded.network_id(), NetworkId::Mainnet);
        assert!(!decoded.is_legacy());
        assert_eq!(decoded.input, message.input);
    }

    #[test]
    fn test_round_trip_testnet() {
        let message = PallasMessage::from_parts(
            ROInput::new().append_bytes(b"testnet payload"),
            NetworkId::Testnet,
            true,
        );
        let decoded = round_trip(&message);
        assert_eq!(decoded.network_id(), NetworkId::Testnet);
        assert!(decoded.is_legacy());
        assert_eq!(decoded.input, message.input);
    }

    #[test]
    fn test_round_trip_custom_network_id() {
        let message = PallasMessage::from_parts(
            ROInput::new().append_bytes(b"devnet payload"),
            NetworkId::Custom("devnet".into()),
            false,
        );
        let decoded = round_trip(&message);
        assert_eq!(decoded.network_id(), NetworkId::Custom("devnet".into()));
        assert!(!decoded.is_legacy());
        assert_eq!(decoded.input, message.input);
    }

    // --- Round-trip with empty ROInput ---

    #[test]
    fn test_round_trip_empty_roi_input() {
        // Empty ROInput exercises the roi_len = 0 path in serialize/deserialize.
        let message = PallasMessage::from_parts(ROInput::new(), NetworkId::Testnet, true);
        let decoded = round_trip(&message);
        assert_eq!(decoded.network_id(), NetworkId::Testnet);
        assert!(decoded.is_legacy());
        assert_eq!(decoded.input, message.input);
    }

    // --- serialize rejects oversized custom network IDs ---

    #[test]
    fn test_serialize_rejects_custom_network_id_exceeding_max_length() {
        let too_long = "x".repeat(MAX_PREFIX_LENGTH + 1);
        let message = PallasMessage::from_parts(ROInput::new(), NetworkId::Custom(too_long), false);
        assert!(matches!(
            message.serialize(),
            Err(crate::errors::MinaTxError::SerializationError(_))
        ));
    }

    // --- Deserialization error cases ---

    #[test]
    fn test_deserialize_rejects_unknown_version() {
        let mut bytes = PallasMessage::from_parts(
            ROInput::new().append_bytes(b"data"),
            NetworkId::Testnet,
            true,
        )
        .serialize()
        .unwrap();
        bytes[0] = 0xFF; // corrupt the version byte

        assert!(matches!(
            PallasMessage::deserialize(&bytes),
            Err(crate::errors::MinaTxError::DeSerializationError(_))
        ));
    }

    #[test]
    fn test_deserialize_rejects_unknown_network_id_discriminant() {
        let mut bytes = PallasMessage::from_parts(
            ROInput::new().append_bytes(b"data"),
            NetworkId::Testnet,
            true,
        )
        .serialize()
        .unwrap();
        bytes[1] = 0x05; // not a valid network discriminant

        assert!(matches!(
            PallasMessage::deserialize(&bytes),
            Err(crate::errors::MinaTxError::DeSerializationError(_))
        ));
    }

    #[test]
    fn test_deserialize_rejects_invalid_legacy_flag() {
        let mut bytes = PallasMessage::from_parts(
            ROInput::new().append_bytes(b"data"),
            NetworkId::Testnet,
            true,
        )
        .serialize()
        .unwrap();
        // For Testnet layout: [version(0), network_tag(1), legacy_flag(2), ...]
        bytes[2] = 0x02; // 0x02 is not a valid bool byte

        assert!(matches!(
            PallasMessage::deserialize(&bytes),
            Err(crate::errors::MinaTxError::DeSerializationError(_))
        ));
    }

    #[test]
    fn test_deserialize_rejects_input_shorter_than_minimum_header() {
        // Fewer than 3 bytes — cannot read version + network tag + anything further.
        assert!(matches!(
            PallasMessage::deserialize(&[PALLAS_MESSAGE_VERSION, 0x00]),
            Err(crate::errors::MinaTxError::DeSerializationError(_))
        ));
    }

    #[test]
    fn test_deserialize_rejects_truncated_custom_network_name() {
        // Claim name_len=10 but supply only 4 bytes of name body.
        let bytes = [
            PALLAS_MESSAGE_VERSION,
            0x02, // Custom
            10,   // name_len = 10
            b'a',
            b'b',
            b'c',
            b'd', // only 4 of the 10 promised bytes
        ];
        assert!(matches!(
            PallasMessage::deserialize(&bytes),
            Err(crate::errors::MinaTxError::DeSerializationError(_))
        ));
    }

    #[test]
    fn test_deserialize_rejects_custom_network_id_exceeding_max_length() {
        // Encode a name_len > MAX_PREFIX_LENGTH directly in the byte stream.
        let too_long = MAX_PREFIX_LENGTH + 1;
        let mut bytes = vec![PALLAS_MESSAGE_VERSION, 0x02, too_long as u8];
        bytes.extend(vec![b'x'; too_long]);

        assert!(matches!(
            PallasMessage::deserialize(&bytes),
            Err(crate::errors::MinaTxError::DeSerializationError(_))
        ));
    }

    #[test]
    fn test_deserialize_rejects_mismatched_roi_length() {
        // Build a valid message then corrupt the declared roi_len to mismatch actual body size.
        let mut bytes = PallasMessage::from_parts(
            ROInput::new().append_bytes(b"payload"),
            NetworkId::Mainnet,
            false,
        )
        .serialize()
        .unwrap();

        // Mainnet layout: version(0) | network_tag(1) | legacy(2) | roi_len_u32_le(3..7) | body
        // Increment the first byte of the little-endian roi_len to inflate the declared length.
        bytes[3] = bytes[3].wrapping_add(1);

        assert!(matches!(
            PallasMessage::deserialize(&bytes),
            Err(crate::errors::MinaTxError::DeSerializationError(_))
        ));
    }

    // --- from_raw_bytes_default contract ---

    #[test]
    fn test_from_raw_bytes_default_uses_testnet_and_legacy() {
        // These defaults are relied upon by the challenge() fallback path; they must not change silently.
        let msg = PallasMessage::from_raw_bytes_default(b"arbitrary");
        assert_eq!(msg.network_id(), NetworkId::Testnet);
        assert!(msg.is_legacy());
    }
}
