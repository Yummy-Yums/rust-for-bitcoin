pub mod commands;
pub mod cli;
pub mod rpc;
pub mod config;
pub mod error;

use clap::Parser;
use cli::{Cli, Commands};
use crate::commands::{address, blockchain, wallet};
use crate::config::Config;
use crate::error::AppErrors;
use crate::rpc::RpcClient;

fn main() -> Result<(), AppErrors> {
    let cli = Cli::parse();
    let config = Config::load(&cli).unwrap();
    let rpc_client = RpcClient::new(&config);
    match cli.command {
        Commands::Balance => wallet::balance(&rpc_client),
        Commands::BlockchainInfo => blockchain::run(&rpc_client),
        Commands::WalletInfo => wallet::wallet_info(&rpc_client),
        Commands::Rpc { method , params} => {
            let res = rpc_client.call(&*method, params)?;
            println!("{}", serde_json::to_string_pretty(&res)?);
            Ok(())
        },
        Commands::NewAddress => {
            let _ = address::generate_new_address(&rpc_client)?;
            Ok(())
        },
    }
}
