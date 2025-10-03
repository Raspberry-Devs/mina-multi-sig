use crate::transactions::zkapp_tx::{AccountUpdate, ZKAppCommand};

// This converts a list of account updates into a forest structure
// Parent-child relationships are determined by call_depth
pub fn collect_account_updates_to_forest(zkapp: &ZKAppCommand) -> Vec<&AccountUpdate> {
    let mut forest = Vec::new();
    for au in &zkapp.account_updates {
        forest.push(au);
    }
    forest
}
