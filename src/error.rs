use thiserror::Error as ThisError;
use std::backtrace::Backtrace;

#[derive(ThisError, Debug)]
pub enum ParseError {
    #[error("{info}")]
    InvalidData {
        backtrace: Backtrace,
        info: String,
    },
    #[error("source")]
    Chrono {
        backtrace: Backtrace,
        #[from] source: chrono::ParseError,
    },
    #[error("source")]
    Int {
        backtrace: Backtrace,
        #[from] source: std::num::ParseIntError,
    }
}

impl From<String> for ParseError {
    fn from(info: String) -> ParseError {
        ParseError::InvalidData {
            info,
            backtrace: Backtrace::capture(),
        }
    }
}

impl From<&str> for ParseError {
    fn from(info: &str) -> ParseError {
        ParseError::InvalidData {
            info: info.to_string(),
            backtrace: Backtrace::capture(),
        }
    }
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("{source}")]
    Json {
        backtrace: Backtrace,
        #[from] source: serde_json::Error,
    },
    #[cfg(feature = "hyper-requester")]
    #[error("{source}")]
    Http {
        backtrace: Backtrace,
        #[from] source: hyper::Error,
    },
    #[error("{source}")]
    Parse {
        #[backtrace]
        #[from] source: ParseError,
    },
    #[error("HAFAS error: {0}")]
    Hafas(String),
    #[error("{0}")]
    InvalidInput(String),
}

pub type Result<T> = std::result::Result<T, Error>;
pub type ParseResult<T> = std::result::Result<T, ParseError>;
