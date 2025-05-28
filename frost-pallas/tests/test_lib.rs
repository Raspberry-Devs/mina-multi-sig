#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    // code snippets from https://frost.zfnd.org/tutorial.html
    #[test]
    fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        // trusted dealer key generation
        let mut rng = rand_core::OsRng;
        let max_signers = 5;
        let min_signers = 3;
        let (shares, pubkey_package) = frost::keys::generate_with_dealer(
            max_signers,
            min_signers,
            frost::keys::IdentifierList::<PallasPoseidon>::Default,
            &mut rng,
        )?;

        // signing
        let (nonces, commitments) = frost::round1::commit(key_package.signing_share(), &mut rng);
        let message = "message to sign".as_bytes();
        let signing_package = frost::SigningPackage::new(commitments_map, message);
        Ok(())
    }
}
