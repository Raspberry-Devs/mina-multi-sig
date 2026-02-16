//! ZkApp transaction commitment computation and hashing utilities
//! This module provides functionality to compute commitments for ZkApp transactions which can be later signed over
use alloc::{boxed::Box, collections::VecDeque, string::ToString, vec::Vec};

use ark_ff::Field;
use mina_hasher::Fp;
use mina_poseidon::{
    constants::PlonkSpongeConstantsKimchi,
    pasta::fp_kimchi,
    poseidon::{ArithmeticSponge, Sponge},
};
use mina_signer::NetworkId;

use crate::{
    errors::{MinaTxError, MinaTxResult},
    transactions::zkapp_tx::{
        constants::{self, ZkAppBodyPrefix, DUMMY_HASH},
        packing::{Packable, PackedInput},
        AccountUpdate, FeePayer, ZKAppCommand,
    },
};

// -------------------------------------------------------------------------------------------------
// ------------------------------------ Commitment Logic -------------------------------------------
// -------------------------------------------------------------------------------------------------

/// Produces a commitment for a ZkApp command by hashing its structure and contents.
/// Validates call depths and authorization kinds before computing the commitment.
/// Returns two Fp elements, representing the accountUpdates commitment and the overall commitment respectively.
/// Overall commitment includes memo, fee payer, and account updates commitments.
pub(crate) fn zk_commit(tx: &ZKAppCommand, network: &NetworkId) -> MinaTxResult<(Fp, Fp)> {
    if !is_call_depth_valid(tx) {
        return Err(Box::new(MinaTxError::InvalidZkAppCommand(
            "Call depths are not valid".to_string(),
        )));
    }

    let forest = CallForest::from(tx.clone());

    // Compute the account-updates commitment using the call forest hashing routine.
    let account_updates_commitment = call_forest_hash(&forest, network)?;

    let memo_hash = memo_hash(tx)?;

    let fee_payer_hash = hash_fee_payer(tx.fee_payer.clone(), network)?;
    let full_commit = hash_with_prefix(
        constants::PREFIX_ACCOUNT_UPDATE_CONS,
        &[memo_hash, fee_payer_hash, account_updates_commitment],
    )?;

    Ok((account_updates_commitment, full_commit))
}

// -------------------------------------------------------------------------------------------------
// -------------------------------- Call Forest ----------------------------------------------------
// -------------------------------------------------------------------------------------------------

/// A single node in the call forest representing an account update and its children
#[derive(Clone)]
pub struct CallTree {
    account_update: AccountUpdate,
    children: CallForest,
}

/// A forest of call trees representing the hierarchical structure of account updates
#[derive(Clone)]
pub struct CallForest(Vec<CallTree>);

impl CallForest {
    fn account_updates_to_call_forest_step(
        updates: &mut VecDeque<AccountUpdate>,
        call_depth: u32,
    ) -> CallForest {
        let mut forest: CallForest = CallForest(Vec::new());

        while !updates.is_empty() {
            let account_update = &updates[0];
            if account_update.body.call_depth < call_depth {
                return forest;
            }

            // Unwrap is safe here because we checked that updates is not empty
            let account_update = updates.pop_front().unwrap();
            let children = Self::account_updates_to_call_forest_step(updates, call_depth + 1);

            forest.0.push(CallTree {
                account_update,
                children,
            });
        }

        forest
    }
}

impl From<Vec<AccountUpdate>> for CallForest {
    fn from(updates: Vec<AccountUpdate>) -> Self {
        Self::account_updates_to_call_forest_step(&mut updates.into(), 0)
    }
}

impl From<ZKAppCommand> for CallForest {
    fn from(tx: ZKAppCommand) -> Self {
        CallForest::from(tx.account_updates)
    }
}

// -------------------------------------------------------------------------------------------------
// ---------------------------------- High Level Hashing Functions ---------------------------------
// -------------------------------------------------------------------------------------------------

/// Computes the hash of a call forest representing account updates.
/// Traverses the forest in reverse order, for each CallTree:
///  - recursively compute calls = hash(children)
///  - tree_hash = hash_account_update(account_update)
///  - node_hash = hash_with_prefix("MinaAcctUpdateNode", [tree_hash, calls])
///  - stack_hash = hash_with_prefix("MinaAcctUpdateCons", [node_hash, stack_hash])
fn call_forest_hash(forest: &CallForest, network: &NetworkId) -> MinaTxResult<Fp> {
    let mut stack_hash = constants::EMPTY_STACK_HASH;

    // iterate in reverse (last -> first)
    for call_tree in forest.0.iter().rev() {
        let calls = call_forest_hash(&call_tree.children, network)?;
        let tree_hash = hash_account_update(&call_tree.account_update, network)?;
        let node_hash =
            hash_with_prefix(constants::PREFIX_ACCOUNT_UPDATE_NODE, &[tree_hash, calls])?;
        stack_hash = hash_with_prefix(
            constants::PREFIX_ACCOUNT_UPDATE_CONS,
            &[node_hash, stack_hash],
        )?;
    }

    Ok(stack_hash)
}

fn memo_hash(tx: &ZKAppCommand) -> MinaTxResult<Fp> {
    let memo_bytes = tx.memo;

    // Convert bytes to bits (little-endian bit order within each byte)
    let bits: Vec<bool> = memo_bytes
        .iter()
        .flat_map(|&byte| (0..8).map(move |i| (byte >> i) & 1 != 0))
        .collect();

    // Pack bits into fields (254 bits per field for Fp)
    let packed_fields = PackedInput::pack_bool_to_field_legacy(&bits);

    hash_with_prefix(constants::ZK_APP_MEMO, &packed_fields)
}

fn hash_fee_payer(fee: FeePayer, network: &NetworkId) -> MinaTxResult<Fp> {
    let fee_account_update = AccountUpdate::from(fee);
    hash_account_update(&fee_account_update, network)
}

fn hash_account_update(account_update: &AccountUpdate, network: &NetworkId) -> MinaTxResult<Fp> {
    // Check that account update is valid
    assert_account_update_authorization_kind(account_update)?;

    let inputs = account_update.pack().pack_to_fields();
    let network_zk = ZkAppBodyPrefix::from(network.clone());
    hash_with_prefix(network_zk.into(), &inputs.fields)
}

// -------------------------------------------------------------------------------------------------
// ---------------------------------- Low Level Hashing Functions ----------------------------------
// -------------------------------------------------------------------------------------------------

pub(crate) fn hash_noinput(prefix: &str) -> MinaTxResult<Fp> {
    let mut sponge = ArithmeticSponge::<
        Fp,
        PlonkSpongeConstantsKimchi,
        { constants::POSEIDON_FULL_ROUNDS },
    >::new(fp_kimchi::static_params());
    sponge.absorb(&[param_to_field(prefix)?]);
    Ok(sponge.squeeze())
}

pub(crate) fn hash_with_prefix(prefix: &str, data: &[Fp]) -> MinaTxResult<Fp> {
    let mut sponge = ArithmeticSponge::<
        Fp,
        PlonkSpongeConstantsKimchi,
        { constants::POSEIDON_FULL_ROUNDS },
    >::new(fp_kimchi::static_params());
    sponge.absorb(&[param_to_field(prefix)?]);

    sponge.squeeze();

    sponge.absorb(data);
    Ok(sponge.squeeze())
}

// -------------------------------------------------------------------------------------------------
// ---------------------------------------- Utils --------------------------------------------------
// -------------------------------------------------------------------------------------------------

fn assert_account_update_authorization_kind(account_update: &AccountUpdate) -> MinaTxResult<()> {
    let authorization_kind = &account_update.body.authorization_kind;
    let is_signed = authorization_kind.is_signed;
    let is_proved = authorization_kind.is_proved;
    let verification_key_hash = authorization_kind.verification_key_hash;

    if is_proved && is_signed {
        return Err(Box::new(MinaTxError::InvalidZkAppCommand(
            "Invalid authorization kind: Only one of `isProved` and `isSigned` may be true."
                .to_string(),
        )));
    }

    if !is_proved && verification_key_hash != *DUMMY_HASH {
        return Err(Box::new(MinaTxError::InvalidZkAppCommand(
            format!(
                "Invalid authorization kind: If `isProved` is false, verification key hash must be {}, got {}",
                *DUMMY_HASH,
                verification_key_hash
            ),
        )));
    }

    Ok(())
}

/// Validates that call depths in a ZkApp command follow the correct pattern.
/// The first account update must have call depth 0, and subsequent call depths
/// must not be negative and can only increase by at most 1 from the previous.
pub fn is_call_depth_valid(zkapp_command: &ZKAppCommand) -> bool {
    let mut call_depths: Vec<u32> = zkapp_command
        .account_updates
        .iter()
        .map(|a| a.body.call_depth)
        .collect();

    let mut current = match call_depths.first() {
        Some(&depth) => {
            call_depths.remove(0);
            depth
        }
        None => 0,
    };

    if current != 0 {
        return false;
    }

    for call_depth in call_depths {
        if call_depth < current {
            return false;
        }
        if call_depth - current > 1 {
            return false;
        }
        current = call_depth;
    }

    true
}

pub(crate) fn param_to_field(param: &str) -> Result<Fp, MinaTxError> {
    const DEFAULT: [u8; 32] = *b"********************\0\0\0\0\0\0\0\0\0\0\0\0";

    let param_bytes = param.as_bytes();
    let len = param_bytes.len();

    if len > DEFAULT.len() {
        return Err(MinaTxError::InvalidZkAppCommand(format!(
            "must be {} byte maximum",
            DEFAULT.len()
        )));
    }

    let mut fp = DEFAULT;
    fp[..len].copy_from_slice(param_bytes);

    Fp::from_random_bytes(&fp).ok_or_else(|| {
        MinaTxError::InvalidZkAppCommand("Failed to convert parameter to field".to_string())
    })
}

// -------------------------------------------------------------------------------------------------
// --------------------------------------- Tests ---------------------------------------------------
// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::transactions::zkapp_tx::zkapp_test_vectors::{
        get_hash_with_prefix_test_vectors, get_zkapp_test_vectors, parse_expected_hash,
    };

    use super::*;

    #[test]
    fn test_hash_with_prefix_vectors() {
        let test_vectors = get_hash_with_prefix_test_vectors();

        // Panic if no vectors provided
        if test_vectors.is_empty() {
            panic!("No test vectors provided for hash_with_prefix tests");
        }

        for test_vector in test_vectors {
            let computed_hash = hash_with_prefix(test_vector.prefix, &test_vector.input_fields)
                .unwrap_or_else(|_| {
                    panic!("Failed to compute hash for test: {}", test_vector.name)
                });

            let expected_hash = parse_expected_hash(test_vector.expected_hash);

            assert_eq!(
                computed_hash, expected_hash,
                "Hash mismatch for test: {}",
                test_vector.name
            );
        }
    }

    #[test]
    fn test_hash_fee_payer() {
        let test_vectors = get_zkapp_test_vectors();

        // Panic if no vectors provided
        if test_vectors.is_empty() {
            panic!("No test vectors provided for fee payer hash tests");
        }

        for test_vector in test_vectors {
            let computed_hash = hash_fee_payer(
                test_vector.zkapp_command.fee_payer.clone(),
                &test_vector.network,
            )
            .unwrap_or_else(|_| {
                panic!(
                    "Failed to compute fee payer hash for test: {}",
                    test_vector.name
                )
            });

            let expected_hash = parse_expected_hash(test_vector.expected_fee_payer_hash);

            assert_eq!(
                computed_hash, expected_hash,
                "Fee payer hash mismatch for test: {}",
                test_vector.name
            );
        }
    }

    #[test]
    fn test_zk_commit() {
        let test_vectors = get_zkapp_test_vectors();

        // Panic if no vectors provided
        if test_vectors.is_empty() {
            panic!("No test vectors provided for zk_commit tests");
        }

        for test_vector in test_vectors {
            let (computed_account_updates_commitment, computed_full_commitment) =
                zk_commit(&test_vector.zkapp_command, &test_vector.network).unwrap_or_else(|_| {
                    panic!(
                        "Failed to compute commitment for test: {}",
                        test_vector.name
                    )
                });

            let expected_account_updates_commitment =
                parse_expected_hash(test_vector.expected_account_updates_commitment);
            let expected_full_commitment =
                parse_expected_hash(test_vector.expected_full_commitment);

            assert_eq!(
                computed_account_updates_commitment, expected_account_updates_commitment,
                "Account updates commitment mismatch for test: {}",
                test_vector.name
            );

            assert_eq!(
                computed_full_commitment, expected_full_commitment,
                "Full commitment mismatch for test: {}",
                test_vector.name
            );
        }
    }

    #[test]
    fn test_memo_hash() {
        let test_vectors = get_zkapp_test_vectors();

        if test_vectors.is_empty() {
            panic!("No test vectors provided for memo hash tests");
        }

        for test_vector in test_vectors {
            let computed_hash = memo_hash(&test_vector.zkapp_command).unwrap_or_else(|_| {
                panic!("Failed to compute memo hash for test: {}", test_vector.name)
            });

            let expected_hash = parse_expected_hash(test_vector.expected_memo_hash);

            assert_eq!(
                computed_hash, expected_hash,
                "Memo hash mismatch for test: {}",
                test_vector.name
            );
        }
    }

    #[test]
    fn test_call_forest_hash() {
        let test_vectors = get_zkapp_test_vectors();

        if test_vectors.is_empty() {
            panic!("No test vectors provided for call forest hash tests");
        }

        for test_vector in test_vectors {
            let call_forest = CallForest::from(test_vector.zkapp_command);
            let computed_hash = call_forest_hash(&call_forest, &test_vector.network)
                .unwrap_or_else(|_| {
                    panic!(
                        "Failed to compute call forest hash for test: {}",
                        test_vector.name
                    )
                });

            let expected_hash =
                parse_expected_hash(test_vector.expected_account_updates_commitment);

            assert_eq!(
                computed_hash, expected_hash,
                "Call forest hash mismatch for test: {}",
                test_vector.name
            );
        }
    }

    #[test]
    fn test_prefix_to_field() {
        let prefix = "MinaAcctUpdateNode";
        let field = param_to_field(prefix).unwrap();
        assert_eq!(
            field.to_string(),
            "240723076190006710499563866323038773312427551053"
        );
    }
}
