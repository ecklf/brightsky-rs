//! Error types for the Bright Sky client.

use core::num::{ParseFloatError, ParseIntError};

#[cfg(feature = "std")]
use crate::http::HttpClientError;

/// Error type for Bright Sky client operations.
#[derive(Debug)]
pub enum BrightSkyError {
    /// Date parameter is required but was not set.
    DateNotSet,
    /// Latitude value is out of valid range (-90 to 90).
    InvalidLatitude(f64),
    /// Longitude value is out of valid range (-180 to 180).
    InvalidLongitude(f64),
    /// Max distance value is out of valid range (0 to 500000 meters).
    InvalidMaxDistance(u32),
    /// Failed to parse an integer value.
    ParseIntError(ParseIntError),
    /// Failed to parse a float value.
    ParseFloatError(ParseFloatError),
    /// URL parsing error.
    #[cfg(feature = "std")]
    UrlParseError(url::ParseError),
    /// URL parsing error (no_std).
    #[cfg(not(feature = "std"))]
    UrlParseError,
    /// HTTP client error.
    HttpError(HttpErrorKind),
    /// JSON serialization/deserialization error.
    SerdeError(serde_json::Error),
}

/// HTTP error details, abstracted over different backends.
#[derive(Debug)]
pub enum HttpErrorKind {
    /// Request failed with an error status code.
    Status {
        /// HTTP status code
        code: u16,
    },
    /// Connection error.
    Connection,
    /// Timeout error.
    Timeout,
    /// TLS/SSL error.
    Tls,
    /// Failed to read response body.
    Body,
    /// Invalid URL.
    InvalidUrl,
    /// Other/unknown error.
    #[cfg(feature = "reqwest-client")]
    Reqwest(reqwest::Error),
    /// Generic error for when no specific backend is available.
    #[cfg(not(feature = "reqwest-client"))]
    Other,
}

impl core::fmt::Display for BrightSkyError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DateNotSet => write!(f, "Date is required, but not set"),
            Self::InvalidLatitude(lat) => {
                write!(f, "Latitude must be between -90 and 90, got {}", lat)
            }
            Self::InvalidLongitude(lon) => {
                write!(f, "Longitude must be between -180 and 180, got {}", lon)
            }
            Self::InvalidMaxDistance(dist) => {
                write!(f, "Max distance must be between 0 and 500000, got {}", dist)
            }
            Self::ParseIntError(_) => write!(f, "Parse int failed"),
            Self::ParseFloatError(_) => write!(f, "Parse float failed"),
            #[cfg(feature = "std")]
            Self::UrlParseError(_) => write!(f, "URL parse error"),
            #[cfg(not(feature = "std"))]
            Self::UrlParseError => write!(f, "URL parse error"),
            Self::HttpError(kind) => write!(f, "HTTP error: {:?}", kind),
            Self::SerdeError(_) => write!(f, "JSON serialization error"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for BrightSkyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ParseIntError(e) => Some(e),
            Self::ParseFloatError(e) => Some(e),
            #[cfg(feature = "std")]
            Self::UrlParseError(e) => Some(e),
            Self::SerdeError(e) => Some(e),
            #[cfg(feature = "reqwest-client")]
            Self::HttpError(HttpErrorKind::Reqwest(e)) => Some(e),
            _ => None,
        }
    }
}

impl From<ParseIntError> for BrightSkyError {
    fn from(err: ParseIntError) -> Self {
        Self::ParseIntError(err)
    }
}

impl From<ParseFloatError> for BrightSkyError {
    fn from(err: ParseFloatError) -> Self {
        Self::ParseFloatError(err)
    }
}

#[cfg(feature = "std")]
impl From<url::ParseError> for BrightSkyError {
    fn from(err: url::ParseError) -> Self {
        Self::UrlParseError(err)
    }
}

impl From<serde_json::Error> for BrightSkyError {
    fn from(err: serde_json::Error) -> Self {
        Self::SerdeError(err)
    }
}

#[cfg(feature = "reqwest-client")]
impl From<reqwest::Error> for BrightSkyError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            Self::HttpError(HttpErrorKind::Timeout)
        } else if err.is_connect() {
            Self::HttpError(HttpErrorKind::Connection)
        } else if err.is_status() {
            Self::HttpError(HttpErrorKind::Status {
                code: err.status().map(|s| s.as_u16()).unwrap_or(0),
            })
        } else {
            Self::HttpError(HttpErrorKind::Reqwest(err))
        }
    }
}



#[cfg(feature = "std")]
impl From<HttpClientError> for BrightSkyError {
    fn from(err: HttpClientError) -> Self {
        match err {
            HttpClientError::Timeout => Self::HttpError(HttpErrorKind::Timeout),
            HttpClientError::Connection => Self::HttpError(HttpErrorKind::Connection),
            HttpClientError::Tls => Self::HttpError(HttpErrorKind::Tls),
            HttpClientError::Body => Self::HttpError(HttpErrorKind::Body),
            HttpClientError::InvalidUrl => Self::HttpError(HttpErrorKind::InvalidUrl),
            HttpClientError::Status { code, .. } => Self::HttpError(HttpErrorKind::Status { code }),
            HttpClientError::Request(req_err) => match req_err {
                #[cfg(feature = "reqwest-client")]
                crate::http::HttpRequestError::Reqwest(e) => {
                    Self::HttpError(HttpErrorKind::Reqwest(e))
                }
                #[allow(unreachable_patterns)]
                _ => Self::HttpError(HttpErrorKind::Connection),
            },
            HttpClientError::Other => Self::HttpError(HttpErrorKind::Connection),
        }
    }
}

// Legacy type alias for backwards compatibility
#[deprecated(since = "0.2.0", note = "Use BrightSkyError instead")]
#[allow(dead_code)]
pub type BlindSkyClientError = BrightSkyError;
