use frost_core as frost;

#[cfg(test)]
mod tests {

    use std::collections::BTreeMap;

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

    #[test]
    fn check_even_y_frost_core() {
        let mut rng = rand_core::OsRng;

        // Since there is a 50% chance of the public key having an odd Y (which
        // we need to actually test), loop until we get an odd Y.
        loop {
            let max_signers = 5;
            let min_signers = 3;
            // Generate keys with frost-core function, which doesn't ensure even Y
            let (shares, public_key_package) =
                frost::keys::generate_with_dealer::<PallasPoseidon, _>(
                    max_signers,
                    min_signers,
                    frost::keys::IdentifierList::Default,
                    &mut rng,
                )
                .unwrap();

            println!("Looping!");
            if !public_key_package.has_even_y() {
                println!("Found odd Y in public key package, testing even_y conversion");
                // Test consistency of into_even_y() for PublicKeyPackage
                let even_public_key_package_is_even_none =
                    public_key_package.clone().into_even_y(None);
                let even_public_key_package_is_even_false =
                    public_key_package.clone().into_even_y(Some(false));
                assert_eq!(
                    even_public_key_package_is_even_false,
                    even_public_key_package_is_even_none
                );
                assert_ne!(public_key_package, even_public_key_package_is_even_false);
                assert_ne!(public_key_package, even_public_key_package_is_even_none);

                // Test consistency of into_even_y() for SecretShare (arbitrarily on
                // the first secret share)
                let secret_share = shares.first_key_value().unwrap().1.clone();
                let even_secret_share_is_even_none = secret_share.clone().into_even_y(None);
                let even_secret_share_is_even_false = secret_share.clone().into_even_y(Some(false));
                assert_eq!(
                    even_secret_share_is_even_false,
                    even_secret_share_is_even_none
                );
                assert_ne!(secret_share, even_secret_share_is_even_false);
                assert_ne!(secret_share, even_secret_share_is_even_none);

                // Make secret shares even, then convert into KeyPackages
                let key_packages_evened_before: BTreeMap<_, _> = shares
                    .clone()
                    .into_iter()
                    .map(|(identifier, share)| {
                        Ok((
                            identifier,
                            frost::keys::KeyPackage::try_from(share.into_even_y(None))?,
                        ))
                    })
                    .collect::<Result<_, frost::Error<PallasPoseidon>>>()
                    .unwrap();
                // Convert into KeyPackages, then make them even
                let key_packages_evened_after: BTreeMap<_, _> = shares
                    .into_iter()
                    .map(|(identifier, share)| {
                        Ok((
                            identifier,
                            frost::keys::KeyPackage::try_from(share)?.into_even_y(None),
                        ))
                    })
                    .collect::<Result<_, frost::Error<PallasPoseidon>>>()
                    .unwrap();
                // Make sure they are equal
                assert_eq!(key_packages_evened_after, key_packages_evened_before);

                #[allow(clippy::needless_borrows_for_generic_args)]
                // Check if signing works with evened keys
                frost::tests::ciphersuite_generic::check_sign(
                    min_signers,
                    key_packages_evened_after,
                    &mut rng,
                    even_public_key_package_is_even_none,
                )
                .unwrap();

                // We managed to test it; break the loop and return
                break;
            }
        }
    }

    #[test]
    fn check_even_y_bluepallas() {
        let mut rng = rand_core::OsRng;

        // Since there is a ~50% chance of having a odd Y internally, to make sure
        // that odd Ys are converted to even, we test multiple times to increase
        // the chance of an odd Y being generated internally
        for _ in 0..32 {
            let max_signers = 5;
            let min_signers = 3;
            // Generate keys with reexposed bluepallas function, which ensures even Y
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
    fn check_even_y_bluepallas_dkg() {
        for _ in 0..32 {
            let mut rng = rand_core::OsRng;

            let max_signers = 5;
            let min_signers = 3;
            // Generate keys with reexposed bluepallas function, which ensures even Y

            ////////////////////////////////////////////////////////////////////////////
            // Key generation, Round 1
            ////////////////////////////////////////////////////////////////////////////

            // Keep track of each participant's round 1 secret package.
            // In practice each participant will keep its copy; no one
            // will have all the participant's packages.
            let mut round1_secret_packages = BTreeMap::new();

            // Keep track of all round 1 packages sent to the given participant.
            // This is used to simulate the broadcast; in practice the packages
            // will be sent through some communication channel.
            let mut received_round1_packages = BTreeMap::new();

            // For each participant, perform the first part of the DKG protocol.
            // In practice, each participant will perform this on their own environments.
            for participant_index in 1..=max_signers {
                let participant_identifier =
                    participant_index.try_into().expect("should be nonzero");

                #[allow(clippy::needless_borrows_for_generic_args)]
                let (round1_secret_package, round1_package) = frost_bluepallas::keys::dkg::part1(
                    participant_identifier,
                    max_signers,
                    min_signers,
                    &mut rng,
                )
                .unwrap();

                // Store the participant's secret package for later use.
                // In practice each participant will store it in their own environment.
                round1_secret_packages.insert(participant_identifier, round1_secret_package);

                // "Send" the round 1 package to all other participants. In this
                // test this is simulated using a BTreeMap; in practice this will be
                // sent through some communication channel.
                for receiver_participant_index in 1..=max_signers {
                    if receiver_participant_index == participant_index {
                        continue;
                    }
                    let receiver_participant_identifier: frost_bluepallas::Identifier =
                        receiver_participant_index
                            .try_into()
                            .expect("should be nonzero");
                    received_round1_packages
                        .entry(receiver_participant_identifier)
                        .or_insert_with(BTreeMap::new)
                        .insert(participant_identifier, round1_package.clone());
                }
            }

            ////////////////////////////////////////////////////////////////////////////
            // Key generation, Round 2
            ////////////////////////////////////////////////////////////////////////////

            // Keep track of each participant's round 2 secret package.
            // In practice each participant will keep its copy; no one
            // will have all the participant's packages.
            let mut round2_secret_packages = BTreeMap::new();

            // Keep track of all round 2 packages sent to the given participant.
            // This is used to simulate the broadcast; in practice the packages
            // will be sent through some communication channel.
            let mut received_round2_packages = BTreeMap::new();

            // For each participant, perform the second part of the DKG protocol.
            // In practice, each participant will perform this on their own environments.
            for participant_index in 1..=max_signers {
                let participant_identifier =
                    participant_index.try_into().expect("should be nonzero");
                let round1_secret_package = round1_secret_packages
                    .remove(&participant_identifier)
                    .unwrap();
                let round1_packages = &received_round1_packages[&participant_identifier];
                let (round2_secret_package, round2_packages) =
                    frost_bluepallas::keys::dkg::part2(round1_secret_package, round1_packages)
                        .unwrap();

                // Store the participant's secret package for later use.
                // In practice each participant will store it in their own environment.
                round2_secret_packages.insert(participant_identifier, round2_secret_package);

                // "Send" the round 2 package to all other participants. In this
                // test this is simulated using a BTreeMap; in practice this will be
                // sent through some communication channel.
                // Note that, in contrast to the previous part, here each other participant
                // gets its own specific package.
                for (receiver_identifier, round2_package) in round2_packages {
                    received_round2_packages
                        .entry(receiver_identifier)
                        .or_insert_with(BTreeMap::new)
                        .insert(participant_identifier, round2_package);
                }
            }

            ////////////////////////////////////////////////////////////////////////////
            // Key generation, final computation
            ////////////////////////////////////////////////////////////////////////////

            // Keep track of each participant's long-lived key package.
            // In practice each participant will keep its copy; no one
            // will have all the participant's packages.
            let mut key_packages = BTreeMap::new();

            // Keep track of each participant's public key package.
            // In practice, if there is a Coordinator, only they need to store the set.
            // If there is not, then all candidates must store their own sets.
            // All participants will have the same exact public key package.
            let mut pubkey_packages = BTreeMap::new();

            // For each participant, perform the third part of the DKG protocol.
            // In practice, each participant will perform this on their own environments.
            for participant_index in 1..=max_signers {
                let participant_identifier =
                    participant_index.try_into().expect("should be nonzero");
                let round2_secret_package = &round2_secret_packages[&participant_identifier];
                let round1_packages = &received_round1_packages[&participant_identifier];
                let round2_packages = &received_round2_packages[&participant_identifier];
                // ANCHOR: dkg_part3
                let (key_package, pubkey_package) = frost_bluepallas::keys::dkg::part3(
                    round2_secret_package,
                    round1_packages,
                    round2_packages,
                )
                .unwrap();

                assert!(key_package.has_even_y());
                assert!(pubkey_package.has_even_y());
                // ANCHOR_END: dkg_part3
                key_packages.insert(participant_identifier, key_package);
                pubkey_packages.insert(participant_identifier, pubkey_package);
            }

            // With its own key package and the pubkey package, each participant can now proceed
            // to sign with FROST.
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
