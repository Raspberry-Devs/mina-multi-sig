ZkAppTestVector {
            name: "complex_zkapp_command",
            zkapp_command: ZKAppCommand {
                fee_payer: FeePayer {
                    body: FeePayerBody {
                        public_key: PublicKey(CompressedPubKey::from_address("B62qr8mHb1dmTK9iQsQmmem5f3LR18LkFTabzWNY1hcZLcbaSg7boqA").unwrap()),
                        fee: 18446744073709551615,
                        valid_until: Some(514004985),
                        nonce: 0,
                    },
                    authorization: "7mWxjLYgbJUkZNcGouvhVj5tJ8yu9hoexb9ntvPK8t5LHqzmrL6QJjjKtf5SgmxB4QWkDw7qoMMbbNGtHVpsbJHPyTy2EzRQ".to_string(),
                },
                account_updates: vec![
// Account update 1
                    AccountUpdate {
                        body: AccountUpdateBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qpxeoHPBkBAhaoWtpuMp3Qp5WKmsrsavNhLRxdphAKmTDk8n3R8f").unwrap()),
                            token_id: TokenId(Field(Fp::from_str("xQTFysPt3XFr9zgWfosyi3MsQLv4TneCm3pWUnVayJFLDVdbEL").unwrap())),
                            update: Update {
                                app_state: [None, Some(Field(Fp::from_str("22391495384865392682644504526639287474733690063174618053463880719985452827421").unwrap())), Some(Field(Fp::from_str("15661882943878691091332728429985411116558436591114876360248864523072735264924").unwrap())), Some(Field(Fp::from_str("1689259").unwrap())), None, Some(Field(Fp::from_str("5133265229526431152629358067249435010197674597040400755517979490632202184732").unwrap())), Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), Some(Field(Fp::from_str("19968052651778491608149470089295539009464513942380265761625525874009871646687").unwrap()))],
                                delegate: None,
                                verification_key: None,
                                permissions: Some(Permissions {
                                        edit_state: AuthRequired::Either,
                                        access: AuthRequired::Proof,
                                        send: AuthRequired::None,
                                        receive: AuthRequired::Proof,
                                        set_delegate: AuthRequired::Signature,
                                        set_permissions: AuthRequired::Signature,
                                        set_verification_key: SetVerificationKey {
                                            auth: AuthRequired::Signature,
                                            txn_version: 2453,
                                        },
                                        set_zkapp_uri: AuthRequired::Either,
                                        edit_action_state: AuthRequired::Either,
                                        set_token_symbol: AuthRequired::Either,
                                        increment_nonce: AuthRequired::None,
                                        set_voting_for: AuthRequired::Impossible,
                                        set_timing: AuthRequired::Either,
                                    }),
                                zkapp_uri: None,
                                token_symbol: None,
                                timing: Some(TimingData {
                                        initial_minimum_balance: 0,
                                        cliff_time: 60852,
                                        cliff_amount: 3933,
                                        vesting_period: 16023,
                                        vesting_increment: 3075949746781736,
                                    }),
                                voting_for: None,
                            },
                            balance_change: BalanceChange {
                                magnitude: 18446744073709551615,
                                sgn: 1,
                            },
                            increment_nonce: true,
                            events: Events {
                                data: vec![
                                    vec![Field(Fp::from_str("11460262854227521764052406647848415226417188898450223582833581233355666059819").unwrap()), Field(Fp::from_str("1").unwrap()), Field(Fp::from_str("5316765498800042531734873879723656889109083076804463892923888437367834979434").unwrap()), Field(Fp::from_str("21073564978891839295341444710436677017369727873796640697152330300589811893343").unwrap()), Field(Fp::from_str("1").unwrap())],
                                ]
                            },
                            actions: Actions {
                                data: vec![]
                            },
                            call_data: Field(Fp::from_str("15843810235585021999847222206077364471398350798533832321311409830592122692579").unwrap()),
                            call_depth: 0,
                            preconditions: Preconditions {
                                network: NetworkPreconditions {
                                    snarked_ledger_hash: None,
                                    blockchain_length: None,
                                    min_window_density: Some(RangeCondition { lower: 173530, upper: 45726 }),
                                    total_currency: None,
                                    global_slot_since_genesis: None,
                                    staking_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: Some(Field(Fp::from_str("960979915518433684746").unwrap())),
                                            total_currency: Some(RangeCondition { lower: 108277737489898340, upper: 156620495353075 }),
                                        },
                                        seed: None,
                                        start_checkpoint: None,
                                        lock_checkpoint: None,
                                        epoch_length: Some(RangeCondition { lower: 8639, upper: 0 }),
                                    },
                                    next_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: Some(Field(Fp::from_str("20589391863338477150122935440878266588579114085086702435802235725370215848946").unwrap())),
                                            total_currency: Some(RangeCondition { lower: 2329838522953452, upper: 1555697 }),
                                        },
                                        seed: Some(Field(Fp::from_str("1").unwrap())),
                                        start_checkpoint: None,
                                        lock_checkpoint: Some(Field(Fp::from_str("1").unwrap())),
                                        epoch_length: None,
                                    },
                                },
                                account: AccountPreconditions {
                                    balance: None,
                                    nonce: None,
                                    receipt_chain_hash: Some(Field(Fp::from_str("0").unwrap())),
                                    delegate: None,
                                    state: [Some(Field(Fp::from_str("27972805851206076938536253677799174209675745163748035091354370497271833733822").unwrap())), Some(Field(Fp::from_str("9817647606518912723144158675066382828285018044272310083250623334965598660434").unwrap())), Some(Field(Fp::from_str("3282585524568924814649560").unwrap())), Some(Field(Fp::from_str("1514592485377976572113699835747598236584215496437845476805755340833134487624").unwrap())), None, Some(Field(Fp::from_str("8474929086710277090424203595320156485455680283076706061930944995803877697271").unwrap())), Some(Field(Fp::from_str("16792520951922568779718319165836520332290602687352306876554941199681802329989").unwrap())), Some(Field(Fp::from_str("24524151434138816540225848797935040849520299772566853046292998096346594814989").unwrap()))],
                                    action_state: None,
                                    proved_state: None,
                                    is_new: Some(false),
                                },
                                valid_while: None,
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
                    },// Account update 2
                    AccountUpdate {
                        body: AccountUpdateBody {
                            public_key: PublicKey(CompressedPubKey::from_address("B62qr8mHb1dmTK9iQsQmmem5f3LR18LkFTabzWNY1hcZLcbaSg7boqA").unwrap()),
                            token_id: TokenId(Field(Fp::from_str("wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf").unwrap())),
                            update: Update {
                                app_state: [None, Some(Field(Fp::from_str("22099339267285580040220587419163137365589441048857911869248777386807817011001").unwrap())), Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), Some(Field(Fp::from_str("17489861719567921303989693159386606155256986629029053682055035656184404065825").unwrap())), Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())), None, Some(Field(Fp::from_str("275760127786").unwrap())), None],
                                delegate: None,
                                verification_key: None,
                                permissions: None,
                                zkapp_uri: Some(ZkappUri("DQß".as_bytes().to_vec())),
                                token_symbol: None,
                                timing: None,
                                voting_for: None,
                            },
                            balance_change: BalanceChange {
                                magnitude: 31442917,
                                sgn: -1,
                            },
                            increment_nonce: false,
                            events: Events {
                                data: vec![
                                    vec![Field(Fp::from_str("25925916482127274311119786994727134730386792385048095033731474705117865055210").unwrap()), Field(Fp::from_str("1").unwrap())],
                                ]
                            },
                            actions: Actions {
                                data: vec![]
                            },
                            call_data: Field(Fp::from_str("11981510817234694322623340519213091148076697653254399734892418556506307973560").unwrap()),
                            call_depth: 0,
                            preconditions: Preconditions {
                                network: NetworkPreconditions {
                                    snarked_ledger_hash: Some(Field(Fp::from_str("0").unwrap())),
                                    blockchain_length: Some(RangeCondition { lower: 65934198, upper: 13152 }),
                                    min_window_density: None,
                                    total_currency: None,
                                    global_slot_since_genesis: Some(RangeCondition { lower: 463, upper: 153 }),
                                    staking_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: Some(Field(Fp::from_str("0").unwrap())),
                                            total_currency: None,
                                        },
                                        seed: Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())),
                                        start_checkpoint: None,
                                        lock_checkpoint: Some(Field(Fp::from_str("3323909930294110987843457379819707561028591208086791284524617494219620937659").unwrap())),
                                        epoch_length: None,
                                    },
                                    next_epoch_data: EpochData {
                                        ledger: EpochLedger {
                                            hash: Some(Field(Fp::from_str("214950335309810179873741312300").unwrap())),
                                            total_currency: Some(RangeCondition { lower: 148778234, upper: 3619868367810 }),
                                        },
                                        seed: None,
                                        start_checkpoint: Some(Field(Fp::from_str("0").unwrap())),
                                        lock_checkpoint: Some(Field(Fp::from_str("28948022309329048855892746252171976963363056481941560715954676764349967630336").unwrap())),
                                        epoch_length: None,
                                    },
                                },
                                account: AccountPreconditions {
                                    balance: None,
                                    nonce: None,
                                    receipt_chain_hash: Some(Field(Fp::from_str("57068863060666246693036865681").unwrap())),
                                    delegate: Some(PublicKey(CompressedPubKey::from_address("B62qpjr9qAynMtK44vAH5Xc8dYaM7JZ4jYcJAvYWcGoy5R1kjqLX3N1").unwrap())),
                                    state: [None, None, Some(Field(Fp::from_str("14465361887502907641568253480101299539974740826170984063557419962968241112798").unwrap())), Some(Field(Fp::from_str("4562101992720423260283154223070083360799831056086725398194765030577847806482").unwrap())), None, Some(Field(Fp::from_str("7785674246514481377184623964614815359596216649453631651646965437094636764719").unwrap())), None, Some(Field(Fp::from_str("219947376674640687395580326484213190183").unwrap()))],
                                    action_state: None,
                                    proved_state: None,
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
                    },
                ],
                memo: decode_memo_from_base58("E4YM2vTHhWEg66xpj52JErHUBU4pZ1yageL4TVDDpTTSsv8mK6YaH"),
            },
            network: NetworkId::MAINNET,
            expected_memo_hash: "0",
            expected_fee_payer_hash: "0",
            expected_account_updates_commitment: "0",
            expected_full_commitment: "0",
        }
