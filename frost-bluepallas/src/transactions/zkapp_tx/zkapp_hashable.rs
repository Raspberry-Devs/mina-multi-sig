use mina_hasher::{Hashable, ROInput};
use mina_signer::NetworkId;

use crate::transactions::zkapp_tx::{commit::zk_commit, ZKAppCommandWithNetwork};

/// This file produces the final ROInput for ZkApp transactions to be hashed and signed over
impl<'a> Hashable for ZKAppCommandWithNetwork<'a> {
    type D = NetworkId;

    fn to_roinput(&self) -> ROInput {
        // Get ZKCommit
        let zk_commit = zk_commit(&self.command, self.network).unwrap();

        ROInput::new().append_field(zk_commit)
    }

    fn domain_string(domain_param: Self::D) -> Option<String> {
        match domain_param {
            NetworkId::MAINNET => "MinaSignatureMainnet",
            NetworkId::TESTNET => "CodaSignature",
        }
        .to_string()
        .into()
    }
}
