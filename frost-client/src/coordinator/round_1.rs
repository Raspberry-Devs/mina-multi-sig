use frost_core::{self as frost, Ciphersuite};

use frost::{keys::PublicKeyPackage, round1::SigningCommitments, Identifier};

use std::{
    collections::BTreeMap,
    io::{BufRead, Write},
};

use super::{args::ProcessedArgs, comms::Comms};

#[derive(PartialEq, Debug)]
pub struct ParticipantsConfig<C: Ciphersuite> {
    pub commitments: BTreeMap<Identifier<C>, SigningCommitments<C>>,
    pub pub_key_package: PublicKeyPackage<C>,
}

pub async fn get_commitments<C: Ciphersuite>(
    args: &ProcessedArgs<C>,
    comms: &mut dyn Comms<C>,
    reader: &mut dyn BufRead,
    logger: &mut dyn Write,
) -> Result<ParticipantsConfig<C>, Box<dyn std::error::Error>> {
    let participants = read_commitments(args, comms, reader, logger).await?;
    if args.cli {
        print_participants(logger, &participants.commitments);
    }
    Ok(participants)
}

// TODO: validate min num of participants

// Input required:
// 1. public key package
// 2. number of participants
// 3. identifiers for all participants
async fn read_commitments<C: Ciphersuite>(
    args: &ProcessedArgs<C>,
    comms: &mut dyn Comms<C>,
    input: &mut dyn BufRead,
    logger: &mut dyn Write,
) -> Result<ParticipantsConfig<C>, Box<dyn std::error::Error>> {
    let commitments_list = comms
        .get_signing_commitments(input, logger, &args.public_key_package, args.num_signers)
        .await?;

    Ok(ParticipantsConfig {
        commitments: commitments_list,
        pub_key_package: args.public_key_package.clone(),
    })
}

pub fn print_participants<C: Ciphersuite>(
    logger: &mut dyn Write,
    participants: &BTreeMap<Identifier<C>, SigningCommitments<C>>,
) {
    writeln!(logger, "Selected participants: ",).unwrap();

    for p in participants.keys() {
        writeln!(logger, "{}", serde_json::to_string(p).unwrap()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use frost_bluepallas::{
        keys::{PublicKeyPackage, VerifyingShare},
        Error, Identifier, VerifyingKey,
    };

    use super::super::comms::cli::validate;

    const PUBLIC_KEY_1: &str = "0e097a0409c1a75f63af2f1d2b1c93384177242f4b06c99c64f92124d988c51e00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
    const PUBLIC_KEY_2: &str = "c0a3dfba8e5a6468aa575514205bbf392e2daee0c0cd69f57c195cb2ee92ea0100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
    const GROUP_PUBLIC_KEY: &str = "d256b73e945ca156dc19a4ea536568822690e3b2bc6f690ed7ac79b37e7db23300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";

    fn build_pub_key_package() -> PublicKeyPackage {
        let id_1 = Identifier::try_from(1).unwrap();
        let id_2: frost_core::Identifier<frost_bluepallas::PallasPoseidon> =
            Identifier::try_from(2).unwrap();

        let mut signer_pubkeys = BTreeMap::new();
        signer_pubkeys.insert(
            id_1,
            VerifyingShare::deserialize(&hex::decode(PUBLIC_KEY_1).unwrap()).unwrap(),
        );
        signer_pubkeys.insert(
            id_2,
            VerifyingShare::deserialize(&hex::decode(PUBLIC_KEY_2).unwrap()).unwrap(),
        );

        let group_public =
            VerifyingKey::deserialize(&hex::decode(GROUP_PUBLIC_KEY).unwrap()).unwrap();

        PublicKeyPackage::new(signer_pubkeys, group_public)
    }

    #[test]
    fn check_validate() {
        let id_1 = Identifier::try_from(1).unwrap();
        let id_2 = Identifier::try_from(2).unwrap();

        let id_list = [id_1];
        let key_package = build_pub_key_package();

        let validated = validate(id_2, &key_package, &id_list);

        assert!(validated.is_ok())
    }

    #[test]
    fn check_validation_errors_for_missing_identifiers() {
        let id_1 = Identifier::try_from(1).unwrap();
        let id_2 = Identifier::try_from(2).unwrap();
        let id_3 = Identifier::try_from(3).unwrap();

        let id_list = [id_1, id_2];
        let key_package = build_pub_key_package();

        let validated = validate(id_3, &key_package, &id_list);
        assert!(validated.is_err());
        assert!(validated == Err(Error::MalformedIdentifier))
    }

    #[test]
    fn check_validation_errors_for_duplicate_identifiers() {
        let id_1 = Identifier::try_from(1).unwrap();
        let id_2 = Identifier::try_from(2).unwrap();

        let id_list = [id_1, id_2];
        let key_package = build_pub_key_package();

        let validated = validate(id_1, &key_package, &id_list);
        assert!(validated.is_err());
        assert!(validated == Err(Error::DuplicatedIdentifier))
    }
}
