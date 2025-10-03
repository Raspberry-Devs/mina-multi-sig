use crate::transactions::zkapp_struct::AccountUpdate;
use mina_hasher::Hashable;
use mina_signer::NetworkId;

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
