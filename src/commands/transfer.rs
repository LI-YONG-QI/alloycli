use alloy::{
    network::{EthereumWallet, NetworkWallet},
    primitives::address,
    providers::{fillers::WalletFiller, Provider, ProviderBuilder},
    signers::local::PrivateKeySigner,
    transports::http::reqwest::Url,
};
use clap::Args;
use eyre::Result;

#[derive(Args)]
pub struct TransferArgs {
    signer: PrivateKeySigner,
}

#[tokio::main(flavor = "current_thread")]
pub async fn transfer(options: &TransferArgs) -> Result<()> {
    let wallet = EthereumWallet::from(options.signer.clone());
    println!("{:?}", wallet);

    let rpc_url: Url = match "https://eth.merkle.io".parse() {
        Ok(n) => n,
        Err(_e) => panic!("Error"),
    };

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(rpc_url);

    // let balance = provider
    //     .get_balance(address!("056703bb4E0866909E1767D9b079237D1C44962f"))
    //     .await?;

    println!("get account");

    let poller = provider.watch_blocks().await?;

    //println!("{:?}", wallet.default_signer_address());

    Ok(())
}
