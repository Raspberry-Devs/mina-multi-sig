use frost_bluepallas::{keys::generate_with_dealer};
use frost_core::{
    keys::{IdentifierList, KeyPackage},
    round1, round2, SigningPackage,
};
use hex;
use rand::SeedableRng;
use serde_json::json;
use std::collections::BTreeMap;

fn main() {
    let max_signers = 3;
    let min_signers = 2;
    let seed = [42u8; 32];
    let mut rng = rand::rngs::StdRng::from_seed(seed);

    // 1. Key Generation
    let (shares, pubkey_package) =
        generate_with_dealer(max_signers, min_signers, IdentifierList::Default, &mut rng)
            .expect("Key generation failed");

    let mut key_packages: BTreeMap<_, _> = BTreeMap::new();
    for (id, share) in shares.iter() {
        key_packages.insert(*id, KeyPackage::try_from(share.clone()).unwrap());
    }

    // Pick participants for this signing round (1 and 3)
    let mut signers = BTreeMap::new();
    let id_1 = shares.keys().nth(0).unwrap();
    let id_2 = shares.keys().nth(1).unwrap(); // Keep id_2 for the pubkey package
    let id_3 = shares.keys().nth(2).unwrap();
    signers.insert(*id_1, key_packages.get(id_1).unwrap());
    signers.insert(*id_3, key_packages.get(id_3).unwrap());

    // 2. Round 1: Commitments
    let mut commitments = BTreeMap::new();
    let mut nonces = BTreeMap::new(); // Private to each participant

    for (id, key_package) in signers.iter() {
        let (nonce, commitment) =
            round1::commit(key_package.signing_share(), &mut rng);
        nonces.insert(*id, nonce);
        commitments.insert(*id, commitment);
    }

    // 3. Create SigningPackage
    let message_hex = "74657374"; // "test" in hex
    let message = hex::decode(message_hex).unwrap();
    let signing_package = SigningPackage::new(commitments.clone(), &message);

    // 4. Round 2: Sign
    let mut signature_shares = BTreeMap::new();
    for id in signers.keys() {
        let key_package = key_packages.get(id).unwrap();
        let nonce = nonces.get(id).unwrap();
        let sig_share = round2::sign(&signing_package, nonce, key_package).unwrap();
        signature_shares.insert(*id, sig_share);
    }

    // 5. Aggregate
    let group_signature =
        frost_core::aggregate(&signing_package, &signature_shares, &pubkey_package).unwrap();

    // 6. Print all values for mod.rs
    println!("// New values generated for bluepallas. Copy and paste this entire block into the get_helpers() function.");
    println!("");

    println!(
        "let participant_id_1 = \"{}\".to_string();",
        hex::encode(id_1.serialize())
    );
    println!(
        "let participant_id_2 = \"{}\".to_string();",
        hex::encode(id_2.serialize())
    );
    println!(
        "let participant_id_3 = \"{}\".to_string();",
        hex::encode(id_3.serialize())
    );
    println!(
        "let public_key_1 = \"{}\".to_string();",
        hex::encode(pubkey_package.verifying_shares().get(id_1).unwrap().serialize().unwrap())
    );
    println!(
        "let public_key_2 = \"{}\".to_string();",
        hex::encode(pubkey_package.verifying_shares().get(id_2).unwrap().serialize().unwrap())
    );
    println!(
        "let public_key_3 = \"{}\".to_string();",
        hex::encode(pubkey_package.verifying_shares().get(id_3).unwrap().serialize().unwrap())
    );
    println!(
        "let verifying_key = \"{}\".to_string();",
        hex::encode(pubkey_package.verifying_key().serialize().unwrap())
    );
    println!("");

    let hiding_commitment_1_str =
        hex::encode(commitments.get(id_1).unwrap().hiding().serialize().unwrap());
    let binding_commitment_1_str =
        hex::encode(commitments.get(id_1).unwrap().binding().serialize().unwrap());
    let hiding_commitment_3_str =
        hex::encode(commitments.get(id_3).unwrap().hiding().serialize().unwrap());
    let binding_commitment_3_str =
        hex::encode(commitments.get(id_3).unwrap().binding().serialize().unwrap());

    println!(
        "let hiding_commitment_1 = \"{}\".to_string();",
        hiding_commitment_1_str
    );
    println!(
        "let hiding_commitment_3 = \"{}\".to_string();",
        hiding_commitment_3_str
    );
    println!(
        "let binding_commitment_1 = \"{}\".to_string();",
        binding_commitment_1_str
    );
    println!(
        "let binding_commitment_3 = \"{}\".to_string();",
        binding_commitment_3_str
    );
    println!("");
    println!("let message = \"{}\".to_string();", message_hex);
    println!(
        "let group_signature = r#\"{}\"#.to_string();",
        serde_json::to_string(&group_signature).unwrap()
    );
    println!("");
    println!("// JSON messages");
    println!(
        "let commitments_from_part_1 = r#\"{}\"#.to_string();",
        json!({
            "identifier": hex::encode(id_1.serialize()),
            "commitments": {
                "hiding": hiding_commitment_1_str,
                "binding": binding_commitment_1_str
            }
        })
    );
    println!(
        "let commitments_from_part_3 = r#\"{}\"#.to_string();",
        json!({
            "identifier": hex::encode(id_3.serialize()),
            "commitments": {
                "hiding": hiding_commitment_3_str,
                "binding": binding_commitment_3_str
            }
        })
    );
    println!("");
    println!(
        "let signing_package_helper = r#\"{}\"#.to_string();",
        serde_json::to_string(&signing_package).unwrap()
    );
    println!("");
    println!(
        "let signature_1 = r#\"{}\"#.to_string();",
        serde_json::to_string(signature_shares.get(id_1).unwrap()).unwrap()
    );
    println!(
        "let signature_3 = r#\"{}\"#.to_string();",
        serde_json::to_string(signature_shares.get(id_3).unwrap()).unwrap()
    );
    println!("");
    println!(
        "let pub_key_package = r#\"{}\"#.to_string();",
        serde_json::to_string(&pubkey_package).unwrap()
    );
    println!("");
    println!(
        "let commitments_input_1 = r#\"{}\"#.to_string();",
        json!({
            "hiding": hiding_commitment_1_str,
            "binding": binding_commitment_1_str
        })
    );
    println!(
        "let commitments_input_3 = r#\"{}\"#.to_string();",
        json!({
            "hiding": hiding_commitment_3_str,
            "binding": binding_commitment_3_str
        })
    );
}
