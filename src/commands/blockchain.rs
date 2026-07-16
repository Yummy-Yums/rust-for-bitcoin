/*
    blockchain-info

    Display:

        Chain
        Number of blocks
        Number of headers
        Difficulty
        Verification progress

*/
use reqwest::Client;
use serde::Deserialize;
use crate::error::AppErrors;
use crate::rpc::RpcClient;

#[derive(Debug, Deserialize)]
pub struct BlockchainInfo {
    pub chain: String,
    pub blocks: u64,
    pub headers: u64,
    #[serde(rename = "bestblockhash")]
    pub best_blockhash: String,
    pub bits: String,
    pub target: String,
    pub difficulty: f64,
    pub time: u128,
    #[serde(rename = "verificationprogress")]
    pub verification_progress: f64,
    #[serde(rename = "initialblockdownload")]
    pub initial_block_download: bool,
    pub chainwork: String,
    pub size_on_disk: i64,
    pub pruned: bool,
    pub warnings: Vec<String>,
}

pub(crate) fn run(client: &RpcClient) -> Result<(), AppErrors> {
    let raw = client.call("getblockchaininfo", vec![])?;
    let info: BlockchainInfo = serde_json::from_value(raw)?;

    println!("chain: {}", info.chain);
    println!("Number of blocks: {}", info.blocks);
    println!("Number of headers: {}", info.headers);
    println!("difficulty: {}", info.difficulty);
    println!("Verification Progress: {}", info.verification_progress);

    Ok(())
}