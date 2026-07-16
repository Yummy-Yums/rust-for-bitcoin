use reqwest::blocking::Client;
use serde_json::{json, Value};
use serde::{ Deserialize};
use crate::config::Config;
use crate::error::AppErrors;


/* Example of Valid Request
test@pop-os:~/Desktop/rust/rust-for-bitcoin$ curl --user polaruser:polarpass  \n
--data-binary '{"jsonrpc": "1.0", "id": "curltest", "method": "getbalance", "params": ["*", 6]}' \n
-H 'content-type: text/plain;'  http://127.0.0.1:18443

{"result":150.00000000,"error":null,"id":"curltest"}
*/
#[derive(Debug, Deserialize)]
pub struct RpcResponse {
    pub result: Option<Value>,
    pub error: Option<RpcErrorBody>
}

/* Example of Invalid Request
test@pop-os:~/Desktop/rust/rust-for-bitcoin$ curl --user polaruser:polarpass  \n
--data-binary '{"jsonrpc": "1.0", "id": "curltest", "method": "getbalan", "params": ["*", 6]}' \n
-H 'content-type: text/plain;'  http://127.0.0.1:18443

{"result":null,"error":{"code":-32601,"message":"Method not found"},"id":"curltest"}
*/
#[derive(Debug, Deserialize)]
pub struct RpcErrorBody {
    pub code: i64,
    pub message: String
}

/// A thin, reusable JSON-RPC client for Bitcoin Core.
///
/// One instance is built once from `Config` and reused across every
/// command, so the URL/auth/wallet-path logic only lives in one place.
pub struct RpcClient {
    http_client: Client,
    url: String,
    username: String,
    password: String
}

impl RpcClient {
    pub fn new(config: &Config) -> RpcClient {
        RpcClient {
            http_client: Client::new(),
            url: config.rpc_url.clone(),
            username: config.user.clone(),
            password: config.password.clone()
        }
    }

    // Call a non-wallet RPC method (e.g. getblockchaininfo).
    pub fn call(
        &self,
        method: &str,
        params: Vec<Value>,
    ) -> Result<Value, AppErrors> {
        self.call_at(&self.url, method, params)
    }

    fn call_at(
        &self,
        url: &str,
        method: &str,
        params: Vec<Value>,
    ) -> Result<Value, AppErrors> {
        let body = json!({
            "jsonrpc": "1.0",
            "id": "curltest",
            "method": method,
            "params": params,
        });

        let response = self
            .http_client
            .post(url)
            .basic_auth(&self.username, Some(&self.password))
            .json(&body)
            .send()
            .map_err(|source| AppErrors::Connection {
                url: url.to_string(),
                source
            })?;

        let status_code = response.status();

        if status_code == reqwest::StatusCode::UNAUTHORIZED {
            return Err(AppErrors::InvalidCredentials);
        }

        let text = response
            .text()
            .map_err(|source| AppErrors::Connection {
                url: url.to_string(),
                source
            })?;

        let parsed = serde_json::from_str::<RpcResponse>(&text)?;

        if let Some(error) = parsed.error {
            return Err(classify_error(method, error))
        }

        parsed
            .result
            .ok_or_else(|| AppErrors::UnexpectedResponse(format!("empty result for {method}")))
    }
}


/*
    Map Bitcoin Core's generic RPC error codes onto our richer types
    where it's obviously useful and fall back to a generic RpcError Otherwise
 */
fn classify_error(method: &str, error: RpcErrorBody) -> AppErrors {
    const RPC_WALLET_NOT_FOUND: i64 = -18;
    const RPC_WALLET_NOT_SPECIFIED: i64 = -19;
    const RPC_INVALID_PARAMETER: i64 = -8;
    const RPC_TYPE_ERROR: i64 = -3;
    const RPC_METHOD_NOT_FOUND: i64 = -32601;

    match error.code {
        RPC_WALLET_NOT_FOUND | RPC_WALLET_NOT_SPECIFIED => AppErrors::MissingWallet,
        RPC_INVALID_PARAMETER | RPC_TYPE_ERROR => AppErrors::InvalidParams {
            method: method.to_string(),
            message: error.message,
        },
        RPC_METHOD_NOT_FOUND => AppErrors::RpcError {
            method: method.to_string(),
            code: error.code,
            message: format!("unknown method: {}", error.message),
        },
        _ => AppErrors::RpcError {
            method: method.to_string(),
            code: error.code,
            message: error.message,
        }
    }
}