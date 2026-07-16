// Error Handling
//
// Your application should gracefully handle:
//
// Invalid credentials
// Connection failures
// Invalid RPC methods
// Invalid parameters
// Missing wallet
//
// Avoid panics and provide clear, user-friendly error messages.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppErrors {
    #[error("Bitcoin Node at {url} unreachable, Check if it's running {source}")]
    Connection {
        url: String,
        #[source]
        source: reqwest::Error,
    },

    #[error("Authentication failed. Double Check your RPC username and/or password and try again")]
    InvalidCredentials,

    #[error("Node returned an error ")]
    RpcError {
        method: String,
        code: i64,
        message: String,
    },

    #[error("Invalid parameters for method '{method}': {message} ")]
    InvalidParams { method : String, message: String },

    #[error(
        "The wallet is missing or you passed the wrong name, please crosscheck and try again"
    )]
    MissingWallet,

    #[error("Failed to parse node's response as JSON: {0}")]
    Json(
        #[from]
        serde_json::Error
    ),

    #[error("Failed to parse node's response as JSON: {0}")]
    UnexpectedResponse(String),

    #[error("{0}")]
    Other(String)
}

