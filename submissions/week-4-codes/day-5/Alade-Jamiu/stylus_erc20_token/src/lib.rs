mod erc20;

use stylus_sdk::prelude::*;

sol_interface! {
    interface IERC20 {
        function name() external view returns (string);
        function symbol() external view returns (string);
        function decimals() external view returns (uint8);
        function totalSupply() external view returns (uint256);
        function balanceOf(address account) external view returns (uint256);
        function transfer(address to, uint256 value) external returns (bool);
        function allowance(address owner, address spender) external view returns (uint256);
        function approve(address spender, uint256 value) external returns (bool);
        function transferFrom(address from, address to, uint256 value) external returns (bool);
    }
}

#[external]
#[inherit(IERC20)]
struct ERC20Token;

impl ERC20Token {
    #[constructor]
    fn constructor(name: String, symbol: String, decimals: u8, initial_supply: U256) {
        erc20::ERC20::new(name, symbol, decimals, initial_supply);
    }
}