# mina-frost-client

Forked from https://github.com/ZcashFoundation/frost-zcash-demo/tree/main/mina-frost-client

## Zcash Foundation original documentation

https://frost.zfnd.org/zcash/ywallet-demo.html

## ⚠️ Security Warning

**This code has not been audited and should be used with extreme caution. Do not use in production environments or with real funds. This is experimental software intended for research and development purposes only.**

## Setting up the `frostd` server
See `zcash` guide [here](https://frost.zfnd.org/zcash/server.html)

## Examples
You can find example workflows in a form of scripts [here](./examples/README.md)


## Commands & Flags

### Network selection

The `coordinator` subcommand accepts a `--network` flag to choose between
`testnet` (default) and `mainnet`. The chosen network is sent to participants
for confirmation before signing.

### -S flag
The `signers` (`-S`) flag used in `dkg`, `participant` and `coordinator` commands, accepts the list of signers separated by `,`, with **NO SPACES**.

### trusted-delaer
The `trusted-dealer` command is **test-only**. To generate keys for actuall usage please use `dkg`.

## Tests

To run all tests for the trusted dealer module:

```bash
cargo test --package mina-frost-client --lib trusted_dealer -- --nocapture
```

### Run all tests in the codebase

```
cargo test --package mina-frost-client --lib -- --nocapture
```
