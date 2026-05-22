# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Experimental FROST (Flexible Round-Optimized Schnorr Threshold signatures) multi-signature tooling for the Mina Protocol. Not security audited — do not use with real funds.

Rust workspace with three crates:
- **`frost-bluepallas`** — FROST library for Mina's Pallas curve with Poseidon hashing. Now a generic ciphersuite crate — does NOT contain Mina transaction types or `PallasMessage`. It is parameterised over a `ChallengeMessage` trait.
- **`mina-tx`** — Mina transaction types, serialisation, `PallasMessage`, `TransactionSignature`, `Sig`, `PubKeySer`, and the `bluepallas_compat` glue. Standalone library; BluePallas bridging code is gated behind the `frost-bluepallas-compat` feature.
- **`mina-frost-client`** — CLI client for distributed key generation and signing sessions. Communicates with the `frostd` server.

## Build & Test Commands

```bash
cargo build --verbose              # Build all crates
cargo test --verbose               # Run all tests
cargo fmt --all                    # Format code
cargo clippy --all-targets --all-features -- -D clippy::all -W clippy::too_many_arguments  # Lint

# Run specific crate tests
cargo test --package frost-bluepallas
cargo test --package mina-frost-client
cargo test --package mina-tx

# Run examples
cargo run --example dkg
cargo run --example mina-sign-tx
cargo run --example mina-gen-pubkey
cargo run --example sign
```

Pre-commit hooks run fmt, clippy, and trailing whitespace checks. CI (`rust.yml`) runs build, test, and `cargo fmt --check` inside a Docker container.

## Architecture

### frost-bluepallas (Library)

The `BluePallas<M>` ciphersuite is now **generic over a `ChallengeMessage` type parameter `M`**. `M` must implement the `ChallengeMessage` trait, which provides the `challenge()` function. This allows the crate to be used without any dependency on Mina transaction types. Key modules:

- `lib.rs` — Ciphersuite definition (`BluePallas<M>`), `ChallengeMessage` trait, FROST round1/round2/aggregate re-exports, y-coordinate evenness enforcement for Mina compatibility
- `keys.rs` — Key generation (trusted dealer and DKG), `KeyPackage`, `PublicKeyPackage`, secret/signing shares
- `hasher.rs` — Low-level Poseidon hash utilities (`hash_to_scalar`, `hash_to_array`). `message_hash` and `PallasMessage` have been **moved to `mina-tx`**
- `negate.rs` — Y-coordinate negation logic required by Mina's signature scheme
- `signing_utilities.rs` — Signing helper utilities
- `errors.rs` — `BluePallasError` and `BluePallasResult` types

**Note**: `pallas_message.rs` and `mina_compat.rs` have been **removed** from `frost-bluepallas` and moved to the `mina-tx` crate.

### mina-tx (Library — new crate)

Contains all Mina-specific transaction logic. Modules:

- `pallas_message.rs` — `PallasMessage` struct (serialise/deserialise), `Hashable` impl, `message_hash`, `translate_pk`, `translate_sig`, `translate_minask`, and the `ChallengeMessage` impl for `PallasMessage` (gated behind `frost-bluepallas-compat` feature)
- `bluepallas_compat.rs` — Bridge code between `frost-bluepallas` types and `mina-tx` types: `TryFrom<FrSig<BluePallasSuite>>` for `Sig`, `TryFrom<VerifyingKey<BluePallasSuite>>` for `PubKeySer`, `TransactionEnvelope::to_pallas_message()`, `TransactionSignature::from_frost_signature()` / `from_frost_signature_bytes()`. Only compiled with the `frost-bluepallas-compat` feature.
- `signatures.rs` — `Sig`, `PubKeySer`, `TransactionSignature`, `TransactionSignature::new_with_zkapp_injection()`
- `errors.rs` — `MinaTxError` enum and `MinaTxResult`
- `transactions/` — `TransactionEnvelope`, `TransactionKind`, legacy/zkapp transaction types
- `graphql.rs`, `base58.rs` — GraphQL output and base58 utilities

**Feature flags for `mina-tx`:**
- `frost-bluepallas-compat` — enables `bluepallas_compat` module and `ChallengeMessage` impl for `PallasMessage`; adds `frost-bluepallas`, `frost-core`, `ark-ec` as dependencies
- `test-utils` — enables test helper utilities

### mina-frost-client (CLI)

Subcommand-based CLI using `clap`. Operation modes:
- **Trusted dealer** — Test-only centralized key generation
- **DKG** — Distributed key generation via `frostd` server
- **Coordinator/Participant** — Signing session roles
- **Contacts/Groups/Sessions** — Management commands
- **GraphQL** — Transaction submission support

Each mode has its own module under `src/` with `config.rs`, `comms/`, and operation logic. Async operations use `tokio`.

### Key Design Patterns

- FROST protocol flow: DKG (or trusted dealer) → Round 1 (nonce commitment) → Round 2 (signature share) → Aggregation
- Mina compatibility requires y-coordinate evenness checks and Poseidon-based hashing — see `frost-bluepallas/src/negate.rs` and `mina-tx/src/pallas_message.rs`
- The canonical `BluePallasSuite` type alias throughout the codebase is `BluePallas<PallasMessage>` (from `frost-bluepallas` + `mina-tx`)
  - In `mina-frost-client`: defined in `src/lib.rs` as `pub type BluePallasSuite = BluePallas<PallasMessage>`
  - In `frost-bluepallas` tests: defined in `tests/helpers/types.rs`
  - In `frost-bluepallas` examples: defined in `examples/_shared/types.rs`
- Serialization uses `postcard` for binary, `serde_json` for JSON, `bs58` for Mina addresses
- Snapshot testing with `insta` crate in `frost-bluepallas/tests/snapshots/`

## Coding Conventions

- Rust 2021 edition, toolchain 1.87.0
- Error handling: `eyre` for applications, `thiserror` for library error types
- Conventional Commits: `feat:`, `fix:`, `doc:`, etc.
- New `frost-bluepallas` features must maintain Mina signature format compatibility — update tests accordingly
- Async code in `mina-frost-client` uses `tokio` — keep consistent with existing `session.rs`, `coordinator/` patterns
- No panics in library code; explicit error handling with `Result`
- Prefer iterators over manual loops; leverage the type system for correctness

## ZKApp Transaction Serialization

The `mina-tx/src/transactions/zkapp_tx/` module handles ZKApp transaction serialization for Mina. Key patterns:

### Custom Serde Types
- **o1js JSON compatibility**: Mina's o1js library serializes numeric types as strings in JSON. Use newtype wrappers with custom `Serialize`/`Deserialize` implementations:
  - `StringU32(u32)` / `StringU64(u64)` — serialize as `"123"` not `123`
  - `Field`, `PublicKey`, `TokenId` — all have custom serde in `zkapp_serde.rs`
- When adding new types that appear in JSON, check o1js output format and match it

### Module Organization
- `zkapp_tx.rs` — Core struct definitions and type aliases
- `zkapp_serde.rs` — Custom `Serialize`/`Deserialize` implementations
- `packing.rs` — `Packable` and `Emptiable` traits for hashing (converts structs to field elements)
- `zkapp_test_vectors.rs` — Test data for commitment function tests (test-only)
- `commit.rs` — Commitment/hashing logic

### Adding New Serializable Types
1. Define the type in `zkapp_tx.rs`
2. Add serde impl in `zkapp_serde.rs` if custom serialization needed
3. Implement `Packable` in `packing.rs` if the type participates in transaction hashing
4. Implement `Emptiable` if the type appears in `Option<T>` fields that need packing
5. Update test vectors in `zkapp_test_vectors.rs` if affected

### Test Data
- Real transaction JSON files in `mina-tx/tests/data/`
- Use `include_str!` to load test fixtures
- Round-trip tests (`serialize` → `deserialize` → compare) catch serde issues

## Key Types and Their Locations

### Signature Output (`TransactionSignature`)
- Defined in `mina-tx/src/signatures.rs`
- Structure: `{ publicKey: PubKeySer, signature: Sig, payload: TransactionEnvelope }`
- `Sig` has fields `field: BigInt<4>` and `scalar: BigInt<4>` with custom serde (decimal strings + base58)
- The `payload` field contains the full `TransactionEnvelope` — for ZkApp transactions, this includes injected signatures
- Accessible via `mina_tx::TransactionSignature` (re-exported from `mina_tx::signatures`)
- FROST-specific constructors (`from_frost_signature`, `from_frost_signature_bytes`) are in `mina-tx/src/bluepallas_compat.rs` and require the `frost-bluepallas-compat` feature

### ZkApp Signature Injection
- `ZKAppCommand::inject_signature()` in `mina-tx/src/transactions/zkapp_tx/signature_injection.rs`
- Injects into `fee_payer.authorization` (a `String` field, base58-encoded)
- Injects into `account_updates[i].authorization.signature` (`Option<String>`) for updates where:
  - `authorization_kind.is_signed == true`
  - `public_key` matches the signer
  - `use_full_commitment == true`
- After injection, the full signed transaction is included in the `TransactionSignature.payload`

### Transaction Types
- `TransactionEnvelope` wraps `TransactionKind` (enum: `ZkApp(ZKAppCommand)` | `Legacy(LegacyTransaction)`) + `NetworkIdEnvelope`
- `TransactionEnvelope::inner()` returns `&TransactionKind`, `inner_mut()` returns `&mut TransactionKind`
- `TransactionEnvelope::from_str_network()` auto-detects transaction type (tries ZkApp first, then Legacy)
- `TransactionEnvelope::to_pallas_message()` and `From<&TransactionEnvelope> for PallasMessage` are in `mina-tx/src/bluepallas_compat.rs`

### ZkApp Structs (in `mina-tx/src/transactions/zkapp_tx.rs`)
- `ZKAppCommand { fee_payer: FeePayer, account_updates: Vec<AccountUpdate>, memo: [u8; MEMO_BYTES] }`
- `FeePayer { body: FeePayerBody, authorization: String }` — authorization is a plain String (base58 signature)
- `AccountUpdate { body: AccountUpdateBody, authorization: Authorization }`
- `Authorization { proof: Option<String>, signature: Option<String> }`
- `AuthorizationKind { is_signed: Bool, is_proved: Bool, verification_key_hash: Field }` — `Bool` is a type alias for `bool`

### Error Types
- `mina_tx::errors::MinaTxError` — error type for all `mina-tx` operations (serialization, deserialization, invalid signatures/public keys, memo errors, zkapp errors, unknown tx type)
- `frost_bluepallas::errors::BluePallasError` — error type for `frost-bluepallas` operations (serialization, no message provided, save signature errors)

## File Structure Gotchas

- `mina-tx/src/transactions/zkapp_tx.rs` is a **file** (not a directory with `mod.rs`). The submodules live in `mina-tx/src/transactions/zkapp_tx/` directory alongside it (Rust 2021 module system).
- `mina-tx/src/base58.rs` module is **public** (`pub mod base58`) — can be imported from outside
- `mina-tx/src/bluepallas_compat.rs` is only compiled when `frost-bluepallas-compat` feature is enabled
- `frost-bluepallas` no longer has a `pallas_message` module — it's in `mina-tx`
- `frost-bluepallas` no longer has a `mina_compat` module — it's in `mina-tx/src/bluepallas_compat.rs`
- `frost-bluepallas` no longer has a `transactions/` directory — it's in `mina-tx/src/transactions/`

## Integration Test (`mina-frost-client/tests/integration-tests.rs`)

- End-to-end test: DKG → signing → cross-package verification with `mina-signer`
- Uses release binary (`cargo build --release`) spawned as subprocesses
- Requires `frostd` server running + `mkcert` TLS certificates
- `parse_and_verify()` deserializes `TransactionSignature` from the signature output file, then:
  - For ZkApp: verifies signature injection into authorization fields, then verifies with `mina_signer::create_kimchi`
  - For Legacy: verifies with `mina_signer::create_legacy`
- Test is slow (spawns multiple processes, network communication) — run manually rather than in quick iteration loops
- `BigInt<4>` from `Sig` converts to mina-signer types via `.into()`
