#!/bin/bash

# Helper functions for managing generated directories in FROST examples
# Strict error handling - exit on any error, undefined variable, or pipe failure
set -euo pipefail

# Function to clean and recreate a generated directory
# Usage: setup_generated_dir "/path/to/generated"
setup_generated_dir() {
    local generated_dir="$1"

    if [ -z "$generated_dir" ]; then
        echo "ERROR: Generated directory path is required"
        return 1
    fi

    echo "Setting up generated directory: $generated_dir"

    # Remove existing directory if it exists
    if [ -d "$generated_dir" ]; then
        rm -rf "$generated_dir"
    fi

    # Create fresh directory
    mkdir -p "$generated_dir"

    echo "Generated directory ready: $generated_dir"
}
