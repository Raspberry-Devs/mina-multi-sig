#!/bin/bash

# FROST Server Initialization Helper
# This script handles starting the frostd server with TLS certificates
# Strict error handling - exit on any error, undefined variable, or pipe failure
set -euo pipefail

# Get the directory where the script is located
HELPER_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Function to initialize and start frostd server
init_frostd() {
    local generated_dir="$1"
    local server_url="${2:-localhost:2744}"

    if [ -z "$generated_dir" ]; then
        echo "ERROR: Generated directory path is required!"
        echo "Usage: init_frostd <generated_dir> [server_url]"
        return 1
    fi

    echo "Setting up TLS certificates"

    # Create generated directory if it doesn't exist
    mkdir -p "$generated_dir"

    # Ensure mkcert CA is installed (needed for Docker containers)
    if ! mkcert -CAROOT >/dev/null 2>&1 || [ ! -f "$(mkcert -CAROOT)/rootCA.pem" ]; then
        echo "Installing mkcert CA..."
        mkcert -install >/dev/null 2>&1 || true
    fi

    # Generate TLS certificates
    cd "$generated_dir"
    mkcert localhost 127.0.0.1 ::1 2>/dev/null || {
        echo "ERROR: mkcert failed. Please install mkcert first:"
        echo "  # On Ubuntu/Debian:"
        echo "  sudo apt install mkcert"
        echo "  # On macOS:"
        echo "  brew install mkcert"
        echo "  Also ensure you have run 'mkcert -install' to set up the local CA."
        cd - > /dev/null
        return 1
    }
    cd - > /dev/null

    echo "Starting frostd server"

    # Check if frostd is installed
    if ! command -v frostd &> /dev/null; then
        echo "ERROR: frostd is not installed or not in PATH!"
        echo ""
        echo "Please install frostd first. You can build it from the FROST server repository:"
        echo "  cargo install --git https://github.com/ZcashFoundation/frost-zcash-demo.git --locked frostd"
        return 1
    fi

    # Start frostd server in the background
    echo "Starting frostd server on $server_url..."

    frostd --tls-cert "$generated_dir/localhost+2.pem" --tls-key "$generated_dir/localhost+2-key.pem" &
    local server_pid=$!

    # Wait for the server to start
    sleep 3

    # Verify the server is running
    if kill -0 "$server_pid" 2>/dev/null; then
        echo "frostd server started successfully with PID: $server_pid"
        # Export the PID for the calling script to use
        export FROSTD_SERVER_PID="$server_pid"
        return 0
    else
        echo "ERROR: Failed to start frostd server!"
        return 1
    fi
}

# Function to stop frostd server
stop_frostd() {
    local server_pid="$1"

    if [ -z "$server_pid" ]; then
        server_pid="$FROSTD_SERVER_PID"
    fi

    if [ ! -z "$server_pid" ] && kill -0 "$server_pid" 2>/dev/null; then
        echo "Stopping frostd server (PID: $server_pid)..."
        kill "$server_pid"
        wait "$server_pid" 2>/dev/null || true
        echo "frostd server stopped."
    fi
}

# If script is run directly (not sourced), call init_frostd with arguments
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    init_frostd "$@"
fi
