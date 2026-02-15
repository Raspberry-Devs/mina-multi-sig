use ark_ff::{BigInteger, PrimeField};
use ark_serialize::CanonicalSerialize;
use frost_bluepallas::{PallasGroup, PallasScalarField};
use frost_core::{Field, Group};

const ONE_LE: [u8; 32] = [
    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

const TWO_FIFTY_SIX_LE: [u8; 32] = [
    0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

const GENERATOR_HEX: &str =
    "010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
const DOUBLE_GENERATOR_HEX: &str =
    "ffffff1f943ebc3fb11bd0859d1f6c150000000000000000000000000000002800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";

#[test]
fn scalar_regression_vectors_are_stable() {
    let one = PallasScalarField::deserialize(&ONE_LE).expect("1 should deserialize");
    assert_eq!(PallasScalarField::serialize(&one), ONE_LE);
    assert_eq!(PallasScalarField::little_endian_serialize(&one), ONE_LE);

    let two_fifty_six =
        PallasScalarField::deserialize(&TWO_FIFTY_SIX_LE).expect("256 should deserialize");
    assert_eq!(
        PallasScalarField::serialize(&two_fifty_six),
        TWO_FIFTY_SIX_LE
    );
    assert_eq!(
        PallasScalarField::little_endian_serialize(&two_fifty_six),
        TWO_FIFTY_SIX_LE
    );
}

#[test]
fn scalar_endianness_is_little_endian() {
    let one = PallasScalarField::deserialize(&ONE_LE).expect("1 should deserialize");
    let big_endian_one = {
        let mut bytes = [0u8; 32];
        bytes[31] = 1;
        bytes
    };
    let be_one =
        PallasScalarField::deserialize(&big_endian_one).expect("big-endian byte pattern is valid");

    assert_eq!(PallasScalarField::serialize(&one), ONE_LE);
    assert_ne!(one, be_one, "byte order changes the represented scalar");
}

#[test]
fn scalar_roundtrip_preserves_value() {
    let field_element = PallasScalarField::random(&mut rand_core::OsRng);
    let serialized = PallasScalarField::serialize(&field_element);

    assert_eq!(
        serialized.len(),
        32,
        "serialized scalar length should be 32"
    );

    let deserialized =
        PallasScalarField::deserialize(&serialized).expect("failed to deserialize field element");

    assert_eq!(
        field_element, deserialized,
        "deserialized scalar should match original",
    );
}

#[test]
fn deserialize_group_element_invalid() {
    let mut buf = [0u8; 96];
    let _encoded_identity = PallasGroup::identity().serialize_compressed(&mut buf[..]);
    let result = PallasGroup::deserialize(&buf);
    assert!(
        result.is_err(),
        "deserialization should fail for identity element",
    );
}

#[test]
fn group_regression_vectors_are_stable() {
    let generator = PallasGroup::generator();
    let generator_bytes = PallasGroup::serialize(&generator).expect("generator should serialize");
    assert_eq!(hex::encode(generator_bytes), GENERATOR_HEX);

    let double_generator = generator + generator;
    let double_generator_bytes =
        PallasGroup::serialize(&double_generator).expect("double generator should serialize");
    assert_eq!(hex::encode(double_generator_bytes), DOUBLE_GENERATOR_HEX);
}

#[test]
fn serialize_group_element_roundtrip() {
    let element = PallasGroup::generator();
    let serialized = PallasGroup::serialize(&element).expect("failed to serialize group element");

    assert_eq!(serialized.len(), 96, "serialized group length should be 96");

    let deserialized =
        PallasGroup::deserialize(&serialized).expect("failed to deserialize group element");

    assert_eq!(
        element, deserialized,
        "deserialized group element should match original",
    );
}

#[test]
fn serialize_group_zero_element() {
    let zero_element = PallasGroup::identity();
    let result = PallasGroup::serialize(&zero_element);

    assert!(
        result.is_err(),
        "serialization should fail for identity element",
    );
}

#[test]
fn deserialize_field_element_endianness_reference() {
    let bytes: [u8; 32] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
        0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d,
        0x1e, 0x1f,
    ];

    let field_element =
        PallasScalarField::deserialize(&bytes).expect("failed to deserialize field element");

    assert_eq!(
        field_element.into_bigint().to_bytes_le(),
        bytes,
        "deserialization should interpret scalar bytes in little-endian order",
    );
}
