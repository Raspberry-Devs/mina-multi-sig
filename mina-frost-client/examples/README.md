# FROST Client Examples

Three examples demonstrating FROST threshold signatures for the Mina protocol.

## Examples

### 1. Trusted Dealer (`trusted_dealer_example/`)

Single party generates and distributes key shares to 3 participants (threshold t=2).

```bash
cd trusted_dealer_example/
./trusted_dealer_example.sh
```

### 2. Distributed Key Generation (`dkg_example/`)

Participants collaboratively generate keys without any single party knowing the complete private key.

```bash
cd dkg_example/
./dkg_example.sh
```

### 3. Signing (`signing_example/`)

Create threshold signatures using keys from previous examples.

```bash
# Then run signing example
cd ../signing_example/
./signing_example.sh
```

### 4. Graphql Broadcasting (`graphql_example/`)

Convert a signed transaction JSON into a GraphQL string

```bash
cd graphql_example/
./graphql_example.sh
```


## Development

Remember to add executable permisions for each example before running:

```bash
chmod +x trusted_dealer_example/trusted_dealer_example.sh
chmod +x dkg_example/dkg_example.sh
chmod +x signing_example/signing_example.sh
```

Examples use `cargo run` to run the client (always uses your latest code changes).
