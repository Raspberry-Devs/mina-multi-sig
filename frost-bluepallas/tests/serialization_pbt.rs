use frost_bluepallas::{PallasGroup, PallasScalarField};
use frost_core::{Field, Group};
use num_traits::identities::Zero;
use proptest::prelude::*;

fn arb_scalar() -> impl Strategy<Value = <PallasScalarField as Field>::Scalar> {
    any::<[u8; 32]>().prop_filter_map("canonical scalar encoding", |bytes| {
        PallasScalarField::deserialize(&bytes).ok()
    })
}

fn arb_non_identity_element() -> impl Strategy<Value = <PallasGroup as Group>::Element> {
    arb_scalar().prop_filter_map("non-identity element", |scalar| {
        let element = PallasGroup::generator() * scalar;
        (!element.is_zero()).then_some(element)
    })
}

proptest! {
    #[test]
    fn scalar_roundtrip(bytes in any::<[u8; 32]>()) {
        let Some(scalar) = PallasScalarField::deserialize(&bytes).ok() else {
            return Ok(());
        };

        let encoded = PallasScalarField::serialize(&scalar);
        let decoded = PallasScalarField::deserialize(&encoded)
            .expect("serialized scalar should always deserialize");

        prop_assert_eq!(decoded, scalar);
    }

    #[test]
    fn scalar_little_endian_matches_serialize(scalar in arb_scalar()) {
        prop_assert_eq!(
            PallasScalarField::serialize(&scalar),
            PallasScalarField::little_endian_serialize(&scalar),
        );
    }

    #[test]
    fn group_roundtrip_non_identity(element in arb_non_identity_element()) {
        let encoded = PallasGroup::serialize(&element)
            .expect("non-identity group element should serialize");
        let decoded = PallasGroup::deserialize(&encoded)
            .expect("serialized group element should deserialize");

        prop_assert_eq!(decoded, element);
    }
}
