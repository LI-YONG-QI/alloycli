pub mod balance;
pub mod block;
pub mod transfer;

use balance::BalanceArgs;
use block::BlockArgs;
use clap::Subcommand;
use transfer::TransferArgs; // Import the TransferArgs struct

#[derive(Subcommand)]
pub enum Commands {
    Block(BlockArgs),
    Transfer(TransferArgs), // Update to use structured TransferArgs
    Balance(BalanceArgs),
}
