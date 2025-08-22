#!/bin/bash

# Build script for ERC20 Token

set -e

echo "Building ERC20 Token..."

# Clean previous builds
cargo clean 2>/dev/null || true

# Build the optimized WASM binary
echo "Building optimized WASM binary..."
cargo build --release --target wasm32-unknown-unknown

# Check build success
if [ -f "target/wasm32-unknown-unknown/release/erc20-implementation.wasm" ]; then
    echo "Build successful!"
    echo "WASM binary size:"
    ls -lh target/wasm32-unknown-unknown/release/erc20-implementation.wasm
    
    # Generate ABI if feature is enabled
    if cargo check --features export-abi &>/dev/null; then
        echo "Generating ABI..."
        cargo build --features export-abi --release --target wasm32-unknown-unknown
        echo "ABI generated successfully!"
    fi
    
    echo ""
    echo "Ready for deployment to Arbitrum Stylus!"
    echo "WASM binary location: target/wasm32-unknown-unknown/release/erc20-implementation.wasm"
else
    echo "Build failed!"
    exit 1
fi