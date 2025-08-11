use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::{BigInt, PrimeField};
use frost_core::Signature;
use mina_signer::PubKey;
use serde::{
    ser::{SerializeStruct, Serializer},
    Serialize,
};

use crate::{
    errors::BluePallasError, transactions::Transaction, translate::translate_pk, PallasPoseidon,
    VerifyingKey,
};

pub struct Sig {
    pub field: BigInt<4>,
    pub scalar: BigInt<4>,
}

impl TryInto<Sig> for Signature<PallasPoseidon> {
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

#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct TransactionSignature {
    pub publicKey: PubKeySer,
    pub signature: Sig,
    pub payload: Transaction,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{helper, translate};
    use mina_signer::Keypair;
    use std::convert::TryInto;

    #[test]
    fn test_frost_sig_to_sig_conversion_matches_translate() -> Result<(), BluePallasError> {
        // Generate a test signature using a known private key
        let private_key_hex = "35dcca7620128d240cc3319c83dc6402ad439038361ba853af538a4cea3ddabc";
        let mina_keypair = Keypair::from_hex(private_key_hex)
            .map_err(|_| BluePallasError::InvalidSignature("Failed to parse keypair".into()))?;

        let signing_key = translate::translate_minask(&mina_keypair)
            .map_err(|_| BluePallasError::InvalidSignature("Failed to translate keypair".into()))?;

        // Create a test message
        let test_msg = b"test message for signature conversion";

        // Generate FROST signature
        let (frost_sig, _vk) =
            helper::generate_signature_from_sk(test_msg, &signing_key, rand_core::OsRng).map_err(
                |_| BluePallasError::InvalidSignature("Failed to generate signature".into()),
            )?;

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
}
