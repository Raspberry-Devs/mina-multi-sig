//! Shared types and helpers for ZkApp test vectors (both pre-Mesa and Mesa)

use alloc::vec::Vec;
use core::str::FromStr;
use mina_hasher::Fp;
use mina_signer::NetworkId;

use crate::transactions::zkapp_tx::ZKAppCommand;
use crate::transactions::TransactionEnvelope;

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

impl From<ZkAppTestVector> for TransactionEnvelope {
    fn from(vector: ZkAppTestVector) -> TransactionEnvelope {
        TransactionEnvelope::new_zkapp(vector.network, vector.zkapp_command)
    }
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

/// Helper function to parse expected hash strings into Fp elements
pub fn parse_expected_hash(hash_str: &str) -> Fp {
    Fp::from_str(hash_str).expect("Invalid expected hash format")
}

/// Decode a base58-encoded memo into a fixed-size byte array
pub fn decode_memo_from_base58(memo_base58: &str) -> [u8; 34] {
    let memo_bytes = bs58::decode(memo_base58)
        .into_vec()
        .expect("Valid base58 memo");

    memo_bytes[1..memo_bytes.len() - 4]
        .try_into()
        .expect("Memo length matches expected size")
}
