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
                name: "mesa_empty_account_updates_mainnet",
                zkapp_command: ZKAppCommand {
                    fee_payer: FeePayer {
                        body: FeePayerBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qpXGrAHNnyE2fAqcsc99VeTeWaH5DAnWqczaB99mMY4rkQZthov5").unwrap()),
                            fee: 109702565486485941,
                            valid_until: None,
                            nonce: 70365,
                        },
                        authorization: "7mWxjLYgbJUkZNcGouvhVj5tJ8yu9hoexb9ntvPK8t5LHqzmrL6QJjjKtf5SgmxB4QWkDw7qoMMbbNGtHVpsbJHPyTy2EzRQ".to_string(),
                    },
                    account_updates: vec![],
                    memo: decode_memo_from_base58("E4YM2vTHhWEg66xpj52JErHUBU4pZ1yageL4TVDDpTTSsv8mK6YaH"),
                },
                network: NetworkId::Mainnet,
                expected_memo_hash: "146624400929844538317466382872834899021794596262855408933526545768996436172",
                expected_fee_payer_hash: "12135646210551944627881048857004506690151937005505702742278473742050772972715",
                expected_account_updates_commitment: "0",
                expected_full_commitment: "22253498370476650242500647536180552827875386018239310251425646820017459544291",
            }
    ,
    ZkAppTestVector {
                name: "mesa_single_or_more_updates_testnet",
                zkapp_command: ZKAppCommand {
                    fee_payer: FeePayer {
                        body: FeePayerBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qoWMyQLTwkUdJCDN4VxPYwyoh3fkT3EXWxueUMg5T25P7hDmbB5C").unwrap()),
                            fee: 4445920051020,
                            valid_until: Some(2734874),
                            nonce: 2897391,
                        },
                        authorization: "7mWxjLYgbJUkZNcGouvhVj5tJ8yu9hoexb9ntvPK8t5LHqzmrL6QJjjKtf5SgmxB4QWkDw7qoMMbbNGtHVpsbJHPyTy2EzRQ".to_string(),
                    },
                    account_updates: vec![
    // Account update 1
                        AccountUpdate {
                            body: AccountUpdateBody {
                                public_key: PublicKey(CompressedPubKey::from_address("B62qoWMyQLTwkUdJCDN4VxPYwyoh3fkT3EXWxueUMg5T25P7hDmbB5C").unwrap()),
                                token_id: serde_json::from_str::<TokenId>(r#""x2wSz9e1jfBT2MEVSm5rJEfqdS4vGYAb7sCejh5RR3WZYUAT7H""#).unwrap(),
                                update: Update {
                                    app_state: [None, None, None, Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), None, None, Some(Field(Fp::from_str("0").unwrap())), None, Some(Field(Fp::from_str("5910959655").unwrap())), Some(Field(Fp::from_str("9670896144163052435093773753233236269334911122004511596390959580813366503734").unwrap())), None, None, None, Some(Field(Fp::from_str("1").unwrap())), Some(Field(Fp::from_str("22290055319701361361563885893232437531187595417290297776910771119433021379905").unwrap())), None, None, Some(Field(Fp::from_str("1").unwrap())), Some(Field(Fp::from_str("1").unwrap())), None, Some(Field(Fp::from_str("7876519288015629565599361766638942696047599603027998152645445237668374419329").unwrap())), None, None, Some(Field(Fp::from_str("1").unwrap())), None, None, Some(Field(Fp::from_str("16462927211749771769628494338560740609769182120251471191287762983277154687645").unwrap())), None, None, None, Some(Field(Fp::from_str("75140470118").unwrap())), None],
                                    delegate: Some(PublicKey(CompressedPubKey::from_address("B62qmh5X3kt8wDJSjgZHshrBW6oubZF9bBPX9StZ4PMWBrb1kZBNDYz").unwrap())),
                                    verification_key: Some(VerificationKeyData {
                                            data: "hr".to_string(),
                                            hash: Field(Fp::from_str("28578217126758946781296974504986505734775498897048013994473301714981761628687").unwrap()),
                                        }),
                                    permissions: Some(Permissions {
                                            edit_state: AuthRequired::Impossible,
                                            access: AuthRequired::Either,
                                            send: AuthRequired::Impossible,
                                            receive: AuthRequired::None,
                                            set_delegate: AuthRequired::Signature,
                                            set_permissions: AuthRequired::None,
                                            set_verification_key: SetVerificationKey {
                                                auth: AuthRequired::Proof,
                                                txn_version: 2,
                                            },
                                            set_zkapp_uri: AuthRequired::Either,
                                            edit_action_state: AuthRequired::Signature,
                                            set_token_symbol: AuthRequired::None,
                                            increment_nonce: AuthRequired::Signature,
                                            set_voting_for: AuthRequired::Proof,
                                            set_timing: AuthRequired::Signature,
                                        }),
                                    zkapp_uri: None,
                                    token_symbol: Some(TokenSymbol(vec![])),
                                    timing: Some(TimingData {
                                            initial_minimum_balance: 18446744073709551615,
                                            cliff_time: 99,
                                            cliff_amount: 846262030,
                                            vesting_period: 4294967295,
                                            vesting_increment: 18446744073709551615,
                                        }),
                                    voting_for: None,
                                },
                                balance_change: BalanceChange {
                                    magnitude: 1,
                                    sgn: 1,
                                },
                                increment_nonce: false,
                                events: Events {
                                    data: vec![
                                        vec![Field(Fp::from_str("15933965051558889727984761452347753837675450301476507905654147887498265500563").unwrap()), Field(Fp::from_str("0").unwrap()), Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())],
                                    ]
                                },
                                actions: Actions {
                                    data: vec![
                                        vec![Field(Fp::from_str("28470866532464326562160473048987049861581876729526184696397692615597868801756").unwrap()), Field(Fp::from_str("4049562955116351477977723002360476173864960578700217294517971725753298676812").unwrap()), Field(Fp::from_str("11076720924320119159553986496577208436871741908404721551350720314576851438153").unwrap())],
                                        vec![Field(Fp::from_str("8534691055942540784439635414940960943444901443412274587841877691610052790021").unwrap()), Field(Fp::from_str("0").unwrap()), Field(Fp::from_str("21585724529773463507990205855721886826899532347459124304633358605982221535675").unwrap()), Field(Fp::from_str("1319601659055102725150819719609988770395384492541391491171058113233170022609").unwrap()), Field(Fp::from_str("5348769480055048872186749670407270815586818932416919025904662229704581248481").unwrap())],
                                    ]
                                },
                                call_data: Field(Fp::from_str("128724458196112977496795119853657368255").unwrap()),
                                call_depth: 0,
                                preconditions: Preconditions {
                                    network: NetworkPreconditions {
                                        snarked_ledger_hash: Some(Field(Fp::from_str("20834303412263557485325316134949546407326105666264934168159910883984408858985").unwrap())),
                                        blockchain_length: None,
                                        min_window_density: Some(RangeCondition { lower: StringU32(4294967295), upper: StringU32(18) }),
                                        total_currency: Some(RangeCondition { lower: StringU64(1), upper: StringU64(927110835881471) }),
                                        global_slot_since_genesis: None,
                                        staking_epoch_data: EpochData {
                                            ledger: EpochLedger {
                                                hash: None,
                                                total_currency: Some(RangeCondition { lower: StringU64(18446744073709551615), upper: StringU64(2) }),
                                            },
                                            seed: None,
                                            start_checkpoint: Some(Field(Fp::from_str("240").unwrap())),
                                            lock_checkpoint: Some(Field(Fp::from_str("9009311783932730556400895325965214134191283064005980544719756690716537751547").unwrap())),
                                            epoch_length: Some(RangeCondition { lower: StringU32(9646083), upper: StringU32(667370737) }),
                                        },
                                        next_epoch_data: EpochData {
                                            ledger: EpochLedger {
                                                hash: Some(Field(Fp::from_str("1492635163387540667807357183658342065379186583235559722126281813977781393205").unwrap())),
                                                total_currency: Some(RangeCondition { lower: StringU64(545797885139), upper: StringU64(2357726972192773) }),
                                            },
                                            seed: Some(Field(Fp::from_str("16686863470468209800039204505707323275000640381340091717691279307649969423435").unwrap())),
                                            start_checkpoint: Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())),
                                            lock_checkpoint: Some(Field(Fp::from_str("5019654477394071855347722099632839240676867106463683985020295547818088553850").unwrap())),
                                            epoch_length: Some(RangeCondition { lower: StringU32(2), upper: StringU32(2) }),
                                        },
                                    },
                                    account: AccountPreconditions {
                                        balance: None,
                                        nonce: None,
                                        receipt_chain_hash: None,
                                        delegate: None,
                                        state: [None, Some(Field(Fp::from_str("10125736485688114394722").unwrap())), None, None, None, None, Some(Field(Fp::from_str("15046026302190837209793594018909836735819974415713111812190147890917061894022").unwrap())), None, None, None, Some(Field(Fp::from_str("5570629120741897939780308233953373547455526195428145976750885615977801662052").unwrap())), Some(Field(Fp::from_str("2879671770808273334956435363204520997221920364616112560070076455760790853479").unwrap())), None, None, Some(Field(Fp::from_str("3322912100894701546102797696135089588303458690066598709553384942488731595126").unwrap())), None, Some(Field(Fp::from_str("20857371310155290285241278212020136730006320750782421018606528080496619831404").unwrap())), Some(Field(Fp::from_str("152184662839026893247").unwrap())), None, Some(Field(Fp::from_str("0").unwrap())), None, None, None, None, None, Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), None, None, Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), Some(Field(Fp::from_str("965958689240668594801516494150929754477536080033465784262190693605755329276").unwrap())), None, Some(Field(Fp::from_str("0").unwrap()))],
                                        action_state: Some(ActionState(Field(Fp::from_str("7038687960684852474737120310146279387827906324831963746710551112434879640287").unwrap()))),
                                        proved_state: Some(true),
                                        is_new: None,
                                    },
                                    valid_while: Some(RangeCondition { lower: StringU32(9371258), upper: StringU32(195836) }),
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
                    memo: decode_memo_from_base58("E4YM2vTHhWEg66xpj52JErHUBU4pZ1yageL4TVDDpTTSsv8mK6YaH"),
                },
                network: NetworkId::Testnet,
                expected_memo_hash: "146624400929844538317466382872834899021794596262855408933526545768996436172",
                expected_fee_payer_hash: "16416027620742437488201792991567926531595233254812689655685926734377093696332",
                expected_account_updates_commitment: "22584091981575182610249909929922799122776313214307590215326217150349862363998",
                expected_full_commitment: "592756645713327642183539401962105189004775012850811492030282327471630566751",
            }
    ,
    ZkAppTestVector {
                name: "mesa_multi_updates_mainnet",
                zkapp_command: ZKAppCommand {
                    fee_payer: FeePayer {
                        body: FeePayerBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qk8hqH2H9ZryemSHGLW1KmhFNWACmP37fURoZPGY4VFKCLuvpYcP").unwrap()),
                            fee: 7102,
                            valid_until: Some(2967416),
                            nonce: 1395771,
                        },
                        authorization: "7mWxjLYgbJUkZNcGouvhVj5tJ8yu9hoexb9ntvPK8t5LHqzmrL6QJjjKtf5SgmxB4QWkDw7qoMMbbNGtHVpsbJHPyTy2EzRQ".to_string(),
                    },
                    account_updates: vec![
    // Account update 1
                        AccountUpdate {
                            body: AccountUpdateBody {
                                public_key: PublicKey(CompressedPubKey::from_address("B62qnJa9QJcMp7kWYQhYWS5QeaJaNYZbHfWJJLvKsDjxpRY2bFEp6Ye").unwrap()),
                                token_id: serde_json::from_str::<TokenId>(r#""x5o3B7pSk6CeB1v8gaJeoRaUaVAMuiYnJFD5AHPSF8hFhHqxw8""#).unwrap(),
                                update: Update {
                                    app_state: [None, None, None, Some(Field(Fp::from_str("24408825635330805320234910647765514817076105943987164899545313318796053143808").unwrap())), None, None, None, None, None, Some(Field(Fp::from_str("5966321282569092762129816400437861370733191004981355590723999712298490270404").unwrap())), None, Some(Field(Fp::from_str("1").unwrap())), Some(Field(Fp::from_str("1").unwrap())), None, Some(Field(Fp::from_str("21534349001120560782786849378708136568790008230915377837888614016449524320191").unwrap())), Some(Field(Fp::from_str("10237232884936347509563506358808477982488429453342436547404647335613477902948").unwrap())), Some(Field(Fp::from_str("22222346555375417496901633951973224134216382376607435315644406586989814233113").unwrap())), None, None, None, None, Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), None, Some(Field(Fp::from_str("0").unwrap())), Some(Field(Fp::from_str("1").unwrap())), None, Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), None, None, None, Some(Field(Fp::from_str("1").unwrap())), None],
                                    delegate: None,
                                    verification_key: Some(VerificationKeyData {
                                            data: "".to_string(),
                                            hash: Field(Fp::from_str("1").unwrap()),
                                        }),
                                    permissions: Some(Permissions {
                                            edit_state: AuthRequired::None,
                                            access: AuthRequired::Impossible,
                                            send: AuthRequired::Either,
                                            receive: AuthRequired::None,
                                            set_delegate: AuthRequired::None,
                                            set_permissions: AuthRequired::Either,
                                            set_verification_key: SetVerificationKey {
                                                auth: AuthRequired::Proof,
                                                txn_version: 35522012,
                                            },
                                            set_zkapp_uri: AuthRequired::Impossible,
                                            edit_action_state: AuthRequired::Impossible,
                                            set_token_symbol: AuthRequired::None,
                                            increment_nonce: AuthRequired::None,
                                            set_voting_for: AuthRequired::Proof,
                                            set_timing: AuthRequired::Impossible,
                                        }),
                                    zkapp_uri: None,
                                    token_symbol: None,
                                    timing: Some(TimingData {
                                            initial_minimum_balance: 27112135,
                                            cliff_time: 34,
                                            cliff_amount: 2116296024,
                                            vesting_period: 22570316,
                                            vesting_increment: 59625233831,
                                        }),
                                    voting_for: Some(Field(Fp::from_str("2052605573749235988502940911728271953619108963114864583899284191619732607508").unwrap())),
                                },
                                balance_change: BalanceChange {
                                    magnitude: 1,
                                    sgn: 1,
                                },
                                increment_nonce: true,
                                events: Events {
                                    data: vec![
                                        vec![Field(Fp::from_str("2275761527760650683854388440237070823754019660126982467848420963254243809727").unwrap()), Field(Fp::from_str("11693740306591870609725409031904378462030611908904279589570464552412234118816").unwrap())],
                                        vec![Field(Fp::from_str("27689011277431228676866102261236517477480902476084452499461870400272002797643").unwrap()), Field(Fp::from_str("9671441553074545970294").unwrap()), Field(Fp::from_str("0").unwrap()), Field(Fp::from_str("2118999286").unwrap()), Field(Fp::from_str("1").unwrap())],
                                    ]
                                },
                                actions: Actions {
                                    data: vec![
                                        vec![Field(Fp::from_str("7970638288767310878043946637266941643131323636197145262463072301038258286111").unwrap()), Field(Fp::from_str("19599379044130784755375747850760596498058178883990732841287997872798527538814").unwrap())],
                                    ]
                                },
                                call_data: Field(Fp::from_str("21165845750850812726727809835318928475354932221513666231230873699022735804117").unwrap()),
                                call_depth: 0,
                                preconditions: Preconditions {
                                    network: NetworkPreconditions {
                                        snarked_ledger_hash: Some(Field(Fp::from_str("0").unwrap())),
                                        blockchain_length: Some(RangeCondition { lower: StringU32(1030449), upper: StringU32(4294967295) }),
                                        min_window_density: None,
                                        total_currency: None,
                                        global_slot_since_genesis: None,
                                        staking_epoch_data: EpochData {
                                            ledger: EpochLedger {
                                                hash: None,
                                                total_currency: None,
                                            },
                                            seed: Some(Field(Fp::from_str("1").unwrap())),
                                            start_checkpoint: None,
                                            lock_checkpoint: Some(Field(Fp::from_str("6933461832443085405172708095237859672674362434956861698472714852809077198806").unwrap())),
                                            epoch_length: Some(RangeCondition { lower: StringU32(0), upper: StringU32(188) }),
                                        },
                                        next_epoch_data: EpochData {
                                            ledger: EpochLedger {
                                                hash: None,
                                                total_currency: Some(RangeCondition { lower: StringU64(4242211157), upper: StringU64(10541640473847183) }),
                                            },
                                            seed: None,
                                            start_checkpoint: None,
                                            lock_checkpoint: Some(Field(Fp::from_str("68187513148643320151968717448670896871").unwrap())),
                                            epoch_length: None,
                                        },
                                    },
                                    account: AccountPreconditions {
                                        balance: None,
                                        nonce: Some(RangeCondition { lower: StringU32(0), upper: StringU32(0) }),
                                        receipt_chain_hash: Some(Field(Fp::from_str("17078643996102370880368295086152713687677929854046147378993883529889521440123").unwrap())),
                                        delegate: None,
                                        state: [None, Some(Field(Fp::from_str("3149174429374372848224536296365956656476630689199177967660926433625303199887").unwrap())), None, None, None, Some(Field(Fp::from_str("0").unwrap())), None, Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), Some(Field(Fp::from_str("26172628037243963189260094409118549936515518137258457712275491772448266998511").unwrap())), Some(Field(Fp::from_str("10693991393128995875905962194800030612465789485582547321456658383081936954148").unwrap())), Some(Field(Fp::from_str("19241625987476071115434419444360943322040876108559760885795131295274109256150").unwrap())), None, Some(Field(Fp::from_str("1").unwrap())), Some(Field(Fp::from_str("1996823741182").unwrap())), None, Some(Field(Fp::from_str("5766528646910935609391555730595050870394004582256347293067198195321003238922").unwrap())), None, None, None, None, Some(Field(Fp::from_str("11125362230233813099955120567516957202873952413841298671505272012185349338534").unwrap())), Some(Field(Fp::from_str("4220987236154774485251224").unwrap())), Some(Field(Fp::from_str("7796753267048464093513210746589944469813543636001008550184042262324165593250").unwrap())), Some(Field(Fp::from_str("9760162387129472396123782179705764683114587103618175231698591187702269458084").unwrap())), Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), None, None, Some(Field(Fp::from_str("3950292083709925678801164507607149583967962097523375772634355521798108149721").unwrap())), Some(Field(Fp::from_str("21293868965699556849161842612441144990560860807641393765829784490984968477434").unwrap())), Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), Some(Field(Fp::from_str("0").unwrap()))],
                                        action_state: Some(ActionState(Field(Fp::from_str("7075262455602522749145091500306305206384950827693527505754789089788100899554").unwrap()))),
                                        proved_state: Some(false),
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
                                public_key: PublicKey(CompressedPubKey::from_address("B62qo7EbWNgqn7EFtdSKUXEz4czy2S5eTZM4p6DNN8uR3X1Ahmk3GwC").unwrap()),
                                token_id: serde_json::from_str::<TokenId>(r#""wRqwVZtruwLmyGkPENyhyngeCF7CibX2tz1qCG6qMBrpVZ1ibr""#).unwrap(),
                                update: Update {
                                    app_state: [None, None, None, None, Some(Field(Fp::from_str("10823499909529060347011219366675643798966567351910121238954444391799282346469").unwrap())), None, Some(Field(Fp::from_str("3544823635936363729800465034030141332835695521957624056118468081546338076298").unwrap())), None, None, None, Some(Field(Fp::from_str("19951248832194796835482644309479434019534845477774680582101038490793616465859").unwrap())), Some(Field(Fp::from_str("1").unwrap())), None, Some(Field(Fp::from_str("26444327027265955381402525994841932345075658448322794500838480589835449325641").unwrap())), None, None, None, None, None, Some(Field(Fp::from_str("12226237685043444933969664020611349147868723198317607661526714038842750188542").unwrap())), Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), None, None, None, None, Some(Field(Fp::from_str("15779000313931440847843530837592894030276068750619898992993042814027575323655").unwrap())), None, Some(Field(Fp::from_str("17689712241340944321198726733111863568047999256098785805064239554389127601397").unwrap())), None, Some(Field(Fp::from_str("9155951549325270170882355078025962303545668180073882363135916343660154725700").unwrap())), Some(Field(Fp::from_str("1").unwrap())), None],
                                    delegate: Some(PublicKey(CompressedPubKey::from_address("B62qn7zSeJKVoGFGfNRGCwJu54kbSvUJAhB3SRetHxvvUBGhFmuUnLa").unwrap())),
                                    verification_key: None,
                                    permissions: Some(Permissions {
                                            edit_state: AuthRequired::Either,
                                            access: AuthRequired::None,
                                            send: AuthRequired::Proof,
                                            receive: AuthRequired::Impossible,
                                            set_delegate: AuthRequired::Signature,
                                            set_permissions: AuthRequired::Either,
                                            set_verification_key: SetVerificationKey {
                                                auth: AuthRequired::Either,
                                                txn_version: 22478713,
                                            },
                                            set_zkapp_uri: AuthRequired::None,
                                            edit_action_state: AuthRequired::None,
                                            set_token_symbol: AuthRequired::Signature,
                                            increment_nonce: AuthRequired::None,
                                            set_voting_for: AuthRequired::Either,
                                            set_timing: AuthRequired::None,
                                        }),
                                    zkapp_uri: Some(ZkappUri(vec![])),
                                    token_symbol: Some(TokenSymbol(vec![195, 132, 195, 147])),
                                    timing: Some(TimingData {
                                            initial_minimum_balance: 174144,
                                            cliff_time: 127877214,
                                            cliff_amount: 87254382681846826,
                                            vesting_period: 2,
                                            vesting_increment: 237086422004781583,
                                        }),
                                    voting_for: Some(Field(Fp::from_str("11301595078920912041264711454269004145781701924670712828086466406727915988157").unwrap())),
                                },
                                balance_change: BalanceChange {
                                    magnitude: 155370542454473358,
                                    sgn: -1,
                                },
                                increment_nonce: false,
                                events: Events {
                                    data: vec![
                                        vec![Field(Fp::from_str("5489320693602898178").unwrap()), Field(Fp::from_str("22827166469333148036313303852215327163306035527551999817196219171870057045955").unwrap()), Field(Fp::from_str("5078885347432216822311264717328391367730948659203272028383344934657580763000").unwrap()), Field(Fp::from_str("13553234213232827207916192266199726813665311424515955718159416247495299921455").unwrap()), Field(Fp::from_str("54135641683304253659335077").unwrap())],
                                        vec![Field(Fp::from_str("1").unwrap()), Field(Fp::from_str("0").unwrap()), Field(Fp::from_str("4678743164290030609432476326998415779695355758958161084512908998501500892861").unwrap()), Field(Fp::from_str("11648853450121824706889032181213011197262187258609054364544175793367059759722").unwrap())],
                                    ]
                                },
                                actions: Actions {
                                    data: vec![
                                        vec![Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap()), Field(Fp::from_str("8054080816490021799240818638092553041204083385158441763151988895089317620421").unwrap()), Field(Fp::from_str("27797234997703757079915075562110142250229026616945935794150292252063070216216").unwrap())],
                                    ]
                                },
                                call_data: Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap()),
                                call_depth: 0,
                                preconditions: Preconditions {
                                    network: NetworkPreconditions {
                                        snarked_ledger_hash: Some(Field(Fp::from_str("23698566093442667735931220710386335060281428386833651622289074716412997560374").unwrap())),
                                        blockchain_length: None,
                                        min_window_density: Some(RangeCondition { lower: StringU32(3), upper: StringU32(176298) }),
                                        total_currency: None,
                                        global_slot_since_genesis: None,
                                        staking_epoch_data: EpochData {
                                            ledger: EpochLedger {
                                                hash: None,
                                                total_currency: Some(RangeCondition { lower: StringU64(0), upper: StringU64(30442917) }),
                                            },
                                            seed: Some(Field(Fp::from_str("20608958054853676003132053237147841569531319424878054771758693129008635643860").unwrap())),
                                            start_checkpoint: Some(Field(Fp::from_str("6041315048026536453138036362076786022851294991062784802271661941609863474409").unwrap())),
                                            lock_checkpoint: None,
                                            epoch_length: Some(RangeCondition { lower: StringU32(1755), upper: StringU32(1) }),
                                        },
                                        next_epoch_data: EpochData {
                                            ledger: EpochLedger {
                                                hash: None,
                                                total_currency: None,
                                            },
                                            seed: None,
                                            start_checkpoint: None,
                                            lock_checkpoint: Some(Field(Fp::from_str("7563961736692985523587391571221219167042024175708266515445316167467582284071").unwrap())),
                                            epoch_length: Some(RangeCondition { lower: StringU32(9496), upper: StringU32(51) }),
                                        },
                                    },
                                    account: AccountPreconditions {
                                        balance: None,
                                        nonce: None,
                                        receipt_chain_hash: None,
                                        delegate: None,
                                        state: [Some(Field(Fp::from_str("7151094697189926986313870430012190").unwrap())), None, None, Some(Field(Fp::from_str("24379293287642777247535617246508178228647976016960151602915878336158822226225").unwrap())), Some(Field(Fp::from_str("3906294422428489202357978919922646752795492997360665069292680037769305408957").unwrap())), Some(Field(Fp::from_str("4614898986510250832492699978401926312633851818490502694191639060107800435006").unwrap())), None, None, Some(Field(Fp::from_str("13151537").unwrap())), None, Some(Field(Fp::from_str("313227510293875059764").unwrap())), None, Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), None, Some(Field(Fp::from_str("10639113444761388583090410735462292140659725884869645647383762115947369733256").unwrap())), None, Some(Field(Fp::from_str("15643033495669878123477164418772819877897483381286008279692958330708246755884").unwrap())), None, None, Some(Field(Fp::from_str("17888375580796040254826690008847733438519463227757009997093489121046086146099").unwrap())), None, None, Some(Field(Fp::from_str("19453744140398486316986987758934852739278603799092254343595139256313772285815").unwrap())), Some(Field(Fp::from_str("14240359861719133004854037780450434423170879356604407617578725487694650915278").unwrap())), None, Some(Field(Fp::from_str("0").unwrap())), Some(Field(Fp::from_str("21681247242622939751084992521650744055016829612537059966709718383044067274836").unwrap())), None, Some(Field(Fp::from_str("4575791272999345303977045070143396096949188760921450600346540068074620278372").unwrap())), Some(Field(Fp::from_str("529871249461").unwrap())), None, None],
                                        action_state: Some(ActionState(Field(Fp::from_str("25079927036070901246064867767436987657692091363973573142121686150614948079097").unwrap()))),
                                        proved_state: None,
                                        is_new: Some(false),
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
                network: NetworkId::Mainnet,
                expected_memo_hash: "146624400929844538317466382872834899021794596262855408933526545768996436172",
                expected_fee_payer_hash: "17625125338631214478067248741698987499857471018898959069936663860859204144447",
                expected_account_updates_commitment: "16701089263020407834843024913003975635578909248980470266874308193692964236257",
                expected_full_commitment: "2959247159365746507908037726855271232638114354746592018706991164070887195661",
            }
    ,
    ZkAppTestVector {
                name: "mesa_multi_updates_testnet",
                zkapp_command: ZKAppCommand {
                    fee_payer: FeePayer {
                        body: FeePayerBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qpjqd8zNT1LrtgwtBXvV55a76An5gjDjeuMP57jjHryBpaU1oddd").unwrap()),
                            fee: 4002727,
                            valid_until: Some(959),
                            nonce: 4294967295,
                        },
                        authorization: "7mWxjLYgbJUkZNcGouvhVj5tJ8yu9hoexb9ntvPK8t5LHqzmrL6QJjjKtf5SgmxB4QWkDw7qoMMbbNGtHVpsbJHPyTy2EzRQ".to_string(),
                    },
                    account_updates: vec![
    // Account update 1
                        AccountUpdate {
                            body: AccountUpdateBody {
                                public_key: PublicKey(CompressedPubKey::from_address("B62qnGKkrCvf52sE1M2BFCzTguHrUg2YEwLF6MfHQFYcfXdvGn8Djr1").unwrap()),
                                token_id: serde_json::from_str::<TokenId>(r#""wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf""#).unwrap(),
                                update: Update {
                                    app_state: [None, None, Some(Field(Fp::from_str("25722879211431342168676264516199397926500046016508834481845014799830070914168").unwrap())), None, Some(Field(Fp::from_str("1").unwrap())), Some(Field(Fp::from_str("1").unwrap())), Some(Field(Fp::from_str("0").unwrap())), Some(Field(Fp::from_str("9572049332126703182114566485522365274657760164576609427163495774267485835500").unwrap())), None, None, Some(Field(Fp::from_str("0").unwrap())), None, None, Some(Field(Fp::from_str("16710756900465977190631457159380522856885125610762424771390773151726859409687").unwrap())), Some(Field(Fp::from_str("19401241769665422854650845701908637202356048610744313537011448483662568575578").unwrap())), Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), None, Some(Field(Fp::from_str("22778031054996822147200102942554986885919947813181401160110797192928697795398").unwrap())), Some(Field(Fp::from_str("2078248153919310885890055121916140720028226982987804635954611540216832366074").unwrap())), None, None, None, None, Some(Field(Fp::from_str("23065344381834759875127246621689598113526294041405954852893534482978267278999").unwrap())), None, None, Some(Field(Fp::from_str("2434882").unwrap())), None, None, None, None, None],
                                    delegate: Some(PublicKey(CompressedPubKey::from_address("B62qpPYFU8pf55dXhPvVHSQrpPmD7nnbtYbtogYdB8PPM4Z6axRLju3").unwrap())),
                                    verification_key: Some(VerificationKeyData {
                                            data: "".to_string(),
                                            hash: Field(Fp::from_str("11443959858456975074010503145613058204824537127799244585759509361628651345657").unwrap()),
                                        }),
                                    permissions: None,
                                    zkapp_uri: Some(ZkappUri(vec![105])),
                                    token_symbol: None,
                                    timing: None,
                                    voting_for: None,
                                },
                                balance_change: BalanceChange {
                                    magnitude: 432000061351,
                                    sgn: 1,
                                },
                                increment_nonce: true,
                                events: Events {
                                    data: vec![]
                                },
                                actions: Actions {
                                    data: vec![]
                                },
                                call_data: Field(Fp::from_str("1118837149008168309163801934842521817045075542485539493707761873799980511680").unwrap()),
                                call_depth: 0,
                                preconditions: Preconditions {
                                    network: NetworkPreconditions {
                                        snarked_ledger_hash: Some(Field(Fp::from_str("7123796704636709729405744247").unwrap())),
                                        blockchain_length: Some(RangeCondition { lower: StringU32(789519892), upper: StringU32(1) }),
                                        min_window_density: Some(RangeCondition { lower: StringU32(6502), upper: StringU32(55) }),
                                        total_currency: Some(RangeCondition { lower: StringU64(184307280), upper: StringU64(1) }),
                                        global_slot_since_genesis: Some(RangeCondition { lower: StringU32(1), upper: StringU32(10) }),
                                        staking_epoch_data: EpochData {
                                            ledger: EpochLedger {
                                                hash: Some(Field(Fp::from_str("20639999900038212839012667428874516506252626116363408010450345342673457520168").unwrap())),
                                                total_currency: Some(RangeCondition { lower: StringU64(0), upper: StringU64(1719578) }),
                                            },
                                            seed: None,
                                            start_checkpoint: Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())),
                                            lock_checkpoint: Some(Field(Fp::from_str("73333771596141465").unwrap())),
                                            epoch_length: None,
                                        },
                                        next_epoch_data: EpochData {
                                            ledger: EpochLedger {
                                                hash: None,
                                                total_currency: Some(RangeCondition { lower: StringU64(0), upper: StringU64(7101433) }),
                                            },
                                            seed: None,
                                            start_checkpoint: Some(Field(Fp::from_str("0").unwrap())),
                                            lock_checkpoint: Some(Field(Fp::from_str("5392822419000830263440209014578444441210215240254311441885820756968443997602").unwrap())),
                                            epoch_length: Some(RangeCondition { lower: StringU32(33), upper: StringU32(37258021) }),
                                        },
                                    },
                                    account: AccountPreconditions {
                                        balance: Some(RangeCondition { lower: StringU64(120113223380), upper: StringU64(149379407182614) }),
                                        nonce: Some(RangeCondition { lower: StringU32(13), upper: StringU32(6729) }),
                                        receipt_chain_hash: Some(Field(Fp::from_str("0").unwrap())),
                                        delegate: Some(PublicKey(CompressedPubKey::from_address("B62qkU89gp5yvr8hhQfzhsa5rw7kXMhTvhhPimrwvCm4LTshUbpot3C").unwrap())),
                                        state: [None, Some(Field(Fp::from_str("0").unwrap())), None, Some(Field(Fp::from_str("8009595412907686774668776250437856919203102183146528889188486707696891279509").unwrap())), Some(Field(Fp::from_str("0").unwrap())), None, Some(Field(Fp::from_str("21851459113326812967805966845114390008880142489738420916933416178985632420624").unwrap())), Some(Field(Fp::from_str("1").unwrap())), Some(Field(Fp::from_str("4153021934882381604168568275026477672090734029844811653088272052360761710375").unwrap())), Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), None, None, Some(Field(Fp::from_str("1").unwrap())), Some(Field(Fp::from_str("11358612193978285779717883351177725887579938761323964234326940290577281091123").unwrap())), Some(Field(Fp::from_str("6827853860506487885").unwrap())), Some(Field(Fp::from_str("1").unwrap())), None, None, Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), Some(Field(Fp::from_str("15139452327931953641987589855825760869953325436788666329768185291999032888644").unwrap())), Some(Field(Fp::from_str("207217321984388276633790684179540603740179154363358374285999372049866157400").unwrap())), Some(Field(Fp::from_str("1").unwrap())), Some(Field(Fp::from_str("2278115033750394906912381593979232935857681467539007097018916446732127482902").unwrap())), None, None, Some(Field(Fp::from_str("623581953118115611870247185755243788714487340179181397495063885874854980589").unwrap())), None, None, None, Some(Field(Fp::from_str("5464907007084465272254637863846437512010540397683595254195236163500374408623").unwrap())), Some(Field(Fp::from_str("20510817613629024263523149496442563367973673965869393786269992706963012488276").unwrap())), None],
                                        action_state: Some(ActionState(Field(Fp::from_str("654581912").unwrap()))),
                                        proved_state: None,
                                        is_new: Some(false),
                                    },
                                    valid_while: None,
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
                        },// Account update 2
                        AccountUpdate {
                            body: AccountUpdateBody {
                                public_key: PublicKey(CompressedPubKey::from_address("B62qmJ3dKVttth7VYSktkQ3WuqfHuVNhBj6kPiRWh66jNndpRDsnLk7").unwrap()),
                                token_id: serde_json::from_str::<TokenId>(r#""xqp7Fc5xyrrn9shNP8YQskYKpbQSZuQfBWvNsBYYMoP84sDokn""#).unwrap(),
                                update: Update {
                                    app_state: [Some(Field(Fp::from_str("1").unwrap())), None, Some(Field(Fp::from_str("27378953626601942296805866275555015362598654523776609878606883510084502625341").unwrap())), None, None, Some(Field(Fp::from_str("4513925475446363860503470277878267236827638751456079026035452522256595185839").unwrap())), None, Some(Field(Fp::from_str("13657259534841432797998053447427140806949465382791699527567968567204273231083").unwrap())), Some(Field(Fp::from_str("65313265406576480089500113633730189").unwrap())), Some(Field(Fp::from_str("26649820124468392183432365729459780757462272882085141149836714539468889863776").unwrap())), None, None, None, Some(Field(Fp::from_str("29491189041241030519489").unwrap())), None, Some(Field(Fp::from_str("13877914242260031729518471844594176883537685937669883248448003084205386627523").unwrap())), Some(Field(Fp::from_str("1").unwrap())), None, Some(Field(Fp::from_str("0").unwrap())), Some(Field(Fp::from_str("9607828188873525678751750637513742794204914213308050519544069621985510518016").unwrap())), None, None, Some(Field(Fp::from_str("340282366920938463463374607431768211455").unwrap())), None, None, Some(Field(Fp::from_str("15612965733361192786846942849799995547165675713722559887017368818869020661690").unwrap())), Some(Field(Fp::from_str("0").unwrap())), None, None, Some(Field(Fp::from_str("23946044388384837356177479175120472014644687827390100871080185627748198517199").unwrap())), Some(Field(Fp::from_str("426944786469765366888").unwrap())), Some(Field(Fp::from_str("340282366920938463463374607431768211455").unwrap()))],
                                    delegate: None,
                                    verification_key: Some(VerificationKeyData {
                                            data: "Q".to_string(),
                                            hash: Field(Fp::from_str("0").unwrap()),
                                        }),
                                    permissions: Some(Permissions {
                                            edit_state: AuthRequired::None,
                                            access: AuthRequired::None,
                                            send: AuthRequired::Impossible,
                                            receive: AuthRequired::Signature,
                                            set_delegate: AuthRequired::None,
                                            set_permissions: AuthRequired::None,
                                            set_verification_key: SetVerificationKey {
                                                auth: AuthRequired::Either,
                                                txn_version: 5,
                                            },
                                            set_zkapp_uri: AuthRequired::Signature,
                                            edit_action_state: AuthRequired::None,
                                            set_token_symbol: AuthRequired::None,
                                            increment_nonce: AuthRequired::Proof,
                                            set_voting_for: AuthRequired::Impossible,
                                            set_timing: AuthRequired::Proof,
                                        }),
                                    zkapp_uri: None,
                                    token_symbol: Some(TokenSymbol(vec![194, 137, 60])),
                                    timing: None,
                                    voting_for: Some(Field(Fp::from_str("23700164602142873132109885746926918207821886289798556070731941331435670338567").unwrap())),
                                },
                                balance_change: BalanceChange {
                                    magnitude: 1668452065,
                                    sgn: -1,
                                },
                                increment_nonce: false,
                                events: Events {
                                    data: vec![
                                        vec![Field(Fp::from_str("1").unwrap()), Field(Fp::from_str("24238227666276526890959447020847098798646643838196621625042333912877478067537").unwrap())],
                                    ]
                                },
                                actions: Actions {
                                    data: vec![
                                        vec![Field(Fp::from_str("989").unwrap())],
                                        vec![Field(Fp::from_str("84624867981297737365677049208").unwrap()), Field(Fp::from_str("14806").unwrap()), Field(Fp::from_str("411899350550222052224036359081181186031015280261549590173393754336147331663").unwrap()), Field(Fp::from_str("5874381986230051155434864043142949545877169810681386709882726530666559288624").unwrap()), Field(Fp::from_str("6046813708069868299857325300120205660623800395565411493678680863320934225440").unwrap())],
                                    ]
                                },
                                call_data: Field(Fp::from_str("3506925884165365440481415664109842714921557751063142377813062873483948726313").unwrap()),
                                call_depth: 0,
                                preconditions: Preconditions {
                                    network: NetworkPreconditions {
                                        snarked_ledger_hash: Some(Field(Fp::from_str("25080830886777569781181671965633141364941268256638067743844422762373237179854").unwrap())),
                                        blockchain_length: None,
                                        min_window_density: Some(RangeCondition { lower: StringU32(2690), upper: StringU32(11582368) }),
                                        total_currency: None,
                                        global_slot_since_genesis: Some(RangeCondition { lower: StringU32(127), upper: StringU32(0) }),
                                        staking_epoch_data: EpochData {
                                            ledger: EpochLedger {
                                                hash: Some(Field(Fp::from_str("7363886724036907587541981010667946094274683695244103450243443613479622636021").unwrap())),
                                                total_currency: Some(RangeCondition { lower: StringU64(157960067752752844), upper: StringU64(7826330258631) }),
                                            },
                                            seed: Some(Field(Fp::from_str("1").unwrap())),
                                            start_checkpoint: None,
                                            lock_checkpoint: Some(Field(Fp::from_str("1").unwrap())),
                                            epoch_length: None,
                                        },
                                        next_epoch_data: EpochData {
                                            ledger: EpochLedger {
                                                hash: None,
                                                total_currency: None,
                                            },
                                            seed: Some(Field(Fp::from_str("0").unwrap())),
                                            start_checkpoint: None,
                                            lock_checkpoint: Some(Field(Fp::from_str("411822408722627669588828491106565487739116346416928496715353449693502735744").unwrap())),
                                            epoch_length: None,
                                        },
                                    },
                                    account: AccountPreconditions {
                                        balance: None,
                                        nonce: None,
                                        receipt_chain_hash: Some(Field(Fp::from_str("18141948382299266043476359897254346762575106827916914806475601123129259656773").unwrap())),
                                        delegate: Some(PublicKey(CompressedPubKey::from_address("B62qpXKVJ2vQLnjKupCbBa61mF2Pwq6MTjHHHWWBzmFkZ9j4Q8uHSuP").unwrap())),
                                        state: [Some(Field(Fp::from_str("21240469496922067411484975485922651396136997213123205662068879910540254959639").unwrap())), None, None, None, Some(Field(Fp::from_str("24644159913519507290781813578021975462785993129282390906462715535236259910509").unwrap())), Some(Field(Fp::from_str("0").unwrap())), Some(Field(Fp::from_str("6669414776907025850505769857942568825925151036157499654335672803127006962704").unwrap())), Some(Field(Fp::from_str("959814042777676860038141335637974772734043006932090571390705528991382427895").unwrap())), Some(Field(Fp::from_str("0").unwrap())), Some(Field(Fp::from_str("340282366920938463463374607431768211455").unwrap())), Some(Field(Fp::from_str("11815847358868725380025614716732175138472475498185898146724277599515896750487").unwrap())), None, Some(Field(Fp::from_str("15589329615122910347319702226551362074593709538711203623377080825561087903754").unwrap())), Some(Field(Fp::from_str("11289952148735379687129404404883487197928880152029397838233932450329970866729").unwrap())), Some(Field(Fp::from_str("589951164666000194380").unwrap())), Some(Field(Fp::from_str("821138287695519199114713").unwrap())), Some(Field(Fp::from_str("4979726405869148373004783809421174333864587217828345227190934485781517366835").unwrap())), None, None, Some(Field(Fp::from_str("0").unwrap())), None, Some(Field(Fp::from_str("22026164584421990944718013210781224827191986439204115932103740899899119099742").unwrap())), Some(Field(Fp::from_str("27035991020275564484263124554053734388953938804374732077789346936833942989033").unwrap())), Some(Field(Fp::from_str("952219001696064266993315386384065658290222441979208593091572772987833935496").unwrap())), None, None, Some(Field(Fp::from_str("7490766108512930110938817804").unwrap())), Some(Field(Fp::from_str("0").unwrap())), None, Some(Field(Fp::from_str("19185563166800775228090905370868468299729308141935749853490428258753500442736").unwrap())), Some(Field(Fp::from_str("1").unwrap())), None],
                                        action_state: Some(ActionState(Field(Fp::from_str("6").unwrap()))),
                                        proved_state: None,
                                        is_new: None,
                                    },
                                    valid_while: None,
                                },
                                use_full_commitment: true,
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
                    memo: decode_memo_from_base58("E4Z1SmRE3NAuW2x2MieoWUhjWxj58JmAjw6mKGU3oEaJsu6gEyynL"),
                },
                network: NetworkId::Testnet,
                expected_memo_hash: "886171315199036757616457119846220609097576016960793878493372838551749984985",
                expected_fee_payer_hash: "6149405278884192689183621063840710190294686807111502140785434478199819947765",
                expected_account_updates_commitment: "5579206366278583927248719941974151681594181400282572410974127081015044309845",
                expected_full_commitment: "12492864672369725550719565699610887650519817426609638071067289372865798929278",
            },
            ZkAppTestVector {
                name: "custom_network_mesa",
                zkapp_command: ZKAppCommand {
                    fee_payer: FeePayer {
                        body: FeePayerBody {
                            public_key: PublicKey(
                                CompressedPubKey::from_address(
                                    "B62qnf1xqEYRRGVsM7tbnFheHoEaT3ANf6MPM5129uSzG1bEw1cLtZm",
                                )
                                .unwrap(),
                            ),
                            fee: 17357990258,
                            valid_until: None,
                            nonce: 1,
                        },
                        authorization: "7mWxjLYgbJUkZNcGouvhVj5tJ8yu9hoexb9ntvPK8t5LHqzmrL6QJjjKtf5SgmxB4QWkDw7qoMMbbNGtHVpsbJHPyTy2EzRQ".to_string(),
                    },
                    account_updates: vec![],
                    memo: decode_memo_from_base58("E4YUMMmkRqpqJcAAixf9g4zsDdXtBxnzCCkay325sv2Wd3qVNLRJe"),
                },
                network: NetworkId::Custom("custom-id".to_string()),
                expected_memo_hash: "414614572568271397074920870226012939469202102804613504114174314175004650696",
                expected_fee_payer_hash: "27839477940695098718573185239421761980280418806388065886555436744674456661831",
                expected_account_updates_commitment: "0",
                expected_full_commitment: "9944833215345814040133998613943029158665949800205019870290739265389261376140",
            }
    ]
}
