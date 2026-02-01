//! Utilities for negating Y coordinates in FROST commitments using the BluePallas curve. This mimics Mina's handling of point negation.

use frost_core::round1::{Nonce, NonceCommitment};

use crate::{
    round1::{SigningCommitments, SigningNonces},
    BluePallas, SigningPackage,
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
        // Negate the commitments and serialize/deserialize roundtrip
        let negated_hiding = -self.hiding().value();
        let negated_binding = -self.binding().value();

        // Create a new SigningCommitments instance with the negated values
        let negated_hiding_nonce = NoncePallas::new(negated_hiding);
        let negated_binding_nonce = NoncePallas::new(negated_binding);

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
    use crate::{PallasGroup, PallasScalarField};

    use super::*;
    use alloc::collections::BTreeMap;
    use ark_ff::UniformRand;
    use frost_core::{
        round1::{Nonce, NonceCommitment},
        Group,
    };
    use mina_curves::pasta::ProjectivePallas;
    use rand_core::OsRng;

    /// Helpers to extract the underlying `PallasGroup` from a `NonceCommitment<BluePallas>`.
    fn commit_to_group(c: &NonceCommitment<BluePallas>) -> ProjectivePallas {
        c.value()
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
