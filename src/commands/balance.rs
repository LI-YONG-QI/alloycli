use alloy::{
    primitives::Address,
    providers::{Provider, ProviderBuilder},
    transports::http::reqwest::Url,
};
use clap::Args;
use eyre::{Ok, Result};

#[derive(Args)]
pub struct BalanceArgs {
    /// The address to get the balance of.
    address: Address,
}

#[tokio::main(flavor = "current_thread")]
pub async fn get_balance(args: &BalanceArgs, rpc_url: Url) -> Result<()> {
    let provider = ProviderBuilder::new().on_http(rpc_url);

    let balance = provider.get_balance(args.address).await?;
    println!("Balance: {:?}", balance);

    Ok(())
}
