use frost_core as frost;

#[cfg(test)]
mod tests {

    use std::collections::BTreeMap;

    use ark_serialize::CanonicalSerialize;
    use frost_bluepallas::{Error, PallasPoseidon};
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

    #[allow(clippy::needless_borrows_for_generic_args)]
    #[test]
    fn check_sign_with_frost_bluepallas_sign() {
        let mut rng = rand_core::OsRng;
        let max_signers = 5;
        let min_signers = 3;
        let (shares, pubkey_package) = frost_bluepallas::keys::generate_with_dealer(
            max_signers,
            min_signers,
            frost_bluepallas::keys::IdentifierList::Default,
            &mut rng,
        )
        .unwrap();

        // Verifies the secret shares from the dealer and store them in a BTreeMap.
        // In practice, the KeyPackages must be sent to its respective participants
        // through a confidential and authenticated channel.
        let mut key_packages: BTreeMap<_, _> = BTreeMap::new();

        for (identifier, secret_share) in shares {
            let key_package = frost_bluepallas::keys::KeyPackage::try_from(secret_share).unwrap();
            key_packages.insert(identifier, key_package);
        }

        let mut nonces_map = BTreeMap::new();
        let mut commitments_map = BTreeMap::new();

        ////////////////////////////////////////////////////////////////////////////
        // Round 1: generating nonces and signing commitments for each participant
        ////////////////////////////////////////////////////////////////////////////

        // In practice, each iteration of this loop will be executed by its respective participant.
        for participant_index in 1..=min_signers {
            let participant_identifier = participant_index.try_into().expect("should be nonzero");
            let key_package = &key_packages[&participant_identifier];
            // Generate one (1) nonce and one SigningCommitments instance for each
            // participant, up to _threshold_.
            let (nonces, commitments) =
                frost_bluepallas::round1::commit(key_package.signing_share(), &mut rng);
            // In practice, the nonces must be kept by the participant to use in the
            // next round, while the commitment must be sent to the coordinator
            // (or to every other participant if there is no coordinator) using
            // an authenticated channel.
            nonces_map.insert(participant_identifier, nonces);
            commitments_map.insert(participant_identifier, commitments);
        }

        // This is what the signature aggregator / coordinator needs to do:
        // - decide what message to sign
        // - take one (unused) commitment per signing participant
        let mut signature_shares = BTreeMap::new();
        let message = "message to sign".as_bytes();
        let signing_package = frost_bluepallas::SigningPackage::new(commitments_map, message);

        ////////////////////////////////////////////////////////////////////////////
        // Round 2: each participant generates their signature share
        ////////////////////////////////////////////////////////////////////////////

        // In practice, each iteration of this loop will be executed by its respective participant.
        for participant_identifier in nonces_map.keys() {
            let key_package = &key_packages[participant_identifier];

            let nonces = &nonces_map[participant_identifier];

            // Each participant generates their signature share.
            let signature_share =
                frost_bluepallas::round2::sign(&signing_package, nonces, key_package).unwrap();

            // In practice, the signature share must be sent to the Coordinator
            // using an authenticated channel.
            signature_shares.insert(*participant_identifier, signature_share);
        }

        ////////////////////////////////////////////////////////////////////////////
        // Aggregation: collects the signing shares from all participants,
        // generates the final signature.
        ////////////////////////////////////////////////////////////////////////////

        // Aggregate (also verifies the signature shares)
        let group_signature =
            frost_bluepallas::aggregate(&signing_package, &signature_shares, &pubkey_package)
                .unwrap();

        // Check that the threshold signature can be verified by the group public
        // key (the verification key).
        let is_signature_valid = pubkey_package
            .verifying_key()
            .verify(message, &group_signature)
            .is_ok();
        assert!(is_signature_valid);
    }
}
