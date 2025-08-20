#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;

use alloc::vec::Vec;

use alloy_primitives::{aliases::U24, Address, FixedBytes, U256};
use alloy_sol_types::{sol, SolValue};
/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{crypto::keccak, prelude::*};

// Define a minimal ERC20 interface, so our contract can transfer ERC-20 tokens when it needs to
sol_interface! {
    interface IERC20 {
        function transferFrom(address from, address to, uint256 value) external returns (bool);
        function transfer(address to, uint256 value) external returns (bool);
    }
}

// Define some persistent storage using the Solidity ABI
// `StylusSwap` will be the entrypoint
sol_storage! {
    #[entrypoint]
    pub struct StylusSwap {
        // Mapping of all pools created within the DEX
        mapping(bytes32 => Pool) pools;
    }

    // A pool is a pair of tokens and a fee which together uniquely identify the pool
    // The struct contains additional data that is used to track the pool's state
    pub struct Pool {
        address token0;
        address token1;
        uint24 fee;
        uint256 liquidity;
        uint256 balance0;
        uint256 balance1;
        mapping(bytes32 => Position) positions;
    }

    // A position is a user's share of the pool's liquidity
    pub struct Position {
        address owner;
        uint256 liquidity;
    }
}

sol! {
    // Thrown when a pool with the same ID already exists
    error PoolAlreadyExists(bytes32 pool_id);
    // Thrown when an action is attempted on a pool that does not exist
    error PoolDoesNotExist(bytes32 pool_id);
    // Thrown when a user attempts to mint liquidity without providing enough tokens
    error InsufficientLiquidityMinted();
    // Thrown when a user attempts to swap with an insufficient amount of tokens
    error InsufficientAmount();
    // Thrown when a user attempts to remove liquidity more than their share of the pool
    error InsufficientLiquidityOwned();
    // Thrown when a token transfer fails
    error FailedOrInsufficientTokenTransfer(address token, address from, address to, uint256 amount);
    // Thrown when the contract fails to refund leftover ETH to the user
    error FailedToReturnExtraEth(address to, uint256 amount);
    // Thrown when the user's swap exceeds their slippage tolerance
    error TooMuchSlippage();

    // Emitted when a pool is created
    event PoolCreated(bytes32 pool_id, address token0, address token1, uint24 fee);
    // Emitted when liquidity is minted
    event LiquidityMinted(bytes32 pool_id, address owner, uint256 liquidity);
    // Emitted when liquidity is burned
    event LiquidityBurned(bytes32 pool_id, address owner, uint256 liquidity);
    // Emitted when a swap is executed
    event Swap(bytes32 pool_id, address user, uint256 input_amount, uint256 output_amount_after_fees, uint256 fees, bool zero_for_one);

}

// Define the Rust-equivalent of the Solidity errors
#[derive(SolidityError)]
pub enum StylusSwapError {
    PoolAlreadyExists(PoolAlreadyExists),
    PoolDoesNotExist(PoolDoesNotExist),
    InsufficientAmount(InsufficientAmount),
    InsufficientLiquidityMinted(InsufficientLiquidityMinted),
    InsufficientLiquidityOwned(InsufficientLiquidityOwned),
    FailedOrInsufficientTokenTransfer(FailedOrInsufficientTokenTransfer),
    FailedToReturnExtraEth(FailedToReturnExtraEth),
    TooMuchSlippage(TooMuchSlippage),
}

impl StylusSwap {
    // TODO: Implementation block for private functions in the contract
}

#[public]
impl StylusSwap {
    pub fn create_pool(
        &mut self,
        token_a: Address,
        token_b: Address,
        fee: U24,
    ) -> Result<(), StylusSwapError> {
        todo!()
    }

    #[payable]
    pub fn add_liquidity(
        &mut self,
        pool_id: FixedBytes<32>,
        amount_0_desired: U256,
        amount_1_desired: U256,
        amount_0_min: U256,
        amount_1_min: U256,
    ) -> Result<(), StylusSwapError> {
        todo!()
    }

    pub fn remove_liquidity(
        &mut self,
        pool_id: FixedBytes<32>,
        liquidity_to_remove: U256,
    ) -> Result<(), StylusSwapError> {
        todo!()
    }

    #[payable]
    pub fn swap(
        &mut self,
        pool_id: FixedBytes<32>,
        input_amount: U256,
        min_output_amount: U256,
        zero_for_one: bool,
    ) -> Result<(), StylusSwapError> {
        todo!()
    }
}