use alloy::{
    providers::{Provider, ProviderBuilder},
    transports::http::reqwest::Url,
};
use clap::Args;
use eyre::Result;

//Subcommand `block` args

#[derive(Args)]
pub struct BlockArgs {
    #[arg(short, long)]
    mainnet: bool,
}

#[tokio::main(flavor = "current_thread")]
pub async fn get_number(option: &BlockArgs) -> Result<()> {
    let rpc_url = if option.mainnet {
        Url::parse("https://eth.merkle.io")?
    } else {
        Url::parse("https://eth-sepolia.g.alchemy.com/v2/demo")?
    };

    // Create a provider with the HTTP transport using the `reqwest` crate.
    let provider = ProviderBuilder::new().on_http(rpc_url);

    // Get latest block number.
    let latest_block = provider.get_block_number().await?;

    println!("Latest block number: {latest_block}");

    Ok(())
}
