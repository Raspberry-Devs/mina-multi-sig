FROM ubuntu:22.04

# Avoid interactive prompts during package installation
ENV DEBIAN_FRONTEND=noninteractive
ARG RUST_VERSION=1.92.0

# Install system dependencies
# curl - Download tools and Rust installer
# build-essential - GCC compiler and build tools for Rust
# pkg-config - Helps find system libraries during compilation
# libssl-dev - SSL/TLS support required by Rust networking crates
# ca-certificates - Trusted certificate authorities for HTTPS
# git - Version control needed by cargo for git dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    ca-certificates \
    git \
    && rm -rf /var/lib/apt/lists/*

# Install Rust programming language and toolchain
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain ${RUST_VERSION} \
    && /root/.cargo/bin/rustup component add rustfmt clippy
ENV PATH="/root/.cargo/bin:${PATH}"

# Install mkcert for generating local TLS certificates (required by frostd server)
RUN curl -JLO "https://dl.filippo.io/mkcert/latest?for=linux/amd64" \
    && chmod +x mkcert-v*-linux-amd64 \
    && mv mkcert-v*-linux-amd64 /usr/local/bin/mkcert

# Setup mkcert CA (this needs to be done at runtime for the examples)
RUN mkcert -install

# Pre-install frostd server
RUN cargo install --git https://github.com/ZcashFoundation/frost-zcash-demo.git --locked frostd

# Create workspace directory
WORKDIR /workspace

# Set environment variables
ENV CARGO_TERM_COLOR=always
ENV RUST_LOG=warn

# Default command
CMD ["/bin/bash"]
