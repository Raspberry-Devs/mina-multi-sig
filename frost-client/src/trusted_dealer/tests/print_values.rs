use frost_bluepallas as frost;

use frost::keys::{IdentifierList, PublicKeyPackage, SecretShare};
use frost::Identifier;
use itertools::Itertools;
use rand::thread_rng;
use std::collections::BTreeMap;
use std::io::BufWriter;

use crate::trusted_dealer::args::Args;
use crate::trusted_dealer::inputs::{print_values, Config};
use crate::trusted_dealer::trusted_dealer_keygen::{split_secret, trusted_dealer_keygen};

fn build_output(shares: BTreeMap<Identifier, SecretShare>, pubkeys: PublicKeyPackage) -> String {
    let pub_key_package = format!(
        "Public key package:\n{}",
        serde_json::to_string(&pubkeys).unwrap()
    );

    let mut out = pub_key_package;

    for (k, v) in shares.iter().sorted_by_key(|x| x.0) {
        out = out
            + &format!("\nParticipant: {}", hex::encode(k.serialize()))
            + &format!("\nSecret share:\n{}", serde_json::to_string(v).unwrap())
    }

    out + "\n"
}

fn cli_args() -> Args {
    let mut args = Args::default();
    args.cli = true;
    args
}

fn assert_print_values(shares: &BTreeMap<Identifier, SecretShare>, pubkeys: &PublicKeyPackage) {
    let mut buf = BufWriter::new(Vec::new());
    print_values(&cli_args(), shares, pubkeys, &mut buf).unwrap();
    let out = String::from_utf8(buf.into_inner().unwrap()).unwrap();
    assert_eq!(out, build_output(shares.clone(), pubkeys.clone()));
}

#[test]
fn check_output_without_secret() {
    let mut rng = thread_rng();
    let config = Config {
        min_signers: 2,
        max_signers: 3,
        secret: Vec::new(),
    };
    let identifiers = IdentifierList::Default;
    let (shares, pubkeys) = trusted_dealer_keygen(&config, identifiers, &mut rng).unwrap();
    assert_print_values(&shares, &pubkeys);
}

#[test]
fn check_output_with_secret() {
    let mut rng = thread_rng();
    let secret: Vec<u8> = vec![
        123, 28, 51, 211, 245, 41, 29, 133, 222, 102, 72, 51, 190, 177, 173, 70, 159, 127, 182, 2,
        90, 14, 199, 139, 58, 121, 12, 110, 19, 169, 131, 4,
    ];
    let config = Config {
        min_signers: 2,
        max_signers: 3,
        secret,
    };
    let (shares, pubkeys) = split_secret(&config, IdentifierList::Default, &mut rng).unwrap();
    assert_print_values(&shares, &pubkeys);
}

#[test]
fn check_output_with_large_num_of_signers() {
    let mut rng = thread_rng();
    let config = Config {
        min_signers: 10,
        max_signers: 20,
        secret: Vec::new(),
    };
    let (shares, pubkeys) =
        trusted_dealer_keygen(&config, IdentifierList::Default, &mut rng).unwrap();
    assert_print_values(&shares, &pubkeys);
}

#[test]
fn check_output_with_secret_with_large_num_of_signers() {
    let mut rng = thread_rng();
    let secret: Vec<u8> = vec![
        123, 28, 51, 211, 245, 41, 29, 133, 222, 102, 72, 51, 190, 177, 173, 70, 159, 127, 182, 2,
        90, 14, 199, 139, 58, 121, 12, 110, 19, 169, 131, 4,
    ];
    let config = Config {
        min_signers: 10,
        max_signers: 20,
        secret,
    };
    let (shares, pubkeys) = split_secret(&config, IdentifierList::Default, &mut rng).unwrap();
    assert_print_values(&shares, &pubkeys);
}
