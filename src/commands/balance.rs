use alloy::{
    primitives::Address,
    providers::{Provider, ProviderBuilder},
    transports::http::reqwest::Url,
};
use clap::Args;
use eyre::{Ok, Result};

#[derive(Args)]
pub struct BalanceArgs {
    pub address: Address,
}

#[tokio::main(flavor = "current_thread")]
pub async fn get_balance(options: &BalanceArgs) -> Result<()> {
    let rpc_url: Url = "https://eth.merkle.io".parse().expect("Error");
    let provider = ProviderBuilder::new().on_http(rpc_url);

    let balance = provider.get_balance(options.address).await?;
    println!("Balance: {:?}", balance);

    Ok(())
}
