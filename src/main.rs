use alloy::{
    providers::{Provider, ProviderBuilder},
    transports::http::reqwest::Url,
};
use clap::{Args, Parser, Subcommand};
use eyre::Result;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Block(AddArgs),
}

#[derive(Args)]
struct AddArgs {
    #[arg(short, long)]
    mainnet: bool,
}

#[tokio::main(flavor = "current_thread")]
async fn get_number(rpc_url: Url) -> Result<()> {
    // Create a provider with the HTTP transport using the `reqwest` crate.
    let provider = ProviderBuilder::new().on_http(rpc_url);

    // Get latest block number.
    let latest_block = provider.get_block_number().await?;

    println!("Latest block number: {latest_block}");

    Ok(())
}

fn main() {
    // Set up the HTTP transport which is consumed by the RPC client.
    let cli = Cli::parse();

    let url = Url::parse("https://eth.merkle.io").unwrap();
    let url_sepolia = Url::parse("https://eth-sepolia.g.alchemy.com/v2/demo").unwrap();

    match &cli.commands {
        Commands::Block(options) => {
            if options.mainnet {
                let _ = get_number(url);
            } else {
                let _ = get_number(url_sepolia);
            }
        }
    }
}
