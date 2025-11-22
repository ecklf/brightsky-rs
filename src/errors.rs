use std::num::{ParseFloatError, ParseIntError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlindSkyClientError {
    #[error("Date is required, but not set")]
    DateNotSet,
    #[error("Latitude must be between -90 and 90, got {0}")]
    InvalidLatitude(f64),
    #[error("Longitude must be between -180 and 180, got {0}")]
    InvalidLongitude(f64),
    #[error("Max distance must be between 0 and 500000, got {0}")]
    InvalidMaxDistance(u32),
    #[error("Parse int failed")]
    ParseIntError(#[from] ParseIntError),
    #[error("Parse float failed")]
    ParseFloatError(#[from] ParseFloatError),
    #[error("Url parse error")]
    UrlParseError(#[from] url::ParseError),
    #[error("Reqwest error")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Serde error")]
    SerdeError(#[from] serde_json::Error),
}
