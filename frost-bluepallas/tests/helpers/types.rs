use frost_bluepallas as frost;
use mina_tx::pallas_message::PallasMessage;

pub type Suite = frost::BluePallas<PallasMessage>;
pub type Identifier = frost::Identifier<PallasMessage>;
pub type SigningNonces = frost::round1::SigningNonces<PallasMessage>;
pub type SigningCommitments = frost::round1::SigningCommitments<PallasMessage>;
pub type SigningPackage = frost::SigningPackage<PallasMessage>;
pub type SignatureShare = frost::round2::SignatureShare<PallasMessage>;
pub type SecretShare = frost::keys::SecretShare<PallasMessage>;
pub type KeyPackage = frost::keys::KeyPackage<PallasMessage>;
pub type PublicKeyPackage = frost::keys::PublicKeyPackage<PallasMessage>;
pub type Signature = frost::Signature<PallasMessage>;
