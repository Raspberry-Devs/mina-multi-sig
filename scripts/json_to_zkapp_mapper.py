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


def format_field(value: Union[str, int, None]) -> str:
    """Format a field value for Rust Field type."""
    if value is None:
        return "Field::default()"
    if isinstance(value, str) and value.isdigit():
        return f'Field(Fp::from_str("{value}").unwrap())'
    if isinstance(value, int):
        return f'Field(Fp::from_str("{value}").unwrap())'
    return f'Field(Fp::from_str("{value}").unwrap())'


def format_public_key(key_data: Union[str, Dict[str, Any]]) -> str:
    """Format a public key for Rust PublicKey type."""
    if isinstance(key_data, str):
        return f'PublicKey(CompressedPubKey::from_address("{key_data}").unwrap())'
    elif isinstance(key_data, dict) and "x" in key_data:
        x = key_data["x"]
        is_odd = key_data.get("isOdd", False)
        return f'PublicKey {{ x: {format_field(x)}, is_odd: {str(is_odd).lower()} }}'
    else:
        return "PublicKey::default()"


def format_optional_value(value: Any, type_formatter) -> str:
    """Format an OptionalValue<T> for Rust."""
    if value is None:
        return f"OptionalValue {{ is_some: false, value: {type_formatter(None)} }}"
    else:
        return f"OptionalValue {{ is_some: true, value: {type_formatter(value)} }}"


def format_range_condition(range_data: Dict[str, Any], type_name: str) -> str:
    """Format a RangeCondition for Rust."""
    if range_data is None:
        return f"OptionalValue {{ is_some: false, value: RangeCondition::default() }}"

    lower = range_data.get("lower", 0)
    upper = range_data.get("upper", 0)
    return f"OptionalValue {{ is_some: true, value: RangeCondition {{ lower: {lower}{type_name}, upper: {upper}{type_name} }} }}"


def format_auth_required(auth_str: str) -> str:
    """Format AuthRequired based on string value."""
    auth_mapping = {
        "Signature": "AuthRequired { constant: true, signature_necessary: true, signature_sufficient: true }",
        "Impossible": "AuthRequired { constant: true, signature_necessary: false, signature_sufficient: false }",
        "Either": "AuthRequired { constant: false, signature_necessary: false, signature_sufficient: false }",
        "Proof": "AuthRequired { constant: true, signature_necessary: false, signature_sufficient: false }",
        "None": "AuthRequired { constant: false, signature_necessary: false, signature_sufficient: false }",
    }
    return auth_mapping.get(auth_str, "AuthRequired::default()")


def format_app_state(app_state: List[Any]) -> str:
    """Format app_state array for Rust."""
    formatted_items = []
    for item in app_state:
        if item is None:
            formatted_items.append("OptionalValue { is_some: false, value: Field::default() }")
        else:
            formatted_items.append(f"OptionalValue {{ is_some: true, value: {format_field(item)} }}")

    joined_items = ',\n                                    '.join(formatted_items)
    return f"vec![\n                                    {joined_items},\n                                ]"


def format_events(events: List[List[str]]) -> str:
    """Format events array for Rust Events struct."""
    if not events:
        data_formatted = "vec![]"
    else:
        formatted_events = []
        for event in events:
            formatted_fields = [format_field(field) for field in event]
            formatted_events.append(f"vec![{', '.join(formatted_fields)}]")

        joined_events = ',\n                                    '.join(formatted_events)
        data_formatted = f"vec![\n                                    {joined_events},\n                                ]"

    return f"""Events {{
                                data: {data_formatted},
                                hash: Field::default(),
                            }}"""


def format_actions(actions: List[List[str]]) -> str:
    """Format actions array for Rust Actions struct."""
    if not actions:
        data_formatted = "vec![]"
    else:
        formatted_actions = []
        for action in actions:
            formatted_fields = [format_field(field) for field in action]
            formatted_actions.append(f"vec![{', '.join(formatted_fields)}]")

        joined_actions = ',\n                                    '.join(formatted_actions)
        data_formatted = f"vec![\n                                    {joined_actions},\n                                ]"

    return f"""Actions {{
                                data: {data_formatted},
                                hash: Field::default(),
                            }}"""


def format_account_state(state: List[Any]) -> str:
    """Format account state array for Rust."""
    formatted_items = []
    for item in state:
        if item is None:
            formatted_items.append("OptionalValue { is_some: false, value: Field::default() }")
        else:
            formatted_items.append(f"OptionalValue {{ is_some: true, value: {format_field(item)} }}")

    joined_items = ',\n                                            '.join(formatted_items)
    return f"vec![\n                                            {joined_items},\n                                        ]"


def format_verification_key(vk_data: Optional[Dict[str, Any]]) -> str:
    """Format verification key data for Rust."""
    if not vk_data:
        return "OptionalValue { is_some: false, value: VerificationKeyData::default() }"

    data = vk_data.get("data", "")
    hash_val = vk_data.get("hash", "0")

    return f"""OptionalValue {{
                                    is_some: true,
                                    value: VerificationKeyData {{
                                        data: "{escape_string(data)}".to_string(),
                                        hash: {format_field(hash_val)},
                                    }}
                                }}"""


def format_permissions(permissions: Optional[Dict[str, Any]]) -> str:
    """Format permissions object for Rust."""
    if not permissions:
        return "OptionalValue { is_some: false, value: Permissions::default() }"

    set_vk = permissions.get("setVerificationKey", {})
    auth = set_vk.get("auth", "None") if isinstance(set_vk, dict) else "None"
    txn_version = set_vk.get("txnVersion", "0") if isinstance(set_vk, dict) else "0"

    return f"""OptionalValue {{
                                    is_some: true,
                                    value: Permissions {{
                                        edit_state: {format_auth_required(permissions.get("editState", "None"))},
                                        access: {format_auth_required(permissions.get("access", "None"))},
                                        send: {format_auth_required(permissions.get("send", "None"))},
                                        receive: {format_auth_required(permissions.get("receive", "None"))},
                                        set_delegate: {format_auth_required(permissions.get("setDelegate", "None"))},
                                        set_permissions: {format_auth_required(permissions.get("setPermissions", "None"))},
                                        set_verification_key: SetVerificationKey {{
                                            auth: {format_auth_required(auth)},
                                            txn_version: {txn_version},
                                        }},
                                        set_zkapp_uri: {format_auth_required(permissions.get("setZkappUri", "None"))},
                                        edit_action_state: {format_auth_required(permissions.get("editActionState", "None"))},
                                        set_token_symbol: {format_auth_required(permissions.get("setTokenSymbol", "None"))},
                                        increment_nonce: {format_auth_required(permissions.get("incrementNonce", "None"))},
                                        set_voting_for: {format_auth_required(permissions.get("setVotingFor", "None"))},
                                        set_timing: {format_auth_required(permissions.get("setTiming", "None"))},
                                    }}
                                }}"""


def format_timing(timing_data: Optional[Dict[str, Any]]) -> str:
    """Format timing data for Rust."""
    if not timing_data:
        return "OptionalValue { is_some: false, value: TimingData::default() }"

    return f"""OptionalValue {{
                                    is_some: true,
                                    value: TimingData {{
                                        initial_minimum_balance: {timing_data.get("initialMinimumBalance", 0)},
                                        cliff_time: {timing_data.get("cliffTime", 0)},
                                        cliff_amount: {timing_data.get("cliffAmount", 0)},
                                        vesting_period: {timing_data.get("vestingPeriod", 0)},
                                        vesting_increment: {timing_data.get("vestingIncrement", 0)},
                                    }}
                                }}"""


def format_optional_field(value: Any) -> str:
    """Format an optional field value."""
    if value is None:
        return "OptionalValue { is_some: false, value: Field::default() }"
    return f"OptionalValue {{ is_some: true, value: {format_field(value)} }}"


def format_optional_public_key(value: Any) -> str:
    """Format an optional public key value."""
    if value is None:
        return "OptionalValue { is_some: false, value: PublicKey::default() }"
    return f"OptionalValue {{ is_some: true, value: {format_public_key(value)} }}"


def format_optional_bool(value: Any) -> str:
    """Format an optional boolean value."""
    if value is None:
        return "OptionalValue { is_some: false, value: false }"
    return f"OptionalValue {{ is_some: true, value: {str(value).lower()} }}"


def format_epoch_data(epoch_data: Dict[str, Any]) -> str:
    """Format epoch data for Rust."""
    ledger = epoch_data.get("ledger", {})
    ledger_hash = ledger.get("hash")
    ledger_currency = ledger.get("totalCurrency")

    seed = epoch_data.get("seed")
    start_checkpoint = epoch_data.get("startCheckpoint")
    lock_checkpoint = epoch_data.get("lockCheckpoint")
    epoch_length = epoch_data.get("epochLength")

    return f"""EpochData {{
                                        ledger: EpochLedger {{
                                            hash: {format_optional_field(ledger_hash)},
                                            total_currency: {format_range_condition(ledger_currency, "u64")},
                                        }},
                                        seed: {format_optional_field(seed)},
                                        start_checkpoint: {format_optional_field(start_checkpoint)},
                                        lock_checkpoint: {format_optional_field(lock_checkpoint)},
                                        epoch_length: {format_range_condition(epoch_length, "u32")},
                                    }}"""


def format_account_update(update: Dict[str, Any], index: int) -> str:
    """Format a single account update for Rust."""
    body = update["body"]

    # Format balance change
    balance_change = body["balanceChange"]
    magnitude = balance_change["magnitude"]
    sgn = balance_change["sgn"]
    # Convert "Positive"/"Negative" to +1/-1
    sgn_value = "1" if sgn == "Positive" else "-1"

    # Format may_use_token
    may_use_token = body["mayUseToken"]

    # Format authorization kind
    auth_kind = body["authorizationKind"]

    # Format preconditions
    preconditions = body["preconditions"]
    network = preconditions["network"]
    account = preconditions["account"]

    # Format update fields
    update_data = body["update"]

    # Handle optional string fields with correct defaults
    zkapp_uri = update_data.get("zkappUri")
    zkapp_uri_formatted = f'OptionalValue {{ is_some: true, value: ZkappUriData::default() }}' if zkapp_uri is not None else "OptionalValue { is_some: false, value: ZkappUriData::default() }"

    token_symbol = update_data.get("tokenSymbol")
    token_symbol_formatted = f'OptionalValue {{ is_some: true, value: TokenSymbolData::default() }}' if token_symbol is not None else "OptionalValue { is_some: false, value: TokenSymbolData::default() }"

    return f"""// Account update {index + 1}
                    AccountUpdate {{
                        body: AccountUpdateBody {{
                            public_key: {format_public_key(body["publicKey"])},
                            token_id: {format_field(body["tokenId"])},
                            update: Update {{
                                app_state: {format_app_state(update_data["appState"])},
                                delegate: {format_optional_public_key(update_data.get("delegate"))},
                                verification_key: {format_verification_key(update_data.get("verificationKey"))},
                                permissions: {format_permissions(update_data.get("permissions"))},
                                zkapp_uri: {zkapp_uri_formatted},
                                token_symbol: {token_symbol_formatted},
                                timing: {format_timing(update_data.get("timing"))},
                                voting_for: {format_optional_field(update_data.get("votingFor"))},
                            }},
                            balance_change: BalanceChange {{
                                magnitude: {magnitude},
                                sgn: {sgn_value},
                            }},
                            increment_nonce: {str(body["incrementNonce"]).lower()},
                            events: {format_events(body["events"])},
                            actions: {format_actions(body["actions"])},
                            call_data: {format_field(body["callData"])},
                            call_depth: {body["callDepth"]},
                            preconditions: Preconditions {{
                                network: NetworkPreconditions {{
                                    snarked_ledger_hash: {format_optional_field(network.get("snarkedLedgerHash"))},
                                    blockchain_length: {format_range_condition(network.get("blockchainLength"), "u32")},
                                    min_window_density: {format_range_condition(network.get("minWindowDensity"), "u32")},
                                    total_currency: {format_range_condition(network.get("totalCurrency"), "u64")},
                                    global_slot_since_genesis: {format_range_condition(network.get("globalSlotSinceGenesis"), "u32")},
                                    staking_epoch_data: {format_epoch_data(network["stakingEpochData"])},
                                    next_epoch_data: {format_epoch_data(network["nextEpochData"])},
                                }},
                                account: AccountPreconditions {{
                                    balance: {format_range_condition(account.get("balance"), "u64")},
                                    nonce: {format_range_condition(account.get("nonce"), "u32")},
                                    receipt_chain_hash: {format_optional_field(account.get("receiptChainHash"))},
                                    delegate: {format_optional_public_key(account.get("delegate"))},
                                    state: {format_account_state(account["state"])},
                                    action_state: {format_optional_field(account.get("actionState"))},
                                    proved_state: {format_optional_bool(account.get("provedState"))},
                                    is_new: {format_optional_bool(account.get("isNew"))},
                                }},
                                valid_while: {format_range_condition(preconditions.get("validWhile"), "u32")},
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
    """Generate the complete ZkAppCommand Rust code from JavaScript object data."""
    # The data is now directly the zkApp command structure
    fee_payer = js_data["feePayer"]
    account_updates = js_data["accountUpdates"]
    memo = js_data["memo"]

    # Format account updates
    formatted_updates = []
    for i, update in enumerate(account_updates):
        formatted_updates.append(format_account_update(update, i))

    return f"""ZkAppTestVector {{
            name: "{test_name}",
            zkapp_command: ZKAppCommand {{
                fee_payer: FeePayer {{
                    body: FeePayerBody {{
                        public_key: {format_public_key(fee_payer["body"]["publicKey"])},
                        fee: {fee_payer["body"]["fee"]},
                        valid_until: Some({fee_payer["body"]["validUntil"]}),
                        nonce: {fee_payer["body"]["nonce"]},
                    }},
                    authorization: "{escape_string(fee_payer["authorization"])}".to_string(),
                }},
                account_updates: vec![
{",".join(formatted_updates)},
                ],
                memo: "{escape_string(memo)}".to_string(),
            }},
            network: NetworkId::MAINNET,
            expected_memo_hash: "0",
            expected_fee_payer_hash: "0",
            expected_account_updates_commitment: "0",
            expected_full_commitment: "0",
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
