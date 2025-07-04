use ark_ff::{BigInt, PrimeField};
use frost_bluepallas::{transactions::Transaction, translate::translate_msg, Error};
use frost_core::Ciphersuite;
use mina_signer::{Keypair, PubKey, Signer};
use serde::{
    ser::{SerializeStruct, Serializer},
    Serialize,
};

struct Sig {
    field: BigInt<4>,
    scalar: BigInt<4>,
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
struct PubKeySer {
    pubKey: PubKey,
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
struct TransactionSignature {
    publicKey: PubKeySer,
    signature: Sig,
    payload: Transaction,
}

fn main() -> Result<(), Error> {
    // Private key in hex format
    let private_key_hex = "35dcca7620128d240cc3319c83dc6402ad439038361ba853af538a4cea3ddabc";
    let mina_keypair =
        Keypair::from_hex(private_key_hex).map_err(|_| Error::DeserializationError)?;

    println!("Private key: {:?}", mina_keypair.secret);

    let recipient_pubkey =
        PubKey::from_address("B62qkcvM4DZE7k23ZHMLt1uaMVcixuxxuyz1XNJNCLkFbitDdUHxWs1")
            .map_err(|_| Error::DeserializationError)?;

    // Generate tx
    let tx = Transaction::new_payment(
        mina_keypair.public.clone(),
        recipient_pubkey,
        1000000000,
        1000000000,
        0,
    );
    //let tx = tx.set_memo_str("Hello World!");
    let signing_key = frost_bluepallas::translate::translate_minask(&mina_keypair)
        .map_err(|_| Error::DeserializationError)?;

    let msg = translate_msg(&tx);

    // Sign the transaction with FROST
    let (sig, vk) = frost_bluepallas::helper::generate_signature_from_sk(
        &msg,
        &signing_key,
        &mut rand_core::OsRng,
    )
    .map_err(|_| Error::MalformedSignature)?;

    // Print out signature and verifying key
    // Convert signature to Mina format
    let mina_sig = frost_bluepallas::translate::translate_sig(&sig)
        .map_err(|_| Error::DeserializationError)?;
    // Print transaction as json

    // Convert signature to big ints
    let sig_base: BigInt<4> = mina_sig.rx.into_bigint();
    let sig_scalar: BigInt<4> = mina_sig.s.into_bigint();

    let tx_sig = TransactionSignature {
        signature: Sig {
            field: sig_base,
            scalar: sig_scalar,
        },
        payload: tx.clone(),
        publicKey: PubKeySer {
            pubKey: mina_keypair.public.clone(),
        },
    };

    let out = serde_json::to_string_pretty(&tx_sig).unwrap();

    println!("Transaction Signature: {}", out);

    let chall = frost_bluepallas::PallasPoseidon::challenge(sig.R(), &vk, &msg)?;
    println!("Challenge: {:?}", chall);

    let mut ctx = mina_signer::create_legacy(mina_signer::NetworkId::TESTNET);
    let res = ctx.verify(&mina_sig, &mina_keypair.public, &tx);
    if res {
        println!("Mina signature verification succeeded");
    } else {
        println!("Mina signature verification failed");
    }

    Ok(())
}
