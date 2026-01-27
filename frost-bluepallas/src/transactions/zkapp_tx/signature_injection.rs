//! Signature injection utilities for ZkApp transactions.
//!
//! This module provides functionality to inject FROST-generated signatures into ZkApp
//! transactions. After FROST signing completes, the signature needs to be placed into
//! the appropriate authorization fields within the transaction structure.

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use mina_signer::pubkey::PubKey;

use crate::mina_compat::Sig;

use super::{AccountUpdate, ZKAppCommand};

/// Warnings emitted during signature injection.
///
/// These warnings indicate potential issues but do not prevent signature injection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignatureInjectionWarning {
    /// Fee payer public key does not match the provided group public key.
    FeePayerKeyMismatch { expected: String, found: String },
    /// An account update uses signature authorization but `use_full_commitment` is false.
    /// The signature was NOT injected for this account update.
    PartialCommitmentSkipped { index: usize, public_key: String },
    /// An account update already had a non-empty signature that was overwritten.
    SignatureOverwritten { index: usize, public_key: String },
    /// Fee payer already had a non-empty authorization that was overwritten.
    FeePayerSignatureOverwritten,
}

/// Result of signature injection operation.
#[derive(Debug, Clone)]
pub struct SignatureInjectionResult {
    /// Number of signatures injected into account updates (excludes fee payer).
    pub account_updates_injected: usize,
    /// Whether the fee payer signature was injected.
    pub fee_payer_injected: bool,
    /// Warnings encountered during injection.
    pub warnings: Vec<SignatureInjectionWarning>,
}

impl ZKAppCommand {
    /// Inject a FROST signature into the ZkApp transaction.
    ///
    /// This method injects the provided signature into:
    /// 1. The fee payer authorization (if the fee payer's public key matches `group_public_key`)
    /// 2. Account updates that:
    ///    - Have `authorization_kind.is_signed = true`
    ///    - Have `use_full_commitment = true`
    ///    - Have a public key matching `group_public_key`
    ///
    /// # Arguments
    /// * `group_public_key` - The FROST group public key
    /// * `signature` - The FROST-generated signature to inject
    ///
    /// # Returns
    /// A `SignatureInjectionResult` containing:
    /// - Count of injected signatures
    /// - Any warnings about skipped or overwritten signatures
    ///
    /// # Warnings
    /// - `FeePayerKeyMismatch`: Fee payer key doesn't match group key (no injection for fee payer)
    /// - `PartialCommitmentSkipped`: Account update has `use_full_commitment = false` (skipped)
    /// - `SignatureOverwritten`: Existing non-empty signature was replaced
    /// - `FeePayerSignatureOverwritten`: Fee payer had existing authorization that was replaced
    pub fn inject_signature(
        &mut self,
        group_public_key: &PubKey,
        signature: &Sig,
    ) -> SignatureInjectionResult {
        let mut warnings = Vec::new();
        let sig_base58 = signature.to_base58();

        // 1. Handle fee payer injection
        let fee_payer_injected =
            self.inject_fee_payer_signature(group_public_key, &sig_base58, &mut warnings);

        // 2. Handle account update injections
        let account_updates_injected =
            self.inject_account_update_signatures(group_public_key, &sig_base58, &mut warnings);

        SignatureInjectionResult {
            account_updates_injected,
            fee_payer_injected,
            warnings,
        }
    }

    /// Inject signature into fee payer if public key matches.
    fn inject_fee_payer_signature(
        &mut self,
        group_public_key: &PubKey,
        sig_base58: &str,
        warnings: &mut Vec<SignatureInjectionWarning>,
    ) -> bool {
        let fee_payer_pk = &self.fee_payer.body.public_key.0;
        let group_compressed = group_public_key.into_compressed();

        // Check if fee payer public key matches the group public key
        if *fee_payer_pk != group_compressed {
            warnings.push(SignatureInjectionWarning::FeePayerKeyMismatch {
                expected: group_compressed.into_address(),
                found: fee_payer_pk.into_address(),
            });
            return false;
        }

        // Warn if overwriting existing authorization
        if !self.fee_payer.authorization.is_empty() {
            warnings.push(SignatureInjectionWarning::FeePayerSignatureOverwritten);
        }

        self.fee_payer.authorization = sig_base58.to_string();
        true
    }

    /// Inject signatures into qualifying account updates.
    fn inject_account_update_signatures(
        &mut self,
        group_public_key: &PubKey,
        sig_base58: &str,
        warnings: &mut Vec<SignatureInjectionWarning>,
    ) -> usize {
        let mut injected_count = 0;
        let group_compressed = group_public_key.into_compressed();

        for (index, update) in self.account_updates.iter_mut().enumerate() {
            if Self::should_inject_signature(update, &group_compressed) {
                // Check use_full_commitment requirement
                if !update.body.use_full_commitment {
                    warnings.push(SignatureInjectionWarning::PartialCommitmentSkipped {
                        index,
                        public_key: update.body.public_key.0.into_address(),
                    });
                    continue;
                }

                // Warn if overwriting existing signature
                if let Some(existing_sig) = &update.authorization.signature {
                    if !existing_sig.is_empty() {
                        warnings.push(SignatureInjectionWarning::SignatureOverwritten {
                            index,
                            public_key: update.body.public_key.0.into_address(),
                        });
                    }
                }

                update.authorization.signature = Some(sig_base58.to_string());
                injected_count += 1;
            }
        }

        injected_count
    }

    /// Determine if an account update should receive the signature.
    ///
    /// Returns true if:
    /// - `authorization_kind.is_signed` is true
    /// - Public key matches the group public key
    fn should_inject_signature(
        update: &AccountUpdate,
        group_public_key: &mina_signer::CompressedPubKey,
    ) -> bool {
        let auth_kind = &update.body.authorization_kind;
        let update_pk = &update.body.public_key.0;

        auth_kind.is_signed && update_pk == group_public_key
    }
}

// -------------------------------------------------------------------------------------------------
// ---------------------------------------- Tests --------------------------------------------------
// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transactions::zkapp_tx::{
        constants::DUMMY_HASH, Authorization, AuthorizationKind, FeePayer, FeePayerBody, PublicKey,
    };
    use alloc::{string::ToString, vec};
    use ark_ff::BigInt;
    use mina_signer::SecKey;
    use rand_core::SeedableRng;

    /// Create a test public key from a simple seed value
    fn make_test_pubkey(seed: u8) -> PubKey {
        let mut seed = rand_chacha::ChaCha12Rng::from_seed([seed; 32]);
        let secret_key = SecKey::rand(&mut seed);
        PubKey::from_secret_key(secret_key).expect("Failed to create test PubKey")
    }

    /// Create a test signature
    fn make_test_signature() -> Sig {
        Sig {
            field: BigInt([1u64, 2u64, 3u64, 4u64]),
            scalar: BigInt([5u64, 6u64, 7u64, 8u64]),
        }
    }

    /// Create a minimal account update with specified parameters
    fn make_account_update(
        public_key: &PubKey,
        is_signed: bool,
        use_full_commitment: bool,
        existing_signature: Option<String>,
    ) -> AccountUpdate {
        let mut update = AccountUpdate::default();
        update.body.public_key = PublicKey(public_key.into_compressed());
        update.body.authorization_kind = AuthorizationKind {
            is_signed,
            is_proved: false,
            verification_key_hash: *DUMMY_HASH,
        };
        update.body.use_full_commitment = use_full_commitment;
        update.authorization = Authorization {
            proof: None,
            signature: existing_signature,
        };
        update
    }

    /// Create a minimal fee payer with specified parameters
    fn make_fee_payer(public_key: &PubKey, existing_auth: &str) -> FeePayer {
        FeePayer {
            body: FeePayerBody {
                public_key: PublicKey(public_key.into_compressed()),
                fee: 1_000_000,
                valid_until: None,
                nonce: 0,
            },
            authorization: existing_auth.to_string(),
        }
    }

    #[test]
    fn test_fee_payer_injection_matching_key() {
        let group_pk = make_test_pubkey(1);
        let sig = make_test_signature();

        let mut cmd = ZKAppCommand {
            fee_payer: make_fee_payer(&group_pk, ""),
            account_updates: vec![],
            memo: [0u8; 34],
        };

        let result = cmd.inject_signature(&group_pk, &sig);

        assert!(result.fee_payer_injected);
        assert_eq!(result.account_updates_injected, 0);
        assert!(result.warnings.is_empty());
        assert!(!cmd.fee_payer.authorization.is_empty());
        assert_eq!(cmd.fee_payer.authorization, sig.to_base58());
    }

    #[test]
    fn test_fee_payer_injection_mismatched_key() {
        let group_pk = make_test_pubkey(1);
        let other_pk = make_test_pubkey(2);
        let sig = make_test_signature();

        let mut cmd = ZKAppCommand {
            fee_payer: make_fee_payer(&other_pk, ""),
            account_updates: vec![],
            memo: [0u8; 34],
        };

        let result = cmd.inject_signature(&group_pk, &sig);

        assert!(!result.fee_payer_injected);
        assert_eq!(result.warnings.len(), 1);
        assert!(matches!(
            &result.warnings[0],
            SignatureInjectionWarning::FeePayerKeyMismatch { .. }
        ));
        // Authorization should remain empty
        assert!(cmd.fee_payer.authorization.is_empty());
    }

    #[test]
    fn test_fee_payer_overwrite_warning() {
        let group_pk = make_test_pubkey(1);
        let sig = make_test_signature();

        let mut cmd = ZKAppCommand {
            fee_payer: make_fee_payer(&group_pk, "existing_signature"),
            account_updates: vec![],
            memo: [0u8; 34],
        };

        let result = cmd.inject_signature(&group_pk, &sig);

        assert!(result.fee_payer_injected);
        assert_eq!(result.warnings.len(), 1);
        assert!(matches!(
            &result.warnings[0],
            SignatureInjectionWarning::FeePayerSignatureOverwritten
        ));
        // Signature should be overwritten
        assert_eq!(cmd.fee_payer.authorization, sig.to_base58());
    }

    #[test]
    fn test_account_update_injection_full_commitment() {
        let group_pk = make_test_pubkey(1);
        let sig = make_test_signature();

        let mut cmd = ZKAppCommand {
            fee_payer: make_fee_payer(&group_pk, ""),
            account_updates: vec![make_account_update(&group_pk, true, true, None)],
            memo: [0u8; 34],
        };

        let result = cmd.inject_signature(&group_pk, &sig);

        assert!(result.fee_payer_injected);
        assert_eq!(result.account_updates_injected, 1);
        assert!(result.warnings.is_empty());
        assert_eq!(
            cmd.account_updates[0].authorization.signature,
            Some(sig.to_base58())
        );
    }

    #[test]
    fn test_account_update_skipped_partial_commitment() {
        let group_pk = make_test_pubkey(1);
        let sig = make_test_signature();

        let mut cmd = ZKAppCommand {
            fee_payer: make_fee_payer(&group_pk, ""),
            account_updates: vec![
                // is_signed=true but use_full_commitment=false -> should be skipped
                make_account_update(&group_pk, true, false, None),
            ],
            memo: [0u8; 34],
        };

        let result = cmd.inject_signature(&group_pk, &sig);

        assert!(result.fee_payer_injected);
        assert_eq!(result.account_updates_injected, 0);
        assert_eq!(result.warnings.len(), 1);
        assert!(matches!(
            &result.warnings[0],
            SignatureInjectionWarning::PartialCommitmentSkipped { index: 0, .. }
        ));
        // Signature should NOT be injected
        assert!(cmd.account_updates[0].authorization.signature.is_none());
    }

    #[test]
    fn test_account_update_not_signed_skipped() {
        let group_pk = make_test_pubkey(1);
        let sig = make_test_signature();

        let mut cmd = ZKAppCommand {
            fee_payer: make_fee_payer(&group_pk, ""),
            account_updates: vec![
                // is_signed=false -> should be skipped entirely (no warning)
                make_account_update(&group_pk, false, true, None),
            ],
            memo: [0u8; 34],
        };

        let result = cmd.inject_signature(&group_pk, &sig);

        assert!(result.fee_payer_injected);
        assert_eq!(result.account_updates_injected, 0);
        // No warning because is_signed is false (doesn't need signature)
        assert!(result.warnings.is_empty());
        assert!(cmd.account_updates[0].authorization.signature.is_none());
    }

    #[test]
    fn test_account_update_different_key_skipped() {
        let group_pk = make_test_pubkey(1);
        let other_pk = make_test_pubkey(2);
        let sig = make_test_signature();

        let mut cmd = ZKAppCommand {
            fee_payer: make_fee_payer(&group_pk, ""),
            account_updates: vec![
                // Different public key -> should be skipped (no warning)
                make_account_update(&other_pk, true, true, None),
            ],
            memo: [0u8; 34],
        };

        let result = cmd.inject_signature(&group_pk, &sig);

        assert!(result.fee_payer_injected);
        assert_eq!(result.account_updates_injected, 0);
        // No warning because key doesn't match (expected behavior)
        assert!(result.warnings.is_empty());
        assert!(cmd.account_updates[0].authorization.signature.is_none());
    }

    #[test]
    fn test_account_update_overwrite_warning() {
        let group_pk = make_test_pubkey(1);
        let sig = make_test_signature();

        let mut cmd = ZKAppCommand {
            fee_payer: make_fee_payer(&group_pk, ""),
            account_updates: vec![make_account_update(
                &group_pk,
                true,
                true,
                Some("old_signature".to_string()),
            )],
            memo: [0u8; 34],
        };

        let result = cmd.inject_signature(&group_pk, &sig);

        assert!(result.fee_payer_injected);
        assert_eq!(result.account_updates_injected, 1);
        assert_eq!(result.warnings.len(), 1);
        assert!(matches!(
            &result.warnings[0],
            SignatureInjectionWarning::SignatureOverwritten { index: 0, .. }
        ));
        // Signature should be overwritten
        assert_eq!(
            cmd.account_updates[0].authorization.signature,
            Some(sig.to_base58())
        );
    }

    #[test]
    fn test_multiple_account_updates_mixed() {
        let group_pk = make_test_pubkey(1);
        let other_pk = make_test_pubkey(2);
        let sig = make_test_signature();

        let mut cmd = ZKAppCommand {
            fee_payer: make_fee_payer(&group_pk, ""),
            account_updates: vec![
                // Should be injected
                make_account_update(&group_pk, true, true, None),
                // Skipped: different key
                make_account_update(&other_pk, true, true, None),
                // Skipped with warning: partial commitment
                make_account_update(&group_pk, true, false, None),
                // Skipped: not signed
                make_account_update(&group_pk, false, true, None),
                // Should be injected with overwrite warning
                make_account_update(&group_pk, true, true, Some("old".to_string())),
            ],
            memo: [0u8; 34],
        };

        let result = cmd.inject_signature(&group_pk, &sig);

        assert!(result.fee_payer_injected);
        assert_eq!(result.account_updates_injected, 2);
        assert_eq!(result.warnings.len(), 2); // partial commitment + overwrite

        // Verify injections
        assert_eq!(
            cmd.account_updates[0].authorization.signature,
            Some(sig.to_base58())
        );
        assert!(cmd.account_updates[1].authorization.signature.is_none());
        assert!(cmd.account_updates[2].authorization.signature.is_none());
        assert!(cmd.account_updates[3].authorization.signature.is_none());
        assert_eq!(
            cmd.account_updates[4].authorization.signature,
            Some(sig.to_base58())
        );
    }

    #[test]
    fn test_idempotent_injection() {
        let group_pk = make_test_pubkey(1);
        let sig = make_test_signature();

        let mut cmd = ZKAppCommand {
            fee_payer: make_fee_payer(&group_pk, ""),
            account_updates: vec![make_account_update(&group_pk, true, true, None)],
            memo: [0u8; 34],
        };

        // First injection
        let result1 = cmd.inject_signature(&group_pk, &sig);
        assert!(result1.warnings.is_empty());

        let sig_value = cmd.fee_payer.authorization.clone();

        // Second injection - should produce overwrite warnings
        let result2 = cmd.inject_signature(&group_pk, &sig);
        assert_eq!(result2.warnings.len(), 2); // fee payer + account update overwrite

        // Values should be the same
        assert_eq!(cmd.fee_payer.authorization, sig_value);
    }
}
