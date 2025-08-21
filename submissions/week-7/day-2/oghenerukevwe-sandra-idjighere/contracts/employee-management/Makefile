# Employee Management Contract Makefile

# Default target
.PHONY: all
all: build

# Build the contract
.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

# Run tests
.PHONY: test
test:
	cargo test

# Check code
.PHONY: check
check:
	cargo check

# Clean build artifacts
.PHONY: clean
clean:
	cargo clean

# Build optimized WASM
.PHONY: build-optimized
build-optimized: build
	@if command -v soroban >/dev/null 2>&1; then \
		soroban contract optimize --wasm target/wasm32-unknown-unknown/release/employee_management.wasm; \
		echo "Optimized WASM built successfully"; \
	else \
		echo "Soroban CLI not found. Please install it to optimize WASM."; \
	fi

# Deploy to testnet (requires soroban CLI and network configuration)
.PHONY: deploy-testnet
deploy-testnet: build-optimized
	@if command -v soroban >/dev/null 2>&1; then \
		echo "Deploying to testnet..."; \
		soroban contract deploy \
			--wasm target/wasm32-unknown-unknown/release/employee_management.wasm \
			--source-account default \
			--network testnet; \
	else \
		echo "Soroban CLI not found. Please install it to deploy."; \
	fi

# Deploy to futurenet
.PHONY: deploy-futurenet
deploy-futurenet: build-optimized
	@if command -v soroban >/dev/null 2>&1; then \
		echo "Deploying to futurenet..."; \
		soroban contract deploy \
			--wasm target/wasm32-unknown-unknown/release/employee_management.wasm \
			--source-account default \
			--network futurenet; \
	else \
		echo "Soroban CLI not found. Please install it to deploy."; \
	fi

# Format code
.PHONY: fmt
fmt:
	cargo fmt

# Lint code
.PHONY: clippy
clippy:
	cargo clippy -- -D warnings

# Run all checks
.PHONY: ci
ci: fmt clippy test build

# Help
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  build           - Build the contract"
	@echo "  test            - Run tests"
	@echo "  check           - Check code without building"
	@echo "  clean           - Clean build artifacts"
	@echo "  build-optimized - Build optimized WASM"
	@echo "  deploy-testnet  - Deploy to Stellar testnet"
	@echo "  deploy-futurenet- Deploy to Stellar futurenet"
	@echo "  fmt             - Format code"
	@echo "  clippy          - Run clippy linter"
	@echo "  ci              - Run all checks (fmt, clippy, test, build)"
	@echo "  help            - Show this help"
