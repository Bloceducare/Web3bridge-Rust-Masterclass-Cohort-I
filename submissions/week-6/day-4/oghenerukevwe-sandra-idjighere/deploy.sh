#!/bin/bash

# SEP-41 Token Deployment Script
# This script automates the deployment and initialization of the SEP-41 token contract

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if required tools are installed
check_dependencies() {
    print_status "Checking dependencies..."
    
    if ! command -v cargo &> /dev/null; then
        print_error "cargo is not installed. Please install Rust toolchain."
        exit 1
    fi
    
    if ! command -v soroban &> /dev/null; then
        print_error "soroban CLI is not installed. Please install Soroban CLI."
        exit 1
    fi
    
    # Check if wasm32-unknown-unknown target is installed
    if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
        print_warning "wasm32-unknown-unknown target not found. Installing..."
        rustup target add wasm32-unknown-unknown
    fi
    
    print_success "All dependencies are available"
}

# Build the contract
build_contract() {
    print_status "Building the contract..."
    cd contracts/sep41-token
    make build
    cd ../..
    print_success "Contract built successfully"
}

# Run tests
run_tests() {
    print_status "Running tests..."
    cd contracts/sep41-token
    make test
    cd ../..
    print_success "All tests passed"
}

# Deploy contract
deploy_contract() {
    print_status "Deploying contract to $NETWORK..."
    
    if [ -z "$SOURCE_ACCOUNT" ]; then
        print_error "SOURCE_ACCOUNT environment variable is required"
        exit 1
    fi
    
    CONTRACT_ID=$(soroban contract deploy \
        --wasm contracts/sep41-token/target/wasm32-unknown-unknown/release/sep41_token.wasm \
        --source-account $SOURCE_ACCOUNT \
        --network $NETWORK)
    
    print_success "Contract deployed with ID: $CONTRACT_ID"
    echo "export CONTRACT_ID=$CONTRACT_ID" >> .env
}

# Initialize contract
initialize_contract() {
    print_status "Initializing contract..."
    
    if [ -z "$CONTRACT_ID" ] || [ -z "$SOURCE_ACCOUNT" ] || [ -z "$TOKEN_NAME" ] || [ -z "$TOKEN_SYMBOL" ]; then
        print_error "CONTRACT_ID, SOURCE_ACCOUNT, TOKEN_NAME, and TOKEN_SYMBOL environment variables are required"
        exit 1
    fi
    
    soroban contract invoke \
        --id $CONTRACT_ID \
        --source-account $SOURCE_ACCOUNT \
        --network $NETWORK \
        -- initialize \
        --admin $SOURCE_ACCOUNT \
        --decimal ${TOKEN_DECIMALS:-18} \
        --name "$TOKEN_NAME" \
        --symbol "$TOKEN_SYMBOL"
    
    print_success "Contract initialized successfully"
}

# Mint initial supply
mint_initial_supply() {
    if [ -n "$INITIAL_SUPPLY" ] && [ -n "$INITIAL_RECIPIENT" ]; then
        print_status "Minting initial supply..."
        
        soroban contract invoke \
            --id $CONTRACT_ID \
            --source-account $SOURCE_ACCOUNT \
            --network $NETWORK \
            -- mint \
            --to $INITIAL_RECIPIENT \
            --amount $INITIAL_SUPPLY
        
        print_success "Initial supply minted successfully"
    else
        print_warning "INITIAL_SUPPLY and INITIAL_RECIPIENT not set, skipping initial mint"
    fi
}

# Display contract info
show_contract_info() {
    print_status "Contract Information:"
    echo "Contract ID: $CONTRACT_ID"
    echo "Network: $NETWORK"
    echo "Admin: $SOURCE_ACCOUNT"
    
    print_status "Token Metadata:"
    echo "Name: $(soroban contract invoke --id $CONTRACT_ID --network $NETWORK -- name)"
    echo "Symbol: $(soroban contract invoke --id $CONTRACT_ID --network $NETWORK -- symbol)"
    echo "Decimals: $(soroban contract invoke --id $CONTRACT_ID --network $NETWORK -- decimals)"
}

# Main deployment function
main() {
    print_status "Starting SEP-41 Token deployment..."
    
    # Set default network if not provided
    NETWORK=${NETWORK:-testnet}
    
    # Load environment variables if .env file exists
    if [ -f .env ]; then
        print_status "Loading environment variables from .env file..."
        source .env
    fi
    
    # Check dependencies
    check_dependencies
    
    # Build and test
    build_contract
    run_tests
    
    # Deploy if CONTRACT_ID is not set
    if [ -z "$CONTRACT_ID" ]; then
        deploy_contract
    else
        print_warning "CONTRACT_ID already set, skipping deployment"
    fi
    
    # Initialize contract
    initialize_contract
    
    # Mint initial supply if specified
    mint_initial_supply
    
    # Show contract info
    show_contract_info
    
    print_success "Deployment completed successfully!"
    print_status "Contract ID: $CONTRACT_ID"
    print_status "Save this Contract ID for future interactions"
}

# Help function
show_help() {
    echo "SEP-41 Token Deployment Script"
    echo ""
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Environment Variables:"
    echo "  SOURCE_ACCOUNT     - Stellar account for deployment (required)"
    echo "  TOKEN_NAME         - Name of the token (required)"
    echo "  TOKEN_SYMBOL       - Symbol of the token (required)"
    echo "  TOKEN_DECIMALS     - Number of decimals (default: 18)"
    echo "  NETWORK            - Stellar network (default: testnet)"
    echo "  INITIAL_SUPPLY     - Initial token supply to mint (optional)"
    echo "  INITIAL_RECIPIENT  - Recipient of initial supply (optional)"
    echo "  CONTRACT_ID        - Existing contract ID (optional, for re-initialization)"
    echo ""
    echo "Example:"
    echo "  export SOURCE_ACCOUNT=GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
    echo "  export TOKEN_NAME=\"My Token\""
    echo "  export TOKEN_SYMBOL=\"MTK\""
    echo "  export INITIAL_SUPPLY=1000000000000000000000000"
    echo "  export INITIAL_RECIPIENT=GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
    echo "  $0"
    echo ""
    echo "Options:"
    echo "  -h, --help         Show this help message"
}

# Parse command line arguments
case "${1:-}" in
    -h|--help)
        show_help
        exit 0
        ;;
    *)
        main "$@"
        ;;
esac
