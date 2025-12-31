# FROST-BluePallas

A FROST (Flexible Round-Optimized Schnorr Threshold Signatures) implementation for the Pallas curve using Poseidon hash, specifically designed for compatibility with the Mina Protocol.

## ⚠️ Security Warning

**This code has not been audited and should be used with extreme caution. Do not use in production environments or with real funds. This is experimental software intended for research and development purposes only.**

## Overview

FROST-BluePallas implements the FROST threshold signature scheme using:

- **Curve**: Pallas (from the Pasta curves family)
- **Hash Function**: Poseidon
- **Protocol Compatibility**: Mina Protocol signatures

This implementation allows multiple parties to collaboratively generate signatures that are compatible with the Mina blockchain's native signature verification, enabling multi-signature wallets and other threshold cryptography applications in the Mina ecosystem.

## Features

- **Threshold Signatures**: Generate valid Mina signatures using a threshold of participants (t-of-n)
- **Mina Compatibility**: Signatures are fully compatible with Mina Protocol's signature verification
- **Trusted Dealer Setup**: Generate key shares from a single master key
- **Distributed Key Generation (DKG)**: Generate keys collaboratively without a trusted dealer
- **Transaction Signing**: Direct support for signing Mina transactions
- **Network Support**: Compatible with both Mainnet and Testnet

## Mina Protocol Compatibility

This implementation follows the Mina Protocol signature specification and produces signatures that:

- Use the same Pallas curve and Poseidon hash as native Mina signatures
- Are verifiable by standard Mina signature verification
- Support both Mainnet and Testnet network domains
- Handle the Mina-specific even y-coordinate requirement for signatures

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
frost-bluepallas = { git = "https://github.com/Raspberry-Devs/mina-multi-sig" }
```

## Quick Start

### Basic Usage with Trusted Dealer

```rust
use frost_bluepallas::{self as frost, hasher::set_network_testnet};

// Set network (TESTNET)
set_network_testnet()?;

// Generate key shares using trusted dealer
let max_signers = 5;
let min_signers = 3;
let mut rng = rand_core::OsRng;

let (shares, pubkey_package) = frost::keys::generate_with_dealer(
    max_signers,
    min_signers,
    frost::keys::IdentifierList::Default,
    &mut rng,
)?;

// Sign a message
let message = b"Hello Mina!";
let (signature, verifying_key) = frost::signing_utilities::sign_from_packages(
    message,
    shares,
    pubkey_package,
    rng,
)?;
```

### Signing Mina Transactions

```rust
use frost_bluepallas::{transactions::LegacyTransaction, translate::translate_msg};
use mina_signer::{PubKey, NetworkId::TESTNET};

// Create a Mina transaction
// Generate tx
let tx = LegacyTransaction::new_payment(
    mina_keypair.public.clone(),
    recipient_pubkey,
    1000000000,
    1000000000,
    1,
)
.set_memo_str("Hello Mina x FROST from the Rasp")
.unwrap();
let tx = TransactionEnvelope::new_legacy(TESTNET, tx);


// Convert transaction to signable message
let message = tx.translate_msg();

// Sign with FROST (using existing shares and pubkey_package)
let (frost_sig, frost_vk) = frost::signing_utilities::sign_from_packages(
    &message,
    shares,
    pubkey_package,
    rng,
)?;

// Convert to Mina format for verification
let mina_sig = frost_bluepallas::translate::translate_sig(&frost_sig)?;
let mina_vk = frost_bluepallas::translate::translate_pk(&frost_vk)?;

// Verify with Mina signer
let mut ctx = mina_signer::create_legacy(NetworkId::TESTNET);
assert!(ctx.verify(&mina_sig, &mina_vk, &tx));
```

## Examples

The `examples/` directory contains several usage examples:

- `dkg.rs` - Distributed Key Generation example
- `mina-sign-tx.rs` - Sign a Mina transaction with FROST
- `mina-gen-pubkey.rs` - Generate Mina-compatible key pairs

Run examples with:

```bash
cargo run --example dkg
cargo run --example mina-sign-tx
```

## API Documentation

### Key Generation

- `frost::keys::generate_with_dealer()` - Generate shares using a trusted dealer
- `frost::keys::dkg::part1/2/3()` - Distributed key generation protocol
- `frost::keys::split()` - Split an existing signing key into shares

### Signing

- `frost::round1::commit()` - Generate signing nonces (Round 1)
- `frost::round2::sign()` - Generate signature shares (Round 2)
- `frost::aggregate()` - Combine signature shares into final signature

### Utilities

- `frost::signing_utilities::sign_from_packages()` - Complete signing process helper
- `frost::translate::translate_sig()` - Convert FROST to Mina signature format
- `frost::translate::translate_pk()` - Convert FROST to Mina public key format

## Network Configuration

Set the network ID before signing:

```rust
use frost_bluepallas::hasher::{set_network_mainnet, set_network_testnet};

// For Testnet
set_network_testnet()?;

// For Mainnet
set_network_mainnet()?;
```

The network ID affects the domain separation in signatures, so signatures generated for one network will not verify on another.

## Testing

Run the test suite:

```bash
cargo test
```

The tests include:

- FROST protocol correctness tests
- Mina signature compatibility tests
- Cross-verification with Mina signer
- Network domain separation tests

## Contributing

This is research software. Contributions are welcome, but please ensure:

- All tests pass
- New features include comprehensive tests
- Mina compatibility is maintained
- Security implications are carefully considered

## References

- [FROST Paper](https://eprint.iacr.org/2020/852.pdf)
- [Mina Protocol Signature Specification](https://github.com/MinaProtocol/mina/blob/develop/docs/specs/signatures/description.md)
- [FROST-core Implementation](https://github.com/ZcashFoundation/frost)
