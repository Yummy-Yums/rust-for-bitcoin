use clap::{Command, Parser, Subcommand};
use serde_json::Value;
use std::path::PathBuf;

/// A CLI program for talking to a Bitcoin Core node over JSON-RPC.
///
/// Connection settings can be provided via flags, environment variables
/// (RFB_RPC_URL / RFB_RPC_USER / RFB_RPC_PASS / RFB_WALLET), or a JSON
/// config file passed with --config. Flags win over env vars, which win
/// over the config file.
#[derive(Parser)]
#[command(name = "rust-for-bitcoin", version, about)]
pub struct Cli {
    /// Bitcoin Core RPC URL, e.g. http://127.0.0.1:18443
    #[arg(long, env = "RPC_URL", global = true)]
    pub rpc_url: Option<String>,

    /// RPC username
    #[arg(long, env = "RPC_USER", global = true)]
    pub rpc_user: Option<String>,

    /// RPC password
    #[arg(long, env = "RPC_PASSWORD", global = true)]
    pub rpc_password: Option<String>,

    /// Wallet name to operate on (required for wallet scoped commands)
    #[arg(long, env = "WALLET", global = true)]
    pub wallet: Option<String>,

    /// Optional path to a JSON config file which contain rpc_url/rpc_password/wallet
    #[arg(long, global = true)]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show chain, block/header counts, difficulty and sync progress
    BlockchainInfo,

    /// Show wallet name, balance, unconfirmed balance and tx count
    WalletInfo,

    /// Print Wallet Balance
    Balance,

    /// Generate and print new receiving address
    NewAddress,

    /// Call an arbitrary Bitcoin Core RPC method
    Rpc {
        /// Rpc method name, e.g. getblockcount
        method: String,

        /// Positional arguments passed through
        params: Vec<Value>,
    },
}
