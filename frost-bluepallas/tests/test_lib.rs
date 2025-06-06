use frost_core as frost;

#[cfg(test)]
mod tests {

    use ark_serialize::CanonicalSerialize;
    use frost_bluepallas::{keys::EvenY, Error, PallasPoseidon};
    use frost_core::{Ciphersuite, Group, GroupError};

    use super::*;

    #[test]
    fn test_sign() {
        let rng = rand_core::OsRng;

        frost::tests::ciphersuite_generic::check_sign_with_dealer::<PallasPoseidon, _>(rng);
    }

    #[test]
    fn check_sign_with_dkg() {
        let rng = rand_core::OsRng;

        frost::tests::ciphersuite_generic::check_sign_with_dkg::<PallasPoseidon, _>(rng);
    }

    #[test]
    fn check_deserialize_identity() {
        let r = <PallasPoseidon as Ciphersuite>::Group::serialize(
            &<PallasPoseidon as Ciphersuite>::Group::identity(),
        );
        assert_eq!(r, Err(GroupError::InvalidIdentityElement));
        let raw_identity = <PallasPoseidon as Ciphersuite>::Group::identity();

        let mut out = [0u8; 96];
        raw_identity.serialize_compressed(&mut out[..]).unwrap();
        let r = <PallasPoseidon as Ciphersuite>::Group::deserialize(&out);
        assert_eq!(r, Err(GroupError::InvalidIdentityElement));
    }

    // #[test]
    // fn check_even_y_frost_core() {
    //     let mut rng = rand_core::OsRng;

    //     // Since there is a 50% chance of the public key having an odd Y (which
    //     // we need to actually test), loop until we get an odd Y.
    //     loop {
    //         let max_signers = 5;
    //         let min_signers = 3;
    //         // Generate keys with frost-core function, which doesn't ensure even Y
    //         let (shares, public_key_package) =
    //             frost::keys::generate_with_dealer::<PallasPoseidon, _>(
    //                 max_signers,
    //                 min_signers,
    //                 frost::keys::IdentifierList::Default,
    //                 &mut rng,
    //             )
    //             .unwrap();

    //         if !public_key_package.has_even_y() {
    //             // Test consistency of into_even_y() for PublicKeyPackage
    //             let even_public_key_package_is_even_none = public_key_package.clone().into_even_y(None);
    //             let even_public_key_package_is_even_false =
    //                 public_key_package.clone().into_even_y(Some(false));
    //             assert_eq!(
    //                 even_public_key_package_is_even_false,
    //                 even_public_key_package_is_even_none
    //             );
    //             assert_ne!(public_key_package, even_public_key_package_is_even_false);
    //             assert_ne!(public_key_package, even_public_key_package_is_even_none);

    //             // Test consistency of into_even_y() for SecretShare (arbitrarily on
    //             // the first secret share)
    //             let secret_share = shares.first_key_value().unwrap().1.clone();
    //             let even_secret_share_is_even_none = secret_share.clone().into_even_y(None);
    //             let even_secret_share_is_even_false = secret_share.clone().into_even_y(Some(false));
    //             assert_eq!(
    //                 even_secret_share_is_even_false,
    //                 even_secret_share_is_even_none
    //             );
    //             assert_ne!(secret_share, even_secret_share_is_even_false);
    //             assert_ne!(secret_share, even_secret_share_is_even_none);

    //             // Make secret shares even, then convert into KeyPackages
    //             let key_packages_evened_before: BTreeMap<_, _> = shares
    //                 .clone()
    //                 .into_iter()
    //                 .map(|(identifier, share)| {
    //                     Ok((
    //                         identifier,
    //                         frost::keys::KeyPackage::try_from(share.into_even_y(None))?,
    //                     ))
    //                 })
    //                 .collect::<Result<_, frost::Error<PallasPoseidon>>>()
    //                 .unwrap();
    //             // Convert into KeyPackages, then make them even
    //             let key_packages_evened_after: BTreeMap<_, _> = shares
    //                 .into_iter()
    //                 .map(|(identifier, share)| {
    //                     Ok((
    //                         identifier,
    //                         frost::keys::KeyPackage::try_from(share)?.into_even_y(None),
    //                     ))
    //                 })
    //                 .collect::<Result<_, frost::Error<PallasPoseidon>>>()
    //                 .unwrap();
    //             // Make sure they are equal
    //             assert_eq!(key_packages_evened_after, key_packages_evened_before);

    //             // Check if signing works with evened keys
    //             frost::tests::ciphersuite_generic::check_sign(
    //                 min_signers,
    //                 key_packages_evened_after,
    //                 &mut rng,
    //                 even_public_key_package_is_even_none,
    //             )
    //             .unwrap();

    //             // We managed to test it; break the loop and return
    //             break;
    //         }
    //     }
    // }

    #[test]
    fn check_even_y_bluepallas() {
        let mut rng = rand_core::OsRng;

        // Since there is a ~50% chance of having a odd Y internally, to make sure
        // that odd Ys are converted to even, we test multiple times to increase
        // the chance of an odd Y being generated internally
        for _ in 0..16 {
            let max_signers = 5;
            let min_signers = 3;
            // Generate keys with reexposed reddsa function, which ensures even Y
            let (shares, public_key_package) = frost_bluepallas::keys::generate_with_dealer::<_>(
                max_signers,
                min_signers,
                frost::keys::IdentifierList::Default,
                &mut rng,
            )
            .unwrap();

            assert!(public_key_package.has_even_y());
            assert!(shares.values().all(|s| s.has_even_y()));
        }
    }

    #[test]
    fn check_zero_key_fails() {
        frost::tests::ciphersuite_generic::check_zero_key_fails::<PallasPoseidon>()
    }

    #[test]
    fn check_share_generation() {
        frost::tests::ciphersuite_generic::check_share_generation::<PallasPoseidon, _>(
            rand_core::OsRng,
        );
    }

    #[test]
    fn check_share_generation_fails_with_invalid_signers() {
        use frost_bluepallas::Error;

        let min_signers = 3;
        let max_signers = 5;
        frost::tests::ciphersuite_generic::check_share_generation_fails_with_invalid_signers::<
            PallasPoseidon,
            _,
        >(
            min_signers,
            max_signers,
            Error::InvalidCoefficients,
            rand_core::OsRng,
        );
    }

    #[test]
    fn check_sign_with_dealer_fails_with_invalid_signers() {
        let rng = rand_core::OsRng;

        frost::tests::ciphersuite_generic::check_sign_with_dealer_fails_with_invalid_signers::<
            PallasPoseidon,
            _,
        >(0, 1, Error::InvalidMinSigners, rng);

        frost::tests::ciphersuite_generic::check_sign_with_dealer_fails_with_invalid_signers::<
            PallasPoseidon,
            _,
        >(5, 0, Error::InvalidMaxSigners, rng);
    }

    #[test]
    fn check_sign_with_dealer_and_identifiers() {
        frost::tests::ciphersuite_generic::check_sign_with_dealer_and_identifiers::<
            PallasPoseidon,
            _,
        >(rand_core::OsRng);
    }

    #[test]
    fn check_sign_with_incorrect_commitments() {
        let rng = rand_core::OsRng;

        frost::tests::ciphersuite_generic::check_sign_with_incorrect_commitments::<PallasPoseidon, _>(
            rng,
        );
    }

    #[test]
    fn check_sign_with_missing_identifier() {
        let rng = rand_core::OsRng;

        frost::tests::ciphersuite_generic::check_sign_with_missing_identifier::<PallasPoseidon, _>(
            rng,
        );
    }
}
