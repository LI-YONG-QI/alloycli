use alloy::{
    providers::{Provider, ProviderBuilder},
    transports::http::reqwest::Url,
};

#[tokio::main(flavor = "current_thread")]
pub async fn get_number(rpc_url: Url) {
    // Create a provider with the HTTP transport using the `reqwest` crate.
    let provider = ProviderBuilder::new().on_http(rpc_url);

    // Get latest block number.

    match provider.get_block_number().await {
        Ok(block_number) => println!("Current block number: {}", block_number),
        Err(e) => eprintln!("Error: {}", e),
    }
}
