//! Test vectors for ZkApp transaction commitment functions
//!
//! This module contains shared test data used across different commitment function tests.
//! All test vectors use empty/default data structures - populate with actual test data as needed.

use mina_hasher::Fp;
use mina_signer::NetworkId;
use std::str::FromStr;

use super::{FeePayer, ZKAppCommand};

/// Comprehensive test vector containing all data needed for commitment function tests
#[derive(Clone)]
pub struct ZkAppTestVector {
    /// Name/description of the test case
    pub name: &'static str,
    /// ZKAppCommand to test
    pub zkapp_command: ZKAppCommand,
    /// Network to use for the test
    pub network: NetworkId,
    /// Expected hash_with_prefix result for memo (as string for parsing)
    pub expected_memo_hash: &'static str,
    /// Expected fee_payer_hash result (as string for parsing)
    pub expected_fee_payer_hash: &'static str,
    /// Expected call_forest_hash result (as string for parsing)
    pub expected_call_forest_hash: &'static str,
    /// Expected account updates commitment from zk_commit (as string for parsing)
    pub expected_account_updates_commitment: &'static str,
    /// Expected full commitment from zk_commit (as string for parsing)
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
/// TODO: Populate with actual test data
pub fn get_zkapp_test_vectors() -> &'static [ZkAppTestVector] {
    &[
        // Example structure - populate with actual test data
        // ZkAppTestVector {
        //     name: "empty_zkapp_command",
        //     zkapp_command: ZKAppCommand::default(),
        //     network: NetworkId::MAINNET,
        //     expected_memo_hash: "0",
        //     expected_fee_payer_hash: "0",
        //     expected_call_forest_hash: "0",
        //     expected_account_updates_commitment: "0",
        //     expected_full_commitment: "0",
        // },
    ]
}

/// Returns additional test vectors for hash_with_prefix function
/// TODO: Populate with actual test data
pub fn get_hash_with_prefix_test_vectors() -> &'static [HashWithPrefixTestVector] {
    &[
        // Example structure - populate with actual test data
        // HashWithPrefixTestVector {
        //     name: "empty_data",
        //     prefix: "TestPrefix",
        //     input_fields: vec![],
        //     expected_hash: "0",
        // },
    ]
}

/// Helper function to parse expected hash strings into Fp elements
pub fn parse_expected_hash(hash_str: &str) -> Fp {
    Fp::from_str(hash_str).expect("Invalid expected hash format")
}
