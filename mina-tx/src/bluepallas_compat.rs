//! BluePallas-specific conversions for `mina-tx`.
//!
//! Keep crypto bridge code here so core transaction modules stay clean.

use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::PrimeField;
use frost_bluepallas::BluePallas;
use frost_core::{Scalar, Signature as FrSig, VerifyingKey};
use mina_hasher::Hashable;

use crate::{
    errors::MinaTxError,
    pallas_message::{translate_pk, PallasMessage},
    signatures::{PubKeySer, Sig, TransactionSignature},
    transactions::TransactionEnvelope,
};

type BluePallasSuite = BluePallas<PallasMessage>;

impl TryFrom<FrSig<BluePallasSuite>> for Sig {
    type Error = MinaTxError;

    fn try_from(value: FrSig<BluePallasSuite>) -> Result<Sig, Self::Error> {
        let x = value
            .R()
            .into_affine()
            .x()
            .ok_or_else(|| {
                MinaTxError::InvalidSignature("Failed to convert x coordinate to bigint".into())
            })?
            .into_bigint();
        let z: Scalar<BluePallasSuite> = *value.z();

        Ok(Sig {
            field: x,
            scalar: z.into_bigint(),
        })
    }
}

impl TryFrom<VerifyingKey<BluePallasSuite>> for PubKeySer {
    type Error = MinaTxError;

    fn try_from(vk: VerifyingKey<BluePallasSuite>) -> Result<Self, Self::Error> {
        translate_pk(&vk)
            .map(|pub_key| PubKeySer { pubKey: pub_key })
            .map_err(|e| MinaTxError::InvalidPublicKey(e.to_string()))
    }
}

impl TransactionEnvelope {
    pub fn to_pallas_message(&self) -> PallasMessage {
        PallasMessage::from_parts(self.to_roinput(), self.network_id(), self.is_legacy())
    }
}

impl From<&TransactionEnvelope> for PallasMessage {
    fn from(value: &TransactionEnvelope) -> Self {
        value.to_pallas_message()
    }
}

impl TransactionSignature {
    pub fn from_frost_signature(
        public_key: VerifyingKey<BluePallasSuite>,
        signature: FrSig<BluePallasSuite>,
        payload: TransactionEnvelope,
    ) -> Result<(Self, Option<crate::zkapp_tx::SignatureInjectionResult>), MinaTxError> {
        let pubkey: PubKeySer = public_key.try_into()?;
        let signature: Sig = signature.try_into()?;
        Ok(Self::new_with_zkapp_injection(pubkey, signature, payload))
    }

    pub fn from_frost_signature_bytes(
        public_key: VerifyingKey<BluePallasSuite>,
        signature_bytes: &[u8],
        payload: TransactionEnvelope,
    ) -> Result<(Self, Option<crate::zkapp_tx::SignatureInjectionResult>), MinaTxError> {
        let signature = FrSig::<BluePallasSuite>::deserialize(signature_bytes)
            .map_err(|e| MinaTxError::DeSerializationError(e.to_string()))?;

        Self::from_frost_signature(public_key, signature, payload)
    }
}
