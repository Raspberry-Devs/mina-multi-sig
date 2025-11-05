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
            name: "complex_zkapp_command",
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
// Account update 1
                    AccountUpdate {
                        body: AccountUpdateBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qrRfsR1JQhuTPjzLfUw1WF7hntQSq87WxkMNSobmab7PKh3iQkQc").unwrap()),
                            token_id: Field(Fp::from_str("17527450560908283403452646309271595298869612331227713511090959271017019787571").unwrap()),
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
                                voting_for: OptionalValue { is_some: true, value: Field(Fp::from_str("3NLmnffaFCyKRBLvXtWvjwoSZtiR4QYecxkkyMRqikWxGqVv1FF7").unwrap()) },
                            },
                            balance_change: BalanceChange {
                                magnitude: 0,
                                sgn: -1,
                            },
                            increment_nonce: true,
                            events: Events {
                                data: vec![
                                    vec![Field(Fp::from_str("3301238257578367786688712103035633655").unwrap())],
                                    vec![Field(Fp::from_str("15216510257962135920275551543957813408159337367915783423301375765145724405170").unwrap()), Field(Fp::from_str("66645483829960185").unwrap()), Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())],
                                ],
                                hash: Field::default(),
                            },
                            actions: Actions {
                                data: vec![
                                    vec![Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap()), Field(Fp::from_str("13830963565397465658640914092409043281987306234863418961312719454794185517620").unwrap())],
                                ],
                                hash: Field::default(),
                            },
                            call_data: Field(Fp::from_str("1523604802809090").unwrap()),
                            call_depth: 0,
                            preconditions: Preconditions {
                                network: NetworkPreconditions {
                                    snarked_ledger_hash: OptionalValue { is_some: false, value: Field::default() },
                                    blockchain_length: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    min_window_density: OptionalValue { is_some: true, value: RangeCondition { lower: 597u32, upper: 0u32 } },
                                    total_currency: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    global_slot_since_genesis: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    staking_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: OptionalValue { is_some: true, value: Field(Fp::from_str("19483323068310525516717683459657623276194645597739865750321350639577327710228").unwrap()) },
                                            total_currency: OptionalValue { is_some: true, value: RangeCondition { lower: 18446744073709551615u64, upper: 1u64 } },
                                        },
                                        seed: OptionalValue { is_some: false, value: Field::default() },
                                        start_checkpoint: OptionalValue { is_some: true, value: Field(Fp::from_str("865627196651").unwrap()) },
                                        lock_checkpoint: OptionalValue { is_some: true, value: Field(Fp::from_str("16561020961037416384288658683361149559355237465563623637376369523776446390939").unwrap()) },
                                        epoch_length: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    },
                                    next_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: OptionalValue { is_some: false, value: Field::default() },
                                            total_currency: OptionalValue { is_some: true, value: RangeCondition { lower: 3161386403838220586u64, upper: 7u64 } },
                                        },
                                        seed: OptionalValue { is_some: true, value: Field(Fp::from_str("0").unwrap()) },
                                        start_checkpoint: OptionalValue { is_some: true, value: Field(Fp::from_str("1").unwrap()) },
                                        lock_checkpoint: OptionalValue { is_some: true, value: Field(Fp::from_str("167455750708583630504227420").unwrap()) },
                                        epoch_length: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    },
                                },
                                account: AccountPreconditions {
                                    balance: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    nonce: OptionalValue { is_some: true, value: RangeCondition { lower: 4294967295u32, upper: 812705u32 } },
                                    receipt_chain_hash: OptionalValue { is_some: true, value: Field(Fp::from_str("20093457472467965285957376064656736627627089904364338912313553049400122776306").unwrap()) },
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
                                    action_state: OptionalValue { is_some: true, value: Field(Fp::from_str("25079927036070901246064867767436987657692091363973573142121686150614948079097").unwrap()) },
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
                    },// Account update 2
                    AccountUpdate {
                        body: AccountUpdateBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qnQnd5eskWPSARt2YPfg5n3FEZMtf1xzz5ZX3Y4zAWahjyP9Xzkm").unwrap()),
                            token_id: Field(Fp::from_str("1").unwrap()),
                            update: Update {
                                app_state: vec![
                                    OptionalValue { is_some: false, value: Field::default() },
                                    OptionalValue { is_some: false, value: Field::default() },
                                    OptionalValue { is_some: false, value: Field::default() },
                                    OptionalValue { is_some: false, value: Field::default() },
                                    OptionalValue { is_some: false, value: Field::default() },
                                    OptionalValue { is_some: true, value: Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap()) },
                                    OptionalValue { is_some: true, value: Field(Fp::from_str("1042455263212066954316135558936").unwrap()) },
                                    OptionalValue { is_some: true, value: Field(Fp::from_str("19848073").unwrap()) },
                                ],
                                delegate: OptionalValue { is_some: true, value: PublicKey(CompressedPubKey::from_address("B62qmDC42QjqdHch36LfCaYf54BPvgHtsDZKRKN9skeZMS5CEbNiatk").unwrap()) },
                                verification_key: OptionalValue { is_some: false, value: VerificationKeyData::default() },
                                permissions: OptionalValue { is_some: false, value: Permissions::default() },
                                zkapp_uri: OptionalValue { is_some: false, value: ZkappUriData::default() },
                                token_symbol: OptionalValue { is_some: true, value: TokenSymbolData::default() },
                                timing: OptionalValue {
                                    is_some: true,
                                    value: TimingData {
                                        initial_minimum_balance: 1,
                                        cliff_time: 566,
                                        cliff_amount: 0,
                                        vesting_period: 0,
                                        vesting_increment: 117096718209304131,
                                    }
                                },
                                voting_for: OptionalValue { is_some: true, value: Field(Fp::from_str("3NLJMHuP8j1Riuw4UmkTm73igAPwB49dQ7gCH63WKWHsBdB8dtCA").unwrap()) },
                            },
                            balance_change: BalanceChange {
                                magnitude: 90512307498715496,
                                sgn: -1,
                            },
                            increment_nonce: true,
                            events: Events {
                                data: vec![],
                                hash: Field::default(),
                            },
                            actions: Actions {
                                data: vec![
                                    vec![Field(Fp::from_str("3705192212466683449368204265740293307876013973485051691184604938253344489642").unwrap()), Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap()), Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap()), Field(Fp::from_str("19087763264793489769785385419863505176941296478888527865130369317678918617200").unwrap()), Field(Fp::from_str("921095295063572708403175").unwrap())],
                                ],
                                hash: Field::default(),
                            },
                            call_data: Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap()),
                            call_depth: 0,
                            preconditions: Preconditions {
                                network: NetworkPreconditions {
                                    snarked_ledger_hash: OptionalValue { is_some: false, value: Field::default() },
                                    blockchain_length: OptionalValue { is_some: true, value: RangeCondition { lower: 0u32, upper: 279764u32 } },
                                    min_window_density: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    total_currency: OptionalValue { is_some: true, value: RangeCondition { lower: 52590806508u64, upper: 34060898627649188u64 } },
                                    global_slot_since_genesis: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    staking_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: OptionalValue { is_some: false, value: Field::default() },
                                            total_currency: OptionalValue { is_some: true, value: RangeCondition { lower: 28227u64, upper: 883u64 } },
                                        },
                                        seed: OptionalValue { is_some: false, value: Field::default() },
                                        start_checkpoint: OptionalValue { is_some: true, value: Field(Fp::from_str("2825445788796665362001888968889410748227869566922001878876430464639834795996").unwrap()) },
                                        lock_checkpoint: OptionalValue { is_some: false, value: Field::default() },
                                        epoch_length: OptionalValue { is_some: true, value: RangeCondition { lower: 39u32, upper: 7389u32 } },
                                    },
                                    next_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: OptionalValue { is_some: false, value: Field::default() },
                                            total_currency: OptionalValue { is_some: false, value: RangeCondition::default() },
                                        },
                                        seed: OptionalValue { is_some: true, value: Field(Fp::from_str("291951363258746800521592").unwrap()) },
                                        start_checkpoint: OptionalValue { is_some: true, value: Field(Fp::from_str("27097426921841235475915828190561706782719634733775923521820407752992000450368").unwrap()) },
                                        lock_checkpoint: OptionalValue { is_some: true, value: Field(Fp::from_str("94453716439919024715574214318891883448929636105233743916484057526553302855").unwrap()) },
                                        epoch_length: OptionalValue { is_some: true, value: RangeCondition { lower: 3216u32, upper: 969950059u32 } },
                                    },
                                },
                                account: AccountPreconditions {
                                    balance: OptionalValue { is_some: true, value: RangeCondition { lower: 2100u64, upper: 651384926222364u64 } },
                                    nonce: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    receipt_chain_hash: OptionalValue { is_some: false, value: Field::default() },
                                    delegate: OptionalValue { is_some: false, value: PublicKey::default() },
                                    state: vec![
                                            OptionalValue { is_some: false, value: Field::default() },
                                            OptionalValue { is_some: true, value: Field(Fp::from_str("27867280189830096928649753592265238510052021355932590900792574644309736546672").unwrap()) },
                                            OptionalValue { is_some: false, value: Field::default() },
                                            OptionalValue { is_some: true, value: Field(Fp::from_str("0").unwrap()) },
                                            OptionalValue { is_some: false, value: Field::default() },
                                            OptionalValue { is_some: false, value: Field::default() },
                                            OptionalValue { is_some: false, value: Field::default() },
                                            OptionalValue { is_some: false, value: Field::default() },
                                        ],
                                    action_state: OptionalValue { is_some: true, value: Field(Fp::from_str("25079927036070901246064867767436987657692091363973573142121686150614948079097").unwrap()) },
                                    proved_state: OptionalValue { is_some: false, value: false },
                                    is_new: OptionalValue { is_some: false, value: false },
                                },
                                valid_while: OptionalValue { is_some: true, value: RangeCondition { lower: 1043u32, upper: 927u32 } },
                            },
                            use_full_commitment: false,
                            implicit_account_creation_fee: false,
                            may_use_token: MayUseToken {
                                parents_own_token: false,
                                inherit_from_parent: false,
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
                    },// Account update 3
                    AccountUpdate {
                        body: AccountUpdateBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qmwhiXeyySsr7xWHhgob1FojqFCXExemvPaBzefihJq7zF4onswW").unwrap()),
                            token_id: Field(Fp::from_str("1").unwrap()),
                            update: Update {
                                app_state: vec![
                                    OptionalValue { is_some: false, value: Field::default() },
                                    OptionalValue { is_some: true, value: Field(Fp::from_str("28643886398241429206974909384179051783497825102418223685790672254864597476716").unwrap()) },
                                    OptionalValue { is_some: false, value: Field::default() },
                                    OptionalValue { is_some: true, value: Field(Fp::from_str("1150737679081066842964613151823").unwrap()) },
                                    OptionalValue { is_some: false, value: Field::default() },
                                    OptionalValue { is_some: true, value: Field(Fp::from_str("0").unwrap()) },
                                    OptionalValue { is_some: false, value: Field::default() },
                                    OptionalValue { is_some: false, value: Field::default() },
                                ],
                                delegate: OptionalValue { is_some: true, value: PublicKey(CompressedPubKey::from_address("B62qobxVLbLLqpdd41ZkBtQZ4C8rwuGxqkLrwMdS3jhoEie7jVWeEe9").unwrap()) },
                                verification_key: OptionalValue { is_some: false, value: VerificationKeyData::default() },
                                permissions: OptionalValue { is_some: false, value: Permissions::default() },
                                zkapp_uri: OptionalValue { is_some: true, value: ZkappUriData::default() },
                                token_symbol: OptionalValue { is_some: false, value: TokenSymbolData::default() },
                                timing: OptionalValue { is_some: false, value: TimingData::default() },
                                voting_for: OptionalValue { is_some: true, value: Field(Fp::from_str("3NLsUE3F9yUwchnA9eDLTixkSE22PQcBiWgrEHBfQvd4TPGrBGA3").unwrap()) },
                            },
                            balance_change: BalanceChange {
                                magnitude: 2,
                                sgn: -1,
                            },
                            increment_nonce: false,
                            events: Events {
                                data: vec![
                                    vec![Field(Fp::from_str("12931957376928697859").unwrap()), Field(Fp::from_str("5849329341512681317823695155736997520327243059695247875299622451606490882452").unwrap())],
                                    vec![Field(Fp::from_str("14816239472201483433804862228907274478773973663705041534715996154206776821281").unwrap()), Field(Fp::from_str("11805595744089006183224499129972161453949700733977767001725427644218065520730").unwrap()), Field(Fp::from_str("1").unwrap()), Field(Fp::from_str("24282105584753328388446010820743372748995444582596069527693257761141101806290").unwrap()), Field(Fp::from_str("28373509369208330455857688966487597390616475982565721300169662732715432093677").unwrap())],
                                ],
                                hash: Field::default(),
                            },
                            actions: Actions {
                                data: vec![
                                    vec![Field(Fp::from_str("16571046021669510874340605996719031661382259508768440674726166858726856476332").unwrap()), Field(Fp::from_str("0").unwrap())],
                                    vec![Field(Fp::from_str("24960222756119922797971232995193716868950338994361948285492726755734677427954").unwrap()), Field(Fp::from_str("1981647075950203256186583143458938425584094178126450698900206068822894987467").unwrap()), Field(Fp::from_str("0").unwrap())],
                                ],
                                hash: Field::default(),
                            },
                            call_data: Field(Fp::from_str("1").unwrap()),
                            call_depth: 0,
                            preconditions: Preconditions {
                                network: NetworkPreconditions {
                                    snarked_ledger_hash: OptionalValue { is_some: true, value: Field(Fp::from_str("1").unwrap()) },
                                    blockchain_length: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    min_window_density: OptionalValue { is_some: true, value: RangeCondition { lower: 4294967295u32, upper: 577u32 } },
                                    total_currency: OptionalValue { is_some: true, value: RangeCondition { lower: 176633269u64, upper: 51718u64 } },
                                    global_slot_since_genesis: OptionalValue { is_some: true, value: RangeCondition { lower: 69543u32, upper: 226u32 } },
                                    staking_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: OptionalValue { is_some: false, value: Field::default() },
                                            total_currency: OptionalValue { is_some: true, value: RangeCondition { lower: 11285887u64, upper: 0u64 } },
                                        },
                                        seed: OptionalValue { is_some: true, value: Field(Fp::from_str("1").unwrap()) },
                                        start_checkpoint: OptionalValue { is_some: false, value: Field::default() },
                                        lock_checkpoint: OptionalValue { is_some: false, value: Field::default() },
                                        epoch_length: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    },
                                    next_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: OptionalValue { is_some: true, value: Field(Fp::from_str("0").unwrap()) },
                                            total_currency: OptionalValue { is_some: false, value: RangeCondition::default() },
                                        },
                                        seed: OptionalValue { is_some: true, value: Field(Fp::from_str("1").unwrap()) },
                                        start_checkpoint: OptionalValue { is_some: false, value: Field::default() },
                                        lock_checkpoint: OptionalValue { is_some: false, value: Field::default() },
                                        epoch_length: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    },
                                },
                                account: AccountPreconditions {
                                    balance: OptionalValue { is_some: true, value: RangeCondition { lower: 0u64, upper: 1u64 } },
                                    nonce: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    receipt_chain_hash: OptionalValue { is_some: false, value: Field::default() },
                                    delegate: OptionalValue { is_some: true, value: PublicKey(CompressedPubKey::from_address("B62qkhSbtmPnafXL2Go9iyFwXm6dp8Lz79RMnTxY5QW9AG2XVKUFfu2").unwrap()) },
                                    state: vec![
                                            OptionalValue { is_some: false, value: Field::default() },
                                            OptionalValue { is_some: true, value: Field(Fp::from_str("11064851401160107").unwrap()) },
                                            OptionalValue { is_some: false, value: Field::default() },
                                            OptionalValue { is_some: true, value: Field(Fp::from_str("15856709527818962643843835197811105072227375016250782768293595314177330135121").unwrap()) },
                                            OptionalValue { is_some: false, value: Field::default() },
                                            OptionalValue { is_some: true, value: Field(Fp::from_str("1172946779036619743").unwrap()) },
                                            OptionalValue { is_some: false, value: Field::default() },
                                            OptionalValue { is_some: false, value: Field::default() },
                                        ],
                                    action_state: OptionalValue { is_some: false, value: Field::default() },
                                    proved_state: OptionalValue { is_some: true, value: true },
                                    is_new: OptionalValue { is_some: false, value: false },
                                },
                                valid_while: OptionalValue { is_some: false, value: RangeCondition::default() },
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
                    },// Account update 4
                    AccountUpdate {
                        body: AccountUpdateBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qq4NCZH2siboFPtj6acz7cqyuULxHgN9rZAGFvDCi26k9QV8eXNL").unwrap()),
                            token_id: Field(Fp::from_str("1").unwrap()),
                            update: Update {
                                app_state: vec![
                                    OptionalValue { is_some: false, value: Field::default() },
                                    OptionalValue { is_some: true, value: Field(Fp::from_str("451570041939895252657184344363021952").unwrap()) },
                                    OptionalValue { is_some: false, value: Field::default() },
                                    OptionalValue { is_some: false, value: Field::default() },
                                    OptionalValue { is_some: true, value: Field(Fp::from_str("1").unwrap()) },
                                    OptionalValue { is_some: true, value: Field(Fp::from_str("0").unwrap()) },
                                    OptionalValue { is_some: false, value: Field::default() },
                                    OptionalValue { is_some: true, value: Field(Fp::from_str("10378118918968422705467602175133176222241675977248766159536415002102309820120").unwrap()) },
                                ],
                                delegate: OptionalValue { is_some: false, value: PublicKey::default() },
                                verification_key: OptionalValue {
                                    is_some: true,
                                    value: VerificationKeyData {
                                        data: "AgIBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAALsq7cojes8ZcUc9M9RbZY9U7nhj8KnfU3yTEgqjtXQbAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAC7Ku3KI3rPGXFHPTPUW2WPVO54Y/Cp31N8kxIKo7V0GwEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAuyrtyiN6zxlxRz0z1Ftlj1TueGPwqd9TfJMSCqO1dBsBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAALsq7cojes8ZcUc9M9RbZY9U7nhj8KnfU3yTEgqjtXQbAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAC7Ku3KI3rPGXFHPTPUW2WPVO54Y/Cp31N8kxIKo7V0GwEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAuyrtyiN6zxlxRz0z1Ftlj1TueGPwqd9TfJMSCqO1dBsBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAALsq7cojes8ZcUc9M9RbZY9U7nhj8KnfU3yTEgqjtXQbAAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAuyrtyiN6zxlxRz0z1Ftlj1TueGPwqd9TfJMSCqO1dBsBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAALsq7cojes8ZcUc9M9RbZY9U7nhj8KnfU3yTEgqjtXQbAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAC7Ku3KI3rPGXFHPTPUW2WPVO54Y/Cp31N8kxIKo7V0GwEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAuyrtyiN6zxlxRz0z1Ftlj1TueGPwqd9TfJMSCqO1dBsBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAALsq7cojes8ZcUc9M9RbZY9U7nhj8KnfU3yTEgqjtXQbAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAC7Ku3KI3rPGXFHPTPUW2WPVO54Y/Cp31N8kxIKo7V0GwEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAuyrtyiN6zxlxRz0z1Ftlj1TueGPwqd9TfJMSCqO1dBsBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAALsq7cojes8ZcUc9M9RbZY9U7nhj8KnfU3yTEgqjtXQbAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAC7Ku3KI3rPGXFHPTPUW2WPVO54Y/Cp31N8kxIKo7V0GwEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAuyrtyiN6zxlxRz0z1Ftlj1TueGPwqd9TfJMSCqO1dBsBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAALsq7cojes8ZcUc9M9RbZY9U7nhj8KnfU3yTEgqjtXQbAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAC7Ku3KI3rPGXFHPTPUW2WPVO54Y/Cp31N8kxIKo7V0GwEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAuyrtyiN6zxlxRz0z1Ftlj1TueGPwqd9TfJMSCqO1dBsBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAALsq7cojes8ZcUc9M9RbZY9U7nhj8KnfU3yTEgqjtXQbAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAC7Ku3KI3rPGXFHPTPUW2WPVO54Y/Cp31N8kxIKo7V0GwABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAALsq7cojes8ZcUc9M9RbZY9U7nhj8KnfU3yTEgqjtXQbAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAC7Ku3KI3rPGXFHPTPUW2WPVO54Y/Cp31N8kxIKo7V0GwEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAuyrtyiN6zxlxRz0z1Ftlj1TueGPwqd9TfJMSCqO1dBsBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAALsq7cojes8ZcUc9M9RbZY9U7nhj8KnfU3yTEgqjtXQbAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAC7Ku3KI3rPGXFHPTPUW2WPVO54Y/Cp31N8kxIKo7V0GwEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAuyrtyiN6zxlxRz0z1Ftlj1TueGPwqd9TfJMSCqO1dBs=".to_string(),
                                        hash: Field(Fp::from_str("3392518251768960475377392625298437850623664973002200885669375116181514017494").unwrap()),
                                    }
                                },
                                permissions: OptionalValue {
                                    is_some: true,
                                    value: Permissions {
                                        edit_state: AuthRequired { constant: true, signature_necessary: false, signature_sufficient: false },
                                        access: AuthRequired { constant: false, signature_necessary: false, signature_sufficient: false },
                                        send: AuthRequired { constant: false, signature_necessary: false, signature_sufficient: false },
                                        receive: AuthRequired { constant: true, signature_necessary: false, signature_sufficient: false },
                                        set_delegate: AuthRequired { constant: true, signature_necessary: true, signature_sufficient: true },
                                        set_permissions: AuthRequired { constant: true, signature_necessary: false, signature_sufficient: false },
                                        set_verification_key: SetVerificationKey {
                                            auth: AuthRequired { constant: true, signature_necessary: false, signature_sufficient: false },
                                            txn_version: 0,
                                        },
                                        set_zkapp_uri: AuthRequired { constant: true, signature_necessary: false, signature_sufficient: false },
                                        edit_action_state: AuthRequired { constant: true, signature_necessary: false, signature_sufficient: false },
                                        set_token_symbol: AuthRequired { constant: true, signature_necessary: true, signature_sufficient: true },
                                        increment_nonce: AuthRequired { constant: false, signature_necessary: false, signature_sufficient: false },
                                        set_voting_for: AuthRequired { constant: true, signature_necessary: false, signature_sufficient: false },
                                        set_timing: AuthRequired { constant: false, signature_necessary: false, signature_sufficient: false },
                                    }
                                },
                                zkapp_uri: OptionalValue { is_some: true, value: ZkappUriData::default() },
                                token_symbol: OptionalValue { is_some: false, value: TokenSymbolData::default() },
                                timing: OptionalValue {
                                    is_some: true,
                                    value: TimingData {
                                        initial_minimum_balance: 468,
                                        cliff_time: 0,
                                        cliff_amount: 4,
                                        vesting_period: 25472475,
                                        vesting_increment: 30708098379397551,
                                    }
                                },
                                voting_for: OptionalValue { is_some: true, value: Field(Fp::from_str("3NL85QAC34ezQa6KSAxHW1izgFNdGF7v8u6kBcC26m721NQCphPu").unwrap()) },
                            },
                            balance_change: BalanceChange {
                                magnitude: 1,
                                sgn: 1,
                            },
                            increment_nonce: false,
                            events: Events {
                                data: vec![
                                    vec![Field(Fp::from_str("0").unwrap())],
                                ],
                                hash: Field::default(),
                            },
                            actions: Actions {
                                data: vec![],
                                hash: Field::default(),
                            },
                            call_data: Field(Fp::from_str("22302029000286186028016719320877888255876699220532741299507011328987201514786").unwrap()),
                            call_depth: 1,
                            preconditions: Preconditions {
                                network: NetworkPreconditions {
                                    snarked_ledger_hash: OptionalValue { is_some: true, value: Field(Fp::from_str("63666211566158").unwrap()) },
                                    blockchain_length: OptionalValue { is_some: true, value: RangeCondition { lower: 1979960u32, upper: 1712u32 } },
                                    min_window_density: OptionalValue { is_some: true, value: RangeCondition { lower: 1u32, upper: 1u32 } },
                                    total_currency: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    global_slot_since_genesis: OptionalValue { is_some: true, value: RangeCondition { lower: 0u32, upper: 1u32 } },
                                    staking_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: OptionalValue { is_some: true, value: Field(Fp::from_str("25184502287165163862094720624267095048321446909488335209361342178003510175800").unwrap()) },
                                            total_currency: OptionalValue { is_some: true, value: RangeCondition { lower: 39u64, upper: 1937575u64 } },
                                        },
                                        seed: OptionalValue { is_some: true, value: Field(Fp::from_str("11372240081513675320754020694727128680755634373429898076733293034086739756848").unwrap()) },
                                        start_checkpoint: OptionalValue { is_some: true, value: Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap()) },
                                        lock_checkpoint: OptionalValue { is_some: false, value: Field::default() },
                                        epoch_length: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    },
                                    next_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: OptionalValue { is_some: false, value: Field::default() },
                                            total_currency: OptionalValue { is_some: true, value: RangeCondition { lower: 15204460786162u64, upper: 1u64 } },
                                        },
                                        seed: OptionalValue { is_some: true, value: Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap()) },
                                        start_checkpoint: OptionalValue { is_some: false, value: Field::default() },
                                        lock_checkpoint: OptionalValue { is_some: false, value: Field::default() },
                                        epoch_length: OptionalValue { is_some: true, value: RangeCondition { lower: 404975140u32, upper: 593u32 } },
                                    },
                                },
                                account: AccountPreconditions {
                                    balance: OptionalValue { is_some: true, value: RangeCondition { lower: 34105707275720u64, upper: 1u64 } },
                                    nonce: OptionalValue { is_some: true, value: RangeCondition { lower: 1263421u32, upper: 1657157293u32 } },
                                    receipt_chain_hash: OptionalValue { is_some: false, value: Field::default() },
                                    delegate: OptionalValue { is_some: true, value: PublicKey(CompressedPubKey::from_address("B62qmQJDp9dMNnKQwHC6nok3RRRUiHUC4fetY9NkhpmjnBnaiWnJz5L").unwrap()) },
                                    state: vec![
                                            OptionalValue { is_some: true, value: Field(Fp::from_str("0").unwrap()) },
                                            OptionalValue { is_some: true, value: Field(Fp::from_str("50").unwrap()) },
                                            OptionalValue { is_some: true, value: Field(Fp::from_str("5674758732042993707116691122418959926680115872862576402928425311806157282493").unwrap()) },
                                            OptionalValue { is_some: false, value: Field::default() },
                                            OptionalValue { is_some: false, value: Field::default() },
                                            OptionalValue { is_some: false, value: Field::default() },
                                            OptionalValue { is_some: true, value: Field(Fp::from_str("15572912111532360127966743233209450525149559056435965070776515328865264980934").unwrap()) },
                                            OptionalValue { is_some: true, value: Field(Fp::from_str("5859864213767151970614484676246925331455226375514571335865110360970329376576").unwrap()) },
                                        ],
                                    action_state: OptionalValue { is_some: true, value: Field(Fp::from_str("25079927036070901246064867767436987657692091363973573142121686150614948079097").unwrap()) },
                                    proved_state: OptionalValue { is_some: true, value: false },
                                    is_new: OptionalValue { is_some: true, value: false },
                                },
                                valid_while: OptionalValue { is_some: false, value: RangeCondition::default() },
                            },
                            use_full_commitment: false,
                            implicit_account_creation_fee: false,
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
                    },// Account update 5
                    AccountUpdate {
                        body: AccountUpdateBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qq4NCZH2siboFPtj6acz7cqyuULxHgN9rZAGFvDCi26k9QV8eXNL").unwrap()),
                            token_id: Field(Fp::from_str("106224659663764323839127091057726919").unwrap()),
                            update: Update {
                                app_state: vec![
                                    OptionalValue { is_some: true, value: Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap()) },
                                    OptionalValue { is_some: true, value: Field(Fp::from_str("8316200588079222329238650589756141799950875720117455686378276924368922597126").unwrap()) },
                                    OptionalValue { is_some: true, value: Field(Fp::from_str("19702631715094903123009251950082958271663223812458646012211590694249456188703").unwrap()) },
                                    OptionalValue { is_some: true, value: Field(Fp::from_str("2051248757893648").unwrap()) },
                                    OptionalValue { is_some: true, value: Field(Fp::from_str("102255001415264281357").unwrap()) },
                                    OptionalValue { is_some: false, value: Field::default() },
                                    OptionalValue { is_some: false, value: Field::default() },
                                    OptionalValue { is_some: false, value: Field::default() },
                                ],
                                delegate: OptionalValue { is_some: true, value: PublicKey(CompressedPubKey::from_address("B62qo8m5axfUwxKikGuyhDxgX2wEAtgyW2PXqyEvJKdCDmaqAkCgh6M").unwrap()) },
                                verification_key: OptionalValue { is_some: false, value: VerificationKeyData::default() },
                                permissions: OptionalValue {
                                    is_some: true,
                                    value: Permissions {
                                        edit_state: AuthRequired { constant: true, signature_necessary: true, signature_sufficient: true },
                                        access: AuthRequired { constant: true, signature_necessary: false, signature_sufficient: false },
                                        send: AuthRequired { constant: false, signature_necessary: false, signature_sufficient: false },
                                        receive: AuthRequired { constant: true, signature_necessary: false, signature_sufficient: false },
                                        set_delegate: AuthRequired { constant: true, signature_necessary: false, signature_sufficient: false },
                                        set_permissions: AuthRequired { constant: false, signature_necessary: false, signature_sufficient: false },
                                        set_verification_key: SetVerificationKey {
                                            auth: AuthRequired { constant: false, signature_necessary: false, signature_sufficient: false },
                                            txn_version: 2,
                                        },
                                        set_zkapp_uri: AuthRequired { constant: true, signature_necessary: true, signature_sufficient: true },
                                        edit_action_state: AuthRequired { constant: false, signature_necessary: false, signature_sufficient: false },
                                        set_token_symbol: AuthRequired { constant: true, signature_necessary: false, signature_sufficient: false },
                                        increment_nonce: AuthRequired { constant: true, signature_necessary: false, signature_sufficient: false },
                                        set_voting_for: AuthRequired { constant: true, signature_necessary: true, signature_sufficient: true },
                                        set_timing: AuthRequired { constant: false, signature_necessary: false, signature_sufficient: false },
                                    }
                                },
                                zkapp_uri: OptionalValue { is_some: false, value: ZkappUriData::default() },
                                token_symbol: OptionalValue { is_some: true, value: TokenSymbolData::default() },
                                timing: OptionalValue {
                                    is_some: true,
                                    value: TimingData {
                                        initial_minimum_balance: 1,
                                        cliff_time: 969,
                                        cliff_amount: 11110192699384127,
                                        vesting_period: 0,
                                        vesting_increment: 3547107707,
                                    }
                                },
                                voting_for: OptionalValue { is_some: false, value: Field::default() },
                            },
                            balance_change: BalanceChange {
                                magnitude: 3,
                                sgn: -1,
                            },
                            increment_nonce: false,
                            events: Events {
                                data: vec![],
                                hash: Field::default(),
                            },
                            actions: Actions {
                                data: vec![],
                                hash: Field::default(),
                            },
                            call_data: Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap()),
                            call_depth: 1,
                            preconditions: Preconditions {
                                network: NetworkPreconditions {
                                    snarked_ledger_hash: OptionalValue { is_some: false, value: Field::default() },
                                    blockchain_length: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    min_window_density: OptionalValue { is_some: true, value: RangeCondition { lower: 2u32, upper: 108u32 } },
                                    total_currency: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    global_slot_since_genesis: OptionalValue { is_some: true, value: RangeCondition { lower: 20952u32, upper: 0u32 } },
                                    staking_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: OptionalValue { is_some: true, value: Field(Fp::from_str("25078643088821286506724735898108690056423496020998611414212280219849830914797").unwrap()) },
                                            total_currency: OptionalValue { is_some: true, value: RangeCondition { lower: 1014248u64, upper: 18446744073709551615u64 } },
                                        },
                                        seed: OptionalValue { is_some: false, value: Field::default() },
                                        start_checkpoint: OptionalValue { is_some: false, value: Field::default() },
                                        lock_checkpoint: OptionalValue { is_some: true, value: Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap()) },
                                        epoch_length: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    },
                                    next_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: OptionalValue { is_some: false, value: Field::default() },
                                            total_currency: OptionalValue { is_some: true, value: RangeCondition { lower: 1599976510u64, upper: 1615060873966949u64 } },
                                        },
                                        seed: OptionalValue { is_some: true, value: Field(Fp::from_str("25009831581590501146512593704076896292119921272978623576942798610420604342310").unwrap()) },
                                        start_checkpoint: OptionalValue { is_some: true, value: Field(Fp::from_str("14865637632457513468025901764324452158679273293264234201172306332516709798495").unwrap()) },
                                        lock_checkpoint: OptionalValue { is_some: true, value: Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap()) },
                                        epoch_length: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    },
                                },
                                account: AccountPreconditions {
                                    balance: OptionalValue { is_some: true, value: RangeCondition { lower: 0u64, upper: 878774388648175u64 } },
                                    nonce: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    receipt_chain_hash: OptionalValue { is_some: true, value: Field(Fp::from_str("12800555570493796051164463178025887632013655099012684868640096898944196232700").unwrap()) },
                                    delegate: OptionalValue { is_some: false, value: PublicKey::default() },
                                    state: vec![
                                            OptionalValue { is_some: false, value: Field::default() },
                                            OptionalValue { is_some: false, value: Field::default() },
                                            OptionalValue { is_some: false, value: Field::default() },
                                            OptionalValue { is_some: true, value: Field(Fp::from_str("1").unwrap()) },
                                            OptionalValue { is_some: true, value: Field(Fp::from_str("20801334907186317317666001709879946162985539212826276788136719247577985672481").unwrap()) },
                                            OptionalValue { is_some: false, value: Field::default() },
                                            OptionalValue { is_some: false, value: Field::default() },
                                            OptionalValue { is_some: true, value: Field(Fp::from_str("467765432011386082959658166").unwrap()) },
                                        ],
                                    action_state: OptionalValue { is_some: true, value: Field(Fp::from_str("27394125218298390538786068474571164040939721657578059637490483762555358823372").unwrap()) },
                                    proved_state: OptionalValue { is_some: true, value: true },
                                    is_new: OptionalValue { is_some: true, value: true },
                                },
                                valid_while: OptionalValue { is_some: true, value: RangeCondition { lower: 353131081u32, upper: 0u32 } },
                            },
                            use_full_commitment: false,
                            implicit_account_creation_fee: true,
                            may_use_token: MayUseToken {
                                parents_own_token: true,
                                inherit_from_parent: false,
                            },
                            authorization_kind: AuthorizationKind {
                                is_signed: false,
                                is_proved: false,
                                verification_key_hash: Field(Fp::from_str("3392518251768960475377392625298437850623664973002200885669375116181514017494").unwrap()),
                            },
                        },
                        authorization: Authorization {
                            proof: None,
                            signature: None,
                        },
                    },// Account update 6
                    AccountUpdate {
                        body: AccountUpdateBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qk4XQwDqiaeAB2TAig8mdJHBwrcg2eXBaEz7HGi6kDUUREZg5Wf8").unwrap()),
                            token_id: Field(Fp::from_str("1").unwrap()),
                            update: Update {
                                app_state: vec![
                                    OptionalValue { is_some: true, value: Field(Fp::from_str("13234088948489202818352537624182591183333887854663474032587368872047705927728").unwrap()) },
                                    OptionalValue { is_some: false, value: Field::default() },
                                    OptionalValue { is_some: false, value: Field::default() },
                                    OptionalValue { is_some: true, value: Field(Fp::from_str("408667380170").unwrap()) },
                                    OptionalValue { is_some: false, value: Field::default() },
                                    OptionalValue { is_some: false, value: Field::default() },
                                    OptionalValue { is_some: false, value: Field::default() },
                                    OptionalValue { is_some: true, value: Field(Fp::from_str("0").unwrap()) },
                                ],
                                delegate: OptionalValue { is_some: false, value: PublicKey::default() },
                                verification_key: OptionalValue { is_some: false, value: VerificationKeyData::default() },
                                permissions: OptionalValue { is_some: false, value: Permissions::default() },
                                zkapp_uri: OptionalValue { is_some: false, value: ZkappUriData::default() },
                                token_symbol: OptionalValue { is_some: false, value: TokenSymbolData::default() },
                                timing: OptionalValue {
                                    is_some: true,
                                    value: TimingData {
                                        initial_minimum_balance: 3835837654,
                                        cliff_time: 1649703,
                                        cliff_amount: 32041401888829845,
                                        vesting_period: 287327,
                                        vesting_increment: 20627,
                                    }
                                },
                                voting_for: OptionalValue { is_some: true, value: Field(Fp::from_str("3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x").unwrap()) },
                            },
                            balance_change: BalanceChange {
                                magnitude: 0,
                                sgn: -1,
                            },
                            increment_nonce: true,
                            events: Events {
                                data: vec![],
                                hash: Field::default(),
                            },
                            actions: Actions {
                                data: vec![],
                                hash: Field::default(),
                            },
                            call_data: Field(Fp::from_str("1").unwrap()),
                            call_depth: 2,
                            preconditions: Preconditions {
                                network: NetworkPreconditions {
                                    snarked_ledger_hash: OptionalValue { is_some: false, value: Field::default() },
                                    blockchain_length: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    min_window_density: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    total_currency: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    global_slot_since_genesis: OptionalValue { is_some: true, value: RangeCondition { lower: 4u32, upper: 64326527u32 } },
                                    staking_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: OptionalValue { is_some: true, value: Field(Fp::from_str("105421194403674826917214360762652").unwrap()) },
                                            total_currency: OptionalValue { is_some: true, value: RangeCondition { lower: 1400993u64, upper: 18446744073709551615u64 } },
                                        },
                                        seed: OptionalValue { is_some: true, value: Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap()) },
                                        start_checkpoint: OptionalValue { is_some: true, value: Field(Fp::from_str("0").unwrap()) },
                                        lock_checkpoint: OptionalValue { is_some: false, value: Field::default() },
                                        epoch_length: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    },
                                    next_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: OptionalValue { is_some: false, value: Field::default() },
                                            total_currency: OptionalValue { is_some: false, value: RangeCondition::default() },
                                        },
                                        seed: OptionalValue { is_some: false, value: Field::default() },
                                        start_checkpoint: OptionalValue { is_some: false, value: Field::default() },
                                        lock_checkpoint: OptionalValue { is_some: false, value: Field::default() },
                                        epoch_length: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    },
                                },
                                account: AccountPreconditions {
                                    balance: OptionalValue { is_some: true, value: RangeCondition { lower: 4273128662623768159u64, upper: 39557u64 } },
                                    nonce: OptionalValue { is_some: true, value: RangeCondition { lower: 164332u32, upper: 16939u32 } },
                                    receipt_chain_hash: OptionalValue { is_some: true, value: Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap()) },
                                    delegate: OptionalValue { is_some: false, value: PublicKey::default() },
                                    state: vec![
                                            OptionalValue { is_some: false, value: Field::default() },
                                            OptionalValue { is_some: true, value: Field(Fp::from_str("0").unwrap()) },
                                            OptionalValue { is_some: true, value: Field(Fp::from_str("17016200266628853718115820331289599217658918393128975249470149956831464208861").unwrap()) },
                                            OptionalValue { is_some: false, value: Field::default() },
                                            OptionalValue { is_some: false, value: Field::default() },
                                            OptionalValue { is_some: true, value: Field(Fp::from_str("4").unwrap()) },
                                            OptionalValue { is_some: true, value: Field(Fp::from_str("1").unwrap()) },
                                            OptionalValue { is_some: true, value: Field(Fp::from_str("0").unwrap()) },
                                        ],
                                    action_state: OptionalValue { is_some: false, value: Field::default() },
                                    proved_state: OptionalValue { is_some: true, value: false },
                                    is_new: OptionalValue { is_some: true, value: true },
                                },
                                valid_while: OptionalValue { is_some: true, value: RangeCondition { lower: 3u32, upper: 71u32 } },
                            },
                            use_full_commitment: true,
                            implicit_account_creation_fee: false,
                            may_use_token: MayUseToken {
                                parents_own_token: false,
                                inherit_from_parent: false,
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
                    },// Account update 7
                    AccountUpdate {
                        body: AccountUpdateBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qmWKg2WEfaDGV3yEUCGZW915moQGESg7nupjLJBSs9VVnLNRpcpi").unwrap()),
                            token_id: Field(Fp::from_str("1").unwrap()),
                            update: Update {
                                app_state: vec![
                                    OptionalValue { is_some: false, value: Field::default() },
                                    OptionalValue { is_some: false, value: Field::default() },
                                    OptionalValue { is_some: true, value: Field(Fp::from_str("4957609544206208293117192850871337616872514062647969128655770159563310185998").unwrap()) },
                                    OptionalValue { is_some: false, value: Field::default() },
                                    OptionalValue { is_some: true, value: Field(Fp::from_str("2745722697544501836971833848952989386120873299180020979381308376620508871781").unwrap()) },
                                    OptionalValue { is_some: false, value: Field::default() },
                                    OptionalValue { is_some: false, value: Field::default() },
                                    OptionalValue { is_some: false, value: Field::default() },
                                ],
                                delegate: OptionalValue { is_some: false, value: PublicKey::default() },
                                verification_key: OptionalValue { is_some: false, value: VerificationKeyData::default() },
                                permissions: OptionalValue {
                                    is_some: true,
                                    value: Permissions {
                                        edit_state: AuthRequired { constant: false, signature_necessary: false, signature_sufficient: false },
                                        access: AuthRequired { constant: false, signature_necessary: false, signature_sufficient: false },
                                        send: AuthRequired { constant: true, signature_necessary: false, signature_sufficient: false },
                                        receive: AuthRequired { constant: true, signature_necessary: false, signature_sufficient: false },
                                        set_delegate: AuthRequired { constant: true, signature_necessary: true, signature_sufficient: true },
                                        set_permissions: AuthRequired { constant: false, signature_necessary: false, signature_sufficient: false },
                                        set_verification_key: SetVerificationKey {
                                            auth: AuthRequired { constant: false, signature_necessary: false, signature_sufficient: false },
                                            txn_version: 104978,
                                        },
                                        set_zkapp_uri: AuthRequired { constant: true, signature_necessary: true, signature_sufficient: true },
                                        edit_action_state: AuthRequired { constant: true, signature_necessary: false, signature_sufficient: false },
                                        set_token_symbol: AuthRequired { constant: false, signature_necessary: false, signature_sufficient: false },
                                        increment_nonce: AuthRequired { constant: false, signature_necessary: false, signature_sufficient: false },
                                        set_voting_for: AuthRequired { constant: false, signature_necessary: false, signature_sufficient: false },
                                        set_timing: AuthRequired { constant: false, signature_necessary: false, signature_sufficient: false },
                                    }
                                },
                                zkapp_uri: OptionalValue { is_some: false, value: ZkappUriData::default() },
                                token_symbol: OptionalValue { is_some: true, value: TokenSymbolData::default() },
                                timing: OptionalValue {
                                    is_some: true,
                                    value: TimingData {
                                        initial_minimum_balance: 361312014999738,
                                        cliff_time: 1526784,
                                        cliff_amount: 3687448543611702,
                                        vesting_period: 921,
                                        vesting_increment: 1641,
                                    }
                                },
                                voting_for: OptionalValue { is_some: false, value: Field::default() },
                            },
                            balance_change: BalanceChange {
                                magnitude: 34821149,
                                sgn: 1,
                            },
                            increment_nonce: true,
                            events: Events {
                                data: vec![
                                    vec![Field(Fp::from_str("340282366920938463463374607431768211455").unwrap()), Field(Fp::from_str("20839735745563651590832045662090728323480189731243673177247455021014564522923").unwrap())],
                                ],
                                hash: Field::default(),
                            },
                            actions: Actions {
                                data: vec![],
                                hash: Field::default(),
                            },
                            call_data: Field(Fp::from_str("1").unwrap()),
                            call_depth: 1,
                            preconditions: Preconditions {
                                network: NetworkPreconditions {
                                    snarked_ledger_hash: OptionalValue { is_some: false, value: Field::default() },
                                    blockchain_length: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    min_window_density: OptionalValue { is_some: true, value: RangeCondition { lower: 376844259u32, upper: 0u32 } },
                                    total_currency: OptionalValue { is_some: true, value: RangeCondition { lower: 63716020847813108u64, upper: 24993604069u64 } },
                                    global_slot_since_genesis: OptionalValue { is_some: true, value: RangeCondition { lower: 131310344u32, upper: 1u32 } },
                                    staking_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: OptionalValue { is_some: false, value: Field::default() },
                                            total_currency: OptionalValue { is_some: true, value: RangeCondition { lower: 0u64, upper: 31852084u64 } },
                                        },
                                        seed: OptionalValue { is_some: true, value: Field(Fp::from_str("22071229747643899929480164116718741255744422667289538799684035351472447523472").unwrap()) },
                                        start_checkpoint: OptionalValue { is_some: true, value: Field(Fp::from_str("1111094845081745512595054203125177614011410980477489992614217751196552884256").unwrap()) },
                                        lock_checkpoint: OptionalValue { is_some: false, value: Field::default() },
                                        epoch_length: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    },
                                    next_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: OptionalValue { is_some: true, value: Field(Fp::from_str("19661131184846407165877288741452503147228103328030428179641786265939781700014").unwrap()) },
                                            total_currency: OptionalValue { is_some: false, value: RangeCondition::default() },
                                        },
                                        seed: OptionalValue { is_some: false, value: Field::default() },
                                        start_checkpoint: OptionalValue { is_some: false, value: Field::default() },
                                        lock_checkpoint: OptionalValue { is_some: false, value: Field::default() },
                                        epoch_length: OptionalValue { is_some: true, value: RangeCondition { lower: 2u32, upper: 165146u32 } },
                                    },
                                },
                                account: AccountPreconditions {
                                    balance: OptionalValue { is_some: false, value: RangeCondition::default() },
                                    nonce: OptionalValue { is_some: true, value: RangeCondition { lower: 7u32, upper: 836147846u32 } },
                                    receipt_chain_hash: OptionalValue { is_some: false, value: Field::default() },
                                    delegate: OptionalValue { is_some: true, value: PublicKey(CompressedPubKey::from_address("B62qkJSn1SXnSgy3FSc4Y81YJimBAdPQLh7JkvKBQbaoPSqwfMxNjfP").unwrap()) },
                                    state: vec![
                                            OptionalValue { is_some: true, value: Field(Fp::from_str("24744082835263256424782378845254299132721380623412292937807121565414784429714").unwrap()) },
                                            OptionalValue { is_some: false, value: Field::default() },
                                            OptionalValue { is_some: true, value: Field(Fp::from_str("11221985696133830319031615539088832734871808264962274552200047618313440626921").unwrap()) },
                                            OptionalValue { is_some: false, value: Field::default() },
                                            OptionalValue { is_some: true, value: Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap()) },
                                            OptionalValue { is_some: true, value: Field(Fp::from_str("28181600982782577010783018611064321041395419418477160873775299792189444111556").unwrap()) },
                                            OptionalValue { is_some: false, value: Field::default() },
                                            OptionalValue { is_some: false, value: Field::default() },
                                        ],
                                    action_state: OptionalValue { is_some: false, value: Field::default() },
                                    proved_state: OptionalValue { is_some: true, value: false },
                                    is_new: OptionalValue { is_some: true, value: true },
                                },
                                valid_while: OptionalValue { is_some: true, value: RangeCondition { lower: 3u32, upper: 147u32 } },
                            },
                            use_full_commitment: false,
                            implicit_account_creation_fee: true,
                            may_use_token: MayUseToken {
                                parents_own_token: true,
                                inherit_from_parent: false,
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
                memo: "E4YVkqyxvLZhRst6Zeyz7z7RoBk8RZrTFdoyJ2wePw8cgdK574MKY".to_string(),
            },
            network: NetworkId::MAINNET,
            expected_memo_hash: "369453247258139461793874281204592948913530799489559288302740003628227053858",
            expected_fee_payer_hash: "10107555265261176496684761383484143750979693352181633907531879785290480643417",
            expected_account_updates_commitment: "23316460487402773494110335361659308674100635476278317671190332182678874982779",
            expected_full_commitment: "3606189568599761746353209221660606816407299472850415535325220362709068199288",
        }
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
