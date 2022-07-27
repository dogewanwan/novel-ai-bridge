use thiserror::Error;
use tokio::io;

#[derive(Debug, Error)]
pub enum BridgeError {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("{0}")]
    Serde(#[from] serde_json::Error),
    #[error("{0}")]
    TokenizerError(#[from] tokenizers::Error),
    #[error("{0}")]
    AcquireError(#[from] tokio::sync::AcquireError),
    #[error("{0}")]
    SystemTimeError(#[from] std::time::SystemTimeError),
}
