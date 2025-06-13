use crate::PallasPoseidon;
use ark_ec::CurveGroup;
use frost_core::{Scalar, Signature as FrSig, VerifyingKey}; // Fr for frost
use mina_signer::{pubkey::PubKey, signature::Signature as MinaSig};

// temporary till we sort out proper error messages
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
// Note
// CurvePoint = Affine<PallasParameters>                                        mina side
// PallasProjective = Projective<PallasParameters> (= Element<PallasPoseidon>)  frost side
// The ScalarField type on the mina and frost side are the same!

pub fn translate_pk(fr_pk: &VerifyingKey<PallasPoseidon>) -> Result<PubKey> {
    // A VerifyingKey is just a group element in some wrapper structs
    // But the api doesn't seem to expose a way to extract the underlying element
    // So I serialize VerifyingKey and deserialize into Element
    // VerifyingKey<C: Ciphersuite>::serialize() is in fact exactly Ciphersuite::Group::serialize
    //     (with an extra `?.as_ref().to_vec()`)
    // reference: https://github.com/ZcashFoundation/frost/blob/frost-secp256k1/v2.1.0/frost-core/src/serialization.rs#L88
    // This is however depending on the implmenetation details of frost not to change not just the
    //     public api
    Ok(PubKey::from_point_unsafe(fr_pk.to_element().into_affine()))
}

pub fn translate_sig(fr_sig: &FrSig<PallasPoseidon>) -> Result<MinaSig> {
    let rx = fr_sig.R().into_affine().x;
    let z: Scalar<PallasPoseidon> = *fr_sig.z();

    Ok(MinaSig { rx, s: z })
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
