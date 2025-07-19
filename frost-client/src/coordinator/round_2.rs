use frost_core::{self as frost, Ciphersuite};

use frost::{Signature, SigningPackage};

use std::{
    fs,
    io::{BufRead, Write},
};

use super::{args::ProcessedArgs, comms::Comms, round_1::ParticipantsConfig};

pub async fn send_signing_package_and_get_signature_shares<C: Ciphersuite + 'static>(
    args: &ProcessedArgs<C>,
    comms: &mut dyn Comms<C>,
    input: &mut dyn BufRead,
    logger: &mut dyn Write,
    participants: ParticipantsConfig<C>,
    signing_package: &SigningPackage<C>,
) -> Result<Signature<C>, Box<dyn std::error::Error>> {
    let group_signature =
        request_inputs_signature_shares(args, comms, input, logger, participants, signing_package)
            .await?;
    print_signature(args, logger, group_signature)?;
    Ok(group_signature)
}

// Input required:
// 1. number of signers (TODO: maybe pass this in?)
// 2. signatures for all signers
async fn request_inputs_signature_shares<C: Ciphersuite + 'static>(
    _args: &ProcessedArgs<C>,
    comms: &mut dyn Comms<C>,
    input: &mut dyn BufRead,
    logger: &mut dyn Write,
    participants: ParticipantsConfig<C>,
    signing_package: &SigningPackage<C>,
) -> Result<Signature<C>, Box<dyn std::error::Error>> {
    // TODO: support multiple

    let signatures_list = comms
        .send_signing_package_and_get_signature_shares(input, logger, signing_package)
        .await?;

    let group_signature = frost::aggregate::<C>(
        signing_package,
        &signatures_list,
        &participants.pub_key_package,
    )
    .unwrap();

    Ok(group_signature)
}

fn print_signature<C: Ciphersuite + 'static>(
    args: &ProcessedArgs<C>,
    logger: &mut dyn Write,
    group_signature: Signature<C>,
) -> Result<(), Box<dyn std::error::Error>> {
    if args.signature.is_empty() {
        writeln!(
            logger,
            "Signature:\n{}",
            hex::encode(&group_signature.serialize()?)
        )?;
    } else {
        fs::write(&args.signature, group_signature.serialize()?)?;
        eprintln!("Raw signature written to {}", &args.signature);
    };
    Ok(())
}
