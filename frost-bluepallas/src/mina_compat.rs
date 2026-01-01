//! Mina compatibility module for FROST signatures.
//! This module provides methods and structs for conversions between FROST types and Mina types, enabling interoperability.

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::{BigInt, PrimeField};
use frost_core::{Scalar, Signature as FrSig, VerifyingKey};
use mina_hasher::{Hashable, ROInput};
use mina_signer::{pubkey::PubKey, signature::Signature as MinaSig, NetworkId};
use serde::{
    ser::{SerializeStruct, Serializer},
    Serialize,
};

use crate::{
    base58::{to_base58_check, SIGNATURE_VERSION_BYTE, SIGNATURE_VERSION_NUMBER},
    errors::{BluePallasError, BluePallasResult},
    transactions::TransactionEnvelope,
    BluePallas, SigningKey,
};

/// Adaptor for the Mina Hashable type, providing compatibility between Mina and FROST.
///
/// The adaptor will attempt to deserialize the input as a [`TransactionEnvelope`] first.
/// If that fails, it will treat the input as raw bytes.
///
/// The `Hashable` implementation is in the `hasher` module.
#[derive(Clone, Debug)]
pub struct PallasMessage {
    /// The ROInput to be hashed.
    pub(crate) input: ROInput,
    /// The network ID for domain string selection during hashing.
    pub network_id: NetworkId,
    pub is_legacy: bool,
}

impl PallasMessage {
    /// Create a new `PallasMessage` from raw bytes.
    ///
    /// Attempts to deserialize as a `TransactionEnvelope` first. If that fails,
    /// treats the input as raw bytes and defaults to TESTNET network ID.
    pub fn new(input: Vec<u8>) -> Self {
        // Try to deserialize as TransactionEnvelope first
        match TransactionEnvelope::deserialize(&input) {
            Ok(roi) => PallasMessage {
                input: roi.to_roinput(),
                network_id: roi.network_id().clone(),
                is_legacy: roi.is_legacy(),
            },
            Err(_) => {
                // If deserialization fails, treat input as raw bytes
                let roi = ROInput::new().append_bytes(&input);
                // Default to TESTNET and legacy hashing if we can't determine network ID
                PallasMessage {
                    input: roi,
                    network_id: NetworkId::TESTNET,
                    is_legacy: true,
                }
            }
        }
    }

    pub fn is_legacy(&self) -> bool {
        self.is_legacy
    }

    pub fn network_id(&self) -> NetworkId {
        self.network_id.clone()
    }
}

/// Serializable signature representation for JSON output.
///
/// This is used for outputting FROST signatures in a format compatible with Mina tooling.
pub struct Sig {
    pub field: BigInt<4>,
    pub scalar: BigInt<4>,
}

impl Sig {
    /// Convert a BigInt<4> to 32 bytes in little-endian format
    fn bigint_to_bytes(value: &BigInt<4>) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        for (i, limb) in value.0.iter().enumerate() {
            let limb_bytes = limb.to_le_bytes();
            bytes[i * 8..(i + 1) * 8].copy_from_slice(&limb_bytes);
        }
        bytes
    }

    /// Convert the signature to bytes in the format expected by Mina
    /// Format: version_number (1 byte) + r (32 bytes LE) + s (32 bytes LE)
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(65);
        bytes.push(SIGNATURE_VERSION_NUMBER);
        bytes.extend_from_slice(&Self::bigint_to_bytes(&self.field));
        bytes.extend_from_slice(&Self::bigint_to_bytes(&self.scalar));
        bytes
    }

    /// Convert the signature to a base58check encoded string compatible with Mina
    pub fn to_base58(&self) -> String {
        let bytes = self.to_bytes();
        to_base58_check(&bytes, SIGNATURE_VERSION_BYTE)
    }
}

impl TryInto<Sig> for FrSig<BluePallas> {
    type Error = BluePallasError;

    fn try_into(self) -> Result<Sig, Self::Error> {
        let x = self
            .R()
            .into_affine()
            .x()
            .ok_or_else(|| {
                BluePallasError::InvalidSignature("Failed to convert x coordinate to bigint".into())
            })?
            .into_bigint();
        let z = self.z().into_bigint();

        Ok(Sig {
            field: x,
            scalar: z,
        })
    }
}

impl Serialize for Sig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("signature", 2)?;
        state.serialize_field("field", &self.field.to_string())?;
        state.serialize_field("scalar", &self.scalar.to_string())?;
        state.serialize_field("base58", self.to_base58().as_str())?;
        state.end()
    }
}

/// Serializable public key wrapper for JSON output.
#[allow(non_snake_case)]
pub struct PubKeySer {
    pub pubKey: PubKey,
}

impl From<PubKey> for PubKeySer {
    #[allow(non_snake_case)]
    fn from(pubKey: PubKey) -> Self {
        PubKeySer { pubKey }
    }
}

impl TryFrom<crate::VerifyingKey> for PubKeySer {
    type Error = BluePallasError;

    fn try_from(vk: crate::VerifyingKey) -> Result<Self, Self::Error> {
        translate_pk(&vk)
            .map(|pub_key| PubKeySer { pubKey: pub_key })
            .map_err(|e| BluePallasError::InvalidPublicKey(e.to_string()))
    }
}

impl Serialize for PubKeySer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("publicKey", 1)?;
        state.serialize_field("address", &self.pubKey.into_address())?;
        state.end()
    }
}

/// Combined transaction signature payload for legacy payments.
///
/// Note that this structure is only correct for legacy payments.
/// ZKApp transactions may include signature payloads within account updates and fee payer.
#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct TransactionSignature {
    pub publicKey: PubKeySer,
    pub signature: Sig,
    pub payload: TransactionEnvelope,
}

// Note
// CurvePoint = Affine<PallasParameters>                                    mina side
// PallasProjective = Projective<PallasParameters> (= Element<BluePallas>)  frost side
// The ScalarField type on the mina and frost side are the same!

/// Convert FROST public key to Mina public key.
///
/// The `VerifyingKey` is the public key in FROST, which is a point on the curve.
pub fn translate_pk(fr_pk: &VerifyingKey<BluePallas>) -> BluePallasResult<PubKey> {
    Ok(PubKey::from_point_unsafe(fr_pk.to_element().into_affine()))
}

/// Convert FROST signature to Mina signature.
///
/// The `R` field is the commitment to the nonce, and `z` is the response to the challenge.
pub fn translate_sig(fr_sig: &FrSig<BluePallas>) -> BluePallasResult<MinaSig> {
    let rx = fr_sig.R().into_affine().x;
    let z: Scalar<BluePallas> = *fr_sig.z();

    Ok(MinaSig { rx, s: z })
}

/// Convert Mina keypair to FROST signing key.
pub fn translate_minask(msg: &mina_signer::Keypair) -> BluePallasResult<SigningKey> {
    // Convert mina SecKey to FROST SigningKey
    let scalar = msg.secret.scalar();
    SigningKey::from_scalar(*scalar).map_err(|e| e.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{signing_utilities, transactions::legacy_tx::LegacyTransaction};
    use ark_ff::fields::models::fp::{Fp, MontBackend};
    use core::convert::TryInto;
    use frost_core::SigningKey;
    use mina_curves::pasta::fields::fq::FrConfig;
    use mina_signer::{seckey::SecKey, Keypair};

    #[test]
    fn test_translate_pk() -> BluePallasResult<()> {
        // We generate scalars (SecretKey) for both the frost and mina sides in the same way
        // Then on each side the appropriate elements (PublicKey) representations are generated
        // Then use the translation function to check if it's the same element on both sides

        // The type of Scalar from which a SecretKey can be made (on Mina side): Fp<MontBackend<FrConfig, 4>, 4>
        let n: u32 = 57639753; // TODO generate loads of random n and test

        // <PallasParameters as CurveConfig>::ScalarField is the same type as Fp<...>
        let scalar: Fp<MontBackend<FrConfig, 4>, 4> = Fp::new(n.into());
        let mina_sk = SecKey::new(scalar);
        let mina_pk = PubKey::from_secret_key(mina_sk)?;

        // Fails if scalar is zero
        let fr_sk = SigningKey::from_scalar(scalar)?;
        let fr_pk: VerifyingKey<BluePallas> = fr_sk.into();

        assert_eq!(translate_pk(&fr_pk)?, mina_pk);
        Ok(())
    }

    #[test]
    fn test_frost_sig_to_sig_conversion_matches_translate() -> Result<(), BluePallasError> {
        // Generate a test signature using a known private key
        let private_key_hex = "35dcca7620128d240cc3319c83dc6402ad439038361ba853af538a4cea3ddabc";
        let mina_keypair = Keypair::from_hex(private_key_hex)
            .map_err(|_| BluePallasError::InvalidSignature("Failed to parse keypair".into()))?;

        let signing_key = translate_minask(&mina_keypair)
            .map_err(|_| BluePallasError::InvalidSignature("Failed to translate keypair".into()))?;

        // Create a test message
        let test_msg = LegacyTransaction::new_payment(
            mina_keypair.public.clone(),
            mina_keypair.public.clone(),
            1000,
            1,
            0,
        );
        let test_msg = TransactionEnvelope::new_legacy(NetworkId::MAINNET, test_msg)
            .serialize()
            .unwrap();

        // Generate FROST signature
        let (frost_sig, _vk) = signing_utilities::generate_signature_from_sk(
            &test_msg,
            &signing_key,
            rand_core::OsRng,
        )
        .unwrap();

        // Method 1: Existing translation approach
        let mina_sig = translate_sig(&frost_sig).map_err(|_| {
            BluePallasError::InvalidSignature("Failed to translate signature".into())
        })?;
        let sig_base_existing: BigInt<4> = mina_sig.rx.into_bigint();
        let sig_scalar_existing: BigInt<4> = mina_sig.s.into_bigint();
        let existing_sig = Sig {
            field: sig_base_existing,
            scalar: sig_scalar_existing,
        };

        // Method 2: TryInto conversion
        let tryinto_sig: Sig = frost_sig.try_into()?;

        // Compare the results
        assert_eq!(
            existing_sig.field, tryinto_sig.field,
            "Field components should match"
        );
        assert_eq!(
            existing_sig.scalar, tryinto_sig.scalar,
            "Scalar components should match"
        );

        Ok(())
    }

    /// Helper to convert a decimal string to BigInt<4>
    fn bigint_from_decimal(s: &str) -> BigInt<4> {
        use num_bigint::BigUint;
        use num_traits::Num;

        let big = BigUint::from_str_radix(s, 10).expect("Invalid decimal string");
        let bytes = big.to_bytes_le();

        let mut limbs = [0u64; 4];
        for (i, chunk) in bytes.chunks(8).enumerate() {
            if i >= 4 {
                break;
            }
            let mut arr = [0u8; 8];
            arr[..chunk.len()].copy_from_slice(chunk);
            limbs[i] = u64::from_le_bytes(arr);
        }
        BigInt(limbs)
    }

    #[test]
    fn test_sig_to_base58_known_vectors() {
        // Test vectors: (r, s, expected_base58)
        let test_vectors = [
            (
                "19534033587754221641582716832950022068620678142901839096898943635476986378719",
                "8239128679179126998396192873114684363951100539025183879566224754754874407061",
                "7mXTsNMuxi8cq83xMjJ52HqP8B16gZ2rYw2om57LUDSekuCxB9GNnVypr1YFNHtgkDhMKFpdHm1GNqAtrw3DJsVNPRT93pdX",
            ),
            (
                "24149846232426282936003668868539013969893921711406495075111854298659928975942",
                "14034827642885705526232744953870469114156457789480445652119376762475291375772",
                "7mX7qxs1u5ZunuXReJcq4qKq84gWRyBCaA2UzApJp2Gb4txp1MMHxZzjCjknE991SxBFU9WJ46ityWtv6ZmJMVcRdErriJEW",
            ),
            (
                "11761945902000965807519434876351280549901415788925705581704314097235572750862",
                "20979239827320446710763939068170686000494489127481282268035737744590947267921",
                "7mWzZQNC4fRtZARbTAju3VL2K4Nd9EnDFnmVAVc2vHJnG86rtZpGAmPtmYMi2K4bhSXh54c7ujUpdF2JmUEzLqoLCndEjG4M",
            ),
        ];

        for (i, (r_str, s_str, expected_base58)) in test_vectors.iter().enumerate() {
            let sig = Sig {
                field: bigint_from_decimal(r_str),
                scalar: bigint_from_decimal(s_str),
            };

            let actual_base58 = sig.to_base58();

            assert_eq!(
                actual_base58, *expected_base58,
                "Test vector {} failed:\n  r: {}\n  s: {}\n  expected: {}\n  actual: {}",
                i, r_str, s_str, expected_base58, actual_base58
            );
        }
    }
}
