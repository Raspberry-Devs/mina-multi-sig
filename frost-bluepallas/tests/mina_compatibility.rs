use ark_ff::PrimeField;
use frost_bluepallas::{
    hasher::{hash_challenge, PallasMessage},
    translate::{translate_pk, translate_sig},
    PallasPoseidon,
};
use frost_core as frost;

use mina_hasher::{Hashable, Hasher, ROInput};
use mina_signer::{BaseField, NetworkId, PubKey, ScalarField, Signer};
use rand_core::SeedableRng;

#[derive(Clone)]
struct Message<H: Hashable> {
    input: H,
    pub_key_x: BaseField,
    pub_key_y: BaseField,
    rx: BaseField,
}

impl<H> Hashable for Message<H>
where
    H: Hashable<D = NetworkId>,
{
    type D = H::D;

    fn to_roinput(&self) -> ROInput {
        self.input
            .to_roinput()
            .append_field(self.pub_key_x)
            .append_field(self.pub_key_y)
            .append_field(self.rx)
    }

    fn domain_string(domain_param: Self::D) -> Option<String> {
        H::domain_string(domain_param)
    }
}

fn message_hash<H>(pub_key: &PubKey, rx: BaseField, input: &H) -> ScalarField
where
    H: Hashable<D = NetworkId>,
{
    let mut hasher = mina_hasher::create_legacy::<Message<H>>(NetworkId::TESTNET);

    let schnorr_input = Message::<H> {
        input: input.clone(),
        pub_key_x: pub_key.point().x,
        pub_key_y: pub_key.point().y,
        rx,
    };

    // Squeeze and convert from base field element to scalar field element
    // Since the difference in modulus between the two fields is < 2^125, w.h.p., a
    // random value from one field will fit in the other field.
    ScalarField::from(hasher.hash(&schnorr_input).into_bigint())
}

#[test]
fn compare_frost_mina_challenge() {
    let rng = rand_chacha::ChaChaRng::seed_from_u64(0);

    let (fr_msg, fr_sig, fr_pk) =
        frost::tests::ciphersuite_generic::check_sign_with_dealer::<PallasPoseidon, _>(rng);

    let frost_out = hash_challenge(fr_sig.R(), &fr_pk, fr_msg.as_ref());

    let mina_pk = translate_pk(&fr_pk).unwrap();
    let mina_sig = translate_sig(&fr_sig).unwrap();
    let mina_msg = PallasMessage(fr_msg.clone());

    // Perform the challenge hash
    let mina_out = message_hash(&mina_pk, mina_sig.rx, &mina_msg);

    // Compare the two outputs
    assert_eq!(
        frost_out, mina_out,
        "Frost and Mina challenge hashes do not match"
    );
}

#[test]
fn frost_sign_mina_verify() -> Result<(), Box<dyn std::error::Error>> {
    let rng = rand_chacha::ChaChaRng::seed_from_u64(0);

    let (fr_msg, fr_sig, fr_pk) =
        frost::tests::ciphersuite_generic::check_sign_with_dealer::<PallasPoseidon, _>(rng);

    let _test = hash_challenge(fr_sig.R(), &fr_pk, fr_msg.as_ref());

    let mina_pk = translate_pk(&fr_pk)?;
    let mina_sig = translate_sig(&fr_sig)?;
    let mina_msg = PallasMessage(fr_msg.clone());

    let mut ctx = mina_signer::create_legacy::<PallasMessage>(NetworkId::TESTNET);
    assert!(ctx.verify(&mina_sig, &mina_pk, &mina_msg));
    Ok(())
}
