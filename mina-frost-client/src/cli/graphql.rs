use super::args::Command;
use frost_bluepallas::mina_compat::TransactionSignature;
use std::fs;

const GRAPHQL_MAINNET_ENDPOINT: &str = "...";
const GRAPHQL_TESTNET_ENDPOINT: &str = "...";

// ------------------------------------------------------------
// Build & save GraphQL JSON
// ------------------------------------------------------------

pub fn graphql_build_command(args: &Command) -> Result<(), Box<dyn std::error::Error>> {
    let Command::GraphqlBuild {
        input_path,
        output_path,
    } = (*args).clone()
    else {
        panic!("invalid Command");
    };
    let file_content = fs::read_to_string(input_path)?;
    let tx_sig: TransactionSignature = serde_json::from_str(&file_content)?;

    let graphql_json = tx_sig.to_graphql_query_json();
    fs::write(output_path, graphql_json)?;

    Ok(())
}

// ------------------------------------------------------------
// Broadcast saved GraphQL JSON
// ------------------------------------------------------------

pub fn graphql_broadcast_command(args: &Command) -> Result<(), Box<dyn std::error::Error>> {
    let Command::GraphqlBroadcast {
        graphql_path,
        endpoint_url,
    } = (*args).clone()
    else {
        panic!("invalid Command");
    };
    let graphql_json = fs::read_to_string(graphql_path)?;

    let endpoint = match endpoint_url {
        Some(url) => url,
        None => {
            println!(
                "Endpoint URL not provided, attempting to infer from transaction signature..."
            );
            let transaction_sig: TransactionSignature = serde_json::from_str(&graphql_json)?;
            if transaction_sig.is_mainnet() {
                GRAPHQL_MAINNET_ENDPOINT.to_string()
            } else if transaction_sig.is_testnet() {
                GRAPHQL_TESTNET_ENDPOINT.to_string()
            } else {
                return Err("Unable to determine network from transaction signature. Please provide an endpoint URL.".into());
            }
        }
    };

    println!("Using GraphQL endpoint: {}", endpoint);

    let client = reqwest::blocking::Client::new();
    let response = client
        .post(&endpoint)
        .header("Content-Type", "application/json")
        .body(graphql_json)
        .send()?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        return Err(format!("GraphQL broadcast failed ({}): {}", status, body).into());
    } else {
        println!("GraphQL broadcast succeeded.");
        println!("Response: {}", response.text()?);
    }

    Ok(())
}
