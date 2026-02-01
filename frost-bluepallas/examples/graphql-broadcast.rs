/// Example: sign transactions with FROST and convert to GraphQL mutations.
///
/// Demonstrates the full pipeline:
///   1. Build legacy payment, delegation, and zkApp transactions.
///   2. Sign each with a FROST threshold signature (trusted-dealer, single key split).
///   3. Convert the signed transactions to Mina GraphQL mutation JSON.
///   4. Write the JSON files to an output directory.
///   5. Optionally broadcast to a Mina GraphQL endpoint.
///
/// Usage:
///   cargo run --example graphql-broadcast -- [--endpoint <URL>] [--output-dir <DIR>]
///
/// Without --endpoint the script only writes JSON files (useful for generating
/// golden-file test vectors). With --endpoint it also POSTs each mutation.
use std::{fs, path::PathBuf};

use ark_ff::PrimeField;
use frost_bluepallas::{
    mina_compat::{PubKeySer, Sig, TransactionSignature},
    signing_utilities::generate_signature_from_sk,
    transactions::{
        legacy_tx::LegacyTransaction,
        zkapp_tx::{
            AccountUpdate, AccountUpdateBody, Actions, Authorization, AuthorizationKind,
            BalanceChange, Events, FeePayer, FeePayerBody, MayUseToken, Preconditions, PublicKey,
            TokenId, Update, ZKAppCommand,
        },
        TransactionEnvelope, MEMO_BYTES,
    },
};
use mina_signer::{CompressedPubKey, Keypair, NetworkId, PubKey};

// ---------------------------------------------------------------------------
// CLI argument parsing (minimal, no extra deps)
// ---------------------------------------------------------------------------

struct Args {
    endpoint: Option<String>,
    output_dir: PathBuf,
}

fn parse_args() -> Args {
    let mut args = std::env::args().skip(1);
    let mut endpoint = None;
    let mut output_dir = PathBuf::from("graphql-output");

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--endpoint" => endpoint = args.next(),
            "--output-dir" => {
                if let Some(dir) = args.next() {
                    output_dir = PathBuf::from(dir);
                }
            }
            other => {
                eprintln!("Unknown argument: {other}");
                eprintln!("Usage: graphql-broadcast [--endpoint <URL>] [--output-dir <DIR>]");
                std::process::exit(1);
            }
        }
    }
    Args {
        endpoint,
        output_dir,
    }
}

// ---------------------------------------------------------------------------
// Transaction builders
// ---------------------------------------------------------------------------

fn build_legacy_payment(from: &PubKey, to: &PubKey) -> TransactionEnvelope {
    let tx = LegacyTransaction::new_payment(from.clone(), to.clone(), 1_000_000_000, 10_000_000, 0)
        .set_memo_str("FROST payment test")
        .expect("memo fits")
        .set_valid_until(4_294_967_295);
    TransactionEnvelope::new_legacy(NetworkId::TESTNET, tx)
}

fn build_legacy_delegation(from: &PubKey, to: &PubKey) -> TransactionEnvelope {
    let tx = LegacyTransaction::new_delegation(from.clone(), to.clone(), 10_000_000, 1)
        .set_memo_str("FROST delegation test")
        .expect("memo fits")
        .set_valid_until(4_294_967_295);
    TransactionEnvelope::new_legacy(NetworkId::TESTNET, tx)
}

fn build_zkapp_tx(from: &CompressedPubKey) -> TransactionEnvelope {
    let cloned_from = (*from).clone();
    let pk = PublicKey(cloned_from);

    let fee_payer = FeePayer {
        body: FeePayerBody {
            public_key: pk.clone(),
            fee: 100_000_000,
            valid_until: Some(4_294_967_295),
            nonce: 0,
        },
        authorization: String::new(), // will be injected
    };

    let account_update = AccountUpdate {
        body: AccountUpdateBody {
            public_key: pk,
            token_id: TokenId::default(),
            update: Update::default(),
            balance_change: BalanceChange {
                magnitude: 0,
                sgn: 1,
            },
            increment_nonce: false,
            events: Events::default(),
            actions: Actions::default(),
            call_data: Default::default(),
            call_depth: 0,
            preconditions: Preconditions::default(),
            use_full_commitment: true,
            implicit_account_creation_fee: false,
            may_use_token: MayUseToken::default(),
            authorization_kind: AuthorizationKind::default(),
        },
        authorization: Authorization {
            proof: None,
            signature: None, // will be injected
        },
    };

    let mut memo = [0u8; MEMO_BYTES];
    memo[0] = 0x01;
    let text = b"FROST zkApp test";
    memo[1] = text.len() as u8;
    memo[2..2 + text.len()].copy_from_slice(text);

    let cmd = ZKAppCommand {
        fee_payer,
        account_updates: vec![account_update],
        memo,
    };

    TransactionEnvelope::new_zkapp(NetworkId::TESTNET, cmd)
}

// ---------------------------------------------------------------------------
// Signing helper
// ---------------------------------------------------------------------------

fn sign_envelope(envelope: TransactionEnvelope, keypair: &Keypair) -> TransactionSignature {
    let signing_key =
        frost_bluepallas::mina_compat::translate_minask(keypair).expect("valid keypair");

    let msg = envelope.serialize().expect("serialize envelope");

    let (frost_sig, _vk) = generate_signature_from_sk(&msg, &signing_key, rand_core::OsRng)
        .expect("FROST signing succeeds");

    let mina_sig = frost_bluepallas::mina_compat::translate_sig(&frost_sig).expect("translate sig");

    let sig = Sig {
        field: mina_sig.rx.into_bigint(),
        scalar: mina_sig.s.into_bigint(),
    };

    let pub_key_ser = PubKeySer {
        pubKey: keypair.public.clone(),
    };

    let (tx_sig, injection_result) =
        TransactionSignature::new_with_zkapp_injection(pub_key_ser, sig, envelope);

    if let Some(result) = &injection_result {
        println!(
            "  Signature injection: fee_payer={}, account_updates={}, warnings={}",
            result.fee_payer_injected,
            result.account_updates_injected,
            result.warnings.len()
        );
        for w in &result.warnings {
            println!("    warning: {w:?}");
        }
    }

    tx_sig
}

// ---------------------------------------------------------------------------
// Broadcast helper
// ---------------------------------------------------------------------------

fn broadcast(endpoint: &str, json: &str, label: &str) {
    println!("  Broadcasting {label} to {endpoint} ...");

    // Use a blocking reqwest client since we're already in main().
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(endpoint)
        .header("Content-Type", "application/json")
        .body(json.to_owned())
        .send();

    match response {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().unwrap_or_default();
            if status.is_success() {
                println!("  {label}: OK ({status})");
                println!("  Response: {body}");
            } else {
                eprintln!("  {label}: FAILED ({status})");
                eprintln!("  Response: {body}");
            }
        }
        Err(e) => eprintln!("  {label}: request error: {e}"),
    }
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() {
    let args = parse_args();

    // Deterministic keypair for reproducible golden files.
    let private_key_hex = "35dcca7620128d240cc3319c83dc6402ad439038361ba853af538a4cea3ddabc";
    let keypair = Keypair::from_hex(private_key_hex).expect("valid hex key");
    let from = &keypair.public;
    let to_pk = PubKey::from_address("B62qkcvM4DZE7k23ZHMLt1uaMVcixuxxuyz1XNJNCLkFbitDdUHxWs1")
        .expect("valid address");

    // Build transactions
    let txs: Vec<(&str, TransactionEnvelope)> = vec![
        ("payment", build_legacy_payment(from, &to_pk)),
        ("delegation", build_legacy_delegation(from, &to_pk)),
        ("zkapp", build_zkapp_tx(&from.into_compressed())),
    ];

    fs::create_dir_all(&args.output_dir).expect("create output dir");

    for (label, envelope) in &txs {
        println!("[{label}] Signing ...");
        let tx_sig = sign_envelope(envelope.clone(), &keypair);

        // -- Write the signed TransactionSignature JSON (input for graphql-build) --
        let sig_json = serde_json::to_string_pretty(&tx_sig).expect("serialize tx_sig");
        let sig_path = args.output_dir.join(format!("{label}_signed.json"));
        fs::write(&sig_path, &sig_json).expect("write signed json");
        println!("  Wrote signed tx -> {}", sig_path.display());

        // -- Convert to GraphQL mutation JSON --
        let graphql_json = tx_sig.to_graphql_query_json().expect("graphql conversion");
        let gql_path = args.output_dir.join(format!("{label}_graphql.json"));
        fs::write(&gql_path, &graphql_json).expect("write graphql json");
        println!("  Wrote GraphQL   -> {}", gql_path.display());

        // -- Optionally broadcast --
        if let Some(ref endpoint) = args.endpoint {
            broadcast(endpoint, &graphql_json, label);
        }
    }

    if args.endpoint.is_none() {
        println!("\nNo --endpoint provided. Re-run with --endpoint <URL> to broadcast.");
    }

    println!("Done.");
}
