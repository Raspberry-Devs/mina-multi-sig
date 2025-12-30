//! This file represents output signatures from the FROST signing process and their corresponding transactions
//! Note, that currently the FROST signature outputs the signature, public key, and transaction separately which is compatible with legacy payments
//! However, ZKApp transactions may include signatures within account updates and fee payer information. For that reason, ZKApp transactions may contain several
//! different signatures which correspond to different signers and so on. However, as FROST signing is expensive, we only sign once over the entire transaction
//! rather than signing several times over different account updates like o1js does. This means frost-bluepallas only supports full commitment ZKApp transactions (as opposed to partial commitment)
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::{BigInt, PrimeField};
use frost_core::Signature;
use mina_signer::PubKey;
use serde::{
    ser::{SerializeStruct, Serializer},
    Serialize,
};

use crate::{
    base58::{to_base58_check, SIGNATURE_VERSION_BYTE, SIGNATURE_VERSION_NUMBER},
    errors::BluePallasError,
    transactions::TransactionEnvelope,
    translate::translate_pk,
    BluePallas, VerifyingKey,
};

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

impl TryInto<Sig> for Signature<BluePallas> {
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

impl TryFrom<VerifyingKey> for PubKeySer {
    type Error = BluePallasError;

    fn try_from(vk: VerifyingKey) -> Result<Self, Self::Error> {
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

/// Note that this structure is only correct for legacy payments
/// ZKApp transactions may include signature payloads within account updates and fee payer
#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct TransactionSignature {
    pub publicKey: PubKeySer,
    pub signature: Sig,
    pub payload: TransactionEnvelope,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        helper,
        transactions::{legacy_tx::LegacyTransaction, TransactionEnvelope},
        translate,
    };
    use core::convert::TryInto;
    use mina_signer::Keypair;

    #[test]
    fn test_frost_sig_to_sig_conversion_matches_translate() -> Result<(), BluePallasError> {
        // Generate a test signature using a known private key
        let private_key_hex = "35dcca7620128d240cc3319c83dc6402ad439038361ba853af538a4cea3ddabc";
        let mina_keypair = Keypair::from_hex(private_key_hex)
            .map_err(|_| BluePallasError::InvalidSignature("Failed to parse keypair".into()))?;

        let signing_key = translate::translate_minask(&mina_keypair)
            .map_err(|_| BluePallasError::InvalidSignature("Failed to translate keypair".into()))?;

        // Create a test message
        let test_msg = LegacyTransaction::new_payment(
            mina_keypair.public.clone(),
            mina_keypair.public.clone(),
            1000,
            1,
            0,
        );
        let test_msg = TransactionEnvelope::new_legacy(mina_signer::NetworkId::MAINNET, test_msg)
            .serialize()
            .unwrap();

        // Generate FROST signature
        let (frost_sig, _vk) =
            helper::generate_signature_from_sk(&test_msg, &signing_key, rand_core::OsRng).unwrap();

        // Method 1: Existing translation approach
        let mina_sig = translate::translate_sig(&frost_sig).map_err(|_| {
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
