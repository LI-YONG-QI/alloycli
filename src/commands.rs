pub mod balance;
pub mod block;
pub mod transfer;

use balance::BalanceArgs;
use clap::Subcommand;
use transfer::TransferArgs;

#[derive(Subcommand)]
pub enum Commands {
    /// Get block number
    Block,

    /// Transfer ETH
    Transfer(TransferArgs),

    /// Get balance of an address
    Balance(BalanceArgs),
}
