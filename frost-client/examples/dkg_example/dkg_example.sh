#!/bin/bash

# Get the directory where the script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
GENERATED_DIR="$SCRIPT_DIR/generated"
HELPERS_DIR="$SCRIPT_DIR/../helpers"

cd "$SCRIPT_DIR"

# Default server URL
SERVER_URL="localhost:2744"

# Source the server initialization helper
source "$HELPERS_DIR/init_frostd.sh"

# Source the file generation helper
source "$HELPERS_DIR/file_generation.sh"

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
cargo run --bin frost-client -- init -c "$GENERATED_DIR/alice.toml"
cargo run --bin frost-client -- init -c "$GENERATED_DIR/bob.toml"
cargo run --bin frost-client -- init -c "$GENERATED_DIR/eve.toml"

echo "Generating contacts..."
ALICE_CONTACT=$(cargo run --bin frost-client -- export --name 'Alice' -c "$GENERATED_DIR/alice.toml" 2>&1 | grep "^minafrost" || true)
BOB_CONTACT=$(cargo run --bin frost-client -- export --name 'Bob' -c "$GENERATED_DIR/bob.toml" 2>&1 | grep "^minafrost" || true)
EVE_CONTACT=$(cargo run --bin frost-client -- export --name 'Eve' -c "$GENERATED_DIR/eve.toml" 2>&1 | grep "^minafrost" || true)

if [ -z "$ALICE_CONTACT" ] || [ -z "$BOB_CONTACT" ] || [ -z "$EVE_CONTACT" ]; then
    echo "ERROR: Failed to generate contact strings"
    exit 1
fi

echo "Importing contacts..."
cargo run --bin frost-client -- import -c "$GENERATED_DIR/alice.toml" "$BOB_CONTACT"
cargo run --bin frost-client -- import -c "$GENERATED_DIR/alice.toml" "$EVE_CONTACT"
cargo run --bin frost-client -- import -c "$GENERATED_DIR/bob.toml" "$ALICE_CONTACT"
cargo run --bin frost-client -- import -c "$GENERATED_DIR/bob.toml" "$EVE_CONTACT"
cargo run --bin frost-client -- import -c "$GENERATED_DIR/eve.toml" "$ALICE_CONTACT"
cargo run --bin frost-client -- import -c "$GENERATED_DIR/eve.toml" "$BOB_CONTACT"

echo ""
echo "Starting DKG process..."

# Extract public keys from contacts  
CONTACTS_OUTPUT=$(cargo run --bin frost-client -- contacts -c "$GENERATED_DIR/alice.toml" 2>&1 | grep -A2 "Name:")
GROUP_NAME="Alice, Bob and Eve"
BOB_PUBLIC_KEY=$(echo "$CONTACTS_OUTPUT" | grep -A1 "Name: Bob" | grep "Public Key:" | cut -d' ' -f3)
EVE_PUBLIC_KEY=$(echo "$CONTACTS_OUTPUT" | grep -A1 "Name: Eve" | grep "Public Key:" | cut -d' ' -f3)

# Run DKG processes
cargo run --bin frost-client -- dkg \
    -d "$GROUP_NAME" \
    -s "$SERVER_URL" \
    -S "$BOB_PUBLIC_KEY,$EVE_PUBLIC_KEY" \
    -t 2 \
    -c "$GENERATED_DIR/alice.toml" &
ALICE_DKG_PID=$!

# Wait a moment for Alice to start
sleep 3

cargo run --bin frost-client -- dkg \
    -d "$GROUP_NAME" \
    -s "$SERVER_URL" \
    -t 2 \
    -c "$GENERATED_DIR/bob.toml" &
BOB_DKG_PID=$!

# Wait a moment
sleep 3

cargo run --bin frost-client -- dkg \
    -d "$GROUP_NAME" \
    -s "$SERVER_URL" \
    -t 2 \
    -c "$GENERATED_DIR/eve.toml" &
EVE_DKG_PID=$!

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
    
    # Show one group info as confirmation
    GROUP_INFO=$(cargo run --bin frost-client -- groups -c "$GENERATED_DIR/alice.toml" | head -2)
    echo "$GROUP_INFO"
    echo "📁 Config files saved to: $GENERATED_DIR/"
else
    echo "❌ DKG failed (exit codes: A=$ALICE_EXIT, B=$BOB_EXIT, E=$EVE_EXIT)"
    exit 1
fi
