#!/bin/bash

# Get the directory where the script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
GENERATED_DIR="$SCRIPT_DIR/generated"
HELPERS_DIR="$SCRIPT_DIR/../helpers"

cd "$SCRIPT_DIR/../.."

# Source the file generation helper
source "$HELPERS_DIR/file_generation.sh"

# Setup
setup_generated_dir "$GENERATED_DIR"

# Initialize configs for three users
cargo run --bin frost-client -- init -c "$GENERATED_DIR/alice.toml"
cargo run --bin frost-client -- init -c "$GENERATED_DIR/bob.toml"
cargo run --bin frost-client -- init -c "$GENERATED_DIR/eve.toml"

echo "Generating FROST key shares..."
# Generate FROST key shares using trusted dealer
cargo run --bin frost-client -- trusted-dealer \
    -d "Alice, Bob and Eve's group" \
    --names Alice,Bob,Eve \
    -c "$GENERATED_DIR/alice.toml" \
    -c "$GENERATED_DIR/bob.toml" \
    -c "$GENERATED_DIR/eve.toml" \
    -t 2

echo "Key generation complete. Check the generated directory for the config files."
