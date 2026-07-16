/*
    new-address

    Generate and print a new receiving address.
 */
use serde_json::Value;
use crate::error::AppErrors;
use crate::rpc::{RpcClient, RpcResponse};

pub fn generate_new_address(client: &RpcClient) -> Result<(), AppErrors> {
    let raw = client.call("getnewaddress", vec![])?;
    let response: Value = serde_json::from_value(raw)?;
    println!("generated new address: {} BTC", response);

    Ok(())
}
