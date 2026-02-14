#!/bin/bash

# Strict error handling - exit on any error, undefined variable, or pipe failure
set -euo pipefail

# Get the directory where the script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
GENERATED_DIR="$SCRIPT_DIR/generated"
HELPERS_DIR="$SCRIPT_DIR/../helpers"

cd "$SCRIPT_DIR"

# Default server URL
SERVER_URL="localhost:2744"

# Source helpers
# shellcheck source=init_frostd.sh
source "$HELPERS_DIR/init_frostd.sh"
# shellcheck source=file_generation.sh
source "$HELPERS_DIR/file_generation.sh"
# Remove release binary if you want the script to use cargo run
# shellcheck source=use_frost_client.sh
source "$HELPERS_DIR/use_frost_client.sh"

# Function to cleanup on exit
cleanup() {
    echo "Cleaning up..."
    stop_frostd "$FROSTD_SERVER_PID"
    exit
}

# Set up cleanup trap
trap cleanup EXIT INT TERM

echo "========================================="
echo "FROST DKG Example"
echo "========================================="

# Setup
setup_generated_dir "$GENERATED_DIR"

# Start server
init_frostd "$GENERATED_DIR" "$SERVER_URL" || {
    echo "ERROR: Failed to initialize server"
    exit 1
}

echo "Initializing participants..."
use_frost_client init -c "$GENERATED_DIR/alice.toml"
use_frost_client init -c "$GENERATED_DIR/bob.toml"
use_frost_client init -c "$GENERATED_DIR/eve.toml"

echo "Generating contacts..."
ALICE_CONTACT=$(use_frost_client export --name 'Alice' -c "$GENERATED_DIR/alice.toml" 2>&1 | grep "^minafrost" || true)
BOB_CONTACT=$(use_frost_client export --name 'Bob' -c "$GENERATED_DIR/bob.toml" 2>&1 | grep "^minafrost" || true)
EVE_CONTACT=$(use_frost_client export --name 'Eve' -c "$GENERATED_DIR/eve.toml" 2>&1 | grep "^minafrost" || true)

if [ -z "$ALICE_CONTACT" ] || [ -z "$BOB_CONTACT" ] || [ -z "$EVE_CONTACT" ]; then
    echo "ERROR: Failed to generate contact strings"
    exit 1
fi

echo "Importing contacts..."
use_frost_client import -c "$GENERATED_DIR/alice.toml" "$BOB_CONTACT"
use_frost_client import -c "$GENERATED_DIR/alice.toml" "$EVE_CONTACT"
use_frost_client import -c "$GENERATED_DIR/bob.toml" "$ALICE_CONTACT"
use_frost_client import -c "$GENERATED_DIR/bob.toml" "$EVE_CONTACT"
use_frost_client import -c "$GENERATED_DIR/eve.toml" "$ALICE_CONTACT"
use_frost_client import -c "$GENERATED_DIR/eve.toml" "$BOB_CONTACT"

echo ""
echo "Starting DKG process..."

# Extract public keys from contacts
CONTACTS_OUTPUT=$(use_frost_client contacts -c "$GENERATED_DIR/alice.toml" 2>&1 | grep -A2 "Name:")
GROUP_NAME="Alice, Bob and Eve"
BOB_PUBLIC_KEY=$(echo "$CONTACTS_OUTPUT" | grep -A1 "Name: Bob" | grep "Public Key:" | cut -d' ' -f3)
EVE_PUBLIC_KEY=$(echo "$CONTACTS_OUTPUT" | grep -A1 "Name: Eve" | grep "Public Key:" | cut -d' ' -f3)

# Run DKG processes with proper timing
echo "Starting Alice (coordinator) DKG process..."
use_frost_client dkg \
    -d "$GROUP_NAME" \
    -s "$SERVER_URL" \
    -S "$BOB_PUBLIC_KEY,$EVE_PUBLIC_KEY" \
    -t 2 \
    -c "$GENERATED_DIR/alice.toml" &
ALICE_DKG_PID=$!

# Give Alice time to create the DKG session
echo "Waiting for Alice to create DKG session..."
sleep 5

echo "Starting Bob DKG process..."
use_frost_client dkg \
    -d "$GROUP_NAME" \
    -s "$SERVER_URL" \
    -t 2 \
    -c "$GENERATED_DIR/bob.toml" &
BOB_DKG_PID=$!

# Brief delay before starting Eve
sleep 2

echo "Starting Eve DKG process..."
use_frost_client dkg \
    -d "$GROUP_NAME" \
    -s "$SERVER_URL" \
    -t 2 \
    -c "$GENERATED_DIR/eve.toml" &
EVE_DKG_PID=$!

echo "Waiting for DKG processes to complete..."
echo "This may take up to 2-3 minutes..."

# Wait for completion
wait $ALICE_DKG_PID
ALICE_EXIT=$?
wait $BOB_DKG_PID
BOB_EXIT=$?
wait $EVE_DKG_PID
EVE_EXIT=$?

# Check results
if [ $ALICE_EXIT -eq 0 ] && [ $BOB_EXIT -eq 0 ] && [ $EVE_EXIT -eq 0 ]; then
    echo "✅ DKG completed successfully!"

    # Validate that DKG actually worked by checking for group keys
    echo "Validating DKG results..."
    for config in "$GENERATED_DIR/alice.toml" "$GENERATED_DIR/bob.toml" "$GENERATED_DIR/eve.toml"; do
        if ! grep -q "key_package" "$config" 2>/dev/null; then
            echo "❌ DKG validation failed: $config missing key_package" >&2
            exit 1
        fi
        if ! grep -q "public_key_package" "$config" 2>/dev/null; then
            echo "❌ DKG validation failed: $config missing public_key_package" >&2
            exit 1
        fi
    done

    # Show one group info as confirmation
    GROUP_INFO=$(use_frost_client groups -c "$GENERATED_DIR/alice.toml" | head -2)
    echo "$GROUP_INFO"
    echo "📁 Config files saved to: $GENERATED_DIR/"
else
    echo "❌ DKG failed (exit codes: A=$ALICE_EXIT, B=$BOB_EXIT, E=$EVE_EXIT)"

    # Print some debug info to help diagnose the issue
    echo "Debug info:"
    echo "Alice config exists: $(test -f "$GENERATED_DIR/alice.toml" && echo "yes" || echo "no")"
    echo "Bob config exists: $(test -f "$GENERATED_DIR/bob.toml" && echo "yes" || echo "no")"
    echo "Eve config exists: $(test -f "$GENERATED_DIR/eve.toml" && echo "yes" || echo "no")"
    exit 1
fi
