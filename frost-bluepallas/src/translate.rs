use crate::{PallasPoseidon, SigningKey};
use ark_ec::CurveGroup;
use frost_core::{Scalar, Signature as FrSig, VerifyingKey};
use mina_hasher::Hashable;
// Fr for frost
use mina_signer::{pubkey::PubKey, signature::Signature as MinaSig, NetworkId};

// temporary till we sort out proper error messages
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
// Note
// CurvePoint = Affine<PallasParameters>                                        mina side
// PallasProjective = Projective<PallasParameters> (= Element<PallasPoseidon>)  frost side
// The ScalarField type on the mina and frost side are the same!

/// Convert FROST public key to Mina public key
/// The `VerifyingKey` is the public key in FROST, which is a point on the curve.
pub fn translate_pk(fr_pk: &VerifyingKey<PallasPoseidon>) -> Result<PubKey> {
    Ok(PubKey::from_point_unsafe(fr_pk.to_element().into_affine()))
}

/// Convert FROST signature to Mina signature
/// The `R` field is the commitment to the nonce, and `z` is the response to the challenge.
pub fn translate_sig(fr_sig: &FrSig<PallasPoseidon>) -> Result<MinaSig> {
    let rx = fr_sig.R().into_affine().x;
    let z: Scalar<PallasPoseidon> = *fr_sig.z();

    Ok(MinaSig { rx, s: z })
}

/// Convert Hashable Mina message to Vec<u8>
pub fn translate_msg<H>(msg: &H) -> Vec<u8>
where
    H: Hashable<D = NetworkId>,
{
    msg.to_roinput().serialize()
}

pub fn translate_minask(msg: &mina_signer::Keypair) -> Result<SigningKey> {
    // Convert mina SecKey to FROST SigningKey
    let scalar = msg.secret.scalar();
    SigningKey::from_scalar(*scalar).map_err(|e| e.into())
}

#[cfg(test)]
mod tests {
    use crate::hasher::PallasMessage;

    use super::*;
    use ark_ff::fields::models::fp::{Fp, MontBackend};
    use frost_core::SigningKey;
    use mina_curves::pasta::fields::fq::FrConfig;
    use mina_signer::{seckey::SecKey, NetworkId};

    #[test]
    fn test_translate_pk() -> Result<()> {
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
        let fr_pk: VerifyingKey<PallasPoseidon> = fr_sk.into();

        assert_eq!(translate_pk(&fr_pk)?, mina_pk);
        Ok(())
    }

    #[test]
    fn check_hashable_impl() -> Result<()> {
        // panics if prefix.len() > MAX_DOMAIN_STRING_LEN
        mina_signer::create_legacy::<PallasMessage>(NetworkId::TESTNET);
        Ok(())
    }
}
