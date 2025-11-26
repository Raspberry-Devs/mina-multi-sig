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
                            token_id: TokenId(Field(Fp::from_str("wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf").unwrap())),
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
                                voting_for: Some(Field(Fp::from_str("3NKqWEMwYeRsRNaSQaBBoTFBKZ7qatW65U3UHn5y2kM7uq72CH6x").unwrap())),
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
                            token_id: TokenId(Field(Fp::from_str("weq5b52jptNgLY7cKHUZ2wAGTSQgm7jD59DvqhVz2HQbF6uX5t").unwrap())),
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
                            token_id: TokenId(Field(Fp::from_str("wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf").unwrap())),
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
                            token_id: TokenId(Field(Fp::from_str("wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf").unwrap())),
                            update: Update {
                                app_state: [Some(Field(Fp::from_str("10796934224552702979618362115631122037483267946478161437563101919023065453886").unwrap())), Some(Field(Fp::from_str("1665702701348261").unwrap())), Some(Field(Fp::from_str("19520046187718255296557963475173868409759330658774802116230322935726722115976").unwrap())), Some(Field(Fp::from_str("1").unwrap())), Some(Field(Fp::from_str("7312170862030764706254802815327734781795053504536007431702726987329616279353").unwrap())), Some(Field(Fp::from_str("13573309228248164810767138546996638425118106865672570529103058718212908407164").unwrap())), None, None],
                                delegate: Some(PublicKey(CompressedPubKey::from_address("B62qnFL8Z5tmzy9DD7qtBKgKUUZ5CLq7UWWWPW9ooLJXZb7GhgB7ViX").unwrap())),
                                verification_key: Some(VerificationKeyData {
                                        data: "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".to_string(),
                                        hash: Field(Fp::from_str("3392518251768960475377392625298437850623664973002200885669375116181514017494").unwrap()),
                                    }),
                                permissions: None,
                                zkapp_uri: Some(ZkappUri(" ;>×7ÄùT.".as_bytes().to_vec())),
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
            expected_memo_hash: "0",
            expected_fee_payer_hash: "0",
            expected_account_updates_commitment: "0",
            expected_full_commitment: "0",
        }
