use alloy::transports::http::reqwest::Url;

pub fn get_url(is_mainnet: bool) -> Url {
    let rpc_url = if is_mainnet {
        Url::parse("https://eth.merkle.io").unwrap()
    } else {
        Url::parse("https://rpc.sepolia.org").unwrap()
    };

    // Create a provider with the HTTP transport using the `reqwest` crate.
    rpc_url
}
