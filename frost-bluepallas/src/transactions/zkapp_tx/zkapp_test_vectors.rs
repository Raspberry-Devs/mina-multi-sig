//! Test vectors for ZkApp transaction commitment functions
//!
//! This module contains shared test data used across different commitment function tests.
//! All test vectors use empty/default data structures - populate with actual test data as needed.

use alloc::{string::ToString, vec::Vec};
use bs58;
use core::str::FromStr;
use mina_hasher::Fp;
use mina_signer::{CompressedPubKey, NetworkId};

use crate::transactions::TransactionEnvelope;

use super::{
    AccountPreconditions, AccountUpdate, AccountUpdateBody, ActionState, Actions, AuthRequired,
    Authorization, AuthorizationKind, BalanceChange, EpochData, EpochLedger, Events, FeePayer,
    FeePayerBody, Field, MayUseToken, NetworkPreconditions, Permissions, Preconditions, PublicKey,
    RangeCondition, SetVerificationKey, TimingData, TokenId, TokenSymbol, Update,
    VerificationKeyData, ZKAppCommand, ZkappUri,
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

impl From<ZkAppTestVector> for TransactionEnvelope {
    fn from(vector: ZkAppTestVector) -> TransactionEnvelope {
        TransactionEnvelope::new_zkapp(vector.network, vector.zkapp_command)
    }
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
            name: "complex_zkapp_command",
            zkapp_command: ZKAppCommand {
                fee_payer: FeePayer {
                    body: FeePayerBody {
                        public_key: PublicKey(CompressedPubKey::from_address("B62qkX8itLmAjvGtC1Viw6Si1cWwANkFxj958H8DdsnCcR98ZsU9ceb").unwrap()),
                        fee: 2,
                        valid_until: None,
                        nonce: 9,
                    },
                    authorization: "7mWxjLYgbJUkZNcGouvhVj5tJ8yu9hoexb9ntvPK8t5LHqzmrL6QJjjKtf5SgmxB4QWkDw7qoMMbbNGtHVpsbJHPyTy2EzRQ".to_string(),
                },
                account_updates: vec![
// Account update 1
                    AccountUpdate {
                        body: AccountUpdateBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qrPoNgE74xmBkpDopdmrv1HRGkpWDmoFpDG8QxLZdrKJW9ZicQCc").unwrap()),
                            token_id: TokenId(Field(Fp::from_str("1").unwrap())),
                            update: Update {
                                app_state: [None, None, None, Some(Field(Fp::from_str("25105").unwrap())), None, None, Some(Field(Fp::from_str("18537964602249661855265151828928769160426956687154688371236567841812022754449").unwrap())), Some(Field(Fp::from_str("48011").unwrap()))],
                                delegate: Some(PublicKey(CompressedPubKey::from_address("B62qokDJ71NiHUg9y8u7AJMnLYLfvsoybjyEmbQFfPnLwXxxpe6BjEM").unwrap())),
                                verification_key: Some(VerificationKeyData {
                                        data: "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".to_string(),
                                        hash: Field(Fp::from_str("3392518251768960475377392625298437850623664973002200885669375116181514017494").unwrap()),
                                    }),
                                permissions: None,
                                zkapp_uri: None,
                                token_symbol: Some(TokenSymbol("".as_bytes().to_vec())),
                                timing: None,
                                voting_for: Some(Field(Fp::from_str("730904656150121144214913717950808411028445154183396618546232938725901981289").unwrap())),
                            },
                            balance_change: BalanceChange {
                                magnitude: 18446744073709551615,
                                sgn: 1,
                            },
                            increment_nonce: true,
                            events: Events {
                                data: vec![]
                            },
                            actions: Actions {
                                data: vec![
                                    vec![Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())],
                                    vec![Field(Fp::from_str("1084483959").unwrap()), Field(Fp::from_str("18011522276971475206716529368463917921983816076585059487487014384480858173290").unwrap()), Field(Fp::from_str("2").unwrap()), Field(Fp::from_str("12686810393491427055300042333018803776909775266091317199197793322608894098020").unwrap()), Field(Fp::from_str("22350045772939879814984214662728757557189645675201885446649001677135622644433").unwrap())],
                                ]
                            },
                            call_data: Field(Fp::from_str("4670016141600497241299216197582256359297180816813920875852695083750233485251").unwrap()),
                            call_depth: 0,
                            preconditions: Preconditions {
                                network: NetworkPreconditions {
                                    snarked_ledger_hash: None,
                                    blockchain_length: Some(RangeCondition { lower: 3367553, upper: 399559988 }),
                                    min_window_density: None,
                                    total_currency: Some(RangeCondition { lower: 0, upper: 2 }),
                                    global_slot_since_genesis: Some(RangeCondition { lower: 44647, upper: 553540 }),
                                    staking_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: Some(Field(Fp::from_str("1").unwrap())),
                                            total_currency: Some(RangeCondition { lower: 4232005208983, upper: 1993887331942805073 }),
                                        },
                                        seed: None,
                                        start_checkpoint: Some(Field(Fp::from_str("8551691513815324128155113273682553456300193355242658631211007401180455349905").unwrap())),
                                        lock_checkpoint: Some(Field(Fp::from_str("9").unwrap())),
                                        epoch_length: Some(RangeCondition { lower: 6837, upper: 28 }),
                                    },
                                    next_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: None,
                                            total_currency: None,
                                        },
                                        seed: Some(Field(Fp::from_str("8750020902194249711532705111822779771664517022978670961502147718286842264203").unwrap())),
                                        start_checkpoint: Some(Field(Fp::from_str("0").unwrap())),
                                        lock_checkpoint: None,
                                        epoch_length: None,
                                    },
                                },
                                account: AccountPreconditions {
                                    balance: None,
                                    nonce: Some(RangeCondition { lower: 79184, upper: 63 }),
                                    receipt_chain_hash: Some(Field(Fp::from_str("21191699883298263006422823142554748231014573704116211684602785011310892024989").unwrap())),
                                    delegate: Some(PublicKey(CompressedPubKey::from_address("B62qirGubpcZsQsjdZKirbmWPPizrh73BfJvPqmfa2MtPssxUAMA7ao").unwrap())),
                                    state: [None, None, Some(Field(Fp::from_str("0").unwrap())), Some(Field(Fp::from_str("0").unwrap())), None, None, None, None],
                                    action_state: Some(ActionState(Field(Fp::from_str("163637140280639863010900713735974192669").unwrap()))),
                                    proved_state: Some(true),
                                    is_new: Some(false),
                                },
                                valid_while: Some(RangeCondition { lower: 2, upper: 0 }),
                            },
                            use_full_commitment: true,
                            implicit_account_creation_fee: false,
                            may_use_token: MayUseToken {
                                parents_own_token: false,
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
                    },// Account update 2
                    AccountUpdate {
                        body: AccountUpdateBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qoDitbuGMutg6Nz5JiehmVfvtcnTqttxJ4fzL8yX1SuPF6tUuLpu").unwrap()),
                            token_id: TokenId(Field(Fp::from_str("162845").unwrap())),
                            update: Update {
                                app_state: [None, Some(Field(Fp::from_str("4818860617370429416329383562473739988920910969577069698828312293370009373288").unwrap())), None, Some(Field(Fp::from_str("0").unwrap())), None, Some(Field(Fp::from_str("8901770897652701922868912684206308351448132115671918244233423975266799715628").unwrap())), Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), Some(Field(Fp::from_str("241996946983").unwrap()))],
                                delegate: None,
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
                                magnitude: 39,
                                sgn: 1,
                            },
                            increment_nonce: true,
                            events: Events {
                                data: vec![
                                    vec![Field(Fp::from_str("14389545885402750351394006706693367722015219703478934688263663251457090895516").unwrap()), Field(Fp::from_str("27389612045616877315129464405394501412947782911616570420908969188372852283090").unwrap())],
                                ]
                            },
                            actions: Actions {
                                data: vec![]
                            },
                            call_data: Field(Fp::from_str("23720830367566401665415010274360515534784540348625327454073685211258996295139").unwrap()),
                            call_depth: 0,
                            preconditions: Preconditions {
                                network: NetworkPreconditions {
                                    snarked_ledger_hash: None,
                                    blockchain_length: None,
                                    min_window_density: None,
                                    total_currency: None,
                                    global_slot_since_genesis: Some(RangeCondition { lower: 4294967295, upper: 10170731 }),
                                    staking_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: Some(Field(Fp::from_str("4569095890697923837264237854655117431245925494697050748652154697884120554153").unwrap())),
                                            total_currency: Some(RangeCondition { lower: 1180559990179017004, upper: 3969572169593 }),
                                        },
                                        seed: Some(Field(Fp::from_str("97200270175").unwrap())),
                                        start_checkpoint: None,
                                        lock_checkpoint: None,
                                        epoch_length: None,
                                    },
                                    next_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: Some(Field(Fp::from_str("5373604327913201568183269282052808405814016194638890793012173303562376536591").unwrap())),
                                            total_currency: None,
                                        },
                                        seed: Some(Field(Fp::from_str("8339017875517482179795821028053168607000431480335084651806802236975010360242").unwrap())),
                                        start_checkpoint: Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())),
                                        lock_checkpoint: None,
                                        epoch_length: Some(RangeCondition { lower: 1, upper: 4067969102 }),
                                    },
                                },
                                account: AccountPreconditions {
                                    balance: Some(RangeCondition { lower: 4151275998, upper: 2 }),
                                    nonce: None,
                                    receipt_chain_hash: Some(Field(Fp::from_str("12289268747890117559755220730604137967513503362584152064993215527813717275300").unwrap())),
                                    delegate: Some(PublicKey(CompressedPubKey::from_address("B62qotoaG8qSVe5RG23EXfCJfCn5qCKPhybhZSWDCD69jiMDmTqJYCo").unwrap())),
                                    state: [Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), None, Some(Field(Fp::from_str("21616506156550872107482999557746067258915772384743336023302224033458602308960").unwrap())), None, None, None, Some(Field(Fp::from_str("115918288728599").unwrap())), None],
                                    action_state: Some(ActionState(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap()))),
                                    proved_state: Some(true),
                                    is_new: Some(true),
                                },
                                valid_while: None,
                            },
                            use_full_commitment: true,
                            implicit_account_creation_fee: true,
                            may_use_token: MayUseToken {
                                parents_own_token: false,
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
                    },// Account update 3
                    AccountUpdate {
                        body: AccountUpdateBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qkX8itLmAjvGtC1Viw6Si1cWwANkFxj958H8DdsnCcR98ZsU9ceb").unwrap()),
                            token_id: TokenId(Field(Fp::from_str("1").unwrap())),
                            update: Update {
                                app_state: [None, None, None, None, None, None, Some(Field(Fp::from_str("16757467784326910763390863080628631989098308338661081254366715945174798479569").unwrap())), Some(Field(Fp::from_str("0").unwrap()))],
                                delegate: None,
                                verification_key: None,
                                permissions: Some(Permissions {
                                        edit_state: AuthRequired::Proof,
                                        access: AuthRequired::Impossible,
                                        send: AuthRequired::Impossible,
                                        receive: AuthRequired::Impossible,
                                        set_delegate: AuthRequired::Either,
                                        set_permissions: AuthRequired::Either,
                                        set_verification_key: SetVerificationKey {
                                            auth: AuthRequired::None,
                                            txn_version: 0,
                                        },
                                        set_zkapp_uri: AuthRequired::Either,
                                        edit_action_state: AuthRequired::Impossible,
                                        set_token_symbol: AuthRequired::Impossible,
                                        increment_nonce: AuthRequired::Impossible,
                                        set_voting_for: AuthRequired::Either,
                                        set_timing: AuthRequired::Either,
                                    }),
                                zkapp_uri: None,
                                token_symbol: None,
                                timing: None,
                                voting_for: None,
                            },
                            balance_change: BalanceChange {
                                magnitude: 3030305645356,
                                sgn: 1,
                            },
                            increment_nonce: false,
                            events: Events {
                                data: vec![
                                    vec![Field(Fp::from_str("788307885077127579329652220796177356937950267282307026010558217024321860750").unwrap()), Field(Fp::from_str("0").unwrap()), Field(Fp::from_str("0").unwrap()), Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap()), Field(Fp::from_str("6807775780499987606555472083988368627603103622407870379744387204898889015688").unwrap())],
                                    vec![Field(Fp::from_str("1858844976259912691062769061504010855277271405246230629288525753769342329291").unwrap())],
                                ]
                            },
                            actions: Actions {
                                data: vec![
                                    vec![Field(Fp::from_str("666324263404").unwrap()), Field(Fp::from_str("21418600859626852119509346253351582275272893589543258038123207286778837519512").unwrap()), Field(Fp::from_str("2657705").unwrap()), Field(Fp::from_str("2999306522636422956369299815925486187117245400982405164734010427727805032344").unwrap())],
                                    vec![Field(Fp::from_str("17964063533351583529153773023048097176674139313505001082722384941672740806254").unwrap()), Field(Fp::from_str("27377755001394789861828287797026005455917757393154704750721417423394547062571").unwrap())],
                                ]
                            },
                            call_data: Field(Fp::from_str("8941503484885784321168325243360").unwrap()),
                            call_depth: 0,
                            preconditions: Preconditions {
                                network: NetworkPreconditions {
                                    snarked_ledger_hash: None,
                                    blockchain_length: None,
                                    min_window_density: Some(RangeCondition { lower: 174, upper: 1 }),
                                    total_currency: Some(RangeCondition { lower: 31037002641868, upper: 9674328 }),
                                    global_slot_since_genesis: Some(RangeCondition { lower: 0, upper: 0 }),
                                    staking_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: None,
                                            total_currency: None,
                                        },
                                        seed: Some(Field(Fp::from_str("1").unwrap())),
                                        start_checkpoint: None,
                                        lock_checkpoint: None,
                                        epoch_length: Some(RangeCondition { lower: 0, upper: 72321381 }),
                                    },
                                    next_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: None,
                                            total_currency: None,
                                        },
                                        seed: None,
                                        start_checkpoint: None,
                                        lock_checkpoint: Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())),
                                        epoch_length: Some(RangeCondition { lower: 868, upper: 1357 }),
                                    },
                                },
                                account: AccountPreconditions {
                                    balance: Some(RangeCondition { lower: 65, upper: 1 }),
                                    nonce: None,
                                    receipt_chain_hash: None,
                                    delegate: Some(PublicKey(CompressedPubKey::from_address("B62qkCnM6zx4tHTy8E2A1VKdcrgUAeydV7kjemLXRjDUk8YQ61f5SD6").unwrap())),
                                    state: [None, None, Some(Field(Fp::from_str("2761263148189846017798272452548971016632299631802393067221503534970399789111").unwrap())), Some(Field(Fp::from_str("4101525148817129745198289792031404926768050918677383549475483249440986658213").unwrap())), None, None, Some(Field(Fp::from_str("1").unwrap())), None],
                                    action_state: None,
                                    proved_state: Some(true),
                                    is_new: Some(true),
                                },
                                valid_while: None,
                            },
                            use_full_commitment: false,
                            implicit_account_creation_fee: true,
                            may_use_token: MayUseToken {
                                parents_own_token: false,
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
                    },// Account update 4
                    AccountUpdate {
                        body: AccountUpdateBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qkcT7L55MZ13tNYXj3AWN1wCzCzYsWVji6seQRujMkATxQQLJoYQ").unwrap()),
                            token_id: TokenId(Field(Fp::from_str("1").unwrap())),
                            update: Update {
                                app_state: [Some(Field(Fp::from_str("10796934224552702979618362115631122037483267946478161437563101919023065453886").unwrap())), Some(Field(Fp::from_str("1665702701348261").unwrap())), Some(Field(Fp::from_str("19520046187718255296557963475173868409759330658774802116230322935726722115976").unwrap())), Some(Field(Fp::from_str("1").unwrap())), Some(Field(Fp::from_str("7312170862030764706254802815327734781795053504536007431702726987329616279353").unwrap())), Some(Field(Fp::from_str("13573309228248164810767138546996638425118106865672570529103058718212908407164").unwrap())), None, None],
                                delegate: Some(PublicKey(CompressedPubKey::from_address("B62qnFL8Z5tmzy9DD7qtBKgKUUZ5CLq7UWWWPW9ooLJXZb7GhgB7ViX").unwrap())),
                                verification_key: Some(VerificationKeyData {
                                        data: "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".to_string(),
                                        hash: Field(Fp::from_str("3392518251768960475377392625298437850623664973002200885669375116181514017494").unwrap()),
                                    }),
                                permissions: None,
                                zkapp_uri: Some(ZkappUri(" \u{0018};>×7Ä\u{001b}ùT.".as_bytes().to_vec())),
                                token_symbol: None,
                                timing: Some(TimingData {
                                        initial_minimum_balance: 3728633945706664709,
                                        cliff_time: 12282,
                                        cliff_amount: 1,
                                        vesting_period: 13092938,
                                        vesting_increment: 1133034378618331073,
                                    }),
                                voting_for: None,
                            },
                            balance_change: BalanceChange {
                                magnitude: 54521,
                                sgn: 1,
                            },
                            increment_nonce: false,
                            events: Events {
                                data: vec![
                                    vec![Field(Fp::from_str("28048383786427492994537461073333392748911566383299414625117388511940693964521").unwrap()), Field(Fp::from_str("8000401666281894935132134775606860865030866610210262436752499311524187930865").unwrap()), Field(Fp::from_str("28810870148825749173797114442563171983229131907930273396283879724892888983668").unwrap()), Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap()), Field(Fp::from_str("0").unwrap())],
                                    vec![Field(Fp::from_str("8989462559799339018873051412633719369082000896075095331035690275537768925544").unwrap())],
                                ]
                            },
                            actions: Actions {
                                data: vec![]
                            },
                            call_data: Field(Fp::from_str("7490649866546718515238753545341769479930365797472880134630000327028080039165").unwrap()),
                            call_depth: 1,
                            preconditions: Preconditions {
                                network: NetworkPreconditions {
                                    snarked_ledger_hash: Some(Field(Fp::from_str("8205019").unwrap())),
                                    blockchain_length: Some(RangeCondition { lower: 4294967295, upper: 3 }),
                                    min_window_density: Some(RangeCondition { lower: 1, upper: 1377887 }),
                                    total_currency: Some(RangeCondition { lower: 1, upper: 18446744073709551615 }),
                                    global_slot_since_genesis: Some(RangeCondition { lower: 1378969044, upper: 0 }),
                                    staking_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: Some(Field(Fp::from_str("1134108913283026455365465700560468162381293165458480311928069693885691063685").unwrap())),
                                            total_currency: Some(RangeCondition { lower: 30129300470, upper: 2835200861991394646 }),
                                        },
                                        seed: Some(Field(Fp::from_str("0").unwrap())),
                                        start_checkpoint: Some(Field(Fp::from_str("0").unwrap())),
                                        lock_checkpoint: Some(Field(Fp::from_str("17963724373401885134953161134596414033").unwrap())),
                                        epoch_length: Some(RangeCondition { lower: 857183953, upper: 14 }),
                                    },
                                    next_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: None,
                                            total_currency: Some(RangeCondition { lower: 492790850673, upper: 88380204 }),
                                        },
                                        seed: Some(Field(Fp::from_str("19257690991050116128398028603031492277042752941393613080353486321748437392288").unwrap())),
                                        start_checkpoint: Some(Field(Fp::from_str("20664131102619957971634997109326932262688408766753233557709363628705464753436").unwrap())),
                                        lock_checkpoint: Some(Field(Fp::from_str("1").unwrap())),
                                        epoch_length: None,
                                    },
                                },
                                account: AccountPreconditions {
                                    balance: None,
                                    nonce: None,
                                    receipt_chain_hash: None,
                                    delegate: None,
                                    state: [Some(Field(Fp::from_str("20643003908031755631913546729523648304987353909912957036192196812302348665462").unwrap())), Some(Field(Fp::from_str("0").unwrap())), Some(Field(Fp::from_str("2677829238679873649744629682533117437173604932784022940143379723833923405541").unwrap())), Some(Field(Fp::from_str("0").unwrap())), Some(Field(Fp::from_str("5141737504652661554922022073957456622197989447805206685189831699900071478323").unwrap())), Some(Field(Fp::from_str("27032487855199755283510254736774481969704539567648794815613661563922322398627").unwrap())), Some(Field(Fp::from_str("1").unwrap())), None],
                                    action_state: Some(ActionState(Field(Fp::from_str("25079927036070901246064867767436987657692091363973573142121686150614948079097").unwrap()))),
                                    proved_state: None,
                                    is_new: None,
                                },
                                valid_while: None,
                            },
                            use_full_commitment: false,
                            implicit_account_creation_fee: false,
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
                    },
                ],
                memo: decode_memo_from_base58("E4YM2vTHhWEg66xpj52JErHUBU4pZ1yageL4TVDDpTTSsv8mK6YaH"),
            },
            network: NetworkId::MAINNET,
            expected_memo_hash: "146624400929844538317466382872834899021794596262855408933526545768996436172",
            expected_fee_payer_hash: "13599943768324718842045631264846766373395338777851778014399949785850374160876",
            expected_account_updates_commitment: "18157416997738750511548808808606476591395902704575661339284356171982987558452",
            expected_full_commitment: "11465652597178027662545139048165778162608334013746722764317712703090464610236",
        },
        ZkAppTestVector {
            name: "double_account_update_with_events",
            zkapp_command: ZKAppCommand {
                fee_payer: FeePayer {
                    body: FeePayerBody {
                        public_key: PublicKey(CompressedPubKey::from_address("B62qppyJBbq3bXjkpRTKQWrRurNJKUCKsVGXWdVpEtxvk7UJbj6uzUt").unwrap()),
                        fee: 151,
                        valid_until: Some(0),
                        nonce: 1928082,
                    },
                    authorization: "7mWxjLYgbJUkZNcGouvhVj5tJ8yu9hoexb9ntvPK8t5LHqzmrL6QJjjKtf5SgmxB4QWkDw7qoMMbbNGtHVpsbJHPyTy2EzRQ".to_string(),
                },
                account_updates: vec![
// Account update 1
                    AccountUpdate {
                        body: AccountUpdateBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qj7GVkFu6FtpAVTmLc4LmzYgt8Afm7CTBCDkoPPUw7vU6f7hpMG7").unwrap()),
                            token_id: TokenId(Field(Fp::from_str("1").unwrap())),
                            update: Update {
                                app_state: [None, None, None, Some(Field(Fp::from_str("8819888681218165626446829170481097373073529465021744751038551417536043148673").unwrap())), None, None, Some(Field(Fp::from_str("1692330205296585158450366281162692382719215091321008255255817727117989698655").unwrap())), None],
                                delegate: Some(PublicKey(CompressedPubKey::from_address("B62qocNdwPaHkTtSB87HLb6RUpSm62YX3Km2d1UiiwKDeZ122qginSZ").unwrap())),
                                verification_key: None,
                                permissions: None,
                                zkapp_uri: None,
                                token_symbol: Some(TokenSymbol("@".as_bytes().to_vec())),
                                timing: None,
                                voting_for: Some(Field(Fp::from_str("12990381296866422932578728271658285").unwrap())),
                            },
                            balance_change: BalanceChange {
                                magnitude: 111,
                                sgn: -1,
                            },
                            increment_nonce: false,
                            events: Events {
                                data: vec![
                                    vec![Field(Fp::from_str("562408180161977288896337561286080521093963918674847434255053002192155663164").unwrap())],
                                    vec![Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap()), Field(Fp::from_str("1").unwrap()), Field(Fp::from_str("29035303265434039087799091422").unwrap())],
                                ]
                            },
                            actions: Actions {
                                data: vec![]
                            },
                            call_data: Field(Fp::from_str("3910415258479541505554893188524842").unwrap()),
                            call_depth: 0,
                            preconditions: Preconditions {
                                network: NetworkPreconditions {
                                    snarked_ledger_hash: Some(Field(Fp::from_str("17832250625107159026622621").unwrap())),
                                    blockchain_length: None,
                                    min_window_density: Some(RangeCondition { lower: 1622547766, upper: 12483700 }),
                                    total_currency: None,
                                    global_slot_since_genesis: None,
                                    staking_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: Some(Field(Fp::from_str("51821503901769895").unwrap())),
                                            total_currency: None,
                                        },
                                        seed: None,
                                        start_checkpoint: None,
                                        lock_checkpoint: None,
                                        epoch_length: None,
                                    },
                                    next_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: None,
                                            total_currency: Some(RangeCondition { lower: 5492, upper: 9240 }),
                                        },
                                        seed: None,
                                        start_checkpoint: None,
                                        lock_checkpoint: Some(Field(Fp::from_str("23620217203579031836388864172183398476037989218729185450324161327256194641491").unwrap())),
                                        epoch_length: Some(RangeCondition { lower: 3, upper: 14 }),
                                    },
                                },
                                account: AccountPreconditions {
                                    balance: None,
                                    nonce: Some(RangeCondition { lower: 131981827, upper: 0 }),
                                    receipt_chain_hash: Some(Field(Fp::from_str("12611067225855484915935185642780160742344057998944763254977936423282972300986").unwrap())),
                                    delegate: None,
                                    state: [None, None, None, None, None, None, None, None],
                                    action_state: Some(ActionState(Field(Fp::from_str("0").unwrap()))),
                                    proved_state: None,
                                    is_new: None,
                                },
                                valid_while: Some(RangeCondition { lower: 0, upper: 11083 }),
                            },
                            use_full_commitment: true,
                            implicit_account_creation_fee: false,
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
                    },// Account update 2
                    AccountUpdate {
                        body: AccountUpdateBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qqxKu6uAH8YE4ruCVEHiFrGEA8yfkgoypyziLPRJyNQhSZko6e6j").unwrap()),
                            token_id: TokenId(Field(Fp::from_str("6146106391706486453845129162516639816604601077972282898764679643756409268549").unwrap())),
                            update: Update {
                                app_state: [None, None, None, None, None, Some(Field(Fp::from_str("1").unwrap())), None, Some(Field(Fp::from_str("323439875152756399374005967069376665314565730243793527987318640003836125956").unwrap()))],
                                delegate: None,
                                verification_key: Some(VerificationKeyData {
                                        data: "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".to_string(),
                                        hash: Field(Fp::from_str("3392518251768960475377392625298437850623664973002200885669375116181514017494").unwrap()),
                                    }),
                                permissions: Some(Permissions {
                                        edit_state: AuthRequired::None,
                                        access: AuthRequired::Signature,
                                        send: AuthRequired::Either,
                                        receive: AuthRequired::None,
                                        set_delegate: AuthRequired::Either,
                                        set_permissions: AuthRequired::Either,
                                        set_verification_key: SetVerificationKey {
                                            auth: AuthRequired::Either,
                                            txn_version: 37676,
                                        },
                                        set_zkapp_uri: AuthRequired::Proof,
                                        edit_action_state: AuthRequired::Signature,
                                        set_token_symbol: AuthRequired::Either,
                                        increment_nonce: AuthRequired::Impossible,
                                        set_voting_for: AuthRequired::Signature,
                                        set_timing: AuthRequired::Signature,
                                    }),
                                zkapp_uri: None,
                                token_symbol: None,
                                timing: None,
                                voting_for: Some(Field(Fp::from_str("19608909489600594446776217430597835525198526882652587387280316089853339299158").unwrap())),
                            },
                            balance_change: BalanceChange {
                                magnitude: 18446744073709551615,
                                sgn: -1,
                            },
                            increment_nonce: true,
                            events: Events {
                                data: vec![]
                            },
                            actions: Actions {
                                data: vec![]
                            },
                            call_data: Field(Fp::from_str("4551716132475598677738840472488214637458989517970441192584623411570370054398").unwrap()),
                            call_depth: 1,
                            preconditions: Preconditions {
                                network: NetworkPreconditions {
                                    snarked_ledger_hash: Some(Field(Fp::from_str("21780810669622180198827412605752019112694213974273572536627080129337651111000").unwrap())),
                                    blockchain_length: None,
                                    min_window_density: Some(RangeCondition { lower: 1, upper: 4294967295 }),
                                    total_currency: Some(RangeCondition { lower: 1, upper: 791419 }),
                                    global_slot_since_genesis: Some(RangeCondition { lower: 1592950278, upper: 0 }),
                                    staking_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: Some(Field(Fp::from_str("0").unwrap())),
                                            total_currency: Some(RangeCondition { lower: 54317587316106, upper: 2 }),
                                        },
                                        seed: None,
                                        start_checkpoint: Some(Field(Fp::from_str("1").unwrap())),
                                        lock_checkpoint: Some(Field(Fp::from_str("15978213110436055127252479336795396787655578520537771434072403615386456001953").unwrap())),
                                        epoch_length: None,
                                    },
                                    next_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: Some(Field(Fp::from_str("13720321199042430609594319210292573869541827991514013501061000440117311411445").unwrap())),
                                            total_currency: None,
                                        },
                                        seed: None,
                                        start_checkpoint: None,
                                        lock_checkpoint: Some(Field(Fp::from_str("710854467491030399989232252308792998948313006762629017627744695857927968609").unwrap())),
                                        epoch_length: None,
                                    },
                                },
                                account: AccountPreconditions {
                                    balance: None,
                                    nonce: Some(RangeCondition { lower: 2, upper: 1441884673 }),
                                    receipt_chain_hash: None,
                                    delegate: Some(PublicKey(CompressedPubKey::from_address("B62qqUYQSrFYwGzLauwLWHUi6dN7TQQcCPs32zu3Ts6Vd3VxwPjEJvq").unwrap())),
                                    state: [Some(Field(Fp::from_str("18918325975863313496709857743978177716638145595943124736161520517413057154437").unwrap())), Some(Field(Fp::from_str("312624389692019").unwrap())), None, Some(Field(Fp::from_str("2718746757160").unwrap())), None, None, None, Some(Field(Fp::from_str("1").unwrap()))],
                                    action_state: Some(ActionState(Field(Fp::from_str("16114744759163440918180162018664357525802952947202795285482829361715282711734").unwrap()))),
                                    proved_state: None,
                                    is_new: Some(false),
                                },
                                valid_while: None,
                            },
                            use_full_commitment: false,
                            implicit_account_creation_fee: true,
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
                    },
                ],
                memo: decode_memo_from_base58("E4YM2vTHhWEg66xpj52JErHUBU4pZ1yageL4TVDDpTTSsv8mK6YaH"),
            },
            network: NetworkId::MAINNET,
            expected_memo_hash: "146624400929844538317466382872834899021794596262855408933526545768996436172",
            expected_fee_payer_hash: "18557821528466210466362221959818444224122964900994620153505407721900866383710",
            expected_account_updates_commitment: "692918817100816493287714686901447970091023252434376857771101589716855696298",
            expected_full_commitment: "74048136809654753455198087562447614502111152254006883311057305232424176558",
        },
        ZkAppTestVector {
            name: "single_account_update_testnet",
            zkapp_command: ZKAppCommand {
                fee_payer: FeePayer {
                    body: FeePayerBody {
                        public_key: PublicKey(CompressedPubKey::from_address("B62qiUPy8wfB76vfLhwBUiYapDykGLk4ExhWDsYdUHSwQrHiycvMs2s").unwrap()),
                        fee: 69039324267186314,
                        valid_until: Some(5070715),
                        nonce: 97154,
                    },
                    authorization: "7mWxjLYgbJUkZNcGouvhVj5tJ8yu9hoexb9ntvPK8t5LHqzmrL6QJjjKtf5SgmxB4QWkDw7qoMMbbNGtHVpsbJHPyTy2EzRQ".to_string(),
                },
                account_updates: vec![
// Account update 1
                    AccountUpdate {
                        body: AccountUpdateBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qn2Tm2iK8CKE1bLewuBWWDqvW3g5S2gLyNhcLpDuy9r8DmEaCDdq").unwrap()),
                            token_id: TokenId(Field(Fp::from_str("983242309351426044275241150").unwrap())),
                            update: Update {
                                app_state: [None, None, None, Some(Field(Fp::from_str("3496483045982956626344235296784258884962324970175121121486871437306983836437").unwrap())), Some(Field(Fp::from_str("28328378550802158595477611399583852661215863011907495366080017130083848444623").unwrap())), Some(Field(Fp::from_str("1").unwrap())), Some(Field(Fp::from_str("5281406273745783538451").unwrap())), Some(Field(Fp::from_str("1").unwrap()))],
                                delegate: None,
                                verification_key: Some(VerificationKeyData {
                                        data: "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".to_string(),
                                        hash: Field(Fp::from_str("3392518251768960475377392625298437850623664973002200885669375116181514017494").unwrap()),
                                    }),
                                permissions: Some(Permissions {
                                        edit_state: AuthRequired::None,
                                        access: AuthRequired::Either,
                                        send: AuthRequired::Either,
                                        receive: AuthRequired::Either,
                                        set_delegate: AuthRequired::Impossible,
                                        set_permissions: AuthRequired::None,
                                        set_verification_key: SetVerificationKey {
                                            auth: AuthRequired::None,
                                            txn_version: 262711,
                                        },
                                        set_zkapp_uri: AuthRequired::None,
                                        edit_action_state: AuthRequired::Proof,
                                        set_token_symbol: AuthRequired::Either,
                                        increment_nonce: AuthRequired::Signature,
                                        set_voting_for: AuthRequired::Impossible,
                                        set_timing: AuthRequired::None,
                                    }),
                                zkapp_uri: Some(ZkappUri("C".as_bytes().to_vec())),
                                token_symbol: Some(TokenSymbol("".as_bytes().to_vec())),
                                timing: Some(TimingData {
                                        initial_minimum_balance: 181413,
                                        cliff_time: 2138,
                                        cliff_amount: 2888735079,
                                        vesting_period: 17259962,
                                        vesting_increment: 2,
                                    }),
                                voting_for: None,
                            },
                            balance_change: BalanceChange {
                                magnitude: 216439,
                                sgn: 1,
                            },
                            increment_nonce: true,
                            events: Events {
                                data: vec![]
                            },
                            actions: Actions {
                                data: vec![]
                            },
                            call_data: Field(Fp::from_str("13940844251910663436331889860060718171445830904412681518568422235019737090171").unwrap()),
                            call_depth: 0,
                            preconditions: Preconditions {
                                network: NetworkPreconditions {
                                    snarked_ledger_hash: Some(Field(Fp::from_str("24970569756830890907032583414011800560825949545614573312036245161921365580155").unwrap())),
                                    blockchain_length: Some(RangeCondition { lower: 1832279324, upper: 15 }),
                                    min_window_density: None,
                                    total_currency: Some(RangeCondition { lower: 1108052, upper: 121104388238 }),
                                    global_slot_since_genesis: Some(RangeCondition { lower: 1, upper: 1 }),
                                    staking_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: Some(Field(Fp::from_str("1").unwrap())),
                                            total_currency: None,
                                        },
                                        seed: Some(Field(Fp::from_str("1069664032282738273837302310296469895222139999747098872502886640413086011037").unwrap())),
                                        start_checkpoint: Some(Field(Fp::from_str("2206947745596275636271014387481303219667876497705740677468373873041316013377").unwrap())),
                                        lock_checkpoint: Some(Field(Fp::from_str("13894592038331160844841226371684122072072682417678163406966544621591962919540").unwrap())),
                                        epoch_length: Some(RangeCondition { lower: 1, upper: 1 }),
                                    },
                                    next_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: Some(Field(Fp::from_str("0").unwrap())),
                                            total_currency: None,
                                        },
                                        seed: None,
                                        start_checkpoint: Some(Field(Fp::from_str("11103193587320135865201097212279431597669221538126176806032109027788707580455").unwrap())),
                                        lock_checkpoint: None,
                                        epoch_length: Some(RangeCondition { lower: 48486, upper: 218 }),
                                    },
                                },
                                account: AccountPreconditions {
                                    balance: Some(RangeCondition { lower: 866, upper: 0 }),
                                    nonce: Some(RangeCondition { lower: 5321269, upper: 2 }),
                                    receipt_chain_hash: Some(Field(Fp::from_str("2886782260929269424992032700419650791772721097705349472210579774432180382123").unwrap())),
                                    delegate: Some(PublicKey(CompressedPubKey::from_address("B62qkGP6ayDU7uByoSCiGnavpJNvEtqSAZB2TVozmRqo4fVUu3DF3es").unwrap())),
                                    state: [None, Some(Field(Fp::from_str("1").unwrap())), None, Some(Field(Fp::from_str("0").unwrap())), None, Some(Field(Fp::from_str("481675703").unwrap())), Some(Field(Fp::from_str("880259374099972146003142351257535").unwrap())), None],
                                    action_state: Some(ActionState(Field(Fp::from_str("27335263573332287112997735065213262568906234416835184540680538945325656448403").unwrap()))),
                                    proved_state: None,
                                    is_new: Some(false),
                                },
                                valid_while: Some(RangeCondition { lower: 1028632684, upper: 0 }),
                            },
                            use_full_commitment: false,
                            implicit_account_creation_fee: false,
                            may_use_token: MayUseToken {
                                parents_own_token: false,
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
                ],
                memo: decode_memo_from_base58("E4YM2vTHhWEg66xpj52JErHUBU4pZ1yageL4TVDDpTTSsv8mK6YaH"),
            },
            network: NetworkId::TESTNET,
            expected_memo_hash: "146624400929844538317466382872834899021794596262855408933526545768996436172",
            expected_fee_payer_hash: "22591001022197211780755176808690376369488425396145396343616148937337644278803",
            expected_account_updates_commitment: "890036752761563610176645072391048796311042767532971976592942060327069338763",
            expected_full_commitment: "7386435648427501000342780996487927693328565178920002003169322679773714537812",
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
