#!/bin/bash

# Get the directory where the script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
GENERATED_DIR="$SCRIPT_DIR/generated"

# Default server URL
SERVER_URL="localhost:2744"
SERVER_PID=""

# Function to cleanup on exit
cleanup() {
    echo "Cleaning up..."
    if [ ! -z "$SERVER_PID" ] && kill -0 "$SERVER_PID" 2>/dev/null; then
        echo "Stopping frostd server (PID: $SERVER_PID)..."
        kill "$SERVER_PID"
        wait "$SERVER_PID" 2>/dev/null
    fi
    exit
}

# Set up cleanup trap
trap cleanup EXIT INT TERM

echo "========================================="
echo "Starting FROST DKG Example"
echo "========================================="

# Clean up generated directory if it exists
if [ -d "$GENERATED_DIR" ]; then
    echo "Cleaning up existing generated directory..."
    rm -rf "$GENERATED_DIR"
fi

# Create directory for generated files
mkdir -p "$GENERATED_DIR"

# Setting up the tls certificates
cd $GENERATED_DIR
mkcert localhost 127.0.0.1 ::1
cd ..

pwd

echo ""
echo "========================================="
echo "Starting frostd server"
echo "========================================="

# Start frostd server in the background
echo "Starting frostd server on $SERVER_URL..."
echo "Using TLS cert: $GENERATED_DIR/localhost+2.pem"
echo "Using TLS key: $GENERATED_DIR/localhost+2-key.pem"
frostd --tls-cert "$GENERATED_DIR/localhost+2.pem" --tls-key "$GENERATED_DIR/localhost+2-key.pem" &
SERVER_PID=$!
echo "Server PID: $SERVER_PID"

# Wait a moment for the server to start
sleep 2
echo "frostd server started with PID: $SERVER_PID"

echo ""
echo "========================================="
echo "Initializing participant configurations"
echo "========================================="

# Initialize configs for three users
echo "Initializing configs for users..."
cargo run --bin frost-client -- init -c "$GENERATED_DIR/alice.toml"
cargo run --bin frost-client -- init -c "$GENERATED_DIR/bob.toml"
cargo run --bin frost-client -- init -c "$GENERATED_DIR/eve.toml"

echo ""
echo "========================================="
echo "Generating contact strings"
echo "========================================="

# Generate contact strings for each participant
echo "Generating contact strings..."

echo "Generating Alice's contact..."
ALICE_CONTACT=$(cargo run --bin frost-client -- export --name 'Alice' -c "$GENERATED_DIR/alice.toml" 2>&1 | grep "^zffrost" || true)
echo "Alice's contact: '$ALICE_CONTACT'"

echo "Generating Bob's contact..."
BOB_CONTACT=$(cargo run --bin frost-client -- export --name 'Bob' -c "$GENERATED_DIR/bob.toml" 2>&1 | grep "^zffrost" || true)
echo "Bob's contact: '$BOB_CONTACT'"

echo "Generating Eve's contact..."
EVE_CONTACT=$(cargo run --bin frost-client -- export --name 'Eve' -c "$GENERATED_DIR/eve.toml" 2>&1 | grep "^zffrost" || true)
echo "Eve's contact: '$EVE_CONTACT'"

echo ""
echo "========================================="
echo "Importing contacts for each participant"
echo "========================================="

# Import contacts for each participant
echo "Importing contacts..."

# Validate contact strings are not empty
if [ -z "$ALICE_CONTACT" ] || [ -z "$BOB_CONTACT" ] || [ -z "$EVE_CONTACT" ]; then
    echo "Error: One or more contact strings are empty!"
    echo "Alice: '$ALICE_CONTACT'"
    echo "Bob: '$BOB_CONTACT'"
    echo "Eve: '$EVE_CONTACT'"
    exit 1
fi

echo "Importing contacts for Alice..."
cargo run --bin frost-client -- import -c "$GENERATED_DIR/alice.toml" "$BOB_CONTACT"
cargo run --bin frost-client -- import -c "$GENERATED_DIR/alice.toml" "$EVE_CONTACT"

echo "Importing contacts for Bob..."
cargo run --bin frost-client -- import -c "$GENERATED_DIR/bob.toml" "$ALICE_CONTACT"
cargo run --bin frost-client -- import -c "$GENERATED_DIR/bob.toml" "$EVE_CONTACT"

echo "Importing contacts for Eve..."
cargo run --bin frost-client -- import -c "$GENERATED_DIR/eve.toml" "$ALICE_CONTACT"
cargo run --bin frost-client -- import -c "$GENERATED_DIR/eve.toml" "$BOB_CONTACT"

echo ""
echo "========================================="
echo "Starting DKG process"
echo "========================================="

# Run DKG process
echo "Starting DKG process..."

# Extract public keys from contacts
echo "Extracting public keys from contacts..."
CONTACTS_OUTPUT=$(cargo run --bin frost-client -- contacts -c "$GENERATED_DIR/alice.toml" 2>&1)



# Define group name
GROUP_NAME="Alice, Bob and Eve"
BOB_PUBLIC_KEY=$(echo "$CONTACTS_OUTPUT" | grep -A1 "Name: Bob" | grep "Public Key:" | cut -d' ' -f3)
EVE_PUBLIC_KEY=$(echo "$CONTACTS_OUTPUT" | grep -A1 "Name: Eve" | grep "Public Key:" | cut -d' ' -f3)

# Alice initiates the DKG
echo "Alice initiating DKG..."
cargo run --bin frost-client -- dkg \
    -d "$GROUP_NAME" \
    -s "$SERVER_URL" \
    -S "$BOB_PUBLIC_KEY,$EVE_PUBLIC_KEY" \
    -t 2 \
    -C bluepallas \
    -c "$GENERATED_DIR/alice.toml" &
ALICE_DKG_PID=$!

# Wait a moment for Alice to start
sleep 3

# Bob joins the DKG
echo ""
echo "Bob joining DKG..."
cargo run --bin frost-client -- dkg \
    -d "$GROUP_NAME" \
    -s "$SERVER_URL" \
    -t 2 \
    -C bluepallas \
    -c "$GENERATED_DIR/bob.toml" &
BOB_DKG_PID=$!

# Wait a moment
sleep 3

# Eve joins the DKG
echo ""
echo "Eve joining DKG..."
cargo run --bin frost-client -- dkg \
    -d "$GROUP_NAME" \
    -s "$SERVER_URL" \
    -t 2 \
    -C bluepallas \
    -c "$GENERATED_DIR/eve.toml" &
EVE_DKG_PID=$!

# Wait for all DKG processes to complete
echo ""
echo "Waiting for DKG processes to complete..."
wait $ALICE_DKG_PID
ALICE_EXIT=$?
wait $BOB_DKG_PID
BOB_EXIT=$?
wait $EVE_DKG_PID
EVE_EXIT=$?

echo ""
echo "========================================="
echo "DKG Results"
echo "========================================="

# Check if DKG completed successfully
if [ $ALICE_EXIT -eq 0 ] && [ $BOB_EXIT -eq 0 ] && [ $EVE_EXIT -eq 0 ]; then
    echo "DKG completed successfully!"
    echo ""
    echo "Checking generated groups..."
    echo "Alice's groups:"
    cargo run --bin frost-client -- groups -c "$GENERATED_DIR/alice.toml"
    echo ""
    echo "Bob's groups:"
    cargo run --bin frost-client -- groups -c "$GENERATED_DIR/bob.toml"
    echo ""
    echo "Eve's groups:"
    cargo run --bin frost-client -- groups -c "$GENERATED_DIR/eve.toml"
    echo ""
    echo "DKG process complete. Check the generated directory for the config files."
else
    echo "DKG process failed. Exit codes: Alice=$ALICE_EXIT, Bob=$BOB_EXIT, Eve=$EVE_EXIT"
    exit 1
fi
