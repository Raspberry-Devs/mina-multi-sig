#!/bin/bash

# Strict error handling - exit on any error, undefined variable, or pipe failure
set -euo pipefail

# Get the directory where the script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
GENERATED_DIR="$SCRIPT_DIR/generated"
HELPERS_DIR="$SCRIPT_DIR/../helpers"

cd "$SCRIPT_DIR"

# Source helpers
# shellcheck source=mina-frost-client/examples/helpers/file_generation.sh
source "$HELPERS_DIR/file_generation.sh"
# shellcheck source=mina-frost-client/examples/helpers/use_frost_client.sh
source "$HELPERS_DIR/use_frost_client.sh"

# Setup
setup_generated_dir "$GENERATED_DIR"

# Initialize configs for three users
use_frost_client init -c "$GENERATED_DIR/alice.toml"
use_frost_client init -c "$GENERATED_DIR/bob.toml"
use_frost_client init -c "$GENERATED_DIR/eve.toml"

echo "Generating FROST key shares..."
# Generate FROST key shares using trusted dealer
use_frost_client trusted-dealer \
    -d "Alice, Bob and Eve's group" \
    --names Alice,Bob,Eve \
    -c "$GENERATED_DIR/alice.toml" \
    -c "$GENERATED_DIR/bob.toml" \
    -c "$GENERATED_DIR/eve.toml" \
    -t 2

# Validate that key generation was successful
echo "Validating generated configuration files..."
if [[ ! -f "$GENERATED_DIR/alice.toml" ]] || [[ ! -f "$GENERATED_DIR/bob.toml" ]] || [[ ! -f "$GENERATED_DIR/eve.toml" ]]; then
    echo "ERROR: Key generation failed - configuration files not found" >&2
    exit 1
fi

# Verify the config files contain key data (not just empty files)
for config in "$GENERATED_DIR/alice.toml" "$GENERATED_DIR/bob.toml" "$GENERATED_DIR/eve.toml"; do
    if ! grep -q "key_package" "$config" 2>/dev/null; then
        echo "ERROR: Configuration file $config appears to be invalid (no key_package found)" >&2
        exit 1
    fi
done

echo "✅ Key generation complete. Check the generated directory for the config files."
