/// ZkApp transaction commitment computation
/// This module provides functionality to compute commitments for ZkApp transactions which can be later signed over
use std::{collections::VecDeque, str::FromStr};

use ark_ff::{AdditiveGroup, BigInt, PrimeField};
use mina_hasher::{Fp, Hashable};
use mina_poseidon::{
    constants::PlonkSpongeConstantsKimchi,
    pasta::fp_kimchi,
    poseidon::{ArithmeticSponge, Sponge},
};
use mina_signer::NetworkId;

use crate::{
    errors::{BluePallasError, BluePallasResult},
    transactions::{
        self,
        zkapp_tx::{
            constants::{self, ZkAppBodyPrefix},
            hash::param_to_field,
            AccountUpdate, ZKAppCommand,
        },
    },
};

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
    let mut updates = tx.account_updates.clone();
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

/// Produces a commitment for a ZkApp command by hashing its structure and contents.
/// Validates call depths and authorization kinds before computing the commitment.
/// Returns two Fp elements, representing the accountUpdates commitment and the overall commitment respectively.
/// Overall commitment includes memo, fee payer, and account updates commitments.
pub fn zk_commit(tx: &ZKAppCommand, network: NetworkId) -> BluePallasResult<(Fp, Fp)> {
    if !is_call_depth_valid(tx) {
        return Err(Box::new(BluePallasError::InvalidZkAppCommand(
            "Call depths are not valid".to_string(),
        )));
    }

    let forest = zkapp_command_to_call_forest(tx);

    // Compute the account-updates commitment using the call forest hashing routine.
    let account_updates_commitment = call_forest_hash(&forest, &network)?;

    // TODO: incorporate memo, fee payer, etc. into the overall commitment per protocol.
    // For now return the account-updates commitment for both values (placeholder for overall).
    Ok((account_updates_commitment, account_updates_commitment))
}

fn hash_with_prefix(prefix: &str, data: &[Fp]) -> BluePallasResult<Fp> {
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
    let inputs = account_update.to_roinput().to_fields();
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

    let dummy_bigint = BigInt::from_str(constants::DUMMY_HASH).map_err(|_| {
        BluePallasError::InvalidZkAppCommand("Failed to parse dummy hash".to_string())
    })?;
    let dummy_verification_key_hash =
        transactions::zkapp_tx::Field(Fp::from_bigint(dummy_bigint).ok_or(
            BluePallasError::InvalidZkAppCommand("Failed to convert dummy hash to Fp".to_string()),
        )?);

    if !is_proved && verification_key_hash != dummy_verification_key_hash {
        return Err(Box::new(BluePallasError::InvalidZkAppCommand(
            format!(
                "Invalid authorization kind: If `isProved` is false, verification key hash must be {}, got {}",
                constants::DUMMY_HASH,
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

    #[test]
    fn test_hash_with_prefix() {
        let prefix = "MinaAcctUpdateNode";
        let strs = [
            "23487734643675003113914430489774334948844391842009122040704261138931555665056",
            "0",
        ];
        let elems = strs
            .iter()
            .map(|f| Fp::from_str(f).unwrap())
            .collect::<Vec<Fp>>();

        let hash = hash_with_prefix(prefix, &elems).unwrap();
        assert_eq!(
            hash.to_string(),
            "20456728518925904340727370305821489989002971473792411299271630913563245218671"
        );
    }
}
