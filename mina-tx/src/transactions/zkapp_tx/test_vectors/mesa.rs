//! Test vectors for ZkApp transaction commitment functions (Mesa hardfork, 32 state fields)

use alloc::vec::Vec;

use super::common::{HashWithPrefixTestVector, ZkAppTestVector};

/// Returns the main test vectors for ZkApp commitment functions (Mesa hardfork)
pub fn get_zkapp_test_vectors() -> Vec<ZkAppTestVector> {
    vec![]
}

/// Returns test vectors for hash_with_prefix function (Mesa hardfork)
pub fn get_hash_with_prefix_test_vectors() -> Vec<HashWithPrefixTestVector> {
    vec![]
}
