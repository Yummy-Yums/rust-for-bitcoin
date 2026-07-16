use std::path::PathBuf;
use anyhow::Context;
use serde::Deserialize;
use anyhow::Result;
use crate::cli::Cli;

/*
    Resolved RPC connection settings. Precedence (highest wins)
    CLI flags > environment variables > config file > error.

    None  is ever hardcoded - if none of the three sources supply a
    required value, `Config::load` returns an error
 */
#[derive(Debug, Clone)]
pub struct Config {
    pub rpc_url: String,
    pub user: String,
    pub password: String,
    pub wallet: Option<String>
}

/*
    Shape of the optional JSON config file, e.g.:

    {
      "rpc_url": "http://127.0.0.1:18443",
      "rpc_user": "polaruser",
      "rpc_pass": "polarpass",
      "wallet": "testwallet"
    }
*/
#[derive(Debug, Deserialize)]
struct FileConfig {
    rpc_url: Option<String>,
    rpc_user: Option<String>,
    rpc_password: Option<String>,
    wallet: Option<String>
}

impl Default for FileConfig {
    fn default() -> Self {
        Self {
            rpc_url: Some(String::from("http://127.0.0.1:18443")),
            rpc_user: Some(String::from("polaruser")),
            rpc_password: Some(String::from("polarpass")),
            wallet: Some(String::from("testwallet2")),
        }
    }
}

impl Config {
    pub fn load(cli: &Cli) -> Result<Self> {
        let file_config: FileConfig = match &cli.config {
            Some(path) => load_file(path)?,
            None => FileConfig::default()
        };

        let rpc_url = cli
            .rpc_url
            .clone()
            .or(file_config.rpc_url)
            .context("Missing RPC URL. Set --rpc-url, RFB_RPC_URL, or add it to --config.")?;

        let rpc_user = cli
            .rpc_user
            .clone()
            .or(file_config.rpc_user)
            .context("Missing RPC USERNAME. Set --rpc-username, RFB_RPC_URL, or add it to --config.")?;

        let rpc_password = cli
            .rpc_password
            .clone()
            .or(file_config.rpc_password)
            .context("Missing RPC USERNAME. Set --rpc-username, RFB_RPC_USERNAME, or add it to --config.")?;

        let wallet = cli.wallet.clone().or(file_config.wallet);

        Ok(Config {
            rpc_url: rpc_url,
            user: rpc_user,
            password: rpc_password,
            wallet: wallet,
        })
    }

}

fn load_file(path: &PathBuf) -> Result<FileConfig> {
    let contents = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file: {}", path.display()))?;
    serde_json::from_str(&contents)
        .with_context(|| format!("Config File at {} is not a valid JSON", path.display()))
}