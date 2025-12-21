use alloc::string::{String, ToString};
use mina_hasher::{Hashable, ROInput};
use mina_signer::NetworkId;

use crate::transactions::zkapp_tx::{commit::zk_commit, ZKAppCommand};

#[derive(Clone, Debug)]
pub struct ZKAppCommandHashable<'a> {
    pub tx: &'a ZKAppCommand,
    pub network: NetworkId,
}

impl<'a> ZKAppCommandHashable<'a> {
    pub fn new(tx: &'a ZKAppCommand, network: NetworkId) -> Self {
        Self { tx, network }
    }
}

impl<'a> Hashable for ZKAppCommandHashable<'a> {
    type D = NetworkId;

    fn domain_string(domain_param: Self::D) -> Option<String> {
        match domain_param {
            NetworkId::MAINNET => "MinaSignatureMainnet",
            NetworkId::TESTNET => "CodaSignature",
        }
        .to_string()
        .into()
    }

    fn to_roinput(&self) -> mina_hasher::ROInput {
        // Convert the ZKAppCommand into a field element by hashing, return single-field ROInput
        // This code follows O1JS logic, where ZKAppCommand is hashed before being passed to the signature
        let (_, commit) = zk_commit(self.tx, &self.network).unwrap();
        ROInput::new().append_field(commit)
    }
}
