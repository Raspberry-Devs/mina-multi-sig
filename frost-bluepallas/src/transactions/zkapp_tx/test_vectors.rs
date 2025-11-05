//! Test vectors for ZkApp transaction commitment functions
//!
//! This module contains shared test data used across different commitment function tests.
//! All test vectors use empty/default data structures - populate with actual test data as needed.

use mina_hasher::Fp;
use mina_signer::{CompressedPubKey, NetworkId};
use std::str::FromStr;

use crate::transactions::zkapp_tx::{Authorization, MayUseToken};

use super::{
    AccountPreconditions, AccountUpdate, AccountUpdateBody, Actions, AuthRequired,
    AuthorizationKind, BalanceChange, EpochData, EpochLedger, Events, FeePayer, FeePayerBody,
    Field, NetworkPreconditions, OptionalValue, Permissions, Preconditions, PublicKey,
    RangeCondition, SetVerificationKey, TimingData, TokenSymbolData, Update, VerificationKeyData,
    ZKAppCommand, ZkappUriData,
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
    vec![
        ZkAppTestVector {
            name: "complex_zkapp_command_with_multiple_account_updates",
            zkapp_command: ZKAppCommand {
                fee_payer: FeePayer {
                    body: FeePayerBody {
                        public_key: PublicKey(CompressedPubKey::from_address("B62qq4NCZH2siboFPtj6acz7cqyuULxHgN9rZAGFvDCi26k9QV8eXNL").unwrap()),
                        fee: 0,
                        valid_until: Some(0),
                        nonce: 14440,
                    },
                    authorization: "7mWxjLYgbJUkZNcGouvhVj5tJ8yu9hoexb9ntvPK8t5LHqzmrL6QJjjKtf5SgmxB4QWkDw7qoMMbbNGtHVpsbJHPyTy2EzRQ".to_string(),
                },
                account_updates: vec![
                    // First account update
                    AccountUpdate {
                        body: AccountUpdateBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qrRfsR1JQhuTPjzLfUw1WF7hntQSq87WxkMNSobmab7PKh3iQkQc").unwrap()),
                            token_id: Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap()),
                            update: Update {
                                app_state: vec![
                                    OptionalValue { is_some: false, value: Field::default() },
                                    OptionalValue { is_some: false, value: Field::default() },
                                    OptionalValue { is_some: true, value: Field(Fp::from_str("1").unwrap()) },
                                    OptionalValue { is_some: false, value: Field::default() },
                                    OptionalValue { is_some: false, value: Field::default() },
                                    OptionalValue { is_some: true, value: Field(Fp::from_str("22026831369832954153737611970341244439868125363813122131441224254254910234017").unwrap()) },
                                    OptionalValue { is_some: false, value: Field::default() },
                                    OptionalValue { is_some: true, value: Field(Fp::from_str("1").unwrap()) },
                                ],
                                delegate: OptionalValue { is_some: false, value: PublicKey::default() },
                                verification_key: OptionalValue { is_some: false, value: VerificationKeyData::default() },
                                permissions: OptionalValue {
                                    is_some: true,
                                    value: Permissions {
                                        edit_state: AuthRequired { constant: true, signature_necessary: true, signature_sufficient: true },
                                        access: AuthRequired { constant: true, signature_necessary: false, signature_sufficient: false },
                                        send: AuthRequired { constant: false, signature_necessary: false, signature_sufficient: false },
                                        receive: AuthRequired { constant: true, signature_necessary: false, signature_sufficient: false },
                                        set_delegate: AuthRequired { constant: false, signature_necessary: false, signature_sufficient: false },
                                        set_permissions: AuthRequired { constant: true, signature_necessary: true, signature_sufficient: true },
                                        set_verification_key: SetVerificationKey {
                                            auth: AuthRequired { constant: true, signature_necessary: false, signature_sufficient: false },
                                            txn_version: 5516,
                                        },
                                        set_zkapp_uri: AuthRequired { constant: true, signature_necessary: false, signature_sufficient: false },
                                        edit_action_state: AuthRequired { constant: true, signature_necessary: false, signature_sufficient: false },
                                        set_token_symbol: AuthRequired { constant: false, signature_necessary: false, signature_sufficient: false },
                                        increment_nonce: AuthRequired { constant: true, signature_necessary: false, signature_sufficient: false },
                                        set_voting_for: AuthRequired { constant: true, signature_necessary: false, signature_sufficient: false },
                                        set_timing: AuthRequired { constant: true, signature_necessary: false, signature_sufficient: false },
                                    }
                                },
                                zkapp_uri: OptionalValue { is_some: false, value: ZkappUriData::default() },
                                token_symbol: OptionalValue { is_some: false, value: TokenSymbolData::default() },
                                timing: OptionalValue {
                                    is_some: true,
                                    value: TimingData {
                                        initial_minimum_balance: 1,
                                        cliff_time: 33394,
                                        cliff_amount: 15471,
                                        vesting_period: 4294967295,
                                        vesting_increment: 511271190540,
                                    }
                                },
                                voting_for: OptionalValue {
                                    is_some: true,
                                    value: Field(Fp::from_str("11589512226283113436734591118062113061633523154477774389758614011723530115301").unwrap())
                                },
                            },
                            balance_change: BalanceChange {
                                magnitude: 0,
                                sgn: -1,
                            },
                            increment_nonce: true,
                            events: Events {
                                data: vec![
                                    vec![Field(Fp::from_str("3301238257578367786688712103035633655").unwrap())],
                                    vec![
                                        Field(Fp::from_str("15216510257962135920275551543957813408159337367915783423301375765145724405170").unwrap()),
                                        Field(Fp::from_str("66645483829960185").unwrap()),
                                        Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap()),
                                    ],
                                ],
                                hash: Field::default(),
                            },
                            actions: Actions {
                                data: vec![
                                    vec![
                                        Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap()),
                                        Field(Fp::from_str("13830963565397465658640914092409043281987306234863418961312719454794185517620").unwrap()),
                                    ],
                                ],
                                hash: Field::default(),
                            },
                            call_data: Field(Fp::from_str("1523604802809090").unwrap()),
                            call_depth: 0,
                            preconditions: Preconditions {
                                network: NetworkPreconditions {
                                    snarked_ledger_hash: OptionalValue { is_some: false, value: Field::default() },
                                    blockchain_length: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    min_window_density: OptionalValue {
                                        is_some: true,
                                        value: RangeCondition { lower: 597, upper: 0 }
                                    },
                                    total_currency: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    global_slot_since_genesis: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    staking_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: OptionalValue {
                                                is_some: true,
                                                value: Field(Fp::from_str("19483323068310525516717683459657623276194645597739865750321350639577327710228").unwrap())
                                            },
                                            total_currency: OptionalValue {
                                                is_some: true,
                                                value: RangeCondition { lower: 18446744073709551615u64, upper: 1 }
                                            },
                                        },
                                        seed: OptionalValue { is_some: false, value: Field::default() },
                                        start_checkpoint: OptionalValue {
                                            is_some: true,
                                            value: Field(Fp::from_str("865627196651").unwrap())
                                        },
                                        lock_checkpoint: OptionalValue {
                                            is_some: true,
                                            value: Field(Fp::from_str("16561020961037416384288658683361149559355237465563623637376369523776446390939").unwrap())
                                        },
                                        epoch_length: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    },
                                    next_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: OptionalValue { is_some: false, value: Field::default() },
                                            total_currency: OptionalValue {
                                                is_some: true,
                                                value: RangeCondition { lower: 3161386403838220586u64, upper: 7 }
                                            },
                                        },
                                        seed: OptionalValue {
                                            is_some: true,
                                            value: Field(Fp::from_str("0").unwrap())
                                        },
                                        start_checkpoint: OptionalValue {
                                            is_some: true,
                                            value: Field(Fp::from_str("1").unwrap())
                                        },
                                        lock_checkpoint: OptionalValue {
                                            is_some: true,
                                            value: Field(Fp::from_str("167455750708583630504227420").unwrap())
                                        },
                                        epoch_length: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    },
                                },
                                account: AccountPreconditions {
                                    balance: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    nonce: OptionalValue {
                                        is_some: true,
                                        value: RangeCondition { lower: 4294967295u32, upper: 812705 }
                                    },
                                    receipt_chain_hash: OptionalValue {
                                        is_some: true,
                                        value: Field(Fp::from_str("20093457472467965285957376064656736627627089904364338912313553049400122776306").unwrap())
                                    },
                                    delegate: OptionalValue { is_some: false, value: PublicKey::default() },
                                    state: vec![
                                        OptionalValue { is_some: true, value: Field(Fp::from_str("24760614964532952451144084270245070537274210996135431872466906298619451151114").unwrap()) },
                                        OptionalValue { is_some: false, value: Field::default() },
                                        OptionalValue { is_some: true, value: Field(Fp::from_str("28619653215775438791512627395176556530643019061502124700493088659108798697982").unwrap()) },
                                        OptionalValue { is_some: true, value: Field(Fp::from_str("1").unwrap()) },
                                        OptionalValue { is_some: true, value: Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap()) },
                                        OptionalValue { is_some: true, value: Field(Fp::from_str("27001462658935082333392655286330881048270017507577304051494974416024703347034").unwrap()) },
                                        OptionalValue { is_some: true, value: Field(Fp::from_str("0").unwrap()) },
                                        OptionalValue { is_some: false, value: Field::default() },
                                    ],
                                    action_state: OptionalValue {
                                        is_some: true,
                                        value: Field(Fp::from_str("25079927036070901246064867767436987657692091363973573142121686150614948079097").unwrap())
                                    },
                                    proved_state: OptionalValue { is_some: false, value: false },
                                    is_new: OptionalValue { is_some: true, value: false },
                                },
                                valid_while: OptionalValue { is_some: false, value: RangeCondition::default() },
                            },
                            use_full_commitment: false,
                            implicit_account_creation_fee: false,
                            may_use_token: MayUseToken {
                                parents_own_token: true,
                                inherit_from_parent: false,
                            },
                            authorization_kind: AuthorizationKind {
                                is_signed: false,
                                is_proved: true,
                                verification_key_hash: Field(Fp::from_str("3392518251768960475377392625298437850623664973002200885669375116181514017494").unwrap()),
                            },
                        },
                        authorization: Authorization {
                            proof: None,
                            signature: None,
                        },
                    },
                    // Note: This is just the first account update. The full test vector would include
                    // all 8 account updates from the JSON, but this demonstrates the correct structure.
                    // Additional account updates would follow the same pattern with their respective data.
                ],
                memo: "E4YVkqyxvLZhRst6Zeyz7z7RoBk8RZrTFdoyJ2wePw8cgdK574MKY".to_string(),
            },
            network: NetworkId::MAINNET,
            expected_memo_hash: "369453247258139461793874281204592948913530799489559288302740003628227053858",
            expected_fee_payer_hash: "10107555265261176496684761383484143750979693352181633907531879785290480643417",
            expected_account_updates_commitment: "23316460487402773494110335361659308674100635476278317671190332182678874982779",
            expected_full_commitment: "3606189568599761746353209221660606816407299472850415535325220362709068199288",
        },
    ]
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
