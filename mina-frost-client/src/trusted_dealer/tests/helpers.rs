use frost_bluepallas as frost;
use mina_hasher::ROInput;
use mina_signer::NetworkId;
use mina_tx::pallas_message::PallasMessage;
use rand::rngs::ThreadRng;
use std::collections::BTreeMap;

type Identifier = frost::Identifier<PallasMessage>;
type SecretShare = frost::keys::SecretShare<PallasMessage>;
type KeyPackage = frost::keys::KeyPackage<PallasMessage>;
type SigningCommitments = frost::round1::SigningCommitments<PallasMessage>;
type SigningNonces = frost::round1::SigningNonces<PallasMessage>;
type SignatureShare = frost::round2::SignatureShare<PallasMessage>;
type SigningPackage = frost::SigningPackage<PallasMessage>;

pub fn key_package(shares: &BTreeMap<Identifier, SecretShare>) -> BTreeMap<Identifier, KeyPackage> {
    let mut key_packages: BTreeMap<_, _> = BTreeMap::new();

    for (identifier, secret_share) in shares {
        let key_package = frost::keys::KeyPackage::try_from(secret_share.clone()).unwrap();
        key_packages.insert(*identifier, key_package);
    }

    key_packages
}

pub fn round_1(
    min_signers: u16,
    mut rng: &mut ThreadRng,
    key_packages: &BTreeMap<Identifier, KeyPackage>,
) -> (
    BTreeMap<Identifier, SigningNonces>,
    BTreeMap<Identifier, SigningCommitments>,
) {
    // Participant Round 1

    let mut nonces_map = BTreeMap::new();
    let mut commitments_map = BTreeMap::new();

    for participant_index in 1..(min_signers + 1) {
        let participant_identifier = participant_index.try_into().expect("should be nonzero");
        let key_package = &key_packages[&participant_identifier];
        let (nonces, commitments) = frost::round1::commit(key_package.signing_share(), &mut rng);
        nonces_map.insert(participant_identifier, nonces);
        commitments_map.insert(participant_identifier, commitments);
    }
    (nonces_map, commitments_map)
}

pub fn round_2(
    nonces_map: BTreeMap<Identifier, SigningNonces>,
    key_packages: &BTreeMap<Identifier, KeyPackage>,
    commitments_map: BTreeMap<Identifier, SigningCommitments>,
    message: &[u8],
) -> (SigningPackage, BTreeMap<Identifier, SignatureShare>) {
    // Signing input must be an explicitly encoded PallasMessage.
    let pallas_message = PallasMessage::from_parts(
        ROInput::new().append_bytes(message),
        NetworkId::TESTNET,
        true,
    );
    let serialized_message = pallas_message.serialize();
    let signing_package = frost::SigningPackage::new(commitments_map, &serialized_message);
    let mut signature_shares = BTreeMap::new();
    for participant_identifier in nonces_map.keys() {
        let key_package = &key_packages[participant_identifier];

        let nonces = &nonces_map[participant_identifier];
        let signature_share = frost::round2::sign(&signing_package, nonces, key_package).unwrap();
        signature_shares.insert(*participant_identifier, signature_share);
    }
    (signing_package, signature_shares)
}
