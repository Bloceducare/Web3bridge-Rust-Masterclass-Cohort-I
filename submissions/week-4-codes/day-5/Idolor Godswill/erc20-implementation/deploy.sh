#!/bin/bash

# Deployment script for ERC20 Token

set -e

# Configuration
PRIVATE_KEY="${PRIVATE_KEY:-}"
RPC_URL="${RPC_URL:-https://sepolia-rollup.arbitrum.io/rpc}"
WASM_FILE="target/wasm32-unknown-unknown/release/erc20-implementation.wasm"

# Token configuration
TOKEN_NAME="${TOKEN_NAME:-MyToken}"
TOKEN_SYMBOL="${TOKEN_SYMBOL:-MTK}"
TOKEN_DECIMALS="${TOKEN_DECIMALS:-18}"
INITIAL_SUPPLY="${INITIAL_SUPPLY:-1000000000000000000000000}"

echo "Deploying ERC20 Token to Arbitrum Stylus..."
echo "Configuration:"
echo "   Token Name: $TOKEN_NAME"
echo "   Token Symbol: $TOKEN_SYMBOL"
echo "   Decimals: $TOKEN_DECIMALS"
echo "   Initial Supply: $INITIAL_SUPPLY"
echo "   RPC URL: $RPC_URL"

# Check if WASM file exists
if [ ! -f "$WASM_FILE" ]; then
    echo "WASM file not found. Please run ./build.sh first."
    exit 1
fi

# Check if private key is provided
if [ -z "$PRIVATE_KEY" ]; then
    echo "PRIVATE_KEY environment variable not set."
    echo "Usage: PRIVATE_KEY=your_private_key ./deploy.sh"
    exit 1
fi

# Check if cargo-stylus is installed
if ! command -v cargo-stylus &> /dev/null; then
    echo "Installing cargo-stylus..."
    cargo install cargo-stylus
fi

echo "Deploying contract..."

# Deploy using cargo-stylus
cargo stylus deploy \
    --private-key="$PRIVATE_KEY" \
    --endpoint="$RPC_URL" \
    --wasm-file="$WASM_FILE"

if [ $? -eq 0 ]; then
    echo "Deployment successful!"
    echo ""
    echo "Your ERC20 token has been deployed to Arbitrum Stylus!"
    echo "Remember to initialize your token with the initialize() function:"
    echo "   - name: $TOKEN_NAME"
    echo "   - symbol: $TOKEN_SYMBOL"
    echo "   - decimals: $TOKEN_DECIMALS"
    echo "   - initial_supply: $INITIAL_SUPPLY"
else
    echo "Deployment failed!"
    exit 1
fi