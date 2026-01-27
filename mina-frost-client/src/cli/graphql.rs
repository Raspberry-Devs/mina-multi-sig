use super::args::Command;
use frost_bluepallas::mina_compat::TransactionSignature;
use std::fs;

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

    let graphql_json = tx_sig
        .to_graphql_query_json()
        .expect("Failed to build GraphQL JSON");
    fs::write(output_path, graphql_json)?;

    Ok(())
}

// ------------------------------------------------------------
// Broadcast saved GraphQL JSON
// ------------------------------------------------------------

pub async fn graphql_broadcast_command(args: &Command) -> Result<(), Box<dyn std::error::Error>> {
    let Command::GraphqlBroadcast {
        graphql_path,
        endpoint_url: endpoint,
    } = (*args).clone()
    else {
        panic!("invalid Command");
    };
    let graphql_json = fs::read_to_string(graphql_path)?;

    println!("Using GraphQL endpoint: {}", endpoint);

    let client = reqwest::Client::new();
    let response = client
        .post(&endpoint)
        .header("Content-Type", "application/json")
        .body(graphql_json)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await?;
        return Err(format!("GraphQL broadcast failed ({}): {}", status, body).into());
    } else {
        println!("GraphQL broadcast succeeded.");
        println!("Response: {}", response.text().await?);
    }

    Ok(())
}
