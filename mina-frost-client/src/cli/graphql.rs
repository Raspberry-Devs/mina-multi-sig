use frost_bluepallas::mina_compat::TransactionSignature;

pub fn run_graphql_command(
    input_path: &str,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;

    // Deserialize the transaction envelope from the file
    let file_content = fs::read_to_string(input_path)?;
    let tx_sig: TransactionSignature = serde_json::from_str(&file_content)?;
    let graphql_json = tx_sig.to_graphql_query_json();

    // Write the GraphQL JSON to the output file
    fs::write(output_path, graphql_json)?;
    Ok(())
}
