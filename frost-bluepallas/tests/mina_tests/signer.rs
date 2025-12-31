#![allow(clippy::needless_borrows_for_generic_args)]
#![cfg(test)]

use frost_bluepallas::{
    hasher::PallasMessage,
    helper,
    transactions::{legacy_tx::LegacyTransaction, TransactionEnvelope},
    translate::{translate_minask, translate_pk},
};
use frost_core::Ciphersuite;
use mina_signer::{Keypair, NetworkId, PubKey, Signer};

#[test]
fn signer_test_raw() {
    let network_id = NetworkId::TESTNET;

    let kp = Keypair::from_hex("164244176fddb5d769b7de2027469d027ad428fadcc0c02396e6280142efb718")
        .expect("failed to create keypair");
    let tx = LegacyTransaction::new_payment(
        kp.public.clone(),
        PubKey::from_address("B62qicipYxyEHu7QjUqS7QvBipTs5CzgkYZZZkPoKVYBu6tnDUcE9Zt")
            .expect("invalid address"),
        1729000000000,
        2000000000,
        16,
    )
    .set_valid_until(271828)
    .set_memo_str("Hello Mina!")
    .unwrap();

    assert_eq!(tx.valid_until, 271828);
    assert_eq!(
        tx.memo,
        [
            0x01, 0x0b, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x4d, 0x69, 0x6e, 0x61, 0x21, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00
        ]
    );

    // Generate FROST signature using the private key
    let tx_env = TransactionEnvelope::new_legacy(NetworkId::TESTNET, tx);
    let msg = tx_env.serialize().unwrap();
    let fr_sk =
        translate_minask(&kp).expect("failed to translate mina keypair to frost signing key");

    let (sig, vk) = helper::generate_signature_from_sk(&msg, &fr_sk, rand_core::OsRng)
        .expect("failed to generate FROST signature");

    // Convert signature to Mina format
    let mina_sig = frost_bluepallas::translate::translate_sig(&sig)
        .expect("failed to translate FROST signature to Mina signature");

    // Convert verifying key to Mina format
    let mina_vk = frost_bluepallas::translate::translate_pk(&vk)
        .expect("failed to translate FROST verifying key to Mina public key");

    // Verify that vk from FROST and Mina matches
    let frost_addr = mina_vk.into_address();
    let mina_addr = kp.public.into_address();
    assert_eq!(
        frost_addr, mina_addr,
        "FROST verifying key address does not match Mina public key address"
    );

    // Create ctx signer and verify the signature
    let mut ctx = mina_signer::create_legacy(network_id.clone());
    let is_valid = ctx.verify(&mina_sig, &mina_vk, &tx_env);

    assert!(is_valid, "Mina signature verification failed");
}

#[test]
fn sign_mina_tx() {
    let mut rng = rand_core::OsRng;

    // Use trusted dealer to setup public and packages
    let max_signers = 5;
    let min_signers = 3;
    let (shares, pubkey_package) = frost_bluepallas::keys::generate_with_dealer(
        max_signers,
        min_signers,
        frost_bluepallas::keys::IdentifierList::Default,
        &mut rng,
    )
    .expect("Failed to generate key shares");

    // Convert pubkey package to Mina format

    // Create a transaction
    let tx = LegacyTransaction::new_payment(
        translate_pk(pubkey_package.verifying_key())
            .expect("failed to translate verifying key to Mina public key"),
        PubKey::from_address("B62qicipYxyEHu7QjUqS7QvBipTs5CzgkYZZZkPoKVYBu6tnDUcE9Zt")
            .expect("invalid address"),
        1729000000000,
        2000000000,
        16,
    )
    .set_valid_until(271828)
    .set_memo_str("Hello Mina!")
    .unwrap();

    // Generate FROST signature
    let tx_env = TransactionEnvelope::new_legacy(NetworkId::TESTNET, tx);
    let msg = tx_env.serialize().unwrap();
    let (sig, vk) = helper::sign_from_packages(&msg, shares, pubkey_package, &mut rng)
        .expect("Failed to sign message with FROST");

    // Verify the signature
    let mina_sig = frost_bluepallas::translate::translate_sig(&sig)
        .expect("Failed to translate FROST signature to Mina signature");
    let mina_vk = frost_bluepallas::translate::translate_pk(&vk)
        .expect("Failed to translate FROST verifying key to Mina public key");

    // Verify the signature using Mina Signer
    let mut ctx = mina_signer::create_legacy(NetworkId::TESTNET);
    let is_valid = ctx.verify(&mina_sig, &mina_vk, &tx_env);
    let mut ctx2 = mina_signer::create_legacy(NetworkId::TESTNET);
    let is_valid_tx = ctx2.verify(&mina_sig, &mina_vk, &tx_env);

    assert!(is_valid, "Mina signature verification failed");
    assert!(is_valid_tx, "Mina transaction verification failed");
}

#[test]
fn sign_mina_tx_mainnet() {
    let mut rng = rand_core::OsRng;

    // Set network id to Mainnet
    let network_id = NetworkId::MAINNET;

    // Use trusted dealer to setup public and packages
    let max_signers = 3;
    let min_signers = 2;
    let (shares, pubkey_package) = frost_bluepallas::keys::generate_with_dealer(
        max_signers,
        min_signers,
        frost_bluepallas::keys::IdentifierList::Default,
        &mut rng,
    )
    .expect("Failed to generate key shares");

    // Create a transaction for mainnet
    let tx = LegacyTransaction::new_payment(
        translate_pk(pubkey_package.verifying_key())
            .expect("failed to translate verifying key to Mina public key"),
        PubKey::from_address("B62qicipYxyEHu7QjUqS7QvBipTs5CzgkYZZZkPoKVYBu6tnDUcE9Zt")
            .expect("invalid address"),
        1500000000000, // Different amount
        1000000000,    // Different fee
        10,            // Different nonce
    )
    .set_valid_until(300000)
    .set_memo_str("Mainnet Test!")
    .unwrap();

    // Generate FROST signature
    let tx_env = TransactionEnvelope::new_legacy(network_id.clone(), tx);
    let msg = tx_env.serialize().unwrap();
    let (sig, vk) = helper::sign_from_packages(&msg, shares, pubkey_package, &mut rng)
        .expect("Failed to sign message with FROST");

    let _chall = frost_bluepallas::BluePallas::challenge(sig.R(), &vk, &msg)
        .expect("Expect challenge to calculate");

    // Convert signature to Mina format
    let mina_sig = frost_bluepallas::translate::translate_sig(&sig)
        .expect("Failed to translate FROST signature to Mina signature");
    let mina_vk = frost_bluepallas::translate::translate_pk(&vk)
        .expect("Failed to translate FROST verifying key to Mina public key");

    // Verify the signature using Mina Signer with MAINNET
    let mut ctx = mina_signer::create_legacy(network_id);
    let is_valid = ctx.verify(
        &mina_sig,
        &mina_vk,
        &TransactionEnvelope::deserialize(&msg).unwrap(),
    );

    assert!(is_valid, "Mina signature verification failed on MAINNET");
}

#[test]
fn transaction_json_deser_with_mina_sign() {
    let mut rng = rand_core::OsRng;

    // Use trusted dealer to setup public and packages
    let max_signers = 5;
    let min_signers = 3;
    let (shares, pubkey_package) = frost_bluepallas::keys::generate_with_dealer(
        max_signers,
        min_signers,
        frost_bluepallas::keys::IdentifierList::Default,
        &mut rng,
    )
    .expect("Failed to generate key shares");

    // Convert pubkey package to Mina format

    // Create a transaction
    let tx = LegacyTransaction::new_payment(
        translate_pk(pubkey_package.verifying_key())
            .expect("failed to translate verifying key to Mina public key"),
        PubKey::from_address("B62qicipYxyEHu7QjUqS7QvBipTs5CzgkYZZZkPoKVYBu6tnDUcE9Zt")
            .expect("invalid address"),
        1729000000000,
        2000000000,
        16,
    )
    .set_valid_until(271828)
    .set_memo_str("Hello Mina!")
    .unwrap();

    // Serialize the transaction to JSON
    let tx_json = serde_json::to_string(&tx).expect("Failed to serialize transaction to JSON");

    // Deserialize the transaction from JSON
    let deserialized_tx: LegacyTransaction =
        serde_json::from_str(&tx_json).expect("Failed to deserialize transaction from JSON");

    // Now sign the deserialized transaction
    let tx_env = TransactionEnvelope::new_legacy(NetworkId::TESTNET, deserialized_tx.clone());
    let msg = tx_env.serialize().unwrap();

    let (sig, vk) = helper::sign_from_packages(&msg, shares, pubkey_package, &mut rng)
        .expect("Failed to sign message with FROST");

    // Convert signature to Mina format
    let mina_sig = frost_bluepallas::translate::translate_sig(&sig)
        .expect("Failed to translate FROST signature to Mina signature");
    let mina_vk = frost_bluepallas::translate::translate_pk(&vk)
        .expect("Failed to translate FROST verifying key to Mina public key");

    // Verify the signature using Mina Signer with TESTNET
    let mut ctx = mina_signer::create_legacy(NetworkId::TESTNET);
    let is_valid = ctx.verify(&mina_sig, &mina_vk, &tx_env);

    let mut ctx2 = mina_signer::create_legacy(NetworkId::TESTNET);
    let is_valid2 = ctx2.verify(&mina_sig, &mina_vk, &PallasMessage::new(msg.clone()));

    let mut ctx3 = mina_signer::create_legacy(NetworkId::TESTNET);
    let is_valid3 = ctx3.verify(&mina_sig, &mina_vk, &deserialized_tx);

    assert!(is_valid, "Mina signature verification failed on TESTNET");
    assert!(is_valid2, "Mina signature verification failed on TESTNET");
    assert!(is_valid3, "Mina signature verification failed on TESTNET");
}

#[test]
fn sign_mina_delegation_tx() {
    let mut rng = rand_core::OsRng;

    // Use trusted dealer to setup public and packages
    let max_signers = 5;
    let min_signers = 3;
    let (shares, pubkey_package) = frost_bluepallas::keys::generate_with_dealer(
        max_signers,
        min_signers,
        frost_bluepallas::keys::IdentifierList::Default,
        &mut rng,
    )
    .expect("Failed to generate key shares");

    // Create a delegation transaction
    let tx = LegacyTransaction::new_delegation(
        translate_pk(pubkey_package.verifying_key())
            .expect("failed to translate verifying key to Mina public key"),
        PubKey::from_address("B62qicipYxyEHu7QjUqS7QvBipTs5CzgkYZZZkPoKVYBu6tnDUcE9Zt")
            .expect("invalid address"),
        2000000000,
        16,
    )
    .set_valid_until(271828)
    .set_memo_str("Hello Mina!")
    .unwrap();

    // Generate FROST signature
    let tx_env = TransactionEnvelope::new_legacy(NetworkId::TESTNET, tx.clone());
    let msg = tx_env.serialize().unwrap();
    let (sig, vk) = helper::sign_from_packages(&msg, shares, pubkey_package, &mut rng)
        .expect("Failed to sign message with FROST");

    // Verify the signature
    let mina_sig = frost_bluepallas::translate::translate_sig(&sig)
        .expect("Failed to translate FROST signature to Mina signature");
    let mina_vk = frost_bluepallas::translate::translate_pk(&vk)
        .expect("Failed to translate FROST verifying key to Mina public key");

    // Verify the signature using Mina Signer
    let mut ctx = mina_signer::create_legacy(NetworkId::TESTNET);
    let is_valid_msg = ctx.verify(&mina_sig, &mina_vk, &PallasMessage::new(msg.clone()));

    let mut ctx2 = mina_signer::create_legacy(NetworkId::TESTNET);
    let is_valid_tx = ctx2.verify(&mina_sig, &mina_vk, &tx);

    assert!(is_valid_msg, "Mina signature verification (message) failed");
    assert!(
        is_valid_tx,
        "Mina delegation transaction verification failed"
    );
}

#[test]
fn delegation_json_deser_with_mina_sign() {
    let mut rng = rand_core::OsRng;

    // Use trusted dealer to setup public and packages
    let max_signers = 5;
    let min_signers = 3;
    let (shares, pubkey_package) = frost_bluepallas::keys::generate_with_dealer(
        max_signers,
        min_signers,
        frost_bluepallas::keys::IdentifierList::Default,
        &mut rng,
    )
    .expect("Failed to generate key shares");

    // Create a delegation transaction
    let tx = LegacyTransaction::new_delegation(
        translate_pk(pubkey_package.verifying_key())
            .expect("failed to translate verifying key to Mina public key"),
        PubKey::from_address("B62qicipYxyEHu7QjUqS7QvBipTs5CzgkYZZZkPoKVYBu6tnDUcE9Zt")
            .expect("invalid address"),
        2000000000,
        16,
    )
    .set_valid_until(271828)
    .set_memo_str("Hello Mina!")
    .unwrap();

    // Serialize the transaction to JSON (amount should be omitted for delegation)
    let tx_json = serde_json::to_string(&tx).expect("Failed to serialize delegation tx to JSON");

    // Deserialize the transaction from JSON
    let deserialized_tx: LegacyTransaction =
        serde_json::from_str(&tx_json).expect("Failed to deserialize delegation tx from JSON");

    // Now sign the deserialized transaction
    let tx_env = TransactionEnvelope::new_legacy(NetworkId::TESTNET, deserialized_tx.clone());
    let msg = tx_env.serialize().unwrap();
    let (sig, vk) = helper::sign_from_packages(&msg, shares, pubkey_package, &mut rng)
        .expect("Failed to sign message with FROST");

    // Convert signature to Mina format
    let mina_sig = frost_bluepallas::translate::translate_sig(&sig)
        .expect("Failed to translate FROST signature to Mina signature");
    let mina_vk = frost_bluepallas::translate::translate_pk(&vk)
        .expect("Failed to translate FROST verifying key to Mina public key");

    // Verify the signature using Mina Signer with TESTNET
    let mut ctx = mina_signer::create_legacy(NetworkId::TESTNET);
    let is_valid_tx = ctx.verify(&mina_sig, &mina_vk, &deserialized_tx);

    let mut ctx2 = mina_signer::create_legacy(NetworkId::TESTNET);
    let is_valid_msg = ctx2.verify(&mina_sig, &mina_vk, &PallasMessage::new(msg.clone()));

    assert!(
        is_valid_tx,
        "Mina delegation transaction verification failed on TESTNET"
    );
    assert!(
        is_valid_msg,
        "Mina signature verification (message) failed on TESTNET"
    );
}

#[test]
fn test_zkapp_tx_mina_signer_compatibility() {
    use frost_bluepallas::transactions::zkapp_tx::zkapp_test_vectors::get_zkapp_test_vectors;

    let mut rng = rand_core::OsRng;
    let test_vectors = get_zkapp_test_vectors();

    // Skip test if no vectors provided
    if test_vectors.is_empty() {
        println!("Warning: No test vectors provided for ZkApp transaction signing");
        return;
    }

    for test_vector in test_vectors {
        println!("Testing ZkApp transaction: {}", test_vector.name);

        // Use trusted dealer to setup public and packages
        let max_signers = 5;
        let min_signers = 3;
        let (shares, pubkey_package) = frost_bluepallas::keys::generate_with_dealer(
            max_signers,
            min_signers,
            frost_bluepallas::keys::IdentifierList::Default,
            &mut rng,
        )
        .unwrap_or_else(|_| {
            panic!(
                "Failed to generate key shares for test: {}",
                test_vector.name
            )
        });

        // Create transaction envelope with the ZkApp command
        let tx_env = TransactionEnvelope::new_zkapp(
            test_vector.network.clone(),
            test_vector.zkapp_command.clone(),
        );
        let msg = tx_env.serialize().unwrap_or_else(|_| {
            panic!(
                "Failed to serialize transaction envelope for test: {}",
                test_vector.name
            )
        });

        // Generate FROST signature
        let (sig, vk) = helper::sign_from_packages(&msg, shares, pubkey_package, &mut rng)
            .unwrap_or_else(|_| {
                panic!(
                    "Failed to sign message with FROST for test: {}",
                    test_vector.name
                )
            });

        // Convert signature to Mina format
        let mina_sig = frost_bluepallas::translate::translate_sig(&sig).unwrap_or_else(|_| {
            panic!(
                "Failed to translate FROST signature to Mina signature for test: {}",
                test_vector.name
            )
        });

        // Convert verifying key to Mina format
        let mina_vk = frost_bluepallas::translate::translate_pk(&vk).unwrap_or_else(|_| {
            panic!(
                "Failed to translate FROST verifying key to Mina public key for test: {}",
                test_vector.name
            )
        });

        // Verify the signature using Mina Signer
        let mut ctx = mina_signer::create_kimchi(test_vector.network.clone());
        let is_valid = ctx.verify(&mina_sig, &mina_vk, &tx_env);

        assert!(
            is_valid,
            "Mina signature verification failed for test: {}",
            test_vector.name
        );

        // Also verify against the deserialized transaction envelope
        let mut ctx2 = mina_signer::create_kimchi(test_vector.network.clone());
        let deserialized_tx_env = TransactionEnvelope::deserialize(&msg).unwrap_or_else(|_| {
            panic!(
                "Failed to deserialize transaction envelope for test: {}",
                test_vector.name
            )
        });
        let is_valid2 = ctx2.verify(&mina_sig, &mina_vk, &deserialized_tx_env);

        assert!(
            is_valid2,
            "Mina signature verification (deserialized) failed for test: {}",
            test_vector.name
        );

        // Verify against the raw message
        let mut ctx3 = mina_signer::create_kimchi(test_vector.network);
        let is_valid3 = ctx3.verify(&mina_sig, &mina_vk, &PallasMessage::new(msg.clone()));

        assert!(
            is_valid3,
            "Mina signature verification (raw message) failed for test: {}",
            test_vector.name
        );

        println!("✓ Test passed: {}", test_vector.name);
    }

    println!("All ZkApp transaction signing tests passed!");
}
