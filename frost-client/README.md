# frost-client
Forked from https://github.com/ZcashFoundation/frost-zcash-demo/tree/main/frost-client

# Zcash Foundation original documentation
https://frost.zfnd.org/zcash/ywallet-demo.html

# Examples

## Trusted Dealer Example
Located in `examples/trusted_dealer_example/`, this example demonstrates how to:
1. Initialize configs for multiple users
2. Generate FROST key shares using the trusted dealer approach with bluepallas

To run the example:
```bash
cd examples/trusted_dealer_example
./trusted_dealer_example.sh
```

The script will create a `generated` directory containing the config files for each user. Note that this directory is gitignored to prevent committing sensitive key material.