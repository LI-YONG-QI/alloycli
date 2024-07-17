use alloy::{
    network::EthereumWallet,
    primitives::{Address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    transports::http::reqwest::Url,
};
use clap::Args;

#[derive(Args)]
pub struct TransferArgs {
    /// Private key of sender
    signer: PrivateKeySigner,

    /// Address of recipient
    to: Address,

    /// Amount to transfer
    #[arg(short, long, default_value = "100")]
    amount: U256,
}

#[tokio::main(flavor = "current_thread")]
pub async fn transfer(args: &TransferArgs, rpc_url: Url) {
    let wallet = EthereumWallet::from(args.signer.clone());
    // Sepolia provider
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(rpc_url);

    let tx = TransactionRequest::default().to(args.to).value(args.amount);

    // Send the transaction and listen for the transaction to be included.
    match provider.send_transaction(tx).await {
        Ok(receipt) => match receipt.watch().await {
            Ok(tx_hash) => {
                println!("Transaction successful: {:?}", tx_hash);
            }
            Err(e) => {
                println!("Transaction not confirmed: {:?}", e);
            }
        },
        Err(e) => {
            println!("Transaction send failed: {:?}", e);
        }
    }
}
