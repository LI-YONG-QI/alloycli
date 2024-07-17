use clap::Parser;
use commands::{balance::get_balance, block::get_number, transfer::transfer, Commands};

pub mod commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

fn main() {
    // Set up the HTTP transport which is consumed by the RPC client.
    let cli = Cli::parse();

    match &cli.commands {
        Commands::Block(options) => {
            let _ = get_number(options);
        }
        Commands::Transfer(options) => {
            let _ = transfer(options);
        }
        Commands::Balance(options) => {
            let _ = get_balance(options);
        }
    }
}
