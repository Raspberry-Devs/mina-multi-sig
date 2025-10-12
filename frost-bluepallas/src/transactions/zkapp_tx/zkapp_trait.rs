use crate::{
    errors::BluePallasResult,
    transactions::zkapp_tx::{commit::zk_commit, AccountUpdate, ZKAppCommand},
};
use mina_hasher::{Fp, Hashable};
use mina_signer::NetworkId;

impl Hashable for ZKAppCommand {
    type D = NetworkId;

    fn to_roinput(&self) -> mina_hasher::ROInput {
        // We would call zk_commit here and create a single field-based mina_hasher::ROInput
        todo!()
    }

    fn domain_string(domain_param: Self::D) -> Option<String> {
        // Domain strings must have length <= 20
        match domain_param {
            NetworkId::MAINNET => "MinaSignatureMainnet",
            NetworkId::TESTNET => "CodaSignature",
        }
        .to_string()
        .into()
    }
}

impl ZKAppCommand {
    pub fn hash(&self, _network_id: NetworkId) -> BluePallasResult<Fp> {
        let (commit, full_commit) = zk_commit(self, _network_id)?;
        Ok(full_commit)
    }
}

impl Hashable for AccountUpdate {
    type D = NetworkId;

    fn to_roinput(&self) -> mina_hasher::ROInput {
        todo!()
    }

    fn domain_string(domain_param: Self::D) -> Option<String> {
        // Domain strings must have length <= 20
        match domain_param {
            NetworkId::MAINNET => "MinaSignatureMainnet",
            NetworkId::TESTNET => "CodaSignature",
        }
        .to_string()
        .into()
    }
}
