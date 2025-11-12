/// ZkApp transaction commitment computation
/// This module provides functionality to compute commitments for ZkApp transactions which can be later signed over
use std::collections::VecDeque;

use ark_ff::PrimeField;
use bitvec::{order::Lsb0, vec::BitVec};
use mina_hasher::Fp;
use mina_poseidon::{
    constants::PlonkSpongeConstantsKimchi,
    pasta::fp_kimchi,
    poseidon::{ArithmeticSponge, Sponge},
};
use mina_signer::NetworkId;

use crate::{
    errors::{BluePallasError, BluePallasResult},
    transactions::zkapp_tx::{
        constants::{self, ZkAppBodyPrefix, DUMMY_HASH},
        hash::param_to_field,
        zkapp_packable::Packable,
        AccountUpdate, Authorization, AuthorizationKind, BalanceChange, FeePayer, RangeCondition,
        ZKAppCommand,
    },
};

const FIELDS_PER_PACKED_MEMO: usize = 254;

/// A single node in the call forest representing an account update and its children
#[derive(Clone)]
pub struct CallTree {
    account_update: AccountUpdate,
    children: CallForest,
}

/// A forest of call trees representing the hierarchical structure of account updates
pub type CallForest = Vec<CallTree>;

/// Converts a flat list of account updates into a hierarchical call forest structure
/// based on their call depths. Each level of the tree represents a call depth.
pub fn account_updates_to_call_forest(
    updates: &mut VecDeque<AccountUpdate>,
    call_depth: u32,
) -> CallForest {
    let mut forest: CallForest = Vec::new();

    while !updates.is_empty() {
        let account_update = &updates[0];
        if account_update.body.call_depth < call_depth {
            return forest;
        }

        // Unwrap is safe here because we checked that updates is not empty
        let account_update = updates.pop_front().unwrap();
        let children = account_updates_to_call_forest(updates, call_depth + 1);

        forest.push(CallTree {
            account_update,
            children,
        });
    }

    forest
}

/// Converts a ZkApp command to a call forest by processing its account updates
/// AccountUpdates are organized into a hierarchical tree-based structure based on their call depths.
/// A parent-child relationship is established where an AccountUpdate with call depth n
/// can have children with call depth n+1.
pub fn zkapp_command_to_call_forest(tx: &ZKAppCommand) -> CallForest {
    let updates = tx.account_updates.clone();
    account_updates_to_call_forest(&mut updates.into(), 0)
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

/// Packs a slice of bits into field elements, taking chunks of 254 bits at a time.
/// This matches the o1js `packToFieldsLegacy` behavior for bit packing.
fn pack_to_field(bits: &[bool]) -> Vec<Fp> {
    let mut packed_fields = Vec::new();
    let mut remaining_bits = bits;

    while !remaining_bits.is_empty() {
        let chunk_size = std::cmp::min(remaining_bits.len(), FIELDS_PER_PACKED_MEMO);
        let field_bits = &remaining_bits[..chunk_size];
        remaining_bits = &remaining_bits[chunk_size..];

        // Convert bits to BigInt using BitVec with LSB order
        let bitvec: BitVec<u8, Lsb0> = BitVec::from_iter(field_bits);
        let field = Fp::from_le_bytes_mod_order(&bitvec.into_vec());
        packed_fields.push(field);
    }

    packed_fields
}

fn memo_hash(tx: &ZKAppCommand) -> BluePallasResult<Fp> {
    let memo_bytes = tx.memo;

    // Convert bytes to bits (little-endian bit order within each byte)
    let bits: Vec<bool> = memo_bytes
        .iter()
        .flat_map(|&byte| (0..8).map(move |i| (byte >> i) & 1 != 0))
        .collect();

    // Pack bits into fields (254 bits per field for Fp)
    let packed_fields = pack_to_field(&bits);

    hash_with_prefix(constants::ZK_APP_MEMO, &packed_fields)
}

/// Produces a commitment for a ZkApp command by hashing its structure and contents.
/// Validates call depths and authorization kinds before computing the commitment.
/// Returns two Fp elements, representing the accountUpdates commitment and the overall commitment respectively.
/// Overall commitment includes memo, fee payer, and account updates commitments.
pub(crate) fn zk_commit(tx: &ZKAppCommand, network: NetworkId) -> BluePallasResult<(Fp, Fp)> {
    if !is_call_depth_valid(tx) {
        return Err(Box::new(BluePallasError::InvalidZkAppCommand(
            "Call depths are not valid".to_string(),
        )));
    }

    let forest = zkapp_command_to_call_forest(tx);

    // Compute the account-updates commitment using the call forest hashing routine.
    let account_updates_commitment = call_forest_hash(&forest, &network)?;

    let memo_hash = memo_hash(tx)?;

    let fee_payer_hash = fee_payer_hash(tx.fee_payer.clone(), &network)?;

    let full_commit = hash_with_prefix(
        constants::PREFIX_ACCOUNT_UPDATE_CONS,
        &[memo_hash, fee_payer_hash, account_updates_commitment],
    )?;

    Ok((account_updates_commitment, full_commit))
}

fn fee_payer_hash(fee: FeePayer, network: &NetworkId) -> BluePallasResult<Fp> {
    let fee_account_update = account_update_from_fee_payer(fee);
    hash_account_update(&fee_account_update, network)
}

fn account_update_from_fee_payer(fee: FeePayer) -> AccountUpdate {
    // Unpack fee payer pieces
    let FeePayer {
        body,
        authorization,
    } = fee;
    let public_key = body.public_key;
    let fee_magnitude = body.fee;
    let nonce = body.nonce;
    let vaild_until = body.valid_until.unwrap_or(u32::MAX);

    let account_update = AccountUpdate::default();
    let mut body = account_update.body;

    body.public_key = public_key;
    body.balance_change = BalanceChange {
        magnitude: fee_magnitude,
        sgn: -1,
    };
    body.increment_nonce = true;

    body.preconditions.network.global_slot_since_genesis = {
        Some(RangeCondition {
            lower: 0,
            upper: vaild_until,
        })
    };
    body.preconditions.account.nonce = {
        Some(RangeCondition {
            lower: nonce,
            upper: nonce,
        })
    };
    body.use_full_commitment = true;
    body.implicit_account_creation_fee = true;
    body.authorization_kind = AuthorizationKind {
        is_proved: false,
        is_signed: true,
        verification_key_hash: *DUMMY_HASH,
    };

    AccountUpdate {
        body,
        authorization: Authorization {
            proof: None,
            signature: Some(authorization),
        },
    }
}

pub(crate) fn hash_noinput(prefix: &str) -> BluePallasResult<Fp> {
    let mut sponge =
        ArithmeticSponge::<Fp, PlonkSpongeConstantsKimchi>::new(fp_kimchi::static_params());
    sponge.absorb(&[param_to_field(prefix)?]);
    Ok(sponge.squeeze())
}

pub(crate) fn hash_with_prefix(prefix: &str, data: &[Fp]) -> BluePallasResult<Fp> {
    let mut sponge =
        ArithmeticSponge::<Fp, PlonkSpongeConstantsKimchi>::new(fp_kimchi::static_params());
    sponge.absorb(&[param_to_field(prefix)?]);

    sponge.squeeze();

    sponge.absorb(data);
    Ok(sponge.squeeze())
}

fn hash_account_update(
    account_update: &AccountUpdate,
    network: &NetworkId,
) -> BluePallasResult<Fp> {
    // Check that account update is valid
    assert_account_update_authorization_kind(account_update)?;

    // TODO: Check whether this is consistent with packToFields() in o1js
    let packed_inputs = account_update.pack().to_mina_hasher_roi();
    let inputs = packed_inputs.to_fields();
    let network_zk = ZkAppBodyPrefix::from(network.clone());
    hash_with_prefix(network_zk.into(), &inputs)
}

fn assert_account_update_authorization_kind(
    account_update: &AccountUpdate,
) -> BluePallasResult<()> {
    let authorization_kind = &account_update.body.authorization_kind;
    let is_signed = authorization_kind.is_signed;
    let is_proved = authorization_kind.is_proved;
    let verification_key_hash = authorization_kind.verification_key_hash;

    if is_proved && is_signed {
        return Err(Box::new(BluePallasError::InvalidZkAppCommand(
            "Invalid authorization kind: Only one of `isProved` and `isSigned` may be true."
                .to_string(),
        )));
    }

    if !is_proved && verification_key_hash != *DUMMY_HASH {
        return Err(Box::new(BluePallasError::InvalidZkAppCommand(
            format!(
                "Invalid authorization kind: If `isProved` is false, verification key hash must be {}, got {}",
                *DUMMY_HASH,
                verification_key_hash
            ),
        )));
    }

    Ok(())
}

/// Computes the hash of a call forest representing account updates.
/// Traverses the forest in reverse order, for each CallTree:
///  - recursively compute calls = hash(children)
///  - tree_hash = hash_account_update(account_update)
///  - node_hash = hash_with_prefix("MinaAcctUpdateNode", [tree_hash, calls])
///  - stack_hash = hash_with_prefix("MinaAcctUpdateCons", [node_hash, stack_hash])
fn call_forest_hash(forest: &CallForest, network: &NetworkId) -> BluePallasResult<Fp> {
    let mut stack_hash = constants::EMPTY_STACK_HASH;

    // iterate in reverse (last -> first)
    for call_tree in forest.iter().rev() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    use super::super::test_vectors::{
        get_hash_with_prefix_test_vectors, get_zkapp_test_vectors, parse_expected_hash,
    };

    #[test]
    fn test_pack_to_fields() {
        let mut bits = vec![true];
        bits.extend(vec![false; 271]);

        let packed_fields = pack_to_field(&bits);
        assert_eq!(packed_fields.len(), 2);
        assert_eq!(packed_fields[0], Fp::from(1u64));
        assert_eq!(packed_fields[1], Fp::from(0u64));

        let bits = vec![
            true, false, false, false, false, false, false, false, false, true, false, false,
            false, false, false, false, false, true, false, false, false, false, true, true, true,
            false, true, false, true, true, false, true, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false,
        ];
        assert_eq!(bits.len(), 272);

        let packed_fields = pack_to_field(&bits);
        assert_eq!(packed_fields.len(), 2);
        assert_eq!(packed_fields[0], Fp::from(3049390593u64));
        assert_eq!(packed_fields[1], Fp::from(0u64));
    }

    #[test]
    fn test_hash_with_prefix_vectors() {
        let test_vectors = get_hash_with_prefix_test_vectors();

        // Skip test if no vectors provided
        if test_vectors.is_empty() {
            println!("Warning: No test vectors provided for hash_with_prefix");
            return;
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
    fn test_fee_payer_hash() {
        let test_vectors = get_zkapp_test_vectors();

        // Skip test if no vectors provided
        if test_vectors.is_empty() {
            println!("Warning: No test vectors provided for fee_payer_hash");
            return;
        }

        for test_vector in test_vectors {
            let computed_hash = fee_payer_hash(
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

        // Skip test if no vectors provided
        if test_vectors.is_empty() {
            println!("Warning: No test vectors provided for zk_commit");
            return;
        }

        for test_vector in test_vectors {
            let (computed_account_updates_commitment, computed_full_commitment) =
                zk_commit(&test_vector.zkapp_command, test_vector.network.clone()).unwrap_or_else(
                    |_| {
                        panic!(
                            "Failed to compute commitment for test: {}",
                            test_vector.name
                        )
                    },
                );

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
            println!("Warning: No test vectors provided for memo_hash");
            return;
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
            println!("Warning: No test vectors provided for call_forest_hash");
            return;
        }

        for test_vector in test_vectors {
            let call_forest = zkapp_command_to_call_forest(&test_vector.zkapp_command);
            let computed_hash = call_forest_hash(&call_forest, &test_vector.network)
                .unwrap_or_else(|_| {
                    panic!(
                        "Failed to compute call forest hash for test: {}",
                        test_vector.name
                    )
                });

            let computed_hash_str = computed_hash.to_string();

            let expected_hash =
                parse_expected_hash(test_vector.expected_account_updates_commitment);

            assert_eq!(
                computed_hash, expected_hash,
                "Call forest hash mismatch for test: {}",
                test_vector.name
            );
        }
    }
}
