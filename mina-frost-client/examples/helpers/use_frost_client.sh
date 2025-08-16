#!/bin/bash

# Helper method for running frost commands
# Strict error handling - exit on any error, undefined variable, or pipe failure
set -euo pipefail

use_frost_client() {
    if ! cargo run --bin mina-frost-client -- "$@"; then
        echo "Error: mina-frost-client command failed with arguments: $*" >&2
        return 1
    fi
}
