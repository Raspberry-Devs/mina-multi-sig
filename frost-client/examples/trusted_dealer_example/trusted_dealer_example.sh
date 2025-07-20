#!/bin/bash

# Get the directory where the script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
GENERATED_DIR="$SCRIPT_DIR/generated"

# Clean up generated directory if it exists
if [ -d "$GENERATED_DIR" ]; then
    echo "Cleaning up existing generated directory..."
    rm -rf "$GENERATED_DIR"
fi

# Create directory for generated files
mkdir -p "$GENERATED_DIR"

# Initialize configs for three users
echo "Initializing configs for users..."
cargo run --bin frost-client -- init -c "$GENERATED_DIR/alice.toml"
cargo run --bin frost-client -- init -c "$GENERATED_DIR/bob.toml"
cargo run --bin frost-client -- init -c "$GENERATED_DIR/eve.toml"

# Generate FROST key shares using trusted dealer
echo "Generating FROST key shares using trusted dealer..."
cargo run --bin frost-client -- trusted-dealer \
    -d "Alice, Bob and Eve's group" \
    --names Alice,Bob,Eve \
    -c "$GENERATED_DIR/alice.toml" \
    -c "$GENERATED_DIR/bob.toml" \
    -c "$GENERATED_DIR/eve.toml"

echo "Key generation complete. Check the generated directory for the config files." 