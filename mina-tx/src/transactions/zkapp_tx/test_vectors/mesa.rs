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
        }

    ]
}
