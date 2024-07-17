use clap::Parser;
use commands::{balance::get_balance, block::get_number, transfer::transfer, Commands};
use factory::get_url;

pub mod commands;
pub mod factory;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]

struct Cli {
    /// Set network to mainnet, default is testnet.
    #[arg(short, long)]
    mainnet: bool,

    #[arg(short, long, default_value_t = 100000u32)]
    amount: u32,

    #[command(subcommand)]
    commands: Commands,
}

fn main() {
    // Set up the HTTP transport which is consumed by the RPC client.
    let cli = Cli::parse();
    let rpc_url = get_url(true);

    match &cli.commands {
        Commands::Block => get_number(rpc_url),
        Commands::Balance(args) => get_balance(args, rpc_url),
        Commands::Transfer(args) => transfer(args, rpc_url),
    }
}
