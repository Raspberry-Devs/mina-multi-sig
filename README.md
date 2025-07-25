# Mina Multi-Sig

This repository provides an experimental implementation of multi-signature tooling for the [Mina Protocol](https://minaprotocol.com/).  The project is built around [FROST](https://github.com/cfrg/draft-irtf-cfrg-frost) (Flexible Round-Optimized Schnorr Threshold signatures) and contains both a reusable library and a command line client.

## ⚠️ Security Warning

**This repository has not undergone a security audit.  It may contain bugs and security vulnerabilities.  Use it at your own risk.  The authors and contributors take no responsibility for any loss or damage resulting from the use of this code.**

## Why Multi‑Sig on Mina?

Besides the usual advantages of shared control over accounts, threshold signatures help mitigate issues around how the Mina protocol handles hard forks.  During a fork zero knowledge verification keys may become obselete which causes all smart contracts to fallback to using signature-based updates. Multi‑sig coordination provides an additional layer of safety when updating verification keys after a hard fork.

## Repository Layout

- **`frost-bluepallas/`** – A Rust crate implementing FROST for Mina's Pallas curve using the Poseidon hash function.  It allows generation of signatures that are compatible with Mina nodes and includes example programs for key generation and transaction signing.
- **`frost-client/`** – A demo client and utilities for running distributed key generation and signing sessions.  It exposes various sub‑commands for initializing participants, running a trusted dealer or DKG, and coordinating signing rounds.

## Example Usage

Below is a minimal outline of how the client can be used.  See the `examples/` folders in each crate for complete scripts.

### Trusted Dealer Setup

```bash
# Initialize participant configurations
cargo run --bin frost-client -- init -c alice.toml
cargo run --bin frost-client -- init -c bob.toml
cargo run --bin frost-client -- init -c eve.toml

# Generate key shares with the trusted dealer helper
cargo run --bin frost-client -- trusted-dealer \
  -d "Alice, Bob and Eve's group" \
  --names Alice,Bob,Eve \
  -c alice.toml -c bob.toml -c eve.toml \
  -C bluepallas
```

### Distributed Key Generation (DKG)

```bash
# Initialize configurations and exchange contacts
cargo run --bin frost-client -- init -c alice.toml
cargo run --bin frost-client -- export --name 'Alice' -c alice.toml
cargo run --bin frost-client -- import -c alice.toml <contact_string>

# Start DKG process (coordinator)
cargo run --bin frost-client -- dkg \
  -d "Alice, Bob and Eve" \
  -s localhost:2744 \
  -S <BOB_PUBLIC_KEY>,<EVE_PUBLIC_KEY> \
  -t 2 -C bluepallas -c alice.toml

# Each participant joins the DKG
cargo run --bin frost-client -- dkg \
  -d "Alice, Bob and Eve" \
  -s localhost:2744 \
  -t 2 -C bluepallas -c bob.toml
```

### Signing Session

```bash
# Start a signing session (coordinator)
cargo run --bin frost-client -- coordinator \
  -c alice.toml \
  --server-url localhost:2744 \
  --group <GROUP_PUBLIC_KEY> \
  -S <BOB_PUBLIC_KEY>,<EVE_PUBLIC_KEY> \
  -m message.txt -o signature.hex

# Each participant joins the session
cargo run --bin frost-client -- participant \
  -c bob.toml \
  --server-url localhost:2744 \
  --group <GROUP_PUBLIC_KEY>
```

## Contributing

Contributions are welcome!  Feel free to open issues or pull requests.  Please note that all code is licensed under the [Apache-2.0](LICENSE) license.

## License

This project is distributed under the terms of the Apache License, Version 2.0.  See the [LICENSE](LICENSE) file for details.
