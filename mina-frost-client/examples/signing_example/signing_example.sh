#!/bin/bash

# Strict error handling - exit on any error, undefined variable, or pipe failure
set -euo pipefail

# Get the directory where the script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
GENERATED_DIR="$SCRIPT_DIR/generated"
TRUSTED_DEALER_DIR="$SCRIPT_DIR/../trusted_dealer_example/generated"
HELPERS_DIR="$SCRIPT_DIR/../helpers"

cd "$SCRIPT_DIR"

# Default server URL
SERVER_URL="localhost:2744"

# Source helpers
source "$HELPERS_DIR/init_frostd.sh"
source "$HELPERS_DIR/file_generation.sh"
# Remove release binary if you want the script to use cargo run
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
echo "FROST Signing Example"
echo "========================================="

# Check if trusted dealer keys exist
if [ ! -d "$TRUSTED_DEALER_DIR" ] || [ ! -f "$TRUSTED_DEALER_DIR/alice.toml" ] || [ ! -f "$TRUSTED_DEALER_DIR/bob.toml" ] || [ ! -f "$TRUSTED_DEALER_DIR/eve.toml" ]; then
    echo "Setting up trusted dealer keys..."

    # Run the trusted dealer example
    if [ -f "$SCRIPT_DIR/../trusted_dealer_example/trusted_dealer_example.sh" ]; then
        "$SCRIPT_DIR/../trusted_dealer_example/trusted_dealer_example.sh"
        TRUSTED_DEALER_EXIT=$?

        if [ $TRUSTED_DEALER_EXIT -ne 0 ]; then
            echo "ERROR: Failed to set up trusted dealer keys"
            exit 1
        fi
        echo "✓ Keys ready"
    else
        echo "ERROR: Trusted dealer script not found!"
        exit 1
    fi
fi

# Clean up and copy configs
setup_generated_dir "$GENERATED_DIR"
cp "$TRUSTED_DEALER_DIR"/*.toml "$GENERATED_DIR/"

# Start server
init_frostd "$GENERATED_DIR" "$SERVER_URL" || {
    echo "ERROR: Failed to initialize server"
    exit 1
}

# Get group information
ALICE_GROUPS=$(use_frost_client groups -c "$GENERATED_DIR/alice.toml" 2>&1)
GROUP_PUBLIC_KEY=$(echo "$ALICE_GROUPS" | grep "Public key (hex format):" | head -1 | sed 's/.*Public key (hex format): \([a-f0-9]*\).*/\1/')
if [ -z "$GROUP_PUBLIC_KEY" ]; then
    echo "ERROR: Could not extract group public key"
    exit 1
fi

# Prepare message
TEST_MESSAGE=$(cat "$SCRIPT_DIR/message.json")
echo -n "$TEST_MESSAGE" > "$GENERATED_DIR/message.json"

echo "Starting signing process..."

# Get participant public keys
ALICE_CONTACTS=$(use_frost_client contacts -c "$GENERATED_DIR/alice.toml" 2>&1)
BOB_PUBLIC_KEY=$(echo "$ALICE_CONTACTS" | grep -A1 "Name: Bob" | grep "Public Key:" | cut -d' ' -f3)
EVE_PUBLIC_KEY=$(echo "$ALICE_CONTACTS" | grep -A1 "Name: Eve" | grep "Public Key:" | cut -d' ' -f3)

if [ -z "$BOB_PUBLIC_KEY" ] || [ -z "$EVE_PUBLIC_KEY" ]; then
    echo "ERROR: Could not extract participant public keys"
    exit 1
fi

# Start coordinator and participants
echo "Starting coordinator..."
use_frost_client coordinator \
    -c "$GENERATED_DIR/alice.toml" \
    --server-url "$SERVER_URL" \
    --group "$GROUP_PUBLIC_KEY" \
    -S "$BOB_PUBLIC_KEY,$EVE_PUBLIC_KEY" \
    -m "$GENERATED_DIR/message.json" \
    -o "$GENERATED_DIR/signature.json" &
COORDINATOR_PID=$!

# Give coordinator time to create the signing session
echo "Waiting for coordinator to create signing session..."
sleep 5

echo "Starting participant (Bob)..."
echo "y" | use_frost_client participant \
    -c "$GENERATED_DIR/bob.toml" \
    --server-url "$SERVER_URL" \
    --group "$GROUP_PUBLIC_KEY" &
BOB_PID=$!

echo "Starting participant (Eve)..."
echo "y" | use_frost_client participant \
    -c "$GENERATED_DIR/eve.toml" \
    --server-url "$SERVER_URL" \
    --group "$GROUP_PUBLIC_KEY" &
EVE_PID=$!

# Wait for completion
echo "Waiting for signing process to complete..."
wait $COORDINATOR_PID
COORDINATOR_EXIT=$?
wait $BOB_PID
BOB_EXIT=$?
wait $EVE_PID
EVE_EXIT=$?

# Check results
if [ $COORDINATOR_EXIT -eq 0 ] && [ $BOB_EXIT -eq 0 ] && [ $EVE_EXIT -eq 0 ]; then
    echo "✅ Signing completed successfully!"
    if [ -f "$GENERATED_DIR/signature.json" ]; then
        echo "📄 Signature saved to: $GENERATED_DIR/signature.json"
        echo "🔑 Group public key: $GROUP_PUBLIC_KEY"
    else
        echo "⚠️  Signature file not found"
        exit 1
    fi
else
    echo "❌ Signing failed (exit codes: C=$COORDINATOR_EXIT, B=$BOB_EXIT, E=$EVE_EXIT)"
    exit 1
fi
