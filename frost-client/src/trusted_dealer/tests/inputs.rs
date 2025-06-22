use frost_bluepallas as frost;

use frost::Error;
use crate::trusted_dealer::inputs::{request_inputs, Config};
use crate::trusted_dealer::args::Args;

fn call_request_inputs(input: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let mut buf = std::io::BufWriter::new(Vec::new());
    let mut input = input.as_bytes();
    let mut args = Args::default();
    args.cli = true;
    request_inputs::<frost_bluepallas::PallasPoseidon>(&args, &mut input, &mut buf)
}

#[test]
fn check_valid_input_for_signers() {
    let config = Config {
        min_signers: 2,
        max_signers: 3,
        secret: Vec::new(),
    };

    let expected = call_request_inputs("2\n3\n\n").unwrap();

    assert_eq!(expected, config);
}

#[test]
fn return_error_if_min_participant_greater_than_max_participant() {
    let expected = call_request_inputs("4\n3\n\n").unwrap_err();

    assert_eq!(
        *expected.downcast::<Error>().unwrap(),
        Error::InvalidMinSigners
    );
}

#[test]
fn return_error_if_min_participant_is_less_than_2() {
    let expected = call_request_inputs("1\n3\n\n").unwrap_err();

    assert_eq!(
        *expected.downcast::<Error>().unwrap(),
        Error::InvalidMinSigners
    );
}

#[test]
fn return_error_if_max_participant_is_less_than_2() {
    let expected = call_request_inputs("2\n1\n\n").unwrap_err();

    assert_eq!(
        *expected.downcast::<Error>().unwrap(),
        Error::InvalidMaxSigners
    );
}

// Testing inclusion of secret input

#[test]
fn check_valid_input_with_secret() {
    let config = call_request_inputs("3\n6\n7b1c33d3f5291d85de664833beb1ad469f7fb6025a0ec78b3a790c6e13a98304\n").unwrap();

    let secret: Vec<u8> = vec![
        123, 28, 51, 211, 245, 41, 29, 133, 222, 102, 72, 51, 190, 177, 173, 70, 159, 127, 182, 2,
        90, 14, 199, 139, 58, 121, 12, 110, 19, 169, 131, 4,
    ];
    let expected = Config {
        min_signers: 3,
        max_signers: 6,
        secret,
    };

    assert_eq!(expected, config)
}

#[test]
fn return_error_if_invalid_min_signers_input() {
    let expected = call_request_inputs("hello\n6\n\n").unwrap_err();

    assert_eq!(
        *expected.downcast::<Error>().unwrap(),
        Error::InvalidMinSigners
    );
}

#[test]
fn return_error_if_invalid_max_signers_input() {
    let expected = call_request_inputs("4\nworld\n\n").unwrap_err();

    assert_eq!(
        *expected.downcast::<Error>().unwrap(),
        Error::InvalidMaxSigners
    );
}

#[test]
fn return_malformed_signing_key_error_if_secret_is_invalid() {
    let expected = call_request_inputs("4\n6\nasecret\n").unwrap_err();

    assert_eq!(
        *expected.downcast::<Error>().unwrap(),
        Error::MalformedSigningKey
    );
}
