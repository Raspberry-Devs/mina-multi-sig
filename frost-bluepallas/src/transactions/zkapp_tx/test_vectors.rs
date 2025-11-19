//! Test vectors for ZkApp transaction commitment functions
//!
//! This module contains shared test data used across different commitment function tests.
//! All test vectors use empty/default data structures - populate with actual test data as needed.

use bs58;
use mina_hasher::Fp;
use mina_signer::{CompressedPubKey, NetworkId};
use std::str::FromStr;

use crate::transactions::zkapp_tx::{
    ActionState, AuthRequired, Authorization, MayUseToken, Permissions, SetVerificationKey,
    TimingData, TokenId, TokenSymbol, VerificationKeyData, ZkappUri,
};

use super::{
    AccountPreconditions, AccountUpdate, AccountUpdateBody, Actions, AuthorizationKind,
    BalanceChange, EpochData, EpochLedger, Events, FeePayer, FeePayerBody, Field,
    NetworkPreconditions, Preconditions, PublicKey, RangeCondition, Update, ZKAppCommand,
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
            name: "empty_account_updates",
            zkapp_command: ZKAppCommand {
                fee_payer: FeePayer {
                    body: FeePayerBody {
                        public_key: PublicKey(CompressedPubKey::from_address("B62qn8EyCTcTBPczwTSxtvHWDVygPTeiEZkYH7m5FAXbym8T7AGXf81").unwrap()),
                        fee: 33034803238,
                        valid_until: Some(187142),
                        nonce: 0,
                    },
                    authorization: "7mWxjLYgbJUkZNcGouvhVj5tJ8yu9hoexb9ntvPK8t5LHqzmrL6QJjjKtf5SgmxB4QWkDw7qoMMbbNGtHVpsbJHPyTy2EzRQ".to_string(),
                },
                account_updates: vec![
                ],
                memo: decode_memo_from_base58("E4YTLoDojpEF9jChvyaDpteawAPPXgdZQukX6UGSKzVVgf17wzQiv"),
            },
            network: NetworkId::TESTNET,
            expected_memo_hash: "11389652194005057975368457399421352653010041179485881427310531175333437124368",
            expected_fee_payer_hash: "7440442690224604593498294045035842737848284124224925036785514405229648680377",
            expected_account_updates_commitment: "0",
            expected_full_commitment: "14398812481809678820938251721678863601825386927848545066309874540801775728297",
        },
        ZkAppTestVector {
            name: "single_account_update",
            zkapp_command: ZKAppCommand {
                fee_payer: FeePayer {
                    body: FeePayerBody {
                        public_key: PublicKey(CompressedPubKey::from_address("B62qpEsE5VWjzZigz6avNSgknrdqW8GrNKvLi3UPASRcj2qY2zn573U").unwrap()),
                        fee: 77038474678412,
                        valid_until: Some(601567),
                        nonce: 3,
                    },
                    authorization: "7mWxjLYgbJUkZNcGouvhVj5tJ8yu9hoexb9ntvPK8t5LHqzmrL6QJjjKtf5SgmxB4QWkDw7qoMMbbNGtHVpsbJHPyTy2EzRQ".to_string(),
                },
                account_updates: vec![
// Account update 1
                    AccountUpdate {
                        body: AccountUpdateBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qiTsjeYstbmxhqBk5zbuxyEAN8NMy32rGV3Ga26qjgWKA4Cfcrwz").unwrap()),
                            token_id: TokenId(Field(Fp::from_str("1").unwrap())),
                            update: Update {
                                app_state: [
                                    None,
                                    None,
                                    Some(Field(Fp::from_str("1874898725280435780385059716368818160494677764219647021698658858000841091319").unwrap())),
                                    None,
                                    None,
                                    Some(Field(Fp::from_str("0").unwrap())),
                                    None,
                                    None,
                                ],
                                delegate: Some(PublicKey(CompressedPubKey::from_address("B62qriymma9nKoQuEECfELgzx2TMUCrUb4NjzfGeNJAnQQzYrdH97tF").unwrap())),
                                verification_key: Some(VerificationKeyData {
                                        data: "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".to_string(),
                                        hash: Field(Fp::from_str("3392518251768960475377392625298437850623664973002200885669375116181514017494").unwrap()),
                                    }),
                                permissions: None,
                                zkapp_uri: None,
                                token_symbol: None,
                                timing: None,
                                voting_for: None,
                            },
                            balance_change: BalanceChange {
                                magnitude: 1938800,
                                sgn: -1,
                            },
                            increment_nonce: true,
                            events: Events {
                                data: vec![
                                    vec![Field(Fp::from_str("0").unwrap()), Field(Fp::from_str("7686176436381634662162039632228205774958105751142314409120065380736266253345").unwrap()), Field(Fp::from_str("0").unwrap()), Field(Fp::from_str("563375822809").unwrap())],
                                ]
                            },
                            actions: Actions {
                                data: vec![],
                            },
                            call_data: Field(Fp::from_str("24373988041660797299206239733224199803150120309055879401062441999335870455199").unwrap()),
                            call_depth: 0,
                            preconditions: Preconditions {
                                network: NetworkPreconditions {
                                    snarked_ledger_hash: None,
                                    blockchain_length: None,
                                    min_window_density: Some(RangeCondition { lower: 968906268, upper: 4294967295 }),
                                    total_currency: Some(RangeCondition { lower: 112903, upper: 963644603788 }),
                                    global_slot_since_genesis: None,
                                    staking_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: None,
                                            total_currency: Some(RangeCondition { lower: 2, upper: 48 }),
                                        },
                                        seed: None,
                                        start_checkpoint: None,
                                        lock_checkpoint: Some(Field(Fp::from_str("23847908551180887451130277032704895438309267854411837564195808854269423867859").unwrap())),
                                        epoch_length: None,
                                    },
                                    next_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: Some(Field(Fp::from_str("10394163103039625710446099656547187497084798423724265210505100291678802149935").unwrap())),
                                            total_currency: None,
                                        },
                                        seed: None,
                                        start_checkpoint: None,
                                        lock_checkpoint: None,
                                        epoch_length: Some(RangeCondition { lower: 18, upper: 8 }),
                                    },
                                },
                                account: AccountPreconditions {
                                    balance: None,
                                    nonce: Some(RangeCondition { lower: 0, upper: 0 }),
                                    receipt_chain_hash: None,
                                    delegate: None,
                                    state: [
                                            Some(Field(Fp::from_str("1").unwrap())),
                                            Some(Field(Fp::from_str("11978514914703649173392787322945850554016733530188102644246554610538062054944").unwrap())),
                                            None,
                                            None,
                                            Some(Field(Fp::from_str("24706128343004564827848079552507271194969935308505494870771374370012173813913").unwrap())),
                                            Some(Field(Fp::from_str("9673777180427467023640824938").unwrap())),
                                            Some(Field(Fp::from_str("0").unwrap())),
                                            None,
                                        ],
                                    action_state: Some(ActionState(Field(Fp::from_str("25079927036070901246064867767436987657692091363973573142121686150614948079097").unwrap()))),
                                    proved_state: Some(true),
                                    is_new: Some(true),
                                },
                                valid_while: Some(RangeCondition { lower: 0, upper: 89383 }),
                            },
                            use_full_commitment: true,
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
                memo: decode_memo_from_base58("E4YM2vTHhWEg66xpj52JErHUBU4pZ1yageL4TVDDpTTSsv8mK6YaH"),
            },
            network: NetworkId::MAINNET,
            expected_memo_hash: "146624400929844538317466382872834899021794596262855408933526545768996436172",
            expected_fee_payer_hash: "19706318721710889218414924651039754460938911066772149869668298883599530223675",
            expected_account_updates_commitment: "20885808005185651189421947183942528580939515770642397950928596427944626893435",
            expected_full_commitment: "18502360245177779299536585941776598593082439497015628774468068447754705113482",
        },
        ZkAppTestVector {
            name: "multiple_account_updates",
            zkapp_command: ZKAppCommand {
                fee_payer: FeePayer {
                    body: FeePayerBody {
                        public_key: PublicKey(CompressedPubKey::from_address("B62qprTzgSBYbZ2RjBT7TcBQMMvWvhrnij8yiooyXcBfmgVmSU5jynd").unwrap()),
                        fee: 1,
                        valid_until: Some(0),
                        nonce: 4294967295,
                    },
                    authorization: "7mWxjLYgbJUkZNcGouvhVj5tJ8yu9hoexb9ntvPK8t5LHqzmrL6QJjjKtf5SgmxB4QWkDw7qoMMbbNGtHVpsbJHPyTy2EzRQ".to_string(),
                },
                account_updates: vec![
// Account update 1
                    AccountUpdate {
                        body: AccountUpdateBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qpsKWJQgHpyi9XJiUJZpXUMb4QhovsWjW46KGdNWUm133d2osgTN").unwrap()),
                            token_id: TokenId(Field(Fp::from_str("1").unwrap())),
                            update: Update {
                                app_state: [
                                    Some(Field(Fp::from_str("27357518").unwrap())),
                                    None,
                                    None,
                                    Some(Field(Fp::from_str("37804").unwrap())),
                                    Some(Field(Fp::from_str("1468182990340317447822511").unwrap())),
                                    Some(Field(Fp::from_str("0").unwrap())),
                                    Some(Field(Fp::from_str("68800348470173840217602").unwrap())),
                                    Some(Field(Fp::from_str("1559478918562214717303847416485262053311249267881690890229308883480699007072").unwrap())),
                                ],
                                delegate: Some(PublicKey(CompressedPubKey::from_address("B62qpDuY8MV5XXWvwQooRw4mfPNpDXwRkXFxpmB4HSBdLRheubUmKHy").unwrap())),
                                verification_key: Some(VerificationKeyData {
                                        data: "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".to_string(),
                                        hash: Field(Fp::from_str("3392518251768960475377392625298437850623664973002200885669375116181514017494").unwrap()),
                                    }),
                                permissions: Some(Permissions {
                                        edit_state: AuthRequired::Either,
                                        access: AuthRequired::Proof,
                                        send: AuthRequired::Either,
                                        receive: AuthRequired::Proof,
                                        set_delegate: AuthRequired::Proof,
                                        set_permissions: AuthRequired::Signature,
                                        set_verification_key: SetVerificationKey {
                                            auth: AuthRequired::Impossible,
                                            txn_version: 690302,
                                        },
                                        set_zkapp_uri: AuthRequired::Signature,
                                        edit_action_state: AuthRequired::Either,
                                        set_token_symbol: AuthRequired::None,
                                        increment_nonce: AuthRequired::Either,
                                        set_voting_for: AuthRequired::Proof,
                                        set_timing: AuthRequired::Signature,
                                    }),
                                zkapp_uri: Some(ZkappUri("".as_bytes().to_vec())),
                                token_symbol: Some(TokenSymbol("_(".as_bytes().to_vec())),
                                timing: Some(TimingData {
                                        initial_minimum_balance: 14350097,
                                        cliff_time: 115571,
                                        cliff_amount: 67644,
                                        vesting_period: 7446283,
                                        vesting_increment: 30902843309,
                                    }),
                                voting_for: None,
                            },
                            balance_change: BalanceChange {
                                magnitude: 69969676595465580,
                                sgn: 1,
                            },
                            increment_nonce: true,
                            events: Events {
                                data: vec![
                                    vec![Field(Fp::from_str("8559286592968855422945713696318604429593672849303620283273405896440217927709").unwrap())],
                                    vec![Field(Fp::from_str("0").unwrap()), Field(Fp::from_str("1").unwrap()), Field(Fp::from_str("16172391553381335586212110061494569024252598340170897572611647016453300653622").unwrap())],
                                ]
                            },
                            actions: Actions {
                                data: vec![
                                    vec![Field(Fp::from_str("0").unwrap()), Field(Fp::from_str("2111995721908824295141414422611812855908589003731439072043978301057815385210").unwrap()), Field(Fp::from_str("2670652806442019749864059087004974159160650899720386776250690994236116182486").unwrap())],
                                ]
                            },
                            call_data: Field(Fp::from_str("26024424311313877122509985895360802117545815652106729316960930080960306097480").unwrap()),
                            call_depth: 0,
                            preconditions: Preconditions {
                                network: NetworkPreconditions {
                                    snarked_ledger_hash: None,
                                    blockchain_length: None,
                                    min_window_density: Some(RangeCondition { lower: 1, upper: 201388398 }),
                                    total_currency: Some(RangeCondition { lower: 234694717578, upper: 59244 }),
                                    global_slot_since_genesis: None,
                                    staking_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: Some(Field(Fp::from_str("565834789984253643144888474739523381309082967656879466731620543313135767399").unwrap())),
                                            total_currency: None,
                                        },
                                        seed: None,
                                        start_checkpoint: Some(Field(Fp::from_str("0").unwrap())),
                                        lock_checkpoint: Some(Field(Fp::from_str("22300686766199438").unwrap())),
                                        epoch_length: None,
                                    },
                                    next_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: None,
                                            total_currency: None,
                                        },
                                        seed: None,
                                        start_checkpoint: Some(Field(Fp::from_str("8802062068912658919185932393316752287").unwrap())),
                                        lock_checkpoint: Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())),
                                        epoch_length: Some(RangeCondition { lower: 0, upper: 77967491 }),
                                    },
                                },
                                account: AccountPreconditions {
                                    balance: None,
                                    nonce: Some(RangeCondition { lower: 5713135, upper: 26 }),
                                    receipt_chain_hash: Some(Field(Fp::from_str("0").unwrap())),
                                    delegate: Some(PublicKey(CompressedPubKey::from_address("B62qotRBSH4nx8GLYjqJY5SvZnjKeHgYGnJvrLER4vpp4uGEkvvrCsK").unwrap())),
                                    state: [
                                            Some(Field(Fp::from_str("340282366920938463463374607431768211455").unwrap())),
                                            Some(Field(Fp::from_str("25102004378482806015739127069341223696195812330431049087810624135113554636076").unwrap())),
                                            Some(Field(Fp::from_str("222797147591774241814463292429130601960484440703928903613884616393795593601").unwrap())),
                                            None,
                                            None,
                                            Some(Field(Fp::from_str("2187528424259532").unwrap())),
                                            None,
                                            Some(Field(Fp::from_str("4591769376782251921800").unwrap())),
                                        ],
                                    action_state: Some(ActionState(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap()))),
                                    proved_state: None,
                                    is_new: None,
                                },
                                valid_while: Some(RangeCondition { lower: 2483116, upper: 1 }),
                            },
                            use_full_commitment: true,
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
                    },// Account update 2
                    AccountUpdate {
                        body: AccountUpdateBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qjbGzrXKKqt9RmpzsDYEDT7jR8d4mSjmDUMboRAYmZmwr7pSteGq").unwrap()),
                            token_id: TokenId(Field(Fp::from_str("2602").unwrap())),
                            update: Update {
                                app_state: [
                                    Some(Field(Fp::from_str("1579435535121890371811706917827131889633430802826494695051305443151179638879").unwrap())),
                                    Some(Field(Fp::from_str("13742918936149074").unwrap())),
                                    None,
                                    Some(Field(Fp::from_str("0").unwrap())),
                                    Some(Field(Fp::from_str("10666803330467984416440210202745492000684049280518050906313776276995995854437").unwrap())),
                                    Some(Field(Fp::from_str("0").unwrap())),
                                    Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())),
                                    None,
                                ],
                                delegate: Some(PublicKey(CompressedPubKey::from_address("B62qpJLwRyZCcvkxpeTcLqUhnZ4te1vaUNKyNWuNHMgu7WN2FHCdWaS").unwrap())),
                                verification_key: Some(VerificationKeyData {
                                        data: "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".to_string(),
                                        hash: Field(Fp::from_str("3392518251768960475377392625298437850623664973002200885669375116181514017494").unwrap()),
                                    }),
                                permissions: Some(Permissions {
                                        edit_state: AuthRequired::Either,
                                        access: AuthRequired::Either,
                                        send: AuthRequired::Signature,
                                        receive: AuthRequired::None,
                                        set_delegate: AuthRequired::Impossible,
                                        set_permissions: AuthRequired::Impossible,
                                        set_verification_key: SetVerificationKey {
                                            auth: AuthRequired::Either,
                                            txn_version: 10781045,
                                        },
                                        set_zkapp_uri: AuthRequired::Impossible,
                                        edit_action_state: AuthRequired::None,
                                        set_token_symbol: AuthRequired::Impossible,
                                        increment_nonce: AuthRequired::Signature,
                                        set_voting_for: AuthRequired::None,
                                        set_timing: AuthRequired::Either,
                                    }),
                                zkapp_uri: None,
                                token_symbol: None,
                                timing: Some(TimingData {
                                        initial_minimum_balance: 1,
                                        cliff_time: 0,
                                        cliff_amount: 0,
                                        vesting_period: 3,
                                        vesting_increment: 2,
                                    }),
                                voting_for: Some(Field(Fp::from_str("16402588322384153852859862003872502569205506011697811016573338007116785581656").unwrap())),
                            },
                            balance_change: BalanceChange {
                                magnitude: 249342570006532838,
                                sgn: -1,
                            },
                            increment_nonce: false,
                            events: Events {
                                data: vec![]
                            },
                            actions: Actions {
                                data: vec![
                                    vec![Field(Fp::from_str("15104656352471618921151686219234512331744914695524365693253704049352096257639").unwrap()), Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap()), Field(Fp::from_str("0").unwrap()), Field(Fp::from_str("73313711864227212483035850037789702915").unwrap()), Field(Fp::from_str("0").unwrap())],
                                    vec![Field(Fp::from_str("953947703815458456197359564239801190188818163637248428898417868028677617730").unwrap()), Field(Fp::from_str("4363904718454446930104988956035682508096147401566389656082751711628414906465").unwrap()), Field(Fp::from_str("0").unwrap()), Field(Fp::from_str("4771722601296404583385334519418833283308783105283474802490704056143594792153").unwrap())],
                                ]
                            },
                            call_data: Field(Fp::from_str("19912643916997028331704239054043722821413345210217273693666326494688202627749").unwrap()),
                            call_depth: 0,
                            preconditions: Preconditions {
                                network: NetworkPreconditions {
                                    snarked_ledger_hash: Some(Field(Fp::from_str("11268313969929439249583851217113201058472324479002024318204333883107101576426").unwrap())),
                                    blockchain_length: Some(RangeCondition { lower: 166976336, upper: 0 }),
                                    min_window_density: None,
                                    total_currency: Some(RangeCondition { lower: 1937, upper: 2459752577603872561 }),
                                    global_slot_since_genesis: None,
                                    staking_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: Some(Field(Fp::from_str("15681").unwrap())),
                                            total_currency: Some(RangeCondition { lower: 1, upper: 2 }),
                                        },
                                        seed: Some(Field(Fp::from_str("22389627824466849812463366737571122988855240405673157254151058732181735941781").unwrap())),
                                        start_checkpoint: None,
                                        lock_checkpoint: Some(Field(Fp::from_str("5233916232731581396859035370162519093936183592847499311086321145435153183230").unwrap())),
                                        epoch_length: None,
                                    },
                                    next_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: None,
                                            total_currency: None,
                                        },
                                        seed: Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())),
                                        start_checkpoint: Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())),
                                        lock_checkpoint: None,
                                        epoch_length: None,
                                    },
                                },
                                account: AccountPreconditions {
                                    balance: None,
                                    nonce: Some(RangeCondition { lower: 0, upper: 1534705856 }),
                                    receipt_chain_hash: Some(Field(Fp::from_str("25825782255539934600060379689985852958919025713936202459628324984450599594121").unwrap())),
                                    delegate: None,
                                    state: [
                                            None,
                                            Some(Field(Fp::from_str("1").unwrap())),
                                            None,
                                            None,
                                            Some(Field(Fp::from_str("269069635508459824720319046").unwrap())),
                                            Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())),
                                            Some(Field(Fp::from_str("22119855123681325604438631278537375381540346575832311251752665001784093294148").unwrap())),
                                            Some(Field(Fp::from_str("21524176654261164869220765208128827783562046206515444324626127965338862481875").unwrap())),
                                        ],
                                    action_state: Some(ActionState(Field(Fp::from_str("25079927036070901246064867767436987657692091363973573142121686150614948079097").unwrap()))),
                                    proved_state: None,
                                    is_new: None,
                                },
                                valid_while: Some(RangeCondition { lower: 3, upper: 0 }),
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
                    },// Account update 3
                    AccountUpdate {
                        body: AccountUpdateBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qo5pxqHCKJHGCCYygmrTuJmULadgXuwfLhSdUz4T5s672BmkzJCq").unwrap()),
                            token_id: TokenId(Field(Fp::from_str("1").unwrap())),
                            update: Update {
                                app_state: [
                                    Some(Field(Fp::from_str("21058377912663485840680016053705154572849805724477445329254066264316858709145").unwrap())),
                                    None,
                                    None,
                                    None,
                                    None,
                                    Some(Field(Fp::from_str("5055401232588783850444517545014705831696322244397945349860823092909267302570").unwrap())),
                                    Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())),
                                    Some(Field(Fp::from_str("13949607923625668277929371339057412922982274541727686499195226242015975635580").unwrap())),
                                ],
                                delegate: None,
                                verification_key: None,
                                permissions: None,
                                zkapp_uri: None,
                                token_symbol: None,
                                timing: None,
                                voting_for: None,
                            },
                            balance_change: BalanceChange {
                                magnitude: 17790146,
                                sgn: 1,
                            },
                            increment_nonce: true,
                            events: Events {
                                data: vec![]
                            },
                            actions: Actions {
                                data: vec![]
                            },
                            call_data: Field(Fp::from_str("0").unwrap()),
                            call_depth: 0,
                            preconditions: Preconditions {
                                network: NetworkPreconditions {
                                    snarked_ledger_hash: Some(Field(Fp::from_str("24884456824891933939917831503920429639218971452288933395752719848303310881155").unwrap())),
                                    blockchain_length: None,
                                    min_window_density: Some(RangeCondition { lower: 8209, upper: 170018 }),
                                    total_currency: None,
                                    global_slot_since_genesis: Some(RangeCondition { lower: 60508, upper: 4294967295 }),
                                    staking_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: None,
                                            total_currency: Some(RangeCondition { lower: 25276917, upper: 667020 }),
                                        },
                                        seed: None,
                                        start_checkpoint: None,
                                        lock_checkpoint: None,
                                        epoch_length: Some(RangeCondition { lower: 107238, upper: 3 }),
                                    },
                                    next_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: Some(Field(Fp::from_str("13434535936549075540594784780097718903669714392906840643679928414389965628080").unwrap())),
                                            total_currency: None,
                                        },
                                        seed: Some(Field(Fp::from_str("2214744384155274549139661518231754747354000572024397372833787992751367117568").unwrap())),
                                        start_checkpoint: Some(Field(Fp::from_str("0").unwrap())),
                                        lock_checkpoint: Some(Field(Fp::from_str("5080075747347429886116163365263542799588367903725651970352231548738894950665").unwrap())),
                                        epoch_length: None,
                                    },
                                },
                                account: AccountPreconditions {
                                    balance: Some(RangeCondition { lower: 66, upper: 254448359107 }),
                                    nonce: Some(RangeCondition { lower: 4294967295, upper: 0 }),
                                    receipt_chain_hash: Some(Field(Fp::from_str("1063916233968371").unwrap())),
                                    delegate: None,
                                    state: [
                                            None,
                                            None,
                                            Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())),
                                            Some(Field(Fp::from_str("15468164313877613406914757429400828302309713165111309569421274529001256282026").unwrap())),
                                            None,
                                            None,
                                            None,
                                            Some(Field(Fp::from_str("774007685590602149847298718259492937383677376498963029739828676053397102780").unwrap())),
                                        ],
                                    action_state: Some(ActionState(Field(Fp::from_str("25079927036070901246064867767436987657692091363973573142121686150614948079097").unwrap()))),
                                    proved_state: Some(false),
                                    is_new: Some(false),
                                },
                                valid_while: Some(RangeCondition { lower: 1, upper: 99187 }),
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
                    },// Account update 4
                    AccountUpdate {
                        body: AccountUpdateBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qprTzgSBYbZ2RjBT7TcBQMMvWvhrnij8yiooyXcBfmgVmSU5jynd").unwrap()),
                            token_id: TokenId(Field(Fp::from_str("1").unwrap())),
                            update: Update {
                                app_state: [
                                    Some(Field(Fp::from_str("24411325758715985507112033504505565622932346450799522811930844547583052211922").unwrap())),
                                    Some(Field(Fp::from_str("22759996341563341604406201828232177080692294225343223141166288516745414971894").unwrap())),
                                    Some(Field(Fp::from_str("0").unwrap())),
                                    Some(Field(Fp::from_str("1").unwrap())),
                                    Some(Field(Fp::from_str("3254799625697868151716521294656197501253189815188524565454305682713547165803").unwrap())),
                                    None,
                                    Some(Field(Fp::from_str("2713660604604084608926484876797976703815362468718729513100863492958826269578").unwrap())),
                                    Some(Field(Fp::from_str("1").unwrap())),
                                ],
                                delegate: Some(PublicKey(CompressedPubKey::from_address("B62qj7aQkaaQJrcxCQVrAhPXi59d6qJuDdg2RhYEG6uHncaNEF7WC6H").unwrap())),
                                verification_key: None,
                                permissions: Some(Permissions {
                                        edit_state: AuthRequired::Proof,
                                        access: AuthRequired::Either,
                                        send: AuthRequired::Proof,
                                        receive: AuthRequired::Impossible,
                                        set_delegate: AuthRequired::Signature,
                                        set_permissions: AuthRequired::Impossible,
                                        set_verification_key: SetVerificationKey {
                                            auth: AuthRequired::Signature,
                                            txn_version: 1,
                                        },
                                        set_zkapp_uri: AuthRequired::Either,
                                        edit_action_state: AuthRequired::Signature,
                                        set_token_symbol: AuthRequired::Impossible,
                                        increment_nonce: AuthRequired::Signature,
                                        set_voting_for: AuthRequired::Signature,
                                        set_timing: AuthRequired::Either,
                                    }),
                                zkapp_uri: Some(ZkappUri(",".as_bytes().to_vec())),
                                token_symbol: Some(TokenSymbol("d".as_bytes().to_vec())),
                                timing: None,
                                voting_for: Some(Field(Fp::from_str("169775011390665").unwrap())),
                            },
                            balance_change: BalanceChange {
                                magnitude: 1363074194182549713,
                                sgn: 1,
                            },
                            increment_nonce: false,
                            events: Events {
                                data: vec![]
                            },
                            actions: Actions {
                                data: vec![
                                    vec![Field(Fp::from_str("2735378780825238").unwrap()), Field(Fp::from_str("0").unwrap()), Field(Fp::from_str("24223713952929065963034699766849427153124236871346341479419704474863873582581").unwrap()), Field(Fp::from_str("18882675716286521355728950796142293099213759582555789518648321568380249227395").unwrap())],
                                ]
                            },
                            call_data: Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap()),
                            call_depth: 0,
                            preconditions: Preconditions {
                                network: NetworkPreconditions {
                                    snarked_ledger_hash: None,
                                    blockchain_length: None,
                                    min_window_density: None,
                                    total_currency: Some(RangeCondition { lower: 4267409404, upper: 0 }),
                                    global_slot_since_genesis: None,
                                    staking_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: Some(Field(Fp::from_str("0").unwrap())),
                                            total_currency: None,
                                        },
                                        seed: None,
                                        start_checkpoint: Some(Field(Fp::from_str("6535398293941205482627458340529489045211845271941213682324637486141664661702").unwrap())),
                                        lock_checkpoint: None,
                                        epoch_length: Some(RangeCondition { lower: 81746, upper: 1 }),
                                    },
                                    next_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: Some(Field(Fp::from_str("0").unwrap())),
                                            total_currency: Some(RangeCondition { lower: 2, upper: 2 }),
                                        },
                                        seed: Some(Field(Fp::from_str("22851940660430741038239474935503467454811702332945699370971394434804274836896").unwrap())),
                                        start_checkpoint: None,
                                        lock_checkpoint: Some(Field(Fp::from_str("6381616425887994211961993476821283313252790018382043371927962929466788812192").unwrap())),
                                        epoch_length: None,
                                    },
                                },
                                account: AccountPreconditions {
                                    balance: None,
                                    nonce: Some(RangeCondition { lower: 144606575, upper: 2 }),
                                    receipt_chain_hash: Some(Field(Fp::from_str("351268230982500463679410401991").unwrap())),
                                    delegate: None,
                                    state: [
                                            None,
                                            None,
                                            Some(Field(Fp::from_str("22282579452935259196976180723039360578075798029703715632848357927746687096513").unwrap())),
                                            None,
                                            None,
                                            Some(Field(Fp::from_str("1").unwrap())),
                                            Some(Field(Fp::from_str("19724938244883969617861407433465213288391077954799194784324774241229272923222").unwrap())),
                                            None,
                                        ],
                                    action_state: None,
                                    proved_state: Some(false),
                                    is_new: Some(true),
                                },
                                valid_while: Some(RangeCondition { lower: 0, upper: 0 }),
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
                    },
                ],
                memo: decode_memo_from_base58("E4YM2vTHhWEg66xpj52JErHUBU4pZ1yageL4TVDDpTTSsv8mK6YaH"),
            },
            network: NetworkId::TESTNET,
            expected_memo_hash: "146624400929844538317466382872834899021794596262855408933526545768996436172",
            expected_fee_payer_hash: "16319141303399253201343167143754976135556961498754090537628742116097232661579",
            expected_account_updates_commitment: "16249562057446426334345556833548191915495848116073970297942347634622645358062",
            expected_full_commitment: "24871450342322961324860825832188313236159210935763239756835206092378808687558",
        }
]
}

fn decode_memo_from_base58(memo_base58: &str) -> [u8; 34] {
    let memo_bytes = bs58::decode(memo_base58)
        .into_vec()
        .expect("Valid base58 memo");

    memo_bytes[1..memo_bytes.len() - 4]
        .try_into()
        .expect("Memo length matches expected size")
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
