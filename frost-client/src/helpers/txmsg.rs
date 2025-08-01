use frost_bluepallas::transactions::Transaction;
use std::fs;
use std::io::BufRead;
use std::path::Path;

pub fn load_transaction_from_json<P: AsRef<Path>>(
    path: P,
) -> Result<Transaction, Box<dyn std::error::Error>> {
    let json_content = fs::read_to_string(path)?;
    let transaction: Transaction = serde_json::from_str(json_content.trim())?;
    Ok(transaction)
}

pub fn load_transaction_from_stdin(
    input: &mut dyn BufRead,
) -> Result<Transaction, Box<dyn std::error::Error>> {
    let mut json_content = String::new();
    input.read_to_string(&mut json_content)?;
    let transaction: Transaction = serde_json::from_str(json_content.trim())?;
    Ok(transaction)
}
