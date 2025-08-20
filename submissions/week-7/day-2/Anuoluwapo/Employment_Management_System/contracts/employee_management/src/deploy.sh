#!/bin/bash

# Employee Management System Deployment Script
# Make sure you have soroban CLI installed and configured

echo " Starting Employee Management System Deployment..."


NETWORK="testnet"
IDENTITY="default" 

echo "Building contracts..."
cd contracts/Sep_41
soroban contract build
cd ../employee_management
soroban contract build
cd ../..

echo " Deploying SEP-41 Token Contract..."
TOKEN_CONTRACT_ID=$(soroban contract deploy \
    --wasm contracts/Sep_41/target/wasm32-unknown-unknown/release/sep_41.wasm \
    --network $NETWORK \
    --source $IDENTITY)

echo "Token Contract deployed at: $TOKEN_CONTRACT_ID"

echo " Deploying Employee Management Contract..."
EMPLOYEE_CONTRACT_ID=$(soroban contract deploy \
    --wasm contracts/employee_management/target/wasm32-unknown-unknown/release/employee_management.wasm \
    --network $NETWORK \
    --source $IDENTITY)

echo "Employee Management Contract deployed at: $EMPLOYEE_CONTRACT_ID"

echo "🔧 Initializing Token Contract..."
ADMIN_ADDRESS=$(soroban keys address $IDENTITY)

soroban contract invoke \
    --id $TOKEN_CONTRACT_ID \
    --network $NETWORK \
    --source $IDENTITY \
    -- initialize \
    --admin $ADMIN_ADDRESS \
    --name "Company Token" \
    --symbol "COMP" \
    --decimals 7 \
    --total_supply 1000000000000000


echo " Initializing Employee Management Contract..."
soroban contract invoke \
    --id $EMPLOYEE_CONTRACT_ID \
    --network $NETWORK \
    --source $IDENTITY \
    -- initialize \
    --admin $ADMIN_ADDRESS \
    --token_contract $TOKEN_CONTRACT_ID

echo "  Deployment completed!"
echo "  Contract Addresses:"
echo "  Token Contract: $TOKEN_CONTRACT_ID"
echo "  Employee Management: $EMPLOYEE_CONTRACT_ID"
echo "  Admin: $ADMIN_ADDRESS"

cat > contract_addresses.txt << EOF
TOKEN_CONTRACT_ID=$TOKEN_CONTRACT_ID
EMPLOYEE_CONTRACT_ID=$EMPLOYEE_CONTRACT_ID
ADMIN_ADDRESS=$ADMIN_ADDRESS
NETWORK=$NETWORK
EOF

echo " Contract addresses saved to contract_addresses.txt"

echo " Would you like to register a sample institution? (y/n)"
read -r REGISTER_SAMPLE

if [ "$REGISTER_SAMPLE" = "y" ] || [ "$REGISTER_SAMPLE" = "Y" ]; then
    INSTITUTION_ADDRESS=$(soroban keys address $IDENTITY)
    
    soroban contract invoke \
        --id $EMPLOYEE_CONTRACT_ID \
        --network $NETWORK \
        --source $IDENTITY \
        -- register_institution \
        --institution_address $INSTITUTION_ADDRESS \
        --name "Sample Tech Corp" \
        --admin $ADMIN_ADDRESS
    
    echo "✅ Sample institution registered!"
    echo "   Institution Address: $INSTITUTION_ADDRESS"
fi

echo "🎉 Setup complete! You can now interact with your Employee Management System."