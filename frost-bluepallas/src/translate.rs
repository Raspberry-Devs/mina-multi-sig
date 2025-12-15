use crate::{errors::BluePallasResult, BluePallas, SigningKey};
use ark_ec::CurveGroup;
use frost_core::{Scalar, Signature as FrSig, VerifyingKey};
use mina_hasher::Hashable;
// Fr for frost
use mina_signer::{pubkey::PubKey, signature::Signature as MinaSig, NetworkId};

// Note
// CurvePoint = Affine<PallasParameters>                                        mina side
// PallasProjective = Projective<PallasParameters> (= Element<BluePallas>)  frost side
// The ScalarField type on the mina and frost side are the same!

/// Convert FROST public key to Mina public key
/// The `VerifyingKey` is the public key in FROST, which is a point on the curve.
pub fn translate_pk(fr_pk: &VerifyingKey<BluePallas>) -> BluePallasResult<PubKey> {
    Ok(PubKey::from_point_unsafe(fr_pk.to_element().into_affine()))
}

/// Convert FROST signature to Mina signature
/// The `R` field is the commitment to the nonce, and `z` is the response to the challenge.
pub fn translate_sig(fr_sig: &FrSig<BluePallas>) -> BluePallasResult<MinaSig> {
    let rx = fr_sig.R().into_affine().x;
    let z: Scalar<BluePallas> = *fr_sig.z();

    Ok(MinaSig { rx, s: z })
}

/// Trait for types that can be translated to a Mina message
pub trait Translatable: Hashable<D = NetworkId> {
    fn translate_msg(&self) -> Vec<u8>;
    fn from_bytes(bytes: &[u8]) -> BluePallasResult<Self>
    where
        Self: Sized;
}

pub fn translate_minask(msg: &mina_signer::Keypair) -> BluePallasResult<SigningKey> {
    // Convert mina SecKey to FROST SigningKey
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
}
