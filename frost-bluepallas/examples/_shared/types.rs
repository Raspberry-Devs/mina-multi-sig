use frost_bluepallas as frost;
use mina_tx::pallas_message::PallasMessage;

pub type Identifier = frost::Identifier<PallasMessage>;
#[allow(dead_code)]
pub type KeyPackage = frost::keys::KeyPackage<PallasMessage>;
#[allow(dead_code)]
pub type SigningNonces = frost::round1::SigningNonces<PallasMessage>;
#[allow(dead_code)]
pub type SigningCommitments = frost::round1::SigningCommitments<PallasMessage>;
