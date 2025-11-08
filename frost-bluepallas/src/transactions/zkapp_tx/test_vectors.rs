//! Test vectors for ZkApp transaction commitment functions
//!
//! This module contains shared test data used across different commitment function tests.
//! All test vectors use empty/default data structures - populate with actual test data as needed.

use bs58;
use mina_hasher::Fp;
use mina_signer::{CompressedPubKey, NetworkId};
use std::str::FromStr;

use crate::transactions::zkapp_tx::{Authorization, MayUseToken};

use super::{
    AccountPreconditions, AccountUpdate, AccountUpdateBody, Actions, AuthRequired,
    AuthorizationKind, BalanceChange, EpochData, EpochLedger, Events, FeePayer, FeePayerBody,
    Field, NetworkPreconditions, Permissions, Preconditions, PublicKey, RangeCondition,
    SetVerificationKey, TimingData, Update, ZKAppCommand,
};

/// Comprehensive test vector containing all data needed for commitment function tests
#[derive(Clone)]
pub struct ZkAppTestVector {
    /// Name/description of the test case
    pub name: &'static str,
    /// ZKAppCommand to test
    pub zkapp_command: ZKAppCommand,
    /// Network to use for the test
    pub network: NetworkId,
    /// Expected hash_with_prefix result for memo
    pub expected_memo_hash: &'static str,
    /// Expected fee_payer_hash result
    pub expected_fee_payer_hash: &'static str,
    /// Expected account updates commitment from zk_commit
    pub expected_account_updates_commitment: &'static str,
    /// Expected full commitment from zk_commit
    pub expected_full_commitment: &'static str,
}

/// Additional test vectors specifically for hash_with_prefix function
#[derive(Clone)]
pub struct HashWithPrefixTestVector {
    /// Name/description of the test case
    pub name: &'static str,
    /// Prefix string to use
    pub prefix: &'static str,
    /// Input field elements
    pub input_fields: Vec<Fp>,
    /// Expected hash result (as string for parsing)
    pub expected_hash: &'static str,
}

/// Returns the main test vectors for ZkApp commitment functions
pub fn get_zkapp_test_vectors() -> Vec<ZkAppTestVector> {
    vec![ZkAppTestVector {
            name: "complex_zkapp_command",
            zkapp_command: ZKAppCommand {
                fee_payer: FeePayer {
                    body: FeePayerBody {
                        public_key: PublicKey(CompressedPubKey::from_address("B62qoKnzqYiFCNrgq8zbSngbwBHXYs9mxfC175RfveP4ePmwEjJqZXv").unwrap()),
                        fee: 7424713804498,
                        valid_until: Some(1),
                        nonce: 7140668,
                    },
                    authorization: "7mWxjLYgbJUkZNcGouvhVj5tJ8yu9hoexb9ntvPK8t5LHqzmrL6QJjjKtf5SgmxB4QWkDw7qoMMbbNGtHVpsbJHPyTy2EzRQ".to_string(),
                },
                account_updates: vec![
                    // Account update 1
                    AccountUpdate {
                        body: AccountUpdateBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qnyC9HBS1S3E4D5z3dQcvh8rMcmj3Z1cEtb9Bouwr5A97gNtgVba").unwrap()),
                            token_id: Field(Fp::from_str("1").unwrap()),
                            update: Update {
                                app_state: vec![
                                    None,
                                    Some(Field(Fp::from_str("1").unwrap())),
                                    None,
                                    Some(Field(Fp::from_str("20119941786739084581315302011557922441170738867231423668630802116858479137879").unwrap())),
                                    None,
                                    Some(Field(Fp::from_str("2569355482979197506241957729043365").unwrap())),
                                    None,
                                    None,
                                ],
                                delegate: None,
                                verification_key: None,
                                permissions: Some(Permissions {
                                        edit_state: AuthRequired::Either,
                                        access: AuthRequired::None,
                                        send: AuthRequired::Signature,
                                        receive: AuthRequired::Impossible,
                                        set_delegate: AuthRequired::None,
                                        set_permissions: AuthRequired::Signature,
                                        set_verification_key: SetVerificationKey {
                                            auth: AuthRequired::Impossible,
                                            txn_version: 8,
                                        },
                                        set_zkapp_uri: AuthRequired::Proof,
                                        edit_action_state: AuthRequired::Impossible,
                                        set_token_symbol: AuthRequired::Signature,
                                        increment_nonce: AuthRequired::Impossible,
                                        set_voting_for: AuthRequired::None,
                                        set_timing: AuthRequired::Proof,
                                    }),
                                zkapp_uri: None,
                                token_symbol: None,
                                timing: Some(TimingData {
                                        initial_minimum_balance: 6178085106025133285,
                                        cliff_time: 12803905,
                                        cliff_amount: 282640072025782999,
                                        vesting_period: 4428622,
                                        vesting_increment: 1691804339219960,
                                    }),
                                voting_for: Some(Field(Fp::from_str("123684910471235457312506695096").unwrap())),
                            },
                            balance_change: BalanceChange {
                                magnitude: 4,
                                sgn: 1,
                            },
                            increment_nonce: false,
                            events: Events {
                                data: vec![
                                    vec![Field(Fp::from_str("498485108934236").unwrap())],
                                    vec![Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap()), Field(Fp::from_str("1141825960530746").unwrap()), Field(Fp::from_str("24329923703056195309097959984771797906187233883508490084880561800601469657934").unwrap()), Field(Fp::from_str("4757528981762345997813821785801003615196279596301953087097640494776895725263").unwrap()), Field(Fp::from_str("34882406696181291554878142460").unwrap())],
                                ],
                                hash: Field::default(),
                            },
                            actions: Actions {
                                data: vec![
                                    vec![Field(Fp::from_str("5411145").unwrap()), Field(Fp::from_str("0").unwrap()), Field(Fp::from_str("13999190644825397199034540171899518447507909334229615107446105734957449528413").unwrap())],
                                ],
                                hash: Field::default(),
                            },
                            call_data: Field(Fp::from_str("1").unwrap()),
                            call_depth: 0,
                            preconditions: Preconditions {
                                network: NetworkPreconditions {
                                    snarked_ledger_hash: Some(Field(Fp::from_str("4160865316201165579317331623933110380444514181416746444152338203186623745748").unwrap())),
                                    blockchain_length: Some(RangeCondition { lower: 15874, upper: 1 }),
                                    min_window_density: None,
                                    total_currency: Some(RangeCondition { lower: 18446744073709551615, upper: 114845785285630 }),
                                    global_slot_since_genesis: None,
                                    staking_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: None,
                                            total_currency: Some(RangeCondition { lower: 2, upper: 15059377 }),
                                        },
                                        seed: Some(Field(Fp::from_str("17065656153764100170783243096089907708319259057648109125868667018680792344053").unwrap())),
                                        start_checkpoint: Some(Field(Fp::from_str("25561007885830281804586374927013510550224596934294111472278672711092738202432").unwrap())),
                                        lock_checkpoint: Some(Field(Fp::from_str("1").unwrap())),
                                        epoch_length: None,
                                    },
                                    next_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: None,
                                            total_currency: Some(RangeCondition { lower: 20720661, upper: 0 }),
                                        },
                                        seed: Some(Field(Fp::from_str("20382163946209849588013964060581359560692577465928892829735869194910526880729").unwrap())),
                                        start_checkpoint: None,
                                        lock_checkpoint: Some(Field(Fp::from_str("16405049556699626286597135691111157690090590928681808724777637837397962298413").unwrap())),
                                        epoch_length: Some(RangeCondition { lower: 145821, upper: 2 }),
                                    },
                                },
                                account: AccountPreconditions {
                                    balance: None,
                                    nonce: Some(RangeCondition { lower: 0, upper: 1162 }),
                                    receipt_chain_hash: None,
                                    delegate: Some(PublicKey(CompressedPubKey::from_address("B62qmcPDgMWATVotfzALywgyAFnTmg2kQm86LdicWxnf4VBjyGWnH52").unwrap())),
                                    state: vec![
                                            None,
                                            Some(Field(Fp::from_str("3879451311908553236192469318842473458747646845541652927115209281249630476696").unwrap())),
                                            None,
                                            Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())),
                                            None,
                                            None,
                                            None,
                                            None,
                                        ],
                                    action_state: Some(Field(Fp::from_str("12513521410028455860379735423848988282379114445190709725768084747642141047963").unwrap())),
                                    proved_state: None,
                                    is_new: None,
                                },
                                valid_while: None,
                            },
                            use_full_commitment: false,
                            implicit_account_creation_fee: true,
                            may_use_token: MayUseToken {
                                parents_own_token: false,
                                inherit_from_parent: true,
                            },
                            authorization_kind: AuthorizationKind {
                                is_signed: true,
                                is_proved: false,
                                verification_key_hash: Field(Fp::from_str("3392518251768960475377392625298437850623664973002200885669375116181514017494").unwrap()),
                            },
                        },
                        authorization: Authorization {
                            proof: None,
                            signature: None,
                        },
                    },
                ],
                memo: {
                    // base58 decode of original memo string; ensure bs58 = "0.4" in Cargo.toml
                    let decoded = bs58::decode("E4YTgPrTgzu8oPBNaMSZTzTurrPRpFQ8cP2hgLCAJD3h64oDbxzMj")
                        .into_vec()
                        .expect("invalid base58 memo");
                    String::from_utf8_lossy(&decoded).to_string()
                },
            },
            network: NetworkId::MAINNET,
            expected_memo_hash: "7058328916357264459393113209042072363968938759462457191302163089925033650967",
            expected_fee_payer_hash: "23266113660560458530300204899910304152144943649268975906345136134187634114743",
            expected_account_updates_commitment: "28758693545720568902829770225552807981665617078386073531248463565309362381334",
            expected_full_commitment: "19160795306106068369715028799222959710783075923317163435087167380705433226448",
        }]
}

/// Returns additional test vectors for hash_with_prefix function
/// TODO: Populate with actual test data
pub fn get_hash_with_prefix_test_vectors() -> Vec<HashWithPrefixTestVector> {
    vec![HashWithPrefixTestVector {
        name: "mina_acct_update_node",
        prefix: "MinaAcctUpdateNode",
        input_fields: vec![
            Fp::from_str(
                "23487734643675003113914430489774334948844391842009122040704261138931555665056",
            )
            .unwrap(),
            Fp::from_str("0").unwrap(),
        ],
        expected_hash:
            "20456728518925904340727370305821489989002971473792411299271630913563245218671",
    }]
}

/// Helper function to parse expected hash strings into Fp elements
pub fn parse_expected_hash(hash_str: &str) -> Fp {
    Fp::from_str(hash_str).expect("Invalid expected hash format")
}
