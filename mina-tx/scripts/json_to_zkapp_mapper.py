#!/usr/bin/env python3
"""
JSON to ZkAppCommand Mapper

Converts a JavaScript object representation of a ZkApp command into Rust code for test vectors.
This script generates the Rust struct initialization code that can be used in test_vectors.rs.
"""

import re
import sys
from typing import Any, Dict, List, Optional, Union


def parse_js_object(content: str) -> Dict[str, Any]:
    """Parse a JavaScript object literal with BigInt support."""
    # Remove comments and normalize whitespace
    content = re.sub(r'//.*$', '', content, flags=re.MULTILINE)

    # Replace BigInt literals (numbers followed by 'n') with regular numbers
    content = re.sub(r'(\d+)n\b', r'\1', content)

    # Replace undefined with null
    content = re.sub(r'\bundefined\b', 'null', content)

    # Replace JavaScript object key syntax (unquoted keys) with JSON syntax
    content = re.sub(r'(\w+):', r'"\1":', content)

    # Handle trailing commas
    content = re.sub(r',(\s*[}\]])', r'\1', content)

    # Now we can parse it as JSON
    import json
    try:
        return json.loads(content)
    except json.JSONDecodeError as e:
        # If JSON parsing fails, provide more helpful error info
        lines = content.split('\n')
        error_line = lines[e.lineno - 1] if e.lineno <= len(lines) else "Unknown line"
        print(f"JSON parsing failed at line {e.lineno}: {error_line}")
        print(f"Error: {e.msg}")
        raise


def escape_string(s: str) -> str:
    """Escape special characters in strings for Rust string literals."""
    return s.replace('\\', '\\\\').replace('"', '\\"').replace('\n', '\\n').replace('\r', '\\r')


def _is_int_like(v: Any) -> bool:
    return isinstance(v, int) or (isinstance(v, str) and v.isdigit())


def _int_token(v: Any) -> str:
    if isinstance(v, int):
        return str(v)
    if isinstance(v, str) and v.isdigit():
        return v
    raise ValueError(f"Expected integer-like value, got: {v!r}")


def format_field(value: Union[str, int, None]) -> str:
    """Format a field value for Rust Field type."""
    if value is None:
        return "Field::default()"
    if isinstance(value, int):
        return f'Field(Fp::from_str("{value}").unwrap())'
    if isinstance(value, str):
        # assume numeric string for Field
        return f'Field(Fp::from_str("{value}").unwrap())'
    raise ValueError(f"Unsupported field value: {value!r}")


def format_field_token(value: Union[str, int, None]) -> str:
    """Format a field used inside TokenId."""
    # TokenId always wraps a Field, but the inner representation is the same.
    inner = format_field(value)
    return f"TokenId({inner})"


def format_public_key(key_data: Union[str, Dict[str, Any], None]) -> str:
    """Format a public key for Rust PublicKey type."""
    if key_data is None:
        return "PublicKey::default()"
    if isinstance(key_data, str):
        return f'PublicKey(CompressedPubKey::from_address("{escape_string(key_data)}").unwrap())'
    # Only address format supported by the Rust type in this codebase
    return "PublicKey::default()"


def format_option(inner: str) -> str:
    """Wrap a Rust expression in Some(...)."""
    return f"Some({inner})"


def format_option_field(value: Any) -> str:
    if value is None:
        return "None"
    return format_option(format_field(value))


def format_option_public_key(value: Any) -> str:
    if value is None:
        return "None"
    return format_option(format_public_key(value))


def format_option_bool(value: Any) -> str:
    if value is None:
        return "None"
    return f"Some({str(value).lower()})"


def format_range_condition_opt(range_data: Optional[Dict[str, Any]], type_name: str) -> str:
    """Format Option<RangeCondition<T>> for Rust."""
    if range_data is None:
        return "None"
    lower = _int_token(range_data.get("lower", 0))
    upper = _int_token(range_data.get("upper", 0))
    return f"Some(RangeCondition {{ lower: {lower}, upper: {upper} }})"


def format_auth_required(auth_str: Optional[str]) -> str:
    """Map string to AuthRequired enum."""
    if not auth_str:
        return "AuthRequired::None"
    mapping = {
        "Signature": "AuthRequired::Signature",
        "Impossible": "AuthRequired::Impossible",
        "Either": "AuthRequired::Either",
        "Proof": "AuthRequired::Proof",
        "None": "AuthRequired::None",
        "Both": "AuthRequired::Both",
    }
    return mapping.get(auth_str, "AuthRequired::None")


def format_app_state(app_state: List[Any]) -> str:
    """Format app_state as a slice [Option<Field>; 8]."""
    items: List[str] = []
    for item in app_state:
        if item is None:
            items.append("None")
        else:
            items.append(f"Some({format_field(item)})")
    joined_inline = ", ".join(items)
    # match zkapp_proper.rs style: slice literal
    return f"[{joined_inline}]"


def format_events(events: List[List[Union[str, int]]]) -> str:
    """Format events for Rust Events struct without hash field."""
    if not events:
        data_formatted = "vec![]"
    else:
        formatted_events: List[str] = []
        for event in events:
            formatted_fields = [format_field(field) for field in event]
            formatted_events.append(f"vec![{', '.join(formatted_fields)}]")
        joined_events = ',\n                                    '.join(formatted_events)
        data_formatted = f"vec![\n                                    {joined_events},\n                                ]"
    return f"""Events {{
                                data: {data_formatted}
                            }}"""


def format_actions(actions: List[List[Union[str, int]]]) -> str:
    """Format actions array for Rust Actions struct without hash field."""
    if not actions:
        data_formatted = "vec![]"
    else:
        formatted_actions: List[str] = []
        for action in actions:
            formatted_fields = [format_field(field) for field in action]
            formatted_actions.append(f"vec![{', '.join(formatted_fields)}]")
        joined_actions = ',\n                                    '.join(formatted_actions)
        data_formatted = f"vec![\n                                    {joined_actions},\n                                ]"
    return f"""Actions {{
                                data: {data_formatted}
                            }}"""


def format_account_state(state: List[Any]) -> str:
    """Format account state as a slice [Option<Field>; 8]."""
    items: List[str] = []
    for item in state:
        if item is None:
            items.append("None")
        else:
            items.append(f"Some({format_field(item)})")
    joined_inline = ", ".join(items)
    return f"[{joined_inline}]"


def format_verification_key(vk_data: Optional[Dict[str, Any]]) -> str:
    """Format Option<VerificationKeyData>."""
    if not vk_data:
        return "None"
    data = vk_data.get("data", "")
    hash_val = vk_data.get("hash", "0")
    return f"""Some(VerificationKeyData {{
                                        data: "{escape_string(data)}".to_string(),
                                        hash: {format_field(hash_val)},
                                    }})"""


def format_permissions(permissions: Optional[Dict[str, Any]]) -> str:
    """Format Option<Permissions>."""
    if not permissions:
        return "None"
    set_vk = permissions.get("setVerificationKey", {})
    auth = set_vk.get("auth", "None") if isinstance(set_vk, dict) else "None"
    txn_version = set_vk.get("txnVersion", "0") if isinstance(set_vk, dict) else "0"
    txn_version_tok = _int_token(txn_version)

    return f"""Some(Permissions {{
                                        edit_state: {format_auth_required(permissions.get("editState"))},
                                        access: {format_auth_required(permissions.get("access"))},
                                        send: {format_auth_required(permissions.get("send"))},
                                        receive: {format_auth_required(permissions.get("receive"))},
                                        set_delegate: {format_auth_required(permissions.get("setDelegate"))},
                                        set_permissions: {format_auth_required(permissions.get("setPermissions"))},
                                        set_verification_key: SetVerificationKey {{
                                            auth: {format_auth_required(auth)},
                                            txn_version: {txn_version_tok},
                                        }},
                                        set_zkapp_uri: {format_auth_required(permissions.get("setZkappUri"))},
                                        edit_action_state: {format_auth_required(permissions.get("editActionState"))},
                                        set_token_symbol: {format_auth_required(permissions.get("setTokenSymbol"))},
                                        increment_nonce: {format_auth_required(permissions.get("incrementNonce"))},
                                        set_voting_for: {format_auth_required(permissions.get("setVotingFor"))},
                                        set_timing: {format_auth_required(permissions.get("setTiming"))},
                                    }})"""


def format_timing(timing_data: Optional[Dict[str, Any]]) -> str:
    """Format Option<TimingData>."""
    if not timing_data:
        return "None"
    return f"""Some(TimingData {{
                                        initial_minimum_balance: {_int_token(timing_data.get("initialMinimumBalance", 0))},
                                        cliff_time: {_int_token(timing_data.get("cliffTime", 0))},
                                        cliff_amount: {_int_token(timing_data.get("cliffAmount", 0))},
                                        vesting_period: {_int_token(timing_data.get("vestingPeriod", 0))},
                                        vesting_increment: {_int_token(timing_data.get("vestingIncrement", 0))},
                                    }})"""


def format_option_token_symbol(value: Any) -> str:
    if value is None:
        return "None"
    s = escape_string(value)
    return f'Some(TokenSymbol("{s}".as_bytes().to_vec()))'


def format_option_zkapp_uri(value: Any) -> str:
    if value is None:
        return "None"
    s = escape_string(value)
    return f'Some(ZkappUri("{s}".as_bytes().to_vec()))'


def format_epoch_data(epoch_data: Dict[str, Any]) -> str:
    """Format EpochData."""
    ledger = epoch_data.get("ledger", {})
    ledger_hash = ledger.get("hash")
    ledger_currency = ledger.get("totalCurrency")

    seed = epoch_data.get("seed")
    start_checkpoint = epoch_data.get("startCheckpoint")
    lock_checkpoint = epoch_data.get("lockCheckpoint")
    epoch_length = epoch_data.get("epochLength")

    return f"""EpochData {{
                                        ledger: EpochLedger {{
                                            hash: {format_option_field(ledger_hash)},
                                            total_currency: {format_range_condition_opt(ledger_currency, "u64")},
                                        }},
                                        seed: {format_option_field(seed)},
                                        start_checkpoint: {format_option_field(start_checkpoint)},
                                        lock_checkpoint: {format_option_field(lock_checkpoint)},
                                        epoch_length: {format_range_condition_opt(epoch_length, "u32")},
                                    }}"""


def format_action_state_opt(value: Any) -> str:
    """Format Option<ActionState>."""
    if value is None:
        return "None"
    inner = format_field(value)
    return f"Some(ActionState({inner}))"


def format_account_update(update: Dict[str, Any], index: int, proper: bool) -> str:
    """Format a single account update for Rust."""
    body = update["body"]

    # Balance change
    balance_change = body["balanceChange"]
    magnitude = _int_token(balance_change["magnitude"])
    sgn = balance_change["sgn"]
    sgn_value = "1" if sgn == "Positive" else "-1"

    # may_use_token
    may_use_token = body["mayUseToken"]

    # authorization kind
    auth_kind = body["authorizationKind"]

    # preconditions
    preconditions = body["preconditions"]
    network = preconditions["network"]
    account = preconditions["account"]

    # update fields
    update_data = body["update"]

    # Always wrap token_id in TokenId(...)
    token_formatter = format_field_token

    return f"""// Account update {index + 1}
                    AccountUpdate {{
                        body: AccountUpdateBody {{
                            public_key: {format_public_key(body["publicKey"])},
                            token_id: {token_formatter(body["tokenId"])},
                            update: Update {{
                                app_state: {format_app_state(update_data["appState"])},
                                delegate: {format_option_public_key(update_data.get("delegate"))},
                                verification_key: {format_verification_key(update_data.get("verificationKey"))},
                                permissions: {format_permissions(update_data.get("permissions"))},
                                zkapp_uri: {format_option_zkapp_uri(update_data.get("zkappUri"))},
                                token_symbol: {format_option_token_symbol(update_data.get("tokenSymbol"))},
                                timing: {format_timing(update_data.get("timing"))},
                                voting_for: {format_option_field(update_data.get("votingFor"))},
                            }},
                            balance_change: BalanceChange {{
                                magnitude: {magnitude},
                                sgn: {sgn_value},
                            }},
                            increment_nonce: {str(body["incrementNonce"]).lower()},
                            events: {format_events(body["events"])},
                            actions: {format_actions(body["actions"])},
                            call_data: {format_field(body["callData"])},
                            call_depth: {_int_token(body["callDepth"])},
                            preconditions: Preconditions {{
                                network: NetworkPreconditions {{
                                    snarked_ledger_hash: {format_option_field(network.get("snarkedLedgerHash"))},
                                    blockchain_length: {format_range_condition_opt(network.get("blockchainLength"), "u32")},
                                    min_window_density: {format_range_condition_opt(network.get("minWindowDensity"), "u32")},
                                    total_currency: {format_range_condition_opt(network.get("totalCurrency"), "u64")},
                                    global_slot_since_genesis: {format_range_condition_opt(network.get("globalSlotSinceGenesis"), "u32")},
                                    staking_epoch_data: {format_epoch_data(network["stakingEpochData"])},
                                    next_epoch_data: {format_epoch_data(network["nextEpochData"])},
                                }},
                                account: AccountPreconditions {{
                                    balance: {format_range_condition_opt(account.get("balance"), "u64")},
                                    nonce: {format_range_condition_opt(account.get("nonce"), "u32")},
                                    receipt_chain_hash: {format_option_field(account.get("receiptChainHash"))},
                                    delegate: {format_option_public_key(account.get("delegate"))},
                                    state: {format_account_state(account["state"])},
                                    action_state: {format_action_state_opt(account.get("actionState"))},
                                    proved_state: {format_option_bool(account.get("provedState"))},
                                    is_new: {format_option_bool(account.get("isNew"))},
                                }},
                                valid_while: {format_range_condition_opt(preconditions.get("validWhile"), "u32")},
                            }},
                            use_full_commitment: {str(body["useFullCommitment"]).lower()},
                            implicit_account_creation_fee: {str(body["implicitAccountCreationFee"]).lower()},
                            may_use_token: MayUseToken {{
                                parents_own_token: {str(may_use_token["parentsOwnToken"]).lower()},
                                inherit_from_parent: {str(may_use_token["inheritFromParent"]).lower()},
                            }},
                            authorization_kind: AuthorizationKind {{
                                is_signed: {str(auth_kind["isSigned"]).lower()},
                                is_proved: {str(auth_kind["isProved"]).lower()},
                                verification_key_hash: {format_field(auth_kind["verificationKeyHash"])},
                            }},
                        }},
                        authorization: Authorization {{
                            proof: None,
                            signature: None,
                        }},
                    }}"""


def generate_zkapp_command(js_data: Dict[str, Any], test_name: str = "complex_zkapp_command") -> str:
    """Generate the complete ZKAppCommand Rust code from JavaScript object data."""
    fee_payer = js_data["feePayer"]
    account_updates = js_data["accountUpdates"]
    memo = js_data["memo"]

    # Heuristic: still used for token_id / action_state / network & hashes.
    proper = test_name == "multiple_account_updates"

    formatted_updates: List[str] = []
    for i, update in enumerate(account_updates):
        formatted_updates.append(format_account_update(update, i, proper))

    valid_until_val = fee_payer["body"].get("validUntil", None)
    valid_until = f"Some({_int_token(valid_until_val)})" if valid_until_val is not None else "None"

    # Always use decode_memo_from_base58 for memo
    memo_expr = f'decode_memo_from_base58("{escape_string(memo)}")'

    if proper:
        network = "NetworkId::TESTNET"
        expected_memo_hash = js_data.get("expectedMemoHash", "0")
        expected_fee_payer_hash = js_data.get("expectedFeePayerHash", "0")
        expected_account_updates_commitment = js_data.get("expectedAccountUpdatesCommitment", "0")
        expected_full_commitment = js_data.get("expectedFullCommitment", "0")
    else:
        network = "NetworkId::MAINNET"
        expected_memo_hash = "0"
        expected_fee_payer_hash = "0"
        expected_account_updates_commitment = "0"
        expected_full_commitment = "0"

    return f"""ZkAppTestVector {{
            name: "{test_name}",
            zkapp_command: ZKAppCommand {{
                fee_payer: FeePayer {{
                    body: FeePayerBody {{
                        public_key: {format_public_key(fee_payer["body"]["publicKey"])},
                        fee: {_int_token(fee_payer["body"]["fee"])},
                        valid_until: {valid_until},
                        nonce: {_int_token(fee_payer["body"]["nonce"])},
                    }},
                    authorization: "{escape_string(fee_payer["authorization"])}".to_string(),
                }},
                account_updates: vec![
{",".join(formatted_updates)},
                ],
                memo: {memo_expr},
            }},
            network: {network},
            expected_memo_hash: "{expected_memo_hash}",
            expected_fee_payer_hash: "{expected_fee_payer_hash}",
            expected_account_updates_commitment: "{expected_account_updates_commitment}",
            expected_full_commitment: "{expected_full_commitment}",
        }}"""


def main():
    """Main function to process JavaScript object input and generate Rust code."""
    if len(sys.argv) != 2:
        print("Usage: python json_to_zkapp_mapper.py <js_file>")
        sys.exit(1)

    js_file = sys.argv[1]

    try:
        with open(js_file, 'r') as f:
            content = f.read()

        js_data = parse_js_object(content)
        rust_code = generate_zkapp_command(js_data)
        print(rust_code)

    except FileNotFoundError:
        print(f"Error: File '{js_file}' not found.")
        sys.exit(1)
    except Exception as e:
        print(f"Error: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
