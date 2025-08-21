//! ReentrancyGuard - Your existing implementation with additional utilities
//! 
//! This builds on your existing ReentrancyGuard to provide additional helper functions
//! and demonstrate usage patterns.

// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;

use alloc::vec::Vec;
use alloy_sol_types::sol;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::U256, prelude::*};

sol! {
    /*
     * @dev Unauthorized reentrant call
     */
    #[derive(Debug)]
    error ReentrancyGuardReentrantCall();
}

#[derive(Debug, SolidityError)]
pub enum ReentrancyGuardErrors {
    ReentrancyGuardReentrantCall(ReentrancyGuardReentrantCall),
}

// Make constants public so they're considered part of the API (no dead_code warning)
pub const NOT_ENTERED: u128 = 1;
pub const ENTERED: u128 = 2;

// Define some persistent storage using the Solidity ABI.
// `ReentrancyGuard` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct ReentrancyGuard {
        uint256 _status;
    }
}

#[public]
/// Declare that `ReentrancyGuard` is a contract with the following private methods.
impl ReentrancyGuard {
    #[constructor] 
    pub fn constructor(&mut self) {
        self._status.set(U256::from(NOT_ENTERED));
    }

    /// Internal function to check and set the guard before entering a protected function
    fn _non_reentrant_before(&mut self) -> Result<(), ReentrancyGuardErrors> {
        // Compare against the entered sentinel value read from storage
        if self._status.get() == U256::from(ENTERED) {
            // Construct the Solidity error value (struct-like)
            return Err(ReentrancyGuardErrors::ReentrancyGuardReentrantCall(
                ReentrancyGuardReentrantCall {}
            ));
        }
        // Set status to the `entered` sentinel value
        self._status.set(U256::from(ENTERED));
        Ok(())
    }

    /// Internal function to reset the guard after exiting a protected function
    fn _non_reentrant_after(&mut self) {
        // Restore sentinel to NOT_ENTERED from storage
        self._status.set(U256::from(NOT_ENTERED));
    }

    /// Check if the guard is currently in the "entered" state
    fn _reentrancy_guard_entered(&self) -> bool {
        self._status.get() == U256::from(ENTERED)
    }

    /// Public view function to check if reentrancy guard is active
    /// Useful for debugging and testing
    pub fn is_guard_active(&self) -> bool {
        self._reentrancy_guard_entered()
    }

    /// Get the current status value (for debugging)
    pub fn get_status(&self) -> U256 {
        self._status.get()
    }
}

/// Macro to apply reentrancy protection to a code block
/// Usage: non_reentrant!(self, { /* your protected code */ })
#[macro_export]
macro_rules! non_reentrant {
    ($self:expr, $body:block) => {{
        (|| -> Result<_, ReentrancyGuardErrors> {
            $self._non_reentrant_before()?;
            let result = (|| $body)();
            $self._non_reentrant_after();
            result
        })()
    }};
}

/// Alternative macro that handles errors more gracefully
/// This version ensures the guard is always reset, even if an error occurs
#[macro_export]
macro_rules! non_reentrant_safe {
    ($self:expr, $body:block) => {{
        $self._non_reentrant_before()?;
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| $body));
        $self._non_reentrant_after();
        match result {
            Ok(value) => value,
            Err(_) => panic!("Function panicked within reentrancy guard"),
        }
    }};
}

#[cfg(test)]
mod test {
    use super::*;
    use stylus_sdk::testing::*;

    #[test]
    fn test_reentrancy_guard_basic() {
         let vm = TestVM::default();
        let mut contract = ReentrancyGuard::from(&vm);
        // initialize the contract

        // initial status should be NOT_ENTERED after constructor
        // we assert that the guard reports "not entered"
        assert_eq!(false, contract._reentrancy_guard_entered());
        assert_eq!(U256::from(0), contract._status.get());

        // call the non-reentrant function
        let result = contract._non_reentrant_before();
        assert!(result.is_ok());
        assert_eq!(true, contract._reentrancy_guard_entered());
        assert_eq!(U256::from(ENTERED), contract._status.get());

        // call the non-reentrant function again
        let result1 = contract._non_reentrant_before();
        assert!(result1.is_err());
        assert_eq!(true, contract._reentrancy_guard_entered());
        assert_eq!(U256::from(ENTERED), contract._status.get());

        contract._non_reentrant_after();
        assert_eq!(false, contract._reentrancy_guard_entered());
        assert_eq!(U256::from(NOT_ENTERED), contract._status.get());
    }

    #[test]
    fn test_reentrancy_guard_macro() {
        let vm = TestVM::default();
        let mut contract = ReentrancyGuard::from(&vm);

        // Test successful execution with macro
        let result: Result<u32, ReentrancyGuardErrors> = non_reentrant!(contract, {
            // Verify we're in the ENTERED state during execution
            assert_eq!(true, contract._reentrancy_guard_entered());
            Ok(42u32)
        });

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        
        // Verify state is reset after execution
        assert_eq!(false, contract._reentrancy_guard_entered());
    }

    #[test]
    fn test_nested_reentrancy_with_macro() {
        let vm = TestVM::default();
        let mut contract = ReentrancyGuard::from(&vm);

        let result: Result<&str, ReentrancyGuardErrors> = non_reentrant!(contract, {
            // Try to make a nested call - this should fail
            let nested_result: Result<&str, ReentrancyGuardErrors> = non_reentrant!(contract, {
                Ok("This should not execute")
            });
            
            assert!(nested_result.is_err());
            Ok("Outer call completed")
        });

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Outer call completed");
        assert_eq!(false, contract._reentrancy_guard_entered());
    }

    #[test]
    fn test_public_view_functions() {
        let vm = TestVM::default();
        let mut contract = ReentrancyGuard::from(&vm);

        // Test public view functions
        assert_eq!(false, contract.is_guard_active());
        assert_eq!(U256::from(0), contract.get_status());

        // Enter protected state
        contract._non_reentrant_before().unwrap();
        
        assert_eq!(true, contract.is_guard_active());
        assert_eq!(U256::from(ENTERED), contract.get_status());

        // Exit protected state
        contract._non_reentrant_after();
        
        assert_eq!(false, contract.is_guard_active());
        assert_eq!(U256::from(NOT_ENTERED), contract.get_status());
    }
}