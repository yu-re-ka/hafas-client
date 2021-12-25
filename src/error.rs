use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("{0}")]
    Missing(String),
    #[error("{0}")]
    Json(#[from] serde_json::Error),
    #[cfg(feature = "hyper-requester")]
    #[error("{0}")]
    Http(#[from] hyper::Error),
    #[error("HAFAS error: {0}")]
    Hafas(String),
    #[error("{0}")]
    InvalidInput(String),
    #[error("Invalid data")]
    InvalidData,
}

pub type Result<T> = std::result::Result<T, Error>;
