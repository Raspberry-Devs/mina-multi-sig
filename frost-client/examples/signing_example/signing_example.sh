#!/bin/bash

# Check if message argument is provided
if [ $# -ne 1 ]; then
    echo "Usage: $0 <message_to_sign>"
    exit 1
fi

MESSAGE=$1
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GENERATED_DIR="$SCRIPT_DIR/../trusted_dealer_example/generated"

# Check if generated files exist
if [ ! -f "$GENERATED_DIR/alice.toml" ] || [ ! -f "$GENERATED_DIR/bob.toml" ] || [ ! -f "$GENERATED_DIR/eve.toml" ]; then
    echo "Error: Required configuration files not found in $GENERATED_DIR"
    echo "Please run the trusted dealer example first to generate the necessary files."
    exit 1
fi

# Create temporary TLS certificate
TEMP_DIR=$(mktemp -d)
openssl req -x509 -newkey rsa:4096 -keyout "$TEMP_DIR/key.pem" -out "$TEMP_DIR/cert.pem" -days 365 -nodes -subj "/CN=localhost"

# Start FROST server in the background
echo "Starting FROST server..."
frostd --tls-cert "$TEMP_DIR/cert.pem" --tls-key "$TEMP_DIR/key.pem" &
SERVER_PID=$!

# Wait for server to start
sleep 2

# Function to cleanup on exit
cleanup() {
    echo "Cleaning up..."
    kill $SERVER_PID 2>/dev/null
    rm -rf "$TEMP_DIR"
    exit
}

# Set up trap for cleanup
trap cleanup EXIT INT TERM

# Get group information from Alice's config
GROUP_INFO=$(frost-client groups -c "$GENERATED_DIR/alice.toml")
GROUP_PUBKEY=$(echo "$GROUP_INFO" | grep "Public key:" | awk '{print $3}')

if [ -z "$GROUP_PUBKEY" ]; then
    echo "Error: Could not get group public key"
    exit 1
fi

echo "Starting signing session with group: $GROUP_PUBKEY"

# Create a temporary file for coordinator output
COORDINATOR_OUTPUT=$(mktemp)

# Start coordinator (Alice) in background and capture output
echo "Starting coordinator..."
frost-client coordinator -c "$GENERATED_DIR/alice.toml" --server-url localhost:2744 --group "$GROUP_PUBKEY" -S "$(frost-client contacts -c "$GENERATED_DIR/alice.toml" | grep "Bob" | awk '{print $2}'),$(frost-client contacts -c "$GENERATED_DIR/alice.toml" | grep "Eve" | awk '{print $2}')" -m "$MESSAGE" -C redpallas > "$COORDINATOR_OUTPUT" 2>&1 &
COORDINATOR_PID=$!

# Start two participants (Bob and Eve)
echo "Starting participants..."
frost-client participant -c "$GENERATED_DIR/bob.toml" --server-url localhost:2744 --group "$GROUP_PUBKEY" -C redpallas &
BOB_PID=$!

frost-client participant -c "$GENERATED_DIR/eve.toml" --server-url localhost:2744 --group "$GROUP_PUBKEY" -C redpallas &
EVE_PID=$!

# Wait for all processes to complete
wait $COORDINATOR_PID $BOB_PID $EVE_PID

echo "Signing complete!"
echo "Signed message:"
cat "$COORDINATOR_OUTPUT"

# Clean up temporary file
rm "$COORDINATOR_OUTPUT" 