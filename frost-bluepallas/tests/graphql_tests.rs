//! Golden-file tests for GraphQL mutation output.
//!
//! Each test loads a signed transaction JSON from `tests/data/`,
//! converts it to a GraphQL mutation via `to_graphql_query_json()`,
//! and asserts the result matches the corresponding golden GraphQL JSON file.
//!
//! To regenerate the golden files after an intentional format change:
//!   cargo run --example graphql-broadcast -- --output-dir frost-bluepallas/tests/data/graphql

use frost_bluepallas::mina_compat::TransactionSignature;

const TEST_DATA_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/graphql");

/// Load a golden file pair and assert the GraphQL conversion matches.
fn assert_graphql_golden(signed_path: &str, graphql_path: &str) {
    let signed_json = std::fs::read_to_string(signed_path)
        .unwrap_or_else(|e| panic!("Failed to read {signed_path}: {e}"));
    let expected_graphql = std::fs::read_to_string(graphql_path)
        .unwrap_or_else(|e| panic!("Failed to read {graphql_path}: {e}"));

    let tx_sig: TransactionSignature =
        serde_json::from_str(&signed_json).expect("Failed to deserialize TransactionSignature");

    let actual_graphql = tx_sig
        .to_graphql_query_json()
        .expect("to_graphql_query_json failed");

    // Compare as parsed JSON values so formatting differences don't cause failures.
    let expected: serde_json::Value =
        serde_json::from_str(&expected_graphql).expect("Golden GraphQL file is not valid JSON");
    let actual: serde_json::Value =
        serde_json::from_str(&actual_graphql).expect("Produced GraphQL is not valid JSON");

    assert_eq!(
        actual, expected,
        "GraphQL output mismatch for {signed_path}.\n\
         --- expected (from {graphql_path}) ---\n{expected_graphql}\n\
         --- actual ---\n{actual_graphql}"
    );
}

#[test]
fn graphql_golden_payment() {
    assert_graphql_golden(
        &format!("{TEST_DATA_DIR}/payment_signed.json"),
        &format!("{TEST_DATA_DIR}/payment_graphql.json"),
    );
}

#[test]
fn graphql_golden_delegation() {
    assert_graphql_golden(
        &format!("{TEST_DATA_DIR}/delegation_signed.json"),
        &format!("{TEST_DATA_DIR}/delegation_graphql.json"),
    );
}

#[test]
fn graphql_golden_zkapp() {
    assert_graphql_golden(
        &format!("{TEST_DATA_DIR}/zkapp_signed.json"),
        &format!("{TEST_DATA_DIR}/zkapp_graphql.json"),
    );
}
