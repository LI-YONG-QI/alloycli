use alloy::{
    primitives::Address,
    providers::{Provider, ProviderBuilder},
    transports::http::reqwest::Url,
};
use clap::Args;

#[derive(Args)]
pub struct BalanceArgs {
    /// The address to get the balance of.
    address: Address,
}

// balance function

#[tokio::main(flavor = "current_thread")]
pub async fn get_balance(args: &BalanceArgs, rpc_url: Url) {
    let provider = ProviderBuilder::new().on_http(rpc_url);

    match provider.get_balance(args.address).await {
        Ok(balance) => println!("Balance: {:?}", balance),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
