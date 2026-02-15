//! Mina compatibility helpers for FROST signatures.

use alloc::{string::ToString, vec::Vec};
use ark_ec::CurveGroup;
use frost_core::{Scalar, Signature as FrSig, VerifyingKey};
use mina_hasher::ROInput;
use mina_signer::{pubkey::PubKey, signature::Signature as MinaSig, NetworkId};

use crate::{
    errors::{BluePallasError, BluePallasResult},
    BluePallas, SigningKey,
};

const PALLAS_MESSAGE_VERSION: u8 = 1;

/// Hashing payload used by BluePallas challenge computation.
#[derive(Clone, Debug)]
pub struct PallasMessage {
    /// The ROInput to be hashed.
    pub(crate) input: ROInput,
    /// The network ID for domain string selection during hashing.
    pub network_id: NetworkId,
    /// Whether legacy hashing mode should be used.
    pub is_legacy: bool,
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

    /// Create a new `PallasMessage` from raw bytes.
    ///
    /// This constructor treats input bytes as opaque payload and defaults to
    /// TESTNET legacy hashing.
    pub fn new(input: Vec<u8>) -> Self {
        Self {
            input: ROInput::new().append_bytes(&input),
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
        out.push(if self.is_legacy { 1 } else { 0 });
        out.extend_from_slice(&(roi_bytes.len() as u32).to_le_bytes());
        out.extend_from_slice(&roi_bytes);
        out
    }

    /// Deserialize a message from the bytes produced by [`Self::serialize`].
    pub fn deserialize(input: &[u8]) -> Result<Self, BluePallasError> {
        if input.len() < 7 {
            return Err(BluePallasError::DeSerializationError(
                "PallasMessage bytes too short".to_string(),
            ));
        }

        if input[0] != PALLAS_MESSAGE_VERSION {
            return Err(BluePallasError::DeSerializationError(
                "Unsupported PallasMessage version".to_string(),
            ));
        }

        let network_id = match input[1] {
            0 => NetworkId::TESTNET,
            1 => NetworkId::MAINNET,
            _ => {
                return Err(BluePallasError::DeSerializationError(
                    "Invalid network id in PallasMessage".to_string(),
                ))
            }
        };

        let is_legacy = match input[2] {
            0 => false,
            1 => true,
            _ => {
                return Err(BluePallasError::DeSerializationError(
                    "Invalid legacy flag in PallasMessage".to_string(),
                ))
            }
        };

        let roi_len = u32::from_le_bytes([input[3], input[4], input[5], input[6]]) as usize;
        if input.len() != 7 + roi_len {
            return Err(BluePallasError::DeSerializationError(
                "Malformed PallasMessage length".to_string(),
            ));
        }

        let roi = ROInput::deserialize(&input[7..]).map_err(|_| {
            BluePallasError::DeSerializationError("Failed to deserialize ROInput".to_string())
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

/// Convert FROST public key to Mina public key.
pub fn translate_pk(fr_pk: &VerifyingKey<BluePallas>) -> BluePallasResult<PubKey> {
    Ok(PubKey::from_point_unsafe(fr_pk.to_element().into_affine()))
}

/// Convert FROST signature to Mina signature.
pub fn translate_sig(fr_sig: &FrSig<BluePallas>) -> BluePallasResult<MinaSig> {
    let rx = fr_sig.R().into_affine().x;
    let z: Scalar<BluePallas> = *fr_sig.z();

    Ok(MinaSig { rx, s: z })
}

/// Convert Mina keypair to FROST signing key.
pub fn translate_minask(msg: &mina_signer::Keypair) -> BluePallasResult<SigningKey> {
    let scalar = msg.secret.scalar();
    SigningKey::from_scalar(*scalar).map_err(|e| e.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::fields::models::fp::{Fp, MontBackend};
    use frost_core::SigningKey;
    use mina_curves::pasta::fields::fq::FrConfig;
    use mina_signer::seckey::SecKey;

    #[test]
    fn test_translate_pk() -> BluePallasResult<()> {
        let n: u32 = 57639753;

        let scalar: Fp<MontBackend<FrConfig, 4>, 4> = Fp::new(n.into());
        let mina_sk = SecKey::new(scalar);
        let mina_pk = PubKey::from_secret_key(mina_sk)?;

        let fr_sk = SigningKey::from_scalar(scalar)?;
        let fr_pk: VerifyingKey<BluePallas> = fr_sk.into();

        assert_eq!(translate_pk(&fr_pk)?, mina_pk);
        Ok(())
    }

    #[test]
    fn test_pallas_message_round_trip() {
        let message = PallasMessage::from_parts(
            ROInput::new().append_bytes(b"hello"),
            NetworkId::MAINNET,
            false,
        );

        let bytes = message.serialize();
        let decoded = PallasMessage::deserialize(&bytes).expect("deserialize should succeed");

        assert_eq!(decoded.network_id, NetworkId::MAINNET);
        assert!(!decoded.is_legacy);
        assert_eq!(decoded.input, message.input);
    }
}
