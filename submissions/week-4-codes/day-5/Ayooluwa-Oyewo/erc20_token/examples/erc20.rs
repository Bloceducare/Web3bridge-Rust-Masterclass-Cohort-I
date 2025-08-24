//! Example: Interacting with a deployed ERC20 contract on Stylus
//! Uses ethers-rs with an ERC20 ABI.

use dotenv::dotenv;
use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, U256},
};
use eyre::eyre;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::Arc;

/// Your private key file path.
const PRIV_KEY_PATH: &str = "PRIV_KEY_PATH";

/// Stylus RPC endpoint url.
const RPC_URL: &str = "RPC_URL";

/// Deployed ERC20 contract address.
const ERC20_CONTRACT_ADDRESS: &str = "ERC20_CONTRACT_ADDRESS";

// Generate Rust bindings for ERC20 ABI
abigen!(
    ERC20,
    r#"[
        function name() external view returns (string)
        function symbol() external view returns (string)
        function decimals() external view returns (uint8)
        function totalSupply() external view returns (uint256)
        function balanceOf(address) external view returns (uint256)
        function transfer(address to, uint256 amount) external returns (bool)
        function approve(address spender, uint256 amount) external returns (bool)
        function allowance(address owner, address spender) external view returns (uint256)
    ]"#
);

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv().ok();

    // Load env vars
    let priv_key_path =
        std::env::var(PRIV_KEY_PATH).map_err(|_| eyre!("No {} env var set", PRIV_KEY_PATH))?;
    let rpc_url = std::env::var(RPC_URL).map_err(|_| eyre!("No {} env var set", RPC_URL))?;
    let contract_address = std::env::var(ERC20_CONTRACT_ADDRESS)
        .map_err(|_| eyre!("No {} env var set", ERC20_CONTRACT_ADDRESS))?;

    // Setup provider & wallet
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let privkey = read_secret_from_file(&priv_key_path)?;
    let wallet = LocalWallet::from_str(&privkey)?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.clone().with_chain_id(chain_id),
    ));

    let address: Address = contract_address.parse()?;
    let erc20 = ERC20::new(address, client);

    // Query token info
    let name = erc20.name().call().await?;
    let symbol = erc20.symbol().call().await?;
    let decimals = erc20.decimals().call().await?;
    let total_supply: U256 = erc20.total_supply().call().await?;

    println!("Token: {} ({})", name, symbol);
    println!("Decimals: {}", decimals);
    println!("Total Supply: {}", total_supply);

    // Query balance of our wallet
    let my_address = wallet.address();
    let balance: U256 = erc20.balance_of(my_address).call().await?;
    println!("My balance: {}", balance);

    // Example transfer
    
    let recipient: Address = "RECEPIENT ADDRESS".parse()?;
    let tx = erc20.transfer(recipient, U256::from(1000u64));
    let receipt = tx.send().await?.await?;
    println!("Transfer receipt: {:?}", receipt);

    Ok(())
}

fn read_secret_from_file(fpath: &str) -> eyre::Result<String> {
    let f = std::fs::File::open(fpath)?;
    let mut buf_reader = BufReader::new(f);
    let mut secret = String::new();
    buf_reader.read_line(&mut secret)?;
    Ok(secret.trim().to_string())
}
