use ark_ff::{BigInteger, PrimeField};
use ark_serialize::CanonicalSerialize;
use frost_bluepallas::PallasGroup;
use frost_core::Group;

#[cfg(test)]

#[test]
fn deserialize_field_element() {
    use frost_bluepallas::PallasScalarField;
    use frost_core::Field;

    let bytes: [u8; 32] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
        0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
        0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
    ];

    let field_element = PallasScalarField::deserialize(&bytes).expect("Failed to deserialize group element");

    assert_eq!(field_element.into_bigint().to_bytes_be(), bytes);
}

#[test]
fn serialize_field_element() {
    use frost_bluepallas::PallasScalarField;
    use frost_core::Field;

    let field_element = PallasScalarField::random(&mut rand_core::OsRng);
    let serialized = PallasScalarField::serialize(&field_element);

    assert_eq!(serialized.len(), 32, "Serialized length should be 32 bytes");

    let deserialized = PallasScalarField::deserialize(&serialized).expect("Failed to deserialize field element");

    assert_eq!(field_element, deserialized, "Deserialized element should match original");
}

#[test]
fn deserialize_group_element_invalid() {
    let mut buf = [0u8; 96];
    let _encoded_identity = PallasGroup::identity().serialize_compressed(&mut buf[..]);
    let result = PallasGroup::deserialize(&buf);
    assert!(result.is_err(), "Deserialization should fail for identity element");
}

#[test]
fn serialize_group_element() {
    use frost_bluepallas::PallasGroup;
    use frost_core::Group;

    let element = PallasGroup::generator();
    let serialized = PallasGroup::serialize(&element).expect("Failed to serialize group element");

    assert_eq!(serialized.len(), 96, "Serialized length should be 96 bytes");

    let deserialized = PallasGroup::deserialize(&serialized).expect("Failed to deserialize group element");

    assert_eq!(element, deserialized, "Deserialized element should match original");
}

#[test]
fn serialize_group_zero_element() {
    use frost_bluepallas::PallasGroup;
    use frost_core::Group;

    let zero_element = PallasGroup::identity();
    let result = PallasGroup::serialize(&zero_element);

    assert!(result.is_err(), "Serialization should fail for identity element");
}
