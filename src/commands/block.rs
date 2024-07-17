use alloy::{
    providers::{Provider, ProviderBuilder},
    transports::http::reqwest::Url,
};
use eyre::Result;

#[tokio::main(flavor = "current_thread")]
pub async fn get_number(rpc_url: Url) -> Result<()> {
    // Create a provider with the HTTP transport using the `reqwest` crate.
    let provider = ProviderBuilder::new().on_http(rpc_url);

    // Get latest block number.
    let latest_block = provider.get_block_number().await?;

    println!("Latest block number: {latest_block}");

    Ok(())
}
