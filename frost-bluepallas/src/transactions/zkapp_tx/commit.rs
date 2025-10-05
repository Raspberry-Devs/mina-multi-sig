use ark_ff::{AdditiveGroup, BigInt};
use mina_hasher::Fp;
use mina_signer::NetworkId;

use crate::{
    errors::{BluePallasError, BluePallasResult},
    transactions::zkapp_tx::{AccountUpdate, ZKAppCommand},
};

/// A single node in the call forest representing an account update and its children
#[derive(Clone)]
pub struct CallTree {
    pub account_update: AccountUpdate,
    pub children: CallForest,
}

/// A forest of call trees representing the hierarchical structure of account updates
pub type CallForest = Vec<CallTree>;

/// Converts a flat list of account updates into a hierarchical call forest structure
/// based on their call depths. Each level of the tree represents a call depth.
pub fn account_updates_to_call_forest(
    updates: &mut Vec<AccountUpdate>,
    call_depth: u32,
) -> CallForest {
    let mut forest: CallForest = Vec::new();

    while !updates.is_empty() {
        let account_update = &updates[0];
        if account_update.body.call_depth < call_depth {
            return forest;
        }

        let account_update = updates.remove(0);
        let children = account_updates_to_call_forest(updates, call_depth + 1);

        forest.push(CallTree {
            account_update,
            children,
        });
    }

    forest
}

/// Converts a ZkApp command to a call forest by processing its account updates
pub fn zkapp_command_to_call_forest(tx: &ZKAppCommand) -> CallForest {
    let mut updates = tx.account_updates.clone();
    account_updates_to_call_forest(&mut updates, 0)
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

    let current = match call_depths.first() {
        Some(&depth) => {
            call_depths.remove(0);
            depth
        }
        None => 0,
    };

    if current != 0 {
        return false;
    }

    let mut current = current;
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

pub fn zk_commit(tx: &ZKAppCommand, network: NetworkId) -> BluePallasResult<Fp> {
    if !is_call_depth_valid(tx) {
        return Err(Box::new(BluePallasError::InvalidZkAppCommand(
            "Call depths are not valid".to_string(),
        )));
    }

    Ok(Fp::ZERO)
}
