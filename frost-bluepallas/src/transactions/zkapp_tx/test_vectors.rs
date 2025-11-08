//! Test vectors for ZkApp transaction commitment functions
//!
//! This module contains shared test data used across different commitment function tests.
//! All test vectors use empty/default data structures - populate with actual test data as needed.

use mina_hasher::Fp;
use mina_signer::{CompressedPubKey, NetworkId};
use std::str::FromStr;

use crate::transactions::zkapp_tx::{Authorization, MayUseToken};

use super::{
    AccountPreconditions, AccountUpdate, AccountUpdateBody, Actions, AuthRequired,
    AuthorizationKind, BalanceChange, EpochData, EpochLedger, Events, FeePayer, FeePayerBody,
    Field, NetworkPreconditions, Permissions, Preconditions, PublicKey, RangeCondition,
    SetVerificationKey, TimingData, Update, VerificationKeyData, ZKAppCommand,
};

/// Comprehensive test vector containing all data needed for commitment function tests
#[derive(Clone)]
pub struct ZkAppTestVector {
    /// Name/description of the test case
    pub name: &'static str,
    /// ZKAppCommand to test
    pub zkapp_command: ZKAppCommand,
    /// Network to use for the test
    pub network: NetworkId,
    /// Expected hash_with_prefix result for memo
    pub expected_memo_hash: &'static str,
    /// Expected fee_payer_hash result
    pub expected_fee_payer_hash: &'static str,
    /// Expected account updates commitment from zk_commit
    pub expected_account_updates_commitment: &'static str,
    /// Expected full commitment from zk_commit
    pub expected_full_commitment: &'static str,
}

/// Additional test vectors specifically for hash_with_prefix function
#[derive(Clone)]
pub struct HashWithPrefixTestVector {
    /// Name/description of the test case
    pub name: &'static str,
    /// Prefix string to use
    pub prefix: &'static str,
    /// Input field elements
    pub input_fields: Vec<Fp>,
    /// Expected hash result (as string for parsing)
    pub expected_hash: &'static str,
}

/// Returns the main test vectors for ZkApp commitment functions
pub fn get_zkapp_test_vectors() -> Vec<ZkAppTestVector> {
    vec![ZkAppTestVector {
        name: "complex_zkapp_command",
        zkapp_command: ZKAppCommand::default(),
        network: NetworkId::MAINNET,
        expected_memo_hash:
            "369453247258139461793874281204592948913530799489559288302740003628227053858",
        expected_fee_payer_hash:
            "10107555265261176496684761383484143750979693352181633907531879785290480643417",
        expected_account_updates_commitment:
            "23316460487402773494110335361659308674100635476278317671190332182678874982779",
        expected_full_commitment:
            "3606189568599761746353209221660606816407299472850415535325220362709068199288",
    }]
}

/// Returns additional test vectors for hash_with_prefix function
/// TODO: Populate with actual test data
pub fn get_hash_with_prefix_test_vectors() -> Vec<HashWithPrefixTestVector> {
    vec![HashWithPrefixTestVector {
        name: "mina_acct_update_node",
        prefix: "MinaAcctUpdateNode",
        input_fields: vec![
            Fp::from_str(
                "23487734643675003113914430489774334948844391842009122040704261138931555665056",
            )
            .unwrap(),
            Fp::from_str("0").unwrap(),
        ],
        expected_hash:
            "20456728518925904340727370305821489989002971473792411299271630913563245218671",
    }]
}

/// Helper function to parse expected hash strings into Fp elements
pub fn parse_expected_hash(hash_str: &str) -> Fp {
    Fp::from_str(hash_str).expect("Invalid expected hash format")
}
