use crate::PallasPoseidon;
use ark_ec::short_weierstrass::{Affine, Projective};
use ark_ec::AffineRepr;
use frost_core::{Ciphersuite, Group, Scalar, Signature as FrSig, VerifyingKey}; // Fr for frost
use mina_curves::pasta::PallasParameters;
use mina_hasher::{Hashable, ROInput};
use mina_signer::{pubkey::PubKey, signature::Signature as MinaSig, NetworkId};

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
    let pk_bytes: Vec<u8> = fr_pk.serialize()?;
    let pk_projective: Projective<PallasParameters> =
        <<PallasPoseidon as Ciphersuite>::Group as Group>::deserialize(
            pk_bytes.as_slice().try_into()?,
        )?;
    let pk_affine: Affine<PallasParameters> = pk_projective.into();

    Ok(PubKey::from_point_unsafe(pk_affine))
}

pub fn translate_sig(fr_sig: &FrSig<PallasPoseidon>) -> Result<MinaSig> {
    let r: &Projective<PallasParameters> = fr_sig.R();
    let z: &Scalar<PallasPoseidon> = fr_sig.z();
    let r_affine: Affine<PallasParameters> = r.clone().into(); // Is this step required?
    let (rx, _ry) = r_affine
        .xy()
        .ok_or("nonce commitment is the point at infinity??!!")?;

    Ok(MinaSig {
        rx: rx.clone(),
        s: z.clone(),
    })
}

// This s temporary: it's easier to test that u8 slices as messages work correctly first
// We can implmement (or reuse) the Hashable trait for transaction as in this example afterwards:
// https://github.com/o1-labs/proof-systems/blob/master/signer/README.md?plain=1#L19-L40

#[derive(Clone, Debug)]
pub struct PallasMessage(pub Vec<u8>);

// Implement a hashable trait for a u8 slice
impl Hashable for PallasMessage {
    type D = NetworkId;

    fn to_roinput(&self) -> ROInput {
        ROInput::new().append_bytes(self.0.as_ref())
    }

    // copied from
    // https://github.com/o1-labs/proof-systems/blob/0.1.0/signer/tests/transaction.rs#L53-L61
    fn domain_string(network_id: NetworkId) -> Option<String> {
        // Domain strings must have length <= 20
        match network_id {
            NetworkId::MAINNET => "MinaSignatureMainnet",
            NetworkId::TESTNET => "CodaSignature", //"FROST-PALLAS-POSEIDON",
        }
        .to_string()
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::fields::models::fp::{Fp, MontBackend};
    use frost_core as frost;
    use frost_core::SigningKey;
    use mina_curves::pasta::fields::fq::FrConfig;
    use mina_signer::{keypair::Keypair, seckey::SecKey, Signer};
    use std::collections::BTreeMap;

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
    /// Generate a secret shares using trusted dealer.
    /// Reconstruct the (joint) secret key for signing using mina-signer
    /// Sign with frost api using the secret shares
    /// Check if the signatures coincide using the `translate_sig` function
    #[test]
    fn test_translate_sig() -> Result<()> {
        let mut rng = rand_core::OsRng;

        // Generate a secret shares using the frost api

        let max_signers = 5;
        let min_signers = 3;
        let (shares, pubkeys) = frost::keys::generate_with_dealer(
            max_signers,
            min_signers,
            frost::keys::IdentifierList::Default,
            &mut rng,
        )
        .unwrap();

        let mut key_packages: BTreeMap<
            frost::Identifier<PallasPoseidon>,
            frost::keys::KeyPackage<PallasPoseidon>,
        > = BTreeMap::new();

        for (k, v) in shares {
            let key_package = frost::keys::KeyPackage::try_from(v).unwrap();
            key_packages.insert(k, key_package);
        }

        // sign with frost

        let (fr_msg, fr_sig, fr_pk) = frost::tests::ciphersuite_generic::check_sign(
            min_signers,
            key_packages.clone(),
            rng,
            pubkeys,
        )
        .unwrap();

        // sign with mina

        let fr_sk = frost::keys::reconstruct(
            key_packages
                .values()
                .map(|x| x.clone())
                .collect::<Vec<_>>()
                .as_slice(),
        )?;

        let mina_sk = SecKey::new(fr_sk.to_scalar());
        // for some reason the mina `sign` function takes public key as well
        let mina_keypair: Keypair = Keypair {
            secret: mina_sk,
            public: translate_pk(&fr_pk)?,
        };

        let mina_msg = PallasMessage(fr_msg.clone());

        let mut ctx = mina_signer::create_legacy::<PallasMessage>(NetworkId::TESTNET);
        let mina_sig = ctx.sign(&mina_keypair, &mina_msg);

        // compare

        assert_eq!(mina_sig, translate_sig(&fr_sig)?);
        Ok(())
    }
}
