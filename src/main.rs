use clap::Parser;
use commands::{balance::get_balance, block::get_number, transfer::transfer, Commands};
use factory::get_url;

pub mod commands;
pub mod factory;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]

struct Cli {
    /// Set network is mainnet [default: sepolia]
    #[arg(short, long)]
    mainnet: bool,

    #[command(subcommand)]
    commands: Commands,
}

fn main() {
    // Set up the HTTP transport which is consumed by the RPC client.
    let cli = Cli::parse();

    let rpc_url = get_url(cli.mainnet);

    match &cli.commands {
        Commands::Block => {
            let _ = get_number(rpc_url);
        }
        Commands::Transfer(args) => {
            let _ = transfer(args, rpc_url);
        }
        Commands::Balance(args) => {
            let _ = get_balance(args, rpc_url);
        }
    }
}
