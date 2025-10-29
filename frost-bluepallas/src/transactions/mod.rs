use mina_hasher::Hashable;
use serde::{Deserialize, Serialize};

use crate::transactions::zkapp_tx::ZKAppCommand;

pub mod generic_tx;
pub mod legacy_tx;
pub mod zkapp_tx;
