//! BluePallas-specific conversions for `mina-tx`.
//!
//! Keep crypto bridge code here so core transaction modules stay clean.

use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::PrimeField;
use frost_bluepallas::{
    pallas_message::{translate_pk, PallasMessage},
    BluePallas,
};
use frost_core::{Scalar, Signature as FrSig, VerifyingKey};
use mina_hasher::Hashable;

use crate::{
    errors::BluePallasError,
    signatures::{PubKeySer, Sig, TransactionSignature},
    transactions::TransactionEnvelope,
};

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
        let z: Scalar<BluePallas> = *self.z();

        Ok(Sig {
            field: x,
            scalar: z.into_bigint(),
        })
    }
}

impl TryFrom<VerifyingKey<BluePallas>> for PubKeySer {
    type Error = BluePallasError;

    fn try_from(vk: VerifyingKey<BluePallas>) -> Result<Self, Self::Error> {
        translate_pk(&vk)
            .map(|pub_key| PubKeySer { pubKey: pub_key })
            .map_err(|e| BluePallasError::InvalidPublicKey(e.to_string()))
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
        public_key: VerifyingKey<BluePallas>,
        signature: FrSig<BluePallas>,
        payload: TransactionEnvelope,
    ) -> Result<(Self, Option<crate::zkapp_tx::SignatureInjectionResult>), BluePallasError> {
        let pubkey: PubKeySer = public_key.try_into()?;
        let signature: Sig = signature.try_into()?;
        Ok(Self::new_with_zkapp_injection(pubkey, signature, payload))
    }

    pub fn from_frost_signature_bytes(
        public_key: VerifyingKey<BluePallas>,
        signature_bytes: &[u8],
        payload: TransactionEnvelope,
    ) -> Result<(Self, Option<crate::zkapp_tx::SignatureInjectionResult>), BluePallasError> {
        let signature = FrSig::<BluePallas>::deserialize(signature_bytes)
            .map_err(|e| BluePallasError::DeSerializationError(e.to_string()))?;

        Self::from_frost_signature(public_key, signature, payload)
    }
}
