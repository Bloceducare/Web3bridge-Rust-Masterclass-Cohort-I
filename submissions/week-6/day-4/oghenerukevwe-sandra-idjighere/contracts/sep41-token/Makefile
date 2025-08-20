# SEP-41 Token Contract Makefile

# Default target
default: build

# Build the contract
build:
	cargo build --target wasm32-unknown-unknown --release
	@echo "Contract built successfully!"

# Run tests
test:
	cargo test
	@echo "All tests passed!"

# Run tests with output
test-verbose:
	cargo test -- --nocapture
	@echo "All tests passed!"

# Clean build artifacts
clean:
	cargo clean
	@echo "Build artifacts cleaned!"

# Check code formatting
fmt-check:
	cargo fmt --all -- --check

# Format code
fmt:
	cargo fmt --all

# Run clippy lints
clippy:
	cargo clippy --all-targets --all-features -- -D warnings

# Run all checks (format, clippy, test)
check: fmt-check clippy test
	@echo "All checks passed!"

# Build optimized version
build-optimized: clean
	RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
	@echo "Optimized contract built successfully!"

# Get contract size
size: build
	@echo "Contract size:"
	@ls -lh target/wasm32-unknown-unknown/release/sep41_token.wasm

# Deploy to testnet (requires CONTRACT_ID and SOURCE_ACCOUNT env vars)
deploy-testnet:
	@if [ -z "$(SOURCE_ACCOUNT)" ]; then \
		echo "Error: SOURCE_ACCOUNT environment variable is required"; \
		exit 1; \
	fi
	soroban contract deploy \
		--wasm target/wasm32-unknown-unknown/release/sep41_token.wasm \
		--source-account $(SOURCE_ACCOUNT) \
		--network testnet

# Initialize contract (requires CONTRACT_ID, SOURCE_ACCOUNT, TOKEN_NAME, TOKEN_SYMBOL env vars)
initialize:
	@if [ -z "$(CONTRACT_ID)" ] || [ -z "$(SOURCE_ACCOUNT)" ] || [ -z "$(TOKEN_NAME)" ] || [ -z "$(TOKEN_SYMBOL)" ]; then \
		echo "Error: CONTRACT_ID, SOURCE_ACCOUNT, TOKEN_NAME, and TOKEN_SYMBOL environment variables are required"; \
		exit 1; \
	fi
	soroban contract invoke \
		--id $(CONTRACT_ID) \
		--source-account $(SOURCE_ACCOUNT) \
		--network testnet \
		-- initialize \
		--admin $(SOURCE_ACCOUNT) \
		--decimal 18 \
		--name "$(TOKEN_NAME)" \
		--symbol "$(TOKEN_SYMBOL)"

# Mint tokens (requires CONTRACT_ID, SOURCE_ACCOUNT, TO_ADDRESS, AMOUNT env vars)
mint:
	@if [ -z "$(CONTRACT_ID)" ] || [ -z "$(SOURCE_ACCOUNT)" ] || [ -z "$(TO_ADDRESS)" ] || [ -z "$(AMOUNT)" ]; then \
		echo "Error: CONTRACT_ID, SOURCE_ACCOUNT, TO_ADDRESS, and AMOUNT environment variables are required"; \
		exit 1; \
	fi
	soroban contract invoke \
		--id $(CONTRACT_ID) \
		--source-account $(SOURCE_ACCOUNT) \
		--network testnet \
		-- mint \
		--to $(TO_ADDRESS) \
		--amount $(AMOUNT)

# Get token info
info:
	@if [ -z "$(CONTRACT_ID)" ]; then \
		echo "Error: CONTRACT_ID environment variable is required"; \
		exit 1; \
	fi
	@echo "Token Name:"
	@soroban contract invoke --id $(CONTRACT_ID) --network testnet -- name
	@echo "Token Symbol:"
	@soroban contract invoke --id $(CONTRACT_ID) --network testnet -- symbol
	@echo "Token Decimals:"
	@soroban contract invoke --id $(CONTRACT_ID) --network testnet -- decimals

# Get balance (requires CONTRACT_ID and ADDRESS env vars)
balance:
	@if [ -z "$(CONTRACT_ID)" ] || [ -z "$(ADDRESS)" ]; then \
		echo "Error: CONTRACT_ID and ADDRESS environment variables are required"; \
		exit 1; \
	fi
	soroban contract invoke \
		--id $(CONTRACT_ID) \
		--network testnet \
		-- balance \
		--id $(ADDRESS)

# Help target
help:
	@echo "Available targets:"
	@echo "  build           - Build the contract"
	@echo "  test            - Run tests"
	@echo "  test-verbose    - Run tests with output"
	@echo "  clean           - Clean build artifacts"
	@echo "  fmt             - Format code"
	@echo "  fmt-check       - Check code formatting"
	@echo "  clippy          - Run clippy lints"
	@echo "  check           - Run all checks (format, clippy, test)"
	@echo "  build-optimized - Build optimized version"
	@echo "  size            - Show contract size"
	@echo "  deploy-testnet  - Deploy to testnet (requires SOURCE_ACCOUNT)"
	@echo "  initialize      - Initialize contract (requires CONTRACT_ID, SOURCE_ACCOUNT, TOKEN_NAME, TOKEN_SYMBOL)"
	@echo "  mint            - Mint tokens (requires CONTRACT_ID, SOURCE_ACCOUNT, TO_ADDRESS, AMOUNT)"
	@echo "  info            - Get token info (requires CONTRACT_ID)"
	@echo "  balance         - Get balance (requires CONTRACT_ID, ADDRESS)"
	@echo "  help            - Show this help"

.PHONY: default build test test-verbose clean fmt fmt-check clippy check build-optimized size deploy-testnet initialize mint info balance help
