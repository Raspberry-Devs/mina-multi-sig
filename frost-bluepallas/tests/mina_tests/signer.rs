#![allow(clippy::needless_borrows_for_generic_args)]

use frost_bluepallas::{
    hasher::{set_network_id, PallasMessage},
    helper,
    transactions::Transaction,
    translate::{translate_minask, translate_msg, translate_pk},
};
use frost_core::Ciphersuite;
use mina_signer::{Keypair, NetworkId, PubKey, Signer};

#[cfg(test)]
#[test]
fn signer_test_raw() {
    use frost_bluepallas::transactions::Transaction;

    set_network_id(NetworkId::TESTNET).expect("Failed to set network ID");

    let kp = Keypair::from_hex("164244176fddb5d769b7de2027469d027ad428fadcc0c02396e6280142efb718")
        .expect("failed to create keypair");
    let tx = Transaction::new_payment(
        kp.public.clone(),
        PubKey::from_address("B62qicipYxyEHu7QjUqS7QvBipTs5CzgkYZZZkPoKVYBu6tnDUcE9Zt")
            .expect("invalid address"),
        1729000000000,
        2000000000,
        16,
    )
    .set_valid_until(271828)
    .set_memo_str("Hello Mina!");

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
    let msg = translate_msg(&tx);
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
    let mut ctx = mina_signer::create_legacy(NetworkId::TESTNET);
    let is_valid = ctx.verify(&mina_sig, &mina_vk, &PallasMessage::new(msg.clone()));

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
    let tx = Transaction::new_payment(
        translate_pk(pubkey_package.verifying_key())
            .expect("failed to translate verifying key to Mina public key"),
        PubKey::from_address("B62qicipYxyEHu7QjUqS7QvBipTs5CzgkYZZZkPoKVYBu6tnDUcE9Zt")
            .expect("invalid address"),
        1729000000000,
        2000000000,
        16,
    )
    .set_valid_until(271828)
    .set_memo_str("Hello Mina!");

    // Generate FROST signature
    let msg = translate_msg(&tx);
    let (sig, vk) = helper::sign_from_packages(&msg, shares, pubkey_package, &mut rng)
        .expect("Failed to sign message with FROST");

    // Verify the signature
    let mina_sig = frost_bluepallas::translate::translate_sig(&sig)
        .expect("Failed to translate FROST signature to Mina signature");
    let mina_vk = frost_bluepallas::translate::translate_pk(&vk)
        .expect("Failed to translate FROST verifying key to Mina public key");

    // Verify the signature using Mina Signer
    let mut ctx = mina_signer::create_legacy(NetworkId::TESTNET);
    let is_valid = ctx.verify(&mina_sig, &mina_vk, &PallasMessage::new(msg.clone()));
    let mut ctx2 = mina_signer::create_legacy(NetworkId::TESTNET);
    let is_valid_tx = ctx2.verify(&mina_sig, &mina_vk, &tx);

    assert!(is_valid, "Mina signature verification failed");
    assert!(is_valid_tx, "Mina transaction verification failed");
}

#[test]
fn sign_mina_tx_mainnet() {
    let mut rng = rand_core::OsRng;

    // Set network id to Mainnet
    set_network_id(NetworkId::MAINNET).expect("Failed to set network ID");

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
    let tx = Transaction::new_payment(
        translate_pk(pubkey_package.verifying_key())
            .expect("failed to translate verifying key to Mina public key"),
        PubKey::from_address("B62qicipYxyEHu7QjUqS7QvBipTs5CzgkYZZZkPoKVYBu6tnDUcE9Zt")
            .expect("invalid address"),
        1500000000000, // Different amount
        1000000000,    // Different fee
        10,            // Different nonce
    )
    .set_valid_until(300000)
    .set_memo_str("Mainnet Test!");

    // Generate FROST signature
    let msg = translate_msg(&tx);
    let (sig, vk) = helper::sign_from_packages(&msg, shares, pubkey_package, &mut rng)
        .expect("Failed to sign message with FROST");

    let _chall = frost_bluepallas::PallasPoseidon::challenge(sig.R(), &vk, &msg)
        .expect("Expect challenge to calculate");

    // Convert signature to Mina format
    let mina_sig = frost_bluepallas::translate::translate_sig(&sig)
        .expect("Failed to translate FROST signature to Mina signature");
    let mina_vk = frost_bluepallas::translate::translate_pk(&vk)
        .expect("Failed to translate FROST verifying key to Mina public key");

    // Verify the signature using Mina Signer with MAINNET
    let mut ctx = mina_signer::create_legacy(NetworkId::MAINNET);
    let is_valid = ctx.verify(&mina_sig, &mina_vk, &PallasMessage::new(msg.clone()));

    assert!(is_valid, "Mina signature verification failed on MAINNET");
}
