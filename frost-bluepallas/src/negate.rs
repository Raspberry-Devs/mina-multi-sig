use frost_core::{
    round1::{Nonce, NonceCommitment},
    Group,
};

use crate::{
    round1::{SigningCommitments, SigningNonces},
    PallasGroup, PallasPoseidon, SigningPackage,
};

/// This trait is used to negate the Y coordinate of the group commitment element with FROST
/// This is achieved by negating all nonces and commitments produced by all participants
pub(crate) trait NegateY {
    /// Negate the Y coordinate of the group element
    fn negate_y(&self) -> Self;
}

impl NegateY for SigningNonces {
    fn negate_y(&self) -> Self {
        let negated_hiding = -self.hiding().to_scalar();
        let negated_binding = -self.binding().to_scalar();
        SigningNonces::from_nonces(
            Nonce::<PallasPoseidon>::from_scalar(negated_hiding),
            Nonce::<PallasPoseidon>::from_scalar(negated_binding),
        )
    }
}

impl NegateY for SigningCommitments {
    fn negate_y(&self) -> Self {
        // Perform serialization roundtrip to get the hiding and binding parts
        let hiding_commitment_ser: <PallasGroup as frost_core::Group>::Serialization = self
            .hiding()
            .serialize()
            .unwrap()
            .as_slice()
            .try_into()
            .expect("Hiding commitment should be 96 bytes long");

        // Deserialize the hiding commitment to get the group element
        let hiding_commitment = PallasGroup::deserialize(&hiding_commitment_ser)
            .expect("Failed to deserialize hiding commitment");

        // Perform the same for the binding commitment
        let binding_commitment_ser: <PallasGroup as frost_core::Group>::Serialization = self
            .binding()
            .serialize()
            .unwrap()
            .as_slice()
            .try_into()
            .expect("Binding commitment should be 32 bytes long");
        let binding_commitment = PallasGroup::deserialize(&binding_commitment_ser)
            .expect("Failed to deserialize binding commitment");

        // Negate the commitments and serialize/deserialize roundtrip
        let negated_hiding = -hiding_commitment;
        let negated_binding = -binding_commitment;

        // Create a new SigningCommitments instance with the negated values
        let negated_hiding_nonce =
            NoncePallas::deserialize(&<PallasGroup as Group>::serialize(&negated_hiding).unwrap())
                .unwrap();
        let negated_binding_nonce =
            NoncePallas::deserialize(&<PallasGroup as Group>::serialize(&negated_binding).unwrap())
                .unwrap();

        SigningCommitments::new(negated_hiding_nonce, negated_binding_nonce)
    }
}

type NoncePallas = NonceCommitment<PallasPoseidon>;

impl NegateY for SigningPackage {
    fn negate_y(&self) -> Self {
        let negated_commitments = self
            .signing_commitments()
            .iter()
            .map(|(id, commitment)| (*id, commitment.negate_y()))
            .collect();
        SigningPackage::new(negated_commitments, self.message())
    }
}
