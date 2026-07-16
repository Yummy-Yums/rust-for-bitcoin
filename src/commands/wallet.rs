/*
    wallet-info

    Display:

        Wallet name
        Balance
        Unconfirmed balance
        Number of transactions

 */
use serde::Deserialize;
use serde_json::{json, Map, Value};
use crate::error::AppErrors;
use crate::rpc::{RpcClient, RpcResponse};

#[derive(Debug, Deserialize)]
pub struct WalletInfo {
    #[serde(rename = "walletname")]
    pub name: String,
    #[serde(rename = "walletversion")]
    version: i64,
    format: String,
    #[serde(rename = "txcount")]
    pub tx_count: f64,
    #[serde(rename = "keypoolsize")]
    key_pool_size: i64,
    #[serde(rename = "keypoolsize_hd_internal")]
    key_pool_size_hd_internal: i64,
    #[serde(rename = "paytxfee")]
    pay_tx_fee: f64,
    private_keys_enabled: bool,
    avoid_reuse: bool,
    scanning: bool,
    descriptors: bool,
    external_signer: bool,
    blank: bool,
    birthtime: i64,
    flags: Vec<String>,
    #[serde(rename = "lastprocessedblock")]
    last_processed_block: Map<String, Value>
}

fn require_wallet(wallet: &Option<String>) -> Result<&str, AppErrors> {
    wallet.as_deref().ok_or(AppErrors::MissingWallet)
}

pub fn wallet_info(client: &RpcClient) -> Result<(), AppErrors> {
    let raw = client.call("getwalletinfo", vec![])?;
    println!("{}", raw);
    let info: WalletInfo = serde_json::from_value(raw)?;

    println!("Wallet name:        {}", info.name);
    println!("Number of Transactions: {}", info.tx_count);

    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct GetBalancesResponse {
    pub mine: Mine,
    #[serde(rename = "lastprocessedblock")]
    pub last_processed_block: Option<Value>
}

#[derive(Debug, Deserialize)]
pub struct Mine {
    pub trusted: f64, // this is identical to balance according to docs; Check balance field https://developer.bitcoin.org/reference/rpc/getwalletinfo.html
    pub untrusted_pending: f64, // this is identical to unconfirmed_balances according to docs
    pub immature: f64,
}

pub fn balance(client: &RpcClient) -> Result<(), AppErrors> {
    let raw = client.call("getbalances", vec![])?;
    let balance: GetBalancesResponse = serde_json::from_value(raw)?;
    println!("balance: {:?} BTC", balance.mine.trusted);
    println!("unconfirmed balance: {:?} BTC", balance.mine.untrusted_pending);

    Ok(())
}

