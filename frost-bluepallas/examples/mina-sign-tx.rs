use frost_bluepallas::{transactions::Transaction, translate::translate_msg, Error};
use mina_signer::{Keypair, PubKey};

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
        1,
    );
    let tx = tx.set_memo_str("Hello World!");
    let signing_key = frost_bluepallas::translate::translate_minask(&mina_keypair)
        .map_err(|_| Error::DeserializationError)?;

    let msg = translate_msg(&tx);

    // Sign the transaction with FROST
    let (sig, _vk) = frost_bluepallas::helper::generate_signature_from_sk(
        &msg,
        &signing_key,
        &mut rand_core::OsRng,
    )
    .map_err(|_| Error::MalformedSignature)?;

    // Print out signature and verifying key
    // Convert signature to Mina format
    let mina_sig = frost_bluepallas::translate::translate_sig(&sig)
        .map_err(|_| Error::DeserializationError)?;
    println!("FROST Signature: {:?}", mina_sig);
    // Print transaction as json
    println!("Transaction: {:#?}", tx);

    Ok(())
}
