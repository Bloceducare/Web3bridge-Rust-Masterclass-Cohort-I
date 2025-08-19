#!/bin/bash

# Employee Management System Deployment Script
# This script deploys the employee management contract and optionally initializes it

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Default values
NETWORK="testnet"
SOURCE_ACCOUNT="default"
CONTRACT_DIR="contracts/employee-management"

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if soroban CLI is installed
check_soroban() {
    if ! command -v soroban &> /dev/null; then
        print_error "Soroban CLI is not installed. Please install it first."
        echo "Visit: https://soroban.stellar.org/docs/getting-started/setup"
        exit 1
    fi
}

# Function to build the contract
build_contract() {
    print_status "Building employee management contract..."
    cd "$CONTRACT_DIR"
    
    # Build the contract
    cargo build --target wasm32-unknown-unknown --release
    
    # Optimize if possible
    if command -v soroban &> /dev/null; then
        print_status "Optimizing WASM..."
        soroban contract optimize --wasm target/wasm32-unknown-unknown/release/employee_management.wasm
    else
        print_warning "Soroban CLI not found. Skipping optimization."
    fi
    
    cd - > /dev/null
}

# Function to deploy the contract
deploy_contract() {
    print_status "Deploying contract to $NETWORK..."
    
    cd "$CONTRACT_DIR"
    
    CONTRACT_ID=$(soroban contract deploy \
        --wasm target/wasm32-unknown-unknown/release/employee_management.wasm \
        --source-account "$SOURCE_ACCOUNT" \
        --network "$NETWORK" 2>/dev/null)
    
    if [ $? -eq 0 ]; then
        print_status "Contract deployed successfully!"
        echo "Contract ID: $CONTRACT_ID"
        echo "$CONTRACT_ID" > contract_id.txt
        print_status "Contract ID saved to contract_id.txt"
    else
        print_error "Failed to deploy contract"
        exit 1
    fi
    
    cd - > /dev/null
    echo "$CONTRACT_ID"
}

# Function to initialize the contract
initialize_contract() {
    local contract_id=$1
    local admin_address=$2
    local institution_name=$3
    local token_contract=$4
    local base_salary=$5
    local promotion_interval=$6
    
    print_status "Initializing contract..."
    
    cd "$CONTRACT_DIR"
    
    soroban contract invoke \
        --id "$contract_id" \
        --source-account "$SOURCE_ACCOUNT" \
        --network "$NETWORK" \
        -- \
        initialize \
        --admin "$admin_address" \
        --institution_name "$institution_name" \
        --token_contract "$token_contract" \
        --base_salary_amount "$base_salary" \
        --min_promotion_interval "$promotion_interval"
    
    if [ $? -eq 0 ]; then
        print_status "Contract initialized successfully!"
    else
        print_error "Failed to initialize contract"
        exit 1
    fi
    
    cd - > /dev/null
}

# Function to show usage
show_usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  -n, --network NETWORK          Network to deploy to (testnet, futurenet, mainnet)"
    echo "  -s, --source-account ACCOUNT   Source account name"
    echo "  -i, --initialize               Initialize contract after deployment"
    echo "  --admin ADMIN_ADDRESS          Admin address for initialization"
    echo "  --institution-name NAME        Institution name for initialization"
    echo "  --token-contract ADDRESS       Token contract address for initialization"
    echo "  --base-salary AMOUNT           Base salary amount for initialization"
    echo "  --promotion-interval BLOCKS    Minimum blocks between promotions"
    echo "  -h, --help                     Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 --network testnet"
    echo "  $0 --initialize --admin GXXXXXXX --institution-name \"Tech Corp\" --token-contract CXXXXXXX --base-salary 1000000000 --promotion-interval 100"
}

# Parse command line arguments
INITIALIZE=false
ADMIN_ADDRESS=""
INSTITUTION_NAME=""
TOKEN_CONTRACT=""
BASE_SALARY=""
PROMOTION_INTERVAL=""

while [[ $# -gt 0 ]]; do
    case $1 in
        -n|--network)
            NETWORK="$2"
            shift 2
            ;;
        -s|--source-account)
            SOURCE_ACCOUNT="$2"
            shift 2
            ;;
        -i|--initialize)
            INITIALIZE=true
            shift
            ;;
        --admin)
            ADMIN_ADDRESS="$2"
            shift 2
            ;;
        --institution-name)
            INSTITUTION_NAME="$2"
            shift 2
            ;;
        --token-contract)
            TOKEN_CONTRACT="$2"
            shift 2
            ;;
        --base-salary)
            BASE_SALARY="$2"
            shift 2
            ;;
        --promotion-interval)
            PROMOTION_INTERVAL="$2"
            shift 2
            ;;
        -h|--help)
            show_usage
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            show_usage
            exit 1
            ;;
    esac
done

# Main execution
main() {
    print_status "Starting Employee Management System deployment..."
    
    # Check prerequisites
    check_soroban
    
    # Build contract
    build_contract
    
    # Deploy contract
    CONTRACT_ID=$(deploy_contract)
    
    # Initialize if requested
    if [ "$INITIALIZE" = true ]; then
        if [ -z "$ADMIN_ADDRESS" ] || [ -z "$INSTITUTION_NAME" ] || [ -z "$TOKEN_CONTRACT" ] || [ -z "$BASE_SALARY" ] || [ -z "$PROMOTION_INTERVAL" ]; then
            print_error "All initialization parameters are required when using --initialize"
            echo "Required: --admin, --institution-name, --token-contract, --base-salary, --promotion-interval"
            exit 1
        fi
        
        initialize_contract "$CONTRACT_ID" "$ADMIN_ADDRESS" "$INSTITUTION_NAME" "$TOKEN_CONTRACT" "$BASE_SALARY" "$PROMOTION_INTERVAL"
    fi
    
    print_status "Deployment completed successfully!"
    echo ""
    echo "Contract ID: $CONTRACT_ID"
    echo "Network: $NETWORK"
    echo "Source Account: $SOURCE_ACCOUNT"
    
    if [ "$INITIALIZE" = true ]; then
        echo "Institution: $INSTITUTION_NAME"
        echo "Admin: $ADMIN_ADDRESS"
        echo "Token Contract: $TOKEN_CONTRACT"
    fi
}

# Run main function
main
