use mina_signer::{keypair::KeypairError, Keypair};

fn main() -> Result<(), KeypairError> {
    // Generate shares from a private key

    // Generate a random private key on the scalar field
    let mut rng = rand_core::OsRng;
    let keypair = Keypair::rand(&mut rng)?;

    println!("Generated keypair secret: {:?}", keypair.secret.scalar());
    println!("Private key: {:?}", keypair.secret.to_hex());
    println!("Public Key addr: {:?}", keypair.get_address());

    Ok(())
}
