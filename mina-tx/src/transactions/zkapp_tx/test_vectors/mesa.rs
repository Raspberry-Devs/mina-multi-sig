//! Test vectors for ZkApp transaction commitment functions (Mesa hardfork, 32 state fields)

use alloc::{string::ToString, vec::Vec};
use core::str::FromStr;
use mina_hasher::Fp;
use mina_signer::CompressedPubKey;

use crate::transactions::network_id::NetworkId;

use super::common::{decode_memo_from_base58, ZkAppTestVector};

use crate::transactions::zkapp_tx::{
    AccountPreconditions, AccountUpdate, AccountUpdateBody, ActionState, Actions, AuthRequired,
    Authorization, AuthorizationKind, BalanceChange, EpochData, EpochLedger, Events, FeePayer,
    FeePayerBody, Field, MayUseToken, NetworkPreconditions, Permissions, Preconditions, PublicKey,
    RangeCondition, SetVerificationKey, StringU32, StringU64, TimingData, TokenId, TokenSymbol,
    Update, VerificationKeyData, ZKAppCommand, ZkappUri,
};

/// Returns the main test vectors for ZkApp commitment functions (Mesa hardfork)
pub fn get_zkapp_test_vectors() -> Vec<ZkAppTestVector> {
    vec![
            ZkAppTestVector {
            name: "mesa_payment_zkapp",
            zkapp_command: ZKAppCommand {
                fee_payer: FeePayer {
                    body: FeePayerBody {
                        public_key: PublicKey(CompressedPubKey::from_address("B62qnXjEzgdcfEHuMPSMTvAojnCV3nUxM4wKRT7PcNPAyYCvKtoQTUs").unwrap()),
                        fee: 1000000,
                        valid_until: None,
                        nonce: 1,
                    },
                    authorization: "7mWxjLYgbJUkZNcGouvhVj5tJ8yu9hoexb9ntvPK8t5LHqzmrL6QJjjKtf5SgmxB4QWkDw7qoMMbbNGtHVpsbJHPyTy2EzRQ".to_string(),
                },
                account_updates: vec![
// Account update 1
                    AccountUpdate {
                        body: AccountUpdateBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qnXjEzgdcfEHuMPSMTvAojnCV3nUxM4wKRT7PcNPAyYCvKtoQTUs").unwrap()),
                            token_id: serde_json::from_str::<TokenId>(r#""wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf""#).unwrap(),
                            update: Update {
                                app_state: [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                                delegate: None,
                                verification_key: None,
                                permissions: None,
                                zkapp_uri: None,
                                token_symbol: None,
                                timing: None,
                                voting_for: None,
                            },
                            balance_change: BalanceChange {
                                magnitude: 1000000000,
                                sgn: -1,
                            },
                            increment_nonce: false,
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
                                    snarked_ledger_hash: None,
                                    blockchain_length: None,
                                    min_window_density: None,
                                    total_currency: None,
                                    global_slot_since_genesis: None,
                                    staking_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: None,
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
                                            total_currency: None,
                                        },
                                        seed: None,
                                        start_checkpoint: None,
                                        lock_checkpoint: None,
                                        epoch_length: None,
                                    },
                                },
                                account: AccountPreconditions {
                                    balance: None,
                                    nonce: None,
                                    receipt_chain_hash: None,
                                    delegate: None,
                                    state: [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                                    action_state: None,
                                    proved_state: None,
                                    is_new: None,
                                },
                                valid_while: None,
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
                    },// Account update 2
                    AccountUpdate {
                        body: AccountUpdateBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qnicPfRtrnWei7XoW16DPGsJAd2VkYDYdCUaUUbuDmN7vfrm6nQh").unwrap()),
                            token_id: serde_json::from_str::<TokenId>(r#""wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf""#).unwrap(),
                            update: Update {
                                app_state: [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                                delegate: None,
                                verification_key: None,
                                permissions: None,
                                zkapp_uri: None,
                                token_symbol: None,
                                timing: None,
                                voting_for: None,
                            },
                            balance_change: BalanceChange {
                                magnitude: 1000000000,
                                sgn: 1,
                            },
                            increment_nonce: false,
                            events: Events {
                                data: vec![]
                            },
                            actions: Actions {
                                data: vec![]
                            },
                            call_data: Field(Fp::from_str("0").unwrap()),
                            call_depth: 1,
                            preconditions: Preconditions {
                                network: NetworkPreconditions {
                                    snarked_ledger_hash: None,
                                    blockchain_length: None,
                                    min_window_density: None,
                                    total_currency: None,
                                    global_slot_since_genesis: None,
                                    staking_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: None,
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
                                            total_currency: None,
                                        },
                                        seed: None,
                                        start_checkpoint: None,
                                        lock_checkpoint: None,
                                        epoch_length: None,
                                    },
                                },
                                account: AccountPreconditions {
                                    balance: None,
                                    nonce: None,
                                    receipt_chain_hash: None,
                                    delegate: None,
                                    state: [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                                    action_state: None,
                                    proved_state: None,
                                    is_new: None,
                                },
                                valid_while: None,
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
                    },
                ],
                memo: decode_memo_from_base58("E4YM2vTHhWEg66xpj52JErHUBU4pZ1yageL4TVDDpTTSsv8mK6YaH"),
            },
            network: NetworkId::Testnet,
            expected_memo_hash: "146624400929844538317466382872834899021794596262855408933526545768996436172",
            expected_fee_payer_hash: "25189535564774509369038038931761428017241366441458702340293237719431446482235",
            expected_account_updates_commitment: "28168929877205958189735813515633987632261907762343340341558140242804898808112",
            expected_full_commitment: "19569355434780774506699780558321519970207445688582468990232012498933912808789",
        },
        ZkAppTestVector {
            name: "empty_account_update",
            zkapp_command: ZKAppCommand {
                fee_payer: FeePayer {
                    body: FeePayerBody {
                        public_key: PublicKey(CompressedPubKey::from_address("B62qqnU5UgAeUfY38XAhHKq1LEmPwQgrT4FceY9DgnXPF9TzwoDz35F").unwrap()),
                        fee: 0,
                        valid_until: Some(10354),
                        nonce: 50211193,
                    },
                    authorization: "7mWxjLYgbJUkZNcGouvhVj5tJ8yu9hoexb9ntvPK8t5LHqzmrL6QJjjKtf5SgmxB4QWkDw7qoMMbbNGtHVpsbJHPyTy2EzRQ".to_string(),
                },
                account_updates: vec![],
                memo: decode_memo_from_base58("E4YM2vTHhWEg66xpj52JErHUBU4pZ1yageL4TVDDpTTSsv8mK6YaH"),
            },
            network: NetworkId::Testnet,
            expected_memo_hash: "146624400929844538317466382872834899021794596262855408933526545768996436172",
            expected_fee_payer_hash: "24861274824563484419248414663531129885460350024752354102115688757629568100619",
            expected_account_updates_commitment: "0",
            expected_full_commitment: "2548699398583139309966585890970411557281770987519695977171116982512518766594",
        },
        ZkAppTestVector {
            name: "single_account_update",
            zkapp_command: ZKAppCommand {
                fee_payer: FeePayer {
                    body: FeePayerBody {
                        public_key: PublicKey(CompressedPubKey::from_address("B62qkbpu4db4DvsA9wrDC9XTFPLweWdf5YvaG7gSQgPUiVaDeAtmsR9").unwrap()),
                        fee: 3754326370,
                        valid_until: None,
                        nonce: 5992,
                    },
                    authorization: "7mWxjLYgbJUkZNcGouvhVj5tJ8yu9hoexb9ntvPK8t5LHqzmrL6QJjjKtf5SgmxB4QWkDw7qoMMbbNGtHVpsbJHPyTy2EzRQ".to_string(),
                },
                account_updates: vec![
// Account update 1
                    AccountUpdate {
                        body: AccountUpdateBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qiVFUKQGE9qM7CwK1A55avSEnDvFRNuVGKSsnHirNz3YQ7ZbEh8W").unwrap()),
                            token_id: serde_json::from_str::<TokenId>(r#""xJ5ZSUQdt6iQgeWJjw1KR48cTaPABdAFiPYTbAJvAtaSm8QSdu""#).unwrap(),
                            update: Update {
                                app_state: [None, None, None, Some(Field(Fp::from_str("5517904617716878").unwrap())), None, None, None, None, Some(Field(Fp::from_str("10059573546544673006757854724801901714042478703356393742537187220431205264706").unwrap())), None, Some(Field(Fp::from_str("23120202922485363071121119459294604173885929181247624592191845720446873762670").unwrap())), None, Some(Field(Fp::from_str("4520171645855641585018022236212706751").unwrap())), Some(Field(Fp::from_str("1200317200").unwrap())), None, Some(Field(Fp::from_str("1").unwrap())), Some(Field(Fp::from_str("0").unwrap())), Some(Field(Fp::from_str("1").unwrap())), Some(Field(Fp::from_str("24867457671570386429093721863878866675014631453516056545076000935872085982117").unwrap())), Some(Field(Fp::from_str("1").unwrap())), Some(Field(Fp::from_str("3443963956508173899381986796002621626298017911718153197999631910683156682672").unwrap())), None, Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), Some(Field(Fp::from_str("7089434870388083842213534830163660401170393287853259918055140661648048795136").unwrap())), Some(Field(Fp::from_str("28466768222680581254854040292690791238219948162790063752941199613622954180192").unwrap())), Some(Field(Fp::from_str("4572412300797333256820271175").unwrap())), None, None, None, None, None, Some(Field(Fp::from_str("6171466459091477607904733906327151045536088803532839719365197270238354894178").unwrap()))],
                                delegate: Some(PublicKey(CompressedPubKey::from_address("B62qj3GdJA7DSBrqnLNwyu1j2fky5SpmsjymHa6PReTuHwHhtAG1fzC").unwrap())),
                                verification_key: None,
                                permissions: Some(Permissions {
                                        edit_state: AuthRequired::Proof,
                                        access: AuthRequired::Signature,
                                        send: AuthRequired::None,
                                        receive: AuthRequired::Proof,
                                        set_delegate: AuthRequired::Either,
                                        set_permissions: AuthRequired::Impossible,
                                        set_verification_key: SetVerificationKey {
                                            auth: AuthRequired::Signature,
                                            txn_version: 11,
                                        },
                                        set_zkapp_uri: AuthRequired::Signature,
                                        edit_action_state: AuthRequired::Impossible,
                                        set_token_symbol: AuthRequired::Signature,
                                        increment_nonce: AuthRequired::Impossible,
                                        set_voting_for: AuthRequired::None,
                                        set_timing: AuthRequired::Impossible,
                                    }),
                                zkapp_uri: None,
                                token_symbol: Some(TokenSymbol(vec![195, 131])),
                                timing: Some(TimingData {
                                        initial_minimum_balance: 531337526,
                                        cliff_time: 1603412,
                                        cliff_amount: 969,
                                        vesting_period: 1323900,
                                        vesting_increment: 86191705454571,
                                    }),
                                voting_for: Some(Field(Fp::from_str("0").unwrap())),
                            },
                            balance_change: BalanceChange {
                                magnitude: 61037817627290,
                                sgn: 1,
                            },
                            increment_nonce: true,
                            events: Events {
                                data: vec![
                                    vec![Field(Fp::from_str("0").unwrap())],
                                ]
                            },
                            actions: Actions {
                                data: vec![
                                    vec![Field(Fp::from_str("4055061354903129884252152382346751").unwrap()), Field(Fp::from_str("8934723182275869585296174898626456194150626589107658048713125303325507516376").unwrap()), Field(Fp::from_str("0").unwrap()), Field(Fp::from_str("17688307176983968710442116704382835351025538767531752975584519435931878584131").unwrap())],
                                ]
                            },
                            call_data: Field(Fp::from_str("7176781108702355911290995644590123623861394049568006845625837553403098320318").unwrap()),
                            call_depth: 0,
                            preconditions: Preconditions {
                                network: NetworkPreconditions {
                                    snarked_ledger_hash: Some(Field(Fp::from_str("22574298793542146784695166055588366312483520855911099187411679627366477026073").unwrap())),
                                    blockchain_length: None,
                                    min_window_density: Some(RangeCondition { lower: StringU32(324385), upper: StringU32(7) }),
                                    total_currency: Some(RangeCondition { lower: StringU64(18446744073709551615), upper: StringU64(0) }),
                                    global_slot_since_genesis: Some(RangeCondition { lower: StringU32(1770), upper: StringU32(4294967295) }),
                                    staking_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: Some(Field(Fp::from_str("1").unwrap())),
                                            total_currency: None,
                                        },
                                        seed: Some(Field(Fp::from_str("1460716788966521801494569865180730").unwrap())),
                                        start_checkpoint: Some(Field(Fp::from_str("9455412648369554275807325454418158059131250040044286234694049496840392388262").unwrap())),
                                        lock_checkpoint: Some(Field(Fp::from_str("9130700541590621364285118037139721337845457368360385025772368385915829780521").unwrap())),
                                        epoch_length: Some(RangeCondition { lower: StringU32(784133), upper: StringU32(8) }),
                                    },
                                    next_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: None,
                                            total_currency: None,
                                        },
                                        seed: Some(Field(Fp::from_str("9122843277759204094271363118792078044198116318920318580138123645503651688971").unwrap())),
                                        start_checkpoint: Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())),
                                        lock_checkpoint: None,
                                        epoch_length: Some(RangeCondition { lower: StringU32(458), upper: StringU32(1) }),
                                    },
                                },
                                account: AccountPreconditions {
                                    balance: Some(RangeCondition { lower: StringU64(36265874431633), upper: StringU64(2026617) }),
                                    nonce: None,
                                    receipt_chain_hash: Some(Field(Fp::from_str("0").unwrap())),
                                    delegate: None,
                                    state: [Some(Field(Fp::from_str("1").unwrap())), Some(Field(Fp::from_str("9364506587449525681492321456405").unwrap())), None, None, None, None, Some(Field(Fp::from_str("246896").unwrap())), Some(Field(Fp::from_str("5900180105037216324694733166031976438893513034591370379214545231840839617202").unwrap())), Some(Field(Fp::from_str("7754686156482822104877781003629065499601836019949597881810482022545245516378").unwrap())), Some(Field(Fp::from_str("1").unwrap())), Some(Field(Fp::from_str("1").unwrap())), None, Some(Field(Fp::from_str("0").unwrap())), Some(Field(Fp::from_str("17396348621388767111319487382696816793718941212601067474605326755070247828213").unwrap())), Some(Field(Fp::from_str("1").unwrap())), None, Some(Field(Fp::from_str("20680223479005188035228345974890791163617681285448447369110763946370318031984").unwrap())), None, Some(Field(Fp::from_str("245155802465528864462016054581").unwrap())), None, Some(Field(Fp::from_str("184334581090110302").unwrap())), Some(Field(Fp::from_str("8673969359401835884314713470520982194502710876012985336869628055346651522663").unwrap())), Some(Field(Fp::from_str("340282366920938463463374607431768211455").unwrap())), None, None, Some(Field(Fp::from_str("1").unwrap())), None, None, Some(Field(Fp::from_str("13127080745029158997317488239559141406385328060621997365469598930344167609876").unwrap())), Some(Field(Fp::from_str("190707665006559785").unwrap())), None, Some(Field(Fp::from_str("11059074008992352552456435011423103829815408072467414771701160320143951581400").unwrap()))],
                                    action_state: Some(ActionState(Field(Fp::from_str("340282366920938463463374607431768211455").unwrap()))),
                                    proved_state: Some(false),
                                    is_new: Some(false),
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
                memo: decode_memo_from_base58("E4YSQYncDt58ZnfiJduGYougdjq2LYT3SSEbiSqhExPtJ2xJeqVaZ"),
            },
            network: NetworkId::Mainnet,
            expected_memo_hash: "10276283370827338429603254516537101768117034204151574204931909157851906964905",
            expected_fee_payer_hash: "27633637254146125327993905284498298629036359366739059425468263830880770054843",
            expected_account_updates_commitment: "26653948808721673534480931365747952672463343584607116989925135404362201892133",
            expected_full_commitment: "23571309785221392308360019044710394860233352437949664841283234135115527932143",
        },
        ZkAppTestVector {
            name: "5_account_updates_mainnet",
            zkapp_command: ZKAppCommand {
                fee_payer: FeePayer {
                    body: FeePayerBody {
                        public_key: PublicKey(CompressedPubKey::from_address("B62qoKHeLQromehPsn48MNXTjiM9Hyy1CCecomS6F2ER4bgxKmfpETw").unwrap()),
                        fee: 357293342,
                        valid_until: None,
                        nonce: 459,
                    },
                    authorization: "7mWxjLYgbJUkZNcGouvhVj5tJ8yu9hoexb9ntvPK8t5LHqzmrL6QJjjKtf5SgmxB4QWkDw7qoMMbbNGtHVpsbJHPyTy2EzRQ".to_string(),
                },
                account_updates: vec![
// Account update 1
                    AccountUpdate {
                        body: AccountUpdateBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qop2xL6T5ELXVnuo8dp2adTPDirZMEJYtYnerhytzBtKMDzDMSZq").unwrap()),
                            token_id: serde_json::from_str::<TokenId>(r#""wRqwVaApoAsjZEnsfWaYuQDYVHwThQVpeW5WbuKGAXsgfFsVyk""#).unwrap(),
                            update: Update {
                                app_state: [None, None, None, None, None, Some(Field(Fp::from_str("1").unwrap())), Some(Field(Fp::from_str("25024932397798367287823532514270016018982906912386465915506334600444716573452").unwrap())), Some(Field(Fp::from_str("20224439999776075651969570093084153156606671215492992247571970259178674852780").unwrap())), None, None, Some(Field(Fp::from_str("24103875492381547109045203490931031731119920349482299990903995224454152368044").unwrap())), Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), Some(Field(Fp::from_str("1").unwrap())), Some(Field(Fp::from_str("1665528590348063702").unwrap())), Some(Field(Fp::from_str("299013679274582996").unwrap())), None, None, Some(Field(Fp::from_str("0").unwrap())), Some(Field(Fp::from_str("1").unwrap())), None, Some(Field(Fp::from_str("1584948566959037552488763279076679280032034891939046387842973597159527597141").unwrap())), Some(Field(Fp::from_str("22380481274269624550272939334837793435819930350096748845509687452942000168696").unwrap())), Some(Field(Fp::from_str("21259253176802051439313673188956857317690619431287159303666747467232002414469").unwrap())), None, Some(Field(Fp::from_str("15963884041495645349947458832937034481443856592126299028089177315754599597066").unwrap())), None, Some(Field(Fp::from_str("19221608673777515383976110131334817494775821767162576359558019414448466154326").unwrap())), None, Some(Field(Fp::from_str("23451804078076506209300550638205286546289770375761247523721735572981107458683").unwrap())), Some(Field(Fp::from_str("7760131790518742973201051319720767624710127853357564287088884009876915372856").unwrap())), Some(Field(Fp::from_str("25321270213089542051435513567944915616555851326093785192866035849103556600540").unwrap())), Some(Field(Fp::from_str("0").unwrap()))],
                                delegate: Some(PublicKey(CompressedPubKey::from_address("B62qkzNGkPH2gXDozurdQJmRRMHTagP41QdzRKqgnRmsYWQcc6irpnr").unwrap())),
                                verification_key: Some(VerificationKeyData {
                                        data: "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".to_string(),
                                        hash: Field(Fp::from_str("3392518251768960475377392625298437850623664973002200885669375116181514017494").unwrap()),
                                    }),
                                permissions: Some(Permissions {
                                        edit_state: AuthRequired::Impossible,
                                        access: AuthRequired::Proof,
                                        send: AuthRequired::None,
                                        receive: AuthRequired::Either,
                                        set_delegate: AuthRequired::Signature,
                                        set_permissions: AuthRequired::Signature,
                                        set_verification_key: SetVerificationKey {
                                            auth: AuthRequired::Proof,
                                            txn_version: 10631950,
                                        },
                                        set_zkapp_uri: AuthRequired::Either,
                                        edit_action_state: AuthRequired::Either,
                                        set_token_symbol: AuthRequired::Either,
                                        increment_nonce: AuthRequired::Impossible,
                                        set_voting_for: AuthRequired::None,
                                        set_timing: AuthRequired::Signature,
                                    }),
                                zkapp_uri: None,
                                token_symbol: Some(TokenSymbol(vec![])),
                                timing: None,
                                voting_for: Some(Field(Fp::from_str("1").unwrap())),
                            },
                            balance_change: BalanceChange {
                                magnitude: 439,
                                sgn: -1,
                            },
                            increment_nonce: true,
                            events: Events {
                                data: vec![
                                    vec![Field(Fp::from_str("2").unwrap()), Field(Fp::from_str("8517254081570701118820009632921361575303797702359765840093434045601427341949").unwrap()), Field(Fp::from_str("2").unwrap())],
                                    vec![Field(Fp::from_str("15023159931209234652349552339450637699355056409848657527375362340476793887323").unwrap()), Field(Fp::from_str("103153235").unwrap()), Field(Fp::from_str("6").unwrap()), Field(Fp::from_str("1").unwrap()), Field(Fp::from_str("12911966140534139033781045071415201475337871148813539968511948432387844331706").unwrap())],
                                ]
                            },
                            actions: Actions {
                                data: vec![
                                    vec![Field(Fp::from_str("21974192414450031160927927337665227680527741603455589225303577480121986385092").unwrap()), Field(Fp::from_str("9305801452617999732529485034468339810446439403406364448631572670608314164345").unwrap()), Field(Fp::from_str("0").unwrap()), Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())],
                                    vec![Field(Fp::from_str("578821135954410143385536").unwrap()), Field(Fp::from_str("27469182355577443093347341786802843817712863917616670758934250990091130060574").unwrap())],
                                ]
                            },
                            call_data: Field(Fp::from_str("12606922661993787762056411420153140479820866051230670133227973689063105978519").unwrap()),
                            call_depth: 0,
                            preconditions: Preconditions {
                                network: NetworkPreconditions {
                                    snarked_ledger_hash: None,
                                    blockchain_length: None,
                                    min_window_density: None,
                                    total_currency: None,
                                    global_slot_since_genesis: Some(RangeCondition { lower: StringU32(101085), upper: StringU32(10823) }),
                                    staking_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: None,
                                            total_currency: None,
                                        },
                                        seed: Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())),
                                        start_checkpoint: Some(Field(Fp::from_str("3501533051848528866137185507607").unwrap())),
                                        lock_checkpoint: None,
                                        epoch_length: None,
                                    },
                                    next_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: None,
                                            total_currency: None,
                                        },
                                        seed: Some(Field(Fp::from_str("0").unwrap())),
                                        start_checkpoint: Some(Field(Fp::from_str("0").unwrap())),
                                        lock_checkpoint: Some(Field(Fp::from_str("0").unwrap())),
                                        epoch_length: None,
                                    },
                                },
                                account: AccountPreconditions {
                                    balance: None,
                                    nonce: None,
                                    receipt_chain_hash: Some(Field(Fp::from_str("11573738751976307437560149536177174194734678037319828684409283985621145855530").unwrap())),
                                    delegate: None,
                                    state: [None, None, None, Some(Field(Fp::from_str("21818611846122663532068849405222766003780569287207801546042095842255299782796").unwrap())), None, None, None, None, Some(Field(Fp::from_str("1591774795152596655979700789316740602669685721569130083547332704476986667252").unwrap())), Some(Field(Fp::from_str("122521904619582796").unwrap())), None, None, None, Some(Field(Fp::from_str("17137132248300489859165674997373313730089052227518501890397970198449892378267").unwrap())), Some(Field(Fp::from_str("14024183766712596951106385269368163885413867209480463707355156607190697460718").unwrap())), Some(Field(Fp::from_str("20448350027726767939967356472826952538050432446827404885181420050677814816932").unwrap())), Some(Field(Fp::from_str("4612491638436485986651146869226390108140071823797807343088164844702282832171").unwrap())), Some(Field(Fp::from_str("1").unwrap())), None, None, Some(Field(Fp::from_str("7285236546494204307400865275879400378474374442943206297895383841216088502829").unwrap())), Some(Field(Fp::from_str("3719300950274281040527983821945681596709468304824232470338265559499826013665").unwrap())), Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), Some(Field(Fp::from_str("1203328703120878436370675549746373002828482492458497076696442092423475243445").unwrap())), None, Some(Field(Fp::from_str("86516345537").unwrap())), None, Some(Field(Fp::from_str("28784617695680378995662618148634381931495010374046385441994090140942285282829").unwrap())), None, None, None, None],
                                    action_state: None,
                                    proved_state: None,
                                    is_new: None,
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
                            public_key: PublicKey(CompressedPubKey::from_address("B62qm8ixRzxFXq4ERCAETbbBVifuVBRMEUrbt7QKnWaG5Daq6RNcKsc").unwrap()),
                            token_id: serde_json::from_str::<TokenId>(r#""xpHHVHLBm43khdAZ2bJen78WBTvHvPiU3irbR2wQ278Apq3ZxE""#).unwrap(),
                            update: Update {
                                app_state: [None, None, Some(Field(Fp::from_str("21932579065931488394053080643480891449271308456659701939527565628871817101377").unwrap())), Some(Field(Fp::from_str("7167521623149181341374530914757901").unwrap())), None, Some(Field(Fp::from_str("26386843962268029660532801897954130079968504058419359812945731861986375900235").unwrap())), Some(Field(Fp::from_str("11918544605431596792146191673351986868231847340401088969914654176532435559971").unwrap())), Some(Field(Fp::from_str("4104148").unwrap())), Some(Field(Fp::from_str("1").unwrap())), None, Some(Field(Fp::from_str("26325341409138494590021490880200820216458408032150940856334158473714857650241").unwrap())), None, Some(Field(Fp::from_str("0").unwrap())), None, Some(Field(Fp::from_str("15195592090555006472260365").unwrap())), None, None, None, None, Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), Some(Field(Fp::from_str("254005894163482180293738087927617").unwrap())), Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), Some(Field(Fp::from_str("23911525140262996434967362817611081028742728898710723992622686543566111811538").unwrap())), Some(Field(Fp::from_str("7554699240980121209066473944645969434119482918267515390506868746761740509695").unwrap())), Some(Field(Fp::from_str("1").unwrap())), Some(Field(Fp::from_str("0").unwrap())), Some(Field(Fp::from_str("1763333680810057616124262970844923945517473524698445425039261503076014816584").unwrap())), Some(Field(Fp::from_str("2755673706840547313588724973935448340706724997796031033641060088822281230070").unwrap())), Some(Field(Fp::from_str("4970322903724456562850641140293").unwrap())), None, None, Some(Field(Fp::from_str("6487766891448960641100879751030225339072566186577891188736680987047536827145").unwrap()))],
                                delegate: None,
                                verification_key: Some(VerificationKeyData {
                                        data: "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".to_string(),
                                        hash: Field(Fp::from_str("3392518251768960475377392625298437850623664973002200885669375116181514017494").unwrap()),
                                    }),
                                permissions: Some(Permissions {
                                        edit_state: AuthRequired::Impossible,
                                        access: AuthRequired::Either,
                                        send: AuthRequired::Proof,
                                        receive: AuthRequired::Impossible,
                                        set_delegate: AuthRequired::Signature,
                                        set_permissions: AuthRequired::Either,
                                        set_verification_key: SetVerificationKey {
                                            auth: AuthRequired::None,
                                            txn_version: 555041,
                                        },
                                        set_zkapp_uri: AuthRequired::None,
                                        edit_action_state: AuthRequired::Signature,
                                        set_token_symbol: AuthRequired::Impossible,
                                        increment_nonce: AuthRequired::Proof,
                                        set_voting_for: AuthRequired::Either,
                                        set_timing: AuthRequired::Either,
                                    }),
                                zkapp_uri: None,
                                token_symbol: None,
                                timing: Some(TimingData {
                                        initial_minimum_balance: 16,
                                        cliff_time: 1143502944,
                                        cliff_amount: 210,
                                        vesting_period: 4294967295,
                                        vesting_increment: 18446744073709551615,
                                    }),
                                voting_for: Some(Field(Fp::from_str("1979429467123542476001132444").unwrap())),
                            },
                            balance_change: BalanceChange {
                                magnitude: 0,
                                sgn: -1,
                            },
                            increment_nonce: false,
                            events: Events {
                                data: vec![
                                    vec![Field(Fp::from_str("1").unwrap())],
                                    vec![Field(Fp::from_str("0").unwrap()), Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap()), Field(Fp::from_str("22830085335333606181751073214786896151880400377913318497498104064680795945500").unwrap()), Field(Fp::from_str("22848499998494140975041340925227700206398569830063564944405602134917949991558").unwrap())],
                                ]
                            },
                            actions: Actions {
                                data: vec![
                                    vec![Field(Fp::from_str("24547024195139397881382081855450645699208347527204108093612985975017199767276").unwrap()), Field(Fp::from_str("8766096324511860725239928207707102972").unwrap()), Field(Fp::from_str("16646526975478815951637015739738595440657056202561708006365367775902869909657").unwrap())],
                                ]
                            },
                            call_data: Field(Fp::from_str("0").unwrap()),
                            call_depth: 1,
                            preconditions: Preconditions {
                                network: NetworkPreconditions {
                                    snarked_ledger_hash: Some(Field(Fp::from_str("1").unwrap())),
                                    blockchain_length: Some(RangeCondition { lower: StringU32(0), upper: StringU32(0) }),
                                    min_window_density: Some(RangeCondition { lower: StringU32(1), upper: StringU32(29199128) }),
                                    total_currency: None,
                                    global_slot_since_genesis: None,
                                    staking_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: Some(Field(Fp::from_str("263").unwrap())),
                                            total_currency: Some(RangeCondition { lower: StringU64(18446744073709551615), upper: StringU64(15457463147673025) }),
                                        },
                                        seed: Some(Field(Fp::from_str("20760056277819354153592312218577601715346947774845568544061412821180421976200").unwrap())),
                                        start_checkpoint: None,
                                        lock_checkpoint: Some(Field(Fp::from_str("28172031152921349099906037243256552").unwrap())),
                                        epoch_length: Some(RangeCondition { lower: StringU32(9732), upper: StringU32(7284784) }),
                                    },
                                    next_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: None,
                                            total_currency: None,
                                        },
                                        seed: Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())),
                                        start_checkpoint: Some(Field(Fp::from_str("25308306683920124442190189746638205904107497246779792336360803930655251985279").unwrap())),
                                        lock_checkpoint: Some(Field(Fp::from_str("23238459217598399370201529391905564200504402316944962313469477311747446789947").unwrap())),
                                        epoch_length: None,
                                    },
                                },
                                account: AccountPreconditions {
                                    balance: None,
                                    nonce: None,
                                    receipt_chain_hash: Some(Field(Fp::from_str("5715317516227017333795606286885641492573938639048800354827294439832656576010").unwrap())),
                                    delegate: Some(PublicKey(CompressedPubKey::from_address("B62qmCHdPT3QtVQCJ8fasYchuDfG8qpdbx1aRDtsXdUVyY9VTuER2p7").unwrap())),
                                    state: [None, Some(Field(Fp::from_str("0").unwrap())), None, Some(Field(Fp::from_str("6985185170381077762243350451091165387239356858142738616185554780490448026505").unwrap())), Some(Field(Fp::from_str("27940043615618961149598822554759884211447761626848508702301479089728484948382").unwrap())), Some(Field(Fp::from_str("15781098314078507325056997767122949532172108439822991799030299961668123207885").unwrap())), Some(Field(Fp::from_str("1").unwrap())), None, None, None, None, None, Some(Field(Fp::from_str("5349215766251822779019290240807").unwrap())), Some(Field(Fp::from_str("15892207966669909419404951421926767985238408090146733694200114633549184487620").unwrap())), Some(Field(Fp::from_str("32243434005059").unwrap())), None, Some(Field(Fp::from_str("3521877954089").unwrap())), Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), Some(Field(Fp::from_str("22803736487810230431805624203046208101320719769140444702870108292019272113125").unwrap())), Some(Field(Fp::from_str("10885439494317420587610040118510965578983950867064806646506868622369963871792").unwrap())), None, Some(Field(Fp::from_str("3363567675080296369352930155848088821008641845942581733786159549458000649938").unwrap())), Some(Field(Fp::from_str("3381553498236538961479906013607").unwrap())), None, Some(Field(Fp::from_str("16273510386419323705686431348270286427921312495533564292555120292031215267639").unwrap())), Some(Field(Fp::from_str("20344198185942539472261676705697579974929860725260966440803117920013334293010").unwrap())), Some(Field(Fp::from_str("2709615096255292936656004949257021687869340513345399244326611605147897328258").unwrap())), None, None, None, Some(Field(Fp::from_str("11401741762263443353642370125344967180695637977793069048976378213217029990686").unwrap())), Some(Field(Fp::from_str("6010010353642571240897833191124999080569574417819930801010974170278733973988").unwrap()))],
                                    action_state: None,
                                    proved_state: Some(false),
                                    is_new: None,
                                },
                                valid_while: None,
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
                    },// Account update 3
                    AccountUpdate {
                        body: AccountUpdateBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qoKHeLQromehPsn48MNXTjiM9Hyy1CCecomS6F2ER4bgxKmfpETw").unwrap()),
                            token_id: serde_json::from_str::<TokenId>(r#""wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf""#).unwrap(),
                            update: Update {
                                app_state: [Some(Field(Fp::from_str("26665195013804907052586597728519645211290592469525195402011216457364336174257").unwrap())), Some(Field(Fp::from_str("0").unwrap())), Some(Field(Fp::from_str("8920452131186598558954460651136015429531768278175420377811213971102312024970").unwrap())), None, None, None, Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), Some(Field(Fp::from_str("0").unwrap())), None, Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), Some(Field(Fp::from_str("1372435453036537223615215516").unwrap())), None, Some(Field(Fp::from_str("1").unwrap())), None, None, Some(Field(Fp::from_str("0").unwrap())), Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), Some(Field(Fp::from_str("1").unwrap())), None, None, None, None, None, None, None, Some(Field(Fp::from_str("77999226821473495125260007889455248").unwrap())), None, None, None, None, None, Some(Field(Fp::from_str("1").unwrap()))],
                                delegate: Some(PublicKey(CompressedPubKey::from_address("B62qmYNuDsc5ZtmL5dBnvXYCGkLHgGdRjjWop73rtSKKoHPjVtpjHVr").unwrap())),
                                verification_key: Some(VerificationKeyData {
                                        data: "AgIBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAALsq7cojes8ZcUc9M9RbZY9U7nhj8KnfU3yTEgqjtXQbAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAC7Ku3KI3rPGXFHPTPUW2WPVO54Y/Cp31N8kxIKo7V0GwEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAuyrtyiN6zxlxRz0z1Ftlj1TueGPwqd9TfJMSCqO1dBsBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAALsq7cojes8ZcUc9M9RbZY9U7nhj8KnfU3yTEgqjtXQbAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAC7Ku3KI3rPGXFHPTPUW2WPVO54Y/Cp31N8kxIKo7V0GwEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAuyrtyiN6zxlxRz0z1Ftlj1TueGPwqd9TfJMSCqO1dBsBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAALsq7cojes8ZcUc9M9RbZY9U7nhj8KnfU3yTEgqjtXQbAAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAuyrtyiN6zxlxRz0z1Ftlj1TueGPwqd9TfJMSCqO1dBsBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAALsq7cojes8ZcUc9M9RbZY9U7nhj8KnfU3yTEgqjtXQbAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAC7Ku3KI3rPGXFHPTPUW2WPVO54Y/Cp31N8kxIKo7V0GwEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAuyrtyiN6zxlxRz0z1Ftlj1TueGPwqd9TfJMSCqO1dBsBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAALsq7cojes8ZcUc9M9RbZY9U7nhj8KnfU3yTEgqjtXQbAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAC7Ku3KI3rPGXFHPTPUW2WPVO54Y/Cp31N8kxIKo7V0GwEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAuyrtyiN6zxlxRz0z1Ftlj1TueGPwqd9TfJMSCqO1dBsBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAALsq7cojes8ZcUc9M9RbZY9U7nhj8KnfU3yTEgqjtXQbAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAC7Ku3KI3rPGXFHPTPUW2WPVO54Y/Cp31N8kxIKo7V0GwEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAuyrtyiN6zxlxRz0z1Ftlj1TueGPwqd9TfJMSCqO1dBsBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAALsq7cojes8ZcUc9M9RbZY9U7nhj8KnfU3yTEgqjtXQbAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAC7Ku3KI3rPGXFHPTPUW2WPVO54Y/Cp31N8kxIKo7V0GwEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAuyrtyiN6zxlxRz0z1Ftlj1TueGPwqd9TfJMSCqO1dBsBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAALsq7cojes8ZcUc9M9RbZY9U7nhj8KnfU3yTEgqjtXQbAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAC7Ku3KI3rPGXFHPTPUW2WPVO54Y/Cp31N8kxIKo7V0GwABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAALsq7cojes8ZcUc9M9RbZY9U7nhj8KnfU3yTEgqjtXQbAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAC7Ku3KI3rPGXFHPTPUW2WPVO54Y/Cp31N8kxIKo7V0GwEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAuyrtyiN6zxlxRz0z1Ftlj1TueGPwqd9TfJMSCqO1dBsBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAALsq7cojes8ZcUc9M9RbZY9U7nhj8KnfU3yTEgqjtXQbAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAC7Ku3KI3rPGXFHPTPUW2WPVO54Y/Cp31N8kxIKo7V0GwEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAuyrtyiN6zxlxRz0z1Ftlj1TueGPwqd9TfJMSCqO1dBs=".to_string(),
                                        hash: Field(Fp::from_str("3392518251768960475377392625298437850623664973002200885669375116181514017494").unwrap()),
                                    }),
                                permissions: None,
                                zkapp_uri: None,
                                token_symbol: None,
                                timing: None,
                                voting_for: None,
                            },
                            balance_change: BalanceChange {
                                magnitude: 5,
                                sgn: -1,
                            },
                            increment_nonce: false,
                            events: Events {
                                data: vec![
                                    vec![Field(Fp::from_str("780158388411006822195").unwrap()), Field(Fp::from_str("95895838239076729197635808366118").unwrap()), Field(Fp::from_str("27587202504191906403411154928422428082038810664941913822565348425870345285265").unwrap()), Field(Fp::from_str("12245628856554528519664579447419480467858381899735002749509511393430279385674").unwrap()), Field(Fp::from_str("89345222948909809305").unwrap())],
                                ]
                            },
                            actions: Actions {
                                data: vec![
                                    vec![Field(Fp::from_str("372097603967445813016345215748863677").unwrap()), Field(Fp::from_str("28476455685362593833519662968698224353031526713536191773380796773090417253202").unwrap())],
                                ]
                            },
                            call_data: Field(Fp::from_str("0").unwrap()),
                            call_depth: 0,
                            preconditions: Preconditions {
                                network: NetworkPreconditions {
                                    snarked_ledger_hash: Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())),
                                    blockchain_length: None,
                                    min_window_density: Some(RangeCondition { lower: StringU32(1), upper: StringU32(0) }),
                                    total_currency: None,
                                    global_slot_since_genesis: None,
                                    staking_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: Some(Field(Fp::from_str("24569591760402050317109452870466960030790939818128919097606955709531646691660").unwrap())),
                                            total_currency: Some(RangeCondition { lower: StringU64(76509044), upper: StringU64(1) }),
                                        },
                                        seed: Some(Field(Fp::from_str("0").unwrap())),
                                        start_checkpoint: None,
                                        lock_checkpoint: None,
                                        epoch_length: None,
                                    },
                                    next_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: None,
                                            total_currency: Some(RangeCondition { lower: StringU64(20215233), upper: StringU64(71442) }),
                                        },
                                        seed: Some(Field(Fp::from_str("201794101642815636804317993950").unwrap())),
                                        start_checkpoint: Some(Field(Fp::from_str("121905990453377513").unwrap())),
                                        lock_checkpoint: None,
                                        epoch_length: None,
                                    },
                                },
                                account: AccountPreconditions {
                                    balance: None,
                                    nonce: Some(RangeCondition { lower: StringU32(4294967295), upper: StringU32(0) }),
                                    receipt_chain_hash: None,
                                    delegate: None,
                                    state: [Some(Field(Fp::from_str("1414323797312780638828916825650426914037506032808167350772308428471815615005").unwrap())), Some(Field(Fp::from_str("27153043299286096385462755767280334743005743312269037617293867281336770306354").unwrap())), None, Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), None, Some(Field(Fp::from_str("24341972492091599114231873579360985596751541261677951087314121419685591261153").unwrap())), Some(Field(Fp::from_str("23357702607718656310601581988240745898447376658453237045487682071483854878315").unwrap())), None, Some(Field(Fp::from_str("4547292493043000955898870795364277110475200016223274563783867449793085366566").unwrap())), None, Some(Field(Fp::from_str("618989078561767534889296317797706246").unwrap())), None, None, None, Some(Field(Fp::from_str("0").unwrap())), None, Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), None, None, Some(Field(Fp::from_str("1").unwrap())), None, None, Some(Field(Fp::from_str("13516392646281811857574923681915770352168407413645417761385241138721932938524").unwrap())), Some(Field(Fp::from_str("1").unwrap())), None, Some(Field(Fp::from_str("3769131039962838855524906945205482812938880244756862602964442043321485847611").unwrap())), None, Some(Field(Fp::from_str("10000654270849510322323991690416118004196411638100369118296495486363862079650").unwrap())), Some(Field(Fp::from_str("15301534848092284180978011243780886150079036829640932335005434136585467647482").unwrap())), Some(Field(Fp::from_str("1").unwrap())), None, None],
                                    action_state: None,
                                    proved_state: None,
                                    is_new: None,
                                },
                                valid_while: None,
                            },
                            use_full_commitment: false,
                            implicit_account_creation_fee: false,
                            may_use_token: MayUseToken {
                                parents_own_token: false,
                                inherit_from_parent: true,
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
                            public_key: PublicKey(CompressedPubKey::from_address("B62qpTd4pE1pNLyg1AYvbyV8jcmBr2aUYsfEy3KGWrvkn3b7hZJRpYv").unwrap()),
                            token_id: serde_json::from_str::<TokenId>(r#""wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf""#).unwrap(),
                            update: Update {
                                app_state: [None, None, Some(Field(Fp::from_str("2660084412364851483915897655420494605273388260345606635467433961143609755043").unwrap())), Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), None, Some(Field(Fp::from_str("23181602707418773213239252184187625568645369640137023509594788353337692772405").unwrap())), Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), None, Some(Field(Fp::from_str("0").unwrap())), None, None, Some(Field(Fp::from_str("2182045946281").unwrap())), None, None, None, None, None, None, None, None, None, Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), None, None, Some(Field(Fp::from_str("22200876690043823536882461378603012195590256456312327405225507556213245195061").unwrap())), None, None, None, Some(Field(Fp::from_str("361556693311147496559302097970").unwrap())), None, None, None],
                                delegate: Some(PublicKey(CompressedPubKey::from_address("B62qrNvyeAE6gkZFg6U51Kmfx5T5gKdAqspThAhuDBkHMZpGCNhy9WN").unwrap())),
                                verification_key: None,
                                permissions: None,
                                zkapp_uri: Some(ZkappUri(vec![194, 156])),
                                token_symbol: None,
                                timing: Some(TimingData {
                                        initial_minimum_balance: 3847196,
                                        cliff_time: 102130282,
                                        cliff_amount: 29660027775904,
                                        vesting_period: 52764091,
                                        vesting_increment: 196,
                                    }),
                                voting_for: Some(Field(Fp::from_str("10466704974137666196343022972749520403743178907013990594636739133174839306148").unwrap())),
                            },
                            balance_change: BalanceChange {
                                magnitude: 18446744073709551615,
                                sgn: 1,
                            },
                            increment_nonce: false,
                            events: Events {
                                data: vec![]
                            },
                            actions: Actions {
                                data: vec![
                                    vec![Field(Fp::from_str("12735909594464152300133968631689091423362613377319722908807250982030671406219").unwrap()), Field(Fp::from_str("18922147924955229103075727024091945785336664093584916601343929319977349154261").unwrap()), Field(Fp::from_str("925479686690034799157775669").unwrap()), Field(Fp::from_str("7296047751850388472626614023761649182580563603917699148559086986849924902994").unwrap()), Field(Fp::from_str("0").unwrap())],
                                ]
                            },
                            call_data: Field(Fp::from_str("22063440245619299090060011453909740153268331048885785193261762053485565177782").unwrap()),
                            call_depth: 0,
                            preconditions: Preconditions {
                                network: NetworkPreconditions {
                                    snarked_ledger_hash: None,
                                    blockchain_length: None,
                                    min_window_density: None,
                                    total_currency: Some(RangeCondition { lower: StringU64(1151247031754897929), upper: StringU64(118389) }),
                                    global_slot_since_genesis: None,
                                    staking_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: Some(Field(Fp::from_str("13954587540099722553140800964367218978723433427382262619494017860348816973152").unwrap())),
                                            total_currency: None,
                                        },
                                        seed: None,
                                        start_checkpoint: None,
                                        lock_checkpoint: Some(Field(Fp::from_str("28815530375840414967091373259421020785641166828663537745004443182756818241340").unwrap())),
                                        epoch_length: Some(RangeCondition { lower: StringU32(12924), upper: StringU32(4294967295) }),
                                    },
                                    next_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: Some(Field(Fp::from_str("11435444712444877307797192614982011254065213967571132056195488970511853077541").unwrap())),
                                            total_currency: Some(RangeCondition { lower: StringU64(1), upper: StringU64(5) }),
                                        },
                                        seed: Some(Field(Fp::from_str("9357518679552806490800600583312264449704132494990566843599600869121334432325").unwrap())),
                                        start_checkpoint: None,
                                        lock_checkpoint: Some(Field(Fp::from_str("3693693156567349059423754434718875553675380107126263564379144751196157253646").unwrap())),
                                        epoch_length: Some(RangeCondition { lower: StringU32(0), upper: StringU32(0) }),
                                    },
                                },
                                account: AccountPreconditions {
                                    balance: None,
                                    nonce: Some(RangeCondition { lower: StringU32(4294967295), upper: StringU32(0) }),
                                    receipt_chain_hash: None,
                                    delegate: None,
                                    state: [None, None, None, Some(Field(Fp::from_str("16194905037539718714072079079540491144404301728577847071689188117357324665750").unwrap())), None, None, Some(Field(Fp::from_str("292864").unwrap())), None, None, None, None, Some(Field(Fp::from_str("14766510121724082419845734056783570285257475555491840852539313896098245851053").unwrap())), None, None, Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), Some(Field(Fp::from_str("0").unwrap())), None, Some(Field(Fp::from_str("6700702547298387473448513844745387379247943485894147490084683439475871803860").unwrap())), None, Some(Field(Fp::from_str("0").unwrap())), Some(Field(Fp::from_str("21918885552777748081322812525708691080660694322247209626390995318244794984265").unwrap())), None, None, None, Some(Field(Fp::from_str("352057056").unwrap())), Some(Field(Fp::from_str("107721396545021736").unwrap())), Some(Field(Fp::from_str("3064987767910811808813601744546265554432773785280668045278065488872332454566").unwrap())), None, None, Some(Field(Fp::from_str("231316637406950116027543007756721237841").unwrap())), Some(Field(Fp::from_str("23672831556115487228131864385265585774117521256725053127524370153885530730684").unwrap())), None],
                                    action_state: Some(ActionState(Field(Fp::from_str("25079927036070901246064867767436987657692091363973573142121686150614948079097").unwrap()))),
                                    proved_state: Some(true),
                                    is_new: None,
                                },
                                valid_while: None,
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
                    },// Account update 5
                    AccountUpdate {
                        body: AccountUpdateBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qoKHeLQromehPsn48MNXTjiM9Hyy1CCecomS6F2ER4bgxKmfpETw").unwrap()),
                            token_id: serde_json::from_str::<TokenId>(r#""y25S1peHEtPRUXQCFqX94UCnzSqGbxjLdJHZbHk7PSbS5cxZ3o""#).unwrap(),
                            update: Update {
                                app_state: [Some(Field(Fp::from_str("1080071846243133").unwrap())), Some(Field(Fp::from_str("1").unwrap())), None, None, None, None, None, None, None, Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), Some(Field(Fp::from_str("3567776995101244424262850").unwrap())), None, None, None, Some(Field(Fp::from_str("230858869687627635894743362149811805").unwrap())), None, None, Some(Field(Fp::from_str("28343357110465176080538347046906609558299776838776567832422132539714527005221").unwrap())), Some(Field(Fp::from_str("5127997399869524448906875486295001567614828696452108130906043499847636933820").unwrap())), Some(Field(Fp::from_str("0").unwrap())), Some(Field(Fp::from_str("25033349488007925155976907842334255870904388620091408653017796803370086016425").unwrap())), None, None, Some(Field(Fp::from_str("28556786030627800109718339424714568514245590068377550151186857541326005414496").unwrap())), None, None, None, Some(Field(Fp::from_str("49600395263246128608929407399").unwrap())), Some(Field(Fp::from_str("5323163033978046086968381337089001172563818193348382345018433947908658774621").unwrap())), Some(Field(Fp::from_str("14811834798251510254123842577031647733403236235324567441227573530980193037983").unwrap())), None, None],
                                delegate: None,
                                verification_key: None,
                                permissions: None,
                                zkapp_uri: Some(ZkappUri(vec![13, 194, 158, 195, 131, 194, 184, 73, 124, 194, 187, 111, 194, 180, 86, 7, 55, 55, 194, 174, 38])),
                                token_symbol: Some(TokenSymbol(vec![106, 194, 184])),
                                timing: Some(TimingData {
                                        initial_minimum_balance: 134412784,
                                        cliff_time: 240029,
                                        cliff_amount: 216887633,
                                        vesting_period: 1461,
                                        vesting_increment: 2732,
                                    }),
                                voting_for: Some(Field(Fp::from_str("21008278699735891685").unwrap())),
                            },
                            balance_change: BalanceChange {
                                magnitude: 33327380759,
                                sgn: -1,
                            },
                            increment_nonce: false,
                            events: Events {
                                data: vec![]
                            },
                            actions: Actions {
                                data: vec![
                                    vec![Field(Fp::from_str("4951622628085001981655873302822569616777753245731412495781898040247004061448").unwrap()), Field(Fp::from_str("9529891572379133000255089391199406266179491499126264529871923607813549751477").unwrap())],
                                    vec![Field(Fp::from_str("1").unwrap()), Field(Fp::from_str("15907092335970729810297357182684520478844423305294324661039021878587352163584").unwrap()), Field(Fp::from_str("9288289605154234300096221235449544222637517311457959311229322899024190482501").unwrap())],
                                ]
                            },
                            call_data: Field(Fp::from_str("536144578203884546350361299178968183159234794486653472867988444339693388027").unwrap()),
                            call_depth: 0,
                            preconditions: Preconditions {
                                network: NetworkPreconditions {
                                    snarked_ledger_hash: None,
                                    blockchain_length: Some(RangeCondition { lower: StringU32(81167776), upper: StringU32(75) }),
                                    min_window_density: Some(RangeCondition { lower: StringU32(4294967295), upper: StringU32(63) }),
                                    total_currency: Some(RangeCondition { lower: StringU64(1426957654180), upper: StringU64(3904695667302406) }),
                                    global_slot_since_genesis: None,
                                    staking_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: Some(Field(Fp::from_str("340282366920938463463374607431768211455").unwrap())),
                                            total_currency: None,
                                        },
                                        seed: None,
                                        start_checkpoint: None,
                                        lock_checkpoint: None,
                                        epoch_length: Some(RangeCondition { lower: StringU32(0), upper: StringU32(3426) }),
                                    },
                                    next_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: Some(Field(Fp::from_str("121883314227900933457144057938").unwrap())),
                                            total_currency: Some(RangeCondition { lower: StringU64(23005533362), upper: StringU64(297696) }),
                                        },
                                        seed: Some(Field(Fp::from_str("1").unwrap())),
                                        start_checkpoint: Some(Field(Fp::from_str("455").unwrap())),
                                        lock_checkpoint: None,
                                        epoch_length: None,
                                    },
                                },
                                account: AccountPreconditions {
                                    balance: Some(RangeCondition { lower: StringU64(2), upper: StringU64(1470838877) }),
                                    nonce: Some(RangeCondition { lower: StringU32(1), upper: StringU32(0) }),
                                    receipt_chain_hash: None,
                                    delegate: None,
                                    state: [Some(Field(Fp::from_str("21826260643800727403868596252418575128028004520667259477058936994639017054827").unwrap())), Some(Field(Fp::from_str("11049637213587748622882489447113659297883793050393270010722063618497039626777").unwrap())), None, None, Some(Field(Fp::from_str("16511102171278264900810798496901896427170828232331068855093352834726896626818").unwrap())), Some(Field(Fp::from_str("0").unwrap())), Some(Field(Fp::from_str("10025346538432359025252751216587176753730951003322281742027085695179410147194").unwrap())), Some(Field(Fp::from_str("1").unwrap())), None, None, Some(Field(Fp::from_str("1").unwrap())), Some(Field(Fp::from_str("623683692207781210856098667370494009414343125045424967714655840961868113897").unwrap())), None, None, Some(Field(Fp::from_str("0").unwrap())), Some(Field(Fp::from_str("8715533973632476213425525417234982090626347498280475004547689161640165755785").unwrap())), Some(Field(Fp::from_str("7420204432542833073716136827431541469188144542177054810655142956666986681340").unwrap())), None, Some(Field(Fp::from_str("0").unwrap())), Some(Field(Fp::from_str("17303125463856642386254473154611565727228903629208171697061842777971894961591").unwrap())), Some(Field(Fp::from_str("19959559881659556409592323784614812046739805329968115572299456763209825707328").unwrap())), Some(Field(Fp::from_str("340282366920938463463374607431768211455").unwrap())), Some(Field(Fp::from_str("16490118008352799118942").unwrap())), None, Some(Field(Fp::from_str("1521624520457695006778130669041108626724218384922553080237836224431287714238").unwrap())), Some(Field(Fp::from_str("0").unwrap())), Some(Field(Fp::from_str("1035330861783685728076078844").unwrap())), Some(Field(Fp::from_str("1").unwrap())), Some(Field(Fp::from_str("3162987144355229016831931380015997437944412208636268063020536753309533438709").unwrap())), None, None, Some(Field(Fp::from_str("481016020148998526219060").unwrap()))],
                                    action_state: None,
                                    proved_state: None,
                                    is_new: Some(true),
                                },
                                valid_while: Some(RangeCondition { lower: StringU32(183744), upper: StringU32(1) }),
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
                memo: decode_memo_from_base58("E4YonkUX9DKcacuLYFSuX2EMfWLGKp8K6pJJRNZTinxDNSY7YePqh"),
            },
            network: NetworkId::Mainnet,
            expected_memo_hash: "5257547173933775597336231680324140365424656775220345555667538793193683457424",
            expected_fee_payer_hash: "9325429652586486418786944064529678321514555014870992471143227537365939431671",
            expected_account_updates_commitment: "28841863419870110097792552660404377648566093810930630314398877960263425195126",
            expected_full_commitment: "1912218377961725891883250134257095659416918430994914676811901829955428769686",
        }
    ]
}
