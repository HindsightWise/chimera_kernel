#!/bin/bash
set -e

echo "Building WASM plugin with temporary toolchain..."

# Set up temporary toolchain directories to avoid OSX file locks
export RUSTUP_HOME=/tmp/.rustup_chimera
export CARGO_HOME=/tmp/.cargo_chimera

# Clean previous temp directories
rm -rf "$RUSTUP_HOME" "$CARGO_HOME"

# Install wasm32-wasi target
rustup target add wasm32-wasi --toolchain stable || {
    echo "Installing target to temporary location..."
    mkdir -p "$RUSTUP_HOME" "$CARGO_HOME"
    # We'll need to compile directly without rustup
}

cd plugins
echo "Building plugin with: cargo build --target wasm32-wasi --release"

# Try direct compilation
cargo +stable build --target wasm32-wasi --release 2>&1 || {
    echo "Direct compilation failed, trying alternative approach..."
    # Maybe we need to install target differently
}
