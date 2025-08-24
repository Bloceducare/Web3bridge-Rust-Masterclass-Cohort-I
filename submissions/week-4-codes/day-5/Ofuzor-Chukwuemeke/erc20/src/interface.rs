
mod token{
    #![allow(missing_docs)]
    #![cfg_attr(coverage_nightly,coverage(off))]
    use alloc::vec;

    use stylus_sdk::prelude::sol_interface;

    sol_interface!{
        interface Erc20Interface{
            function totalSupply() external view returns(uint256);
            function balanceOf(address account) external view returns (uint256);
            function transfer(address recipient, uint256 amount) external returns (bool);
            function transferFrom(address sender, address recipient, uint256 amount) external returns (bool);
            function approve(address spender, uint256 amount) external returns (bool);
            function allowance(address owner, address spender) external view returns (uint256);
        }
    }

    sol_interface! {
        interface IErc20MetadataInterface{
            function name() external view returns(string);

            function symbol() external view returns(string);

            function decimals() external view returns(uint8);
        }
    }
}