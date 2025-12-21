use frost_core::{
    round1::{Nonce, NonceCommitment},
    Group,
};

use crate::{
    round1::{SigningCommitments, SigningNonces},
    BluePallas, PallasGroup, SigningPackage,
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
            Nonce::<BluePallas>::from_scalar(negated_hiding),
            Nonce::<BluePallas>::from_scalar(negated_binding),
        )
    }
}

/// Negate the Y coordinate of the group commitment element with FROST
impl NegateY for SigningCommitments {
    fn negate_y(&self) -> Self {
        // Perform serialization roundtrip to get the hiding and binding parts
        // Note that we use expect() clauses here because these are ALWAYS expected to succeed
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

type NoncePallas = NonceCommitment<BluePallas>;

/// Take all commitments with a signing package and negate their Y coordinates
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

#[cfg(test)]
mod tests {
    use crate::PallasScalarField;

    use super::*;
    use alloc::collections::BTreeMap;
    use ark_ff::UniformRand;
    use frost_core::round1::{Nonce, NonceCommitment};
    use frost_core::Group;
    use mina_curves::pasta::ProjectivePallas;
    use rand_core::OsRng;

    /// Helpers to extract the underlying `PallasGroup` from a `NonceCommitment<BluePallas>`.
    fn commit_to_group(c: &NonceCommitment<BluePallas>) -> ProjectivePallas {
        let ser = c.serialize().unwrap();
        let ser: [u8; 96] = ser
            .as_slice()
            .try_into()
            .expect("Commitment should be 96 bytes long");
        PallasGroup::deserialize(&ser).unwrap()
    }

    #[test]
    fn signing_nonces_negate_y_inverts_scalars() {
        let mut rng = OsRng;
        // make two secret nonces
        let r1 = <PallasScalarField as frost_core::Field>::Scalar::rand(&mut rng);
        let r2 = <PallasScalarField as frost_core::Field>::Scalar::rand(&mut rng);
        let nonces = SigningNonces::from_nonces(
            Nonce::<BluePallas>::from_scalar(r1),
            Nonce::<BluePallas>::from_scalar(r2),
        );

        let neg = nonces.negate_y();

        // check hiding nonce
        let orig_h = nonces.hiding().to_scalar();
        let new_h = neg.hiding().to_scalar();
        assert_eq!(
            orig_h + new_h,
            <PallasScalarField as frost_core::Field>::zero()
        );

        // check binding nonce
        let orig_b = nonces.binding().to_scalar();
        let new_b = neg.binding().to_scalar();
        assert_eq!(
            orig_b + new_b,
            <PallasScalarField as frost_core::Field>::zero()
        );
    }

    #[test]
    fn signing_commitments_negate_y_inverts_group_elements() {
        // pick the group generator so we know Y ≠ 0
        let g = PallasGroup::generator();
        let g_ser = PallasGroup::serialize(&g).unwrap();
        let comm = NonceCommitment::<BluePallas>::deserialize(&g_ser).unwrap();

        let commitments = SigningCommitments::new(comm, comm);
        let neg = commitments.negate_y();

        // hiding
        let orig_h = commit_to_group(commitments.hiding());
        let new_h = commit_to_group(neg.hiding());
        assert_eq!(new_h, -orig_h);

        // binding
        let orig_b = commit_to_group(commitments.binding());
        let new_b = commit_to_group(neg.binding());
        assert_eq!(new_b, -orig_b);
    }

    #[test]
    fn signing_package_negate_y_inverts_all_commitments_and_preserves_message() {
        // set up one participant (id = 1) with the generator commitment
        let g = PallasGroup::generator();
        let g_ser = PallasGroup::serialize(&g).unwrap();
        let comm = NonceCommitment::<BluePallas>::deserialize(&g_ser).unwrap();
        let single = SigningCommitments::new(comm, comm);

        let mut map = BTreeMap::new();
        let participant_identifier = frost_core::Identifier::try_from(1u16).unwrap();
        map.insert(participant_identifier, single);
        let message = b"hello frost".to_vec();

        let pkg = SigningPackage::new(map.clone(), message.as_slice());
        let neg_pkg = pkg.negate_y();

        // should preserve the message exactly
        assert_eq!(neg_pkg.message(), &message[..]);

        // each commitment should be the negation of the original
        for (id, neg_comm) in neg_pkg.signing_commitments() {
            let orig = &map[id];
            let orig_h = commit_to_group(orig.hiding());
            let new_h = commit_to_group(neg_comm.hiding());
            assert_eq!(new_h, -orig_h);

            let orig_b = commit_to_group(orig.binding());
            let new_b = commit_to_group(neg_comm.binding());
            assert_eq!(new_b, -orig_b);
        }
    }
}
