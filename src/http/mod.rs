//! HTTP client abstraction for pluggable backends.
//!
//! This module provides a trait-based abstraction over HTTP clients, allowing
//! the library to work with different backends:
//!
//! - **reqwest** (default): Full-featured HTTP client for std environments
//! - **reqwless**: Lightweight HTTP client for embedded/no_std environments
//!
//! # Feature Flags
//!
//! - `reqwest-client` (default): Enables the reqwest backend (requires std)
//! - `reqwless-client`: Enables the reqwless backend for embedded systems
//!
//! # Example with reqwest (default)
//!
//! ```rust
//! use brightsky::BrightSkyClient;
//!
//! let client = BrightSkyClient::new();
//! ```
//!
//! # Example with reqwless (embedded)
//!
//! ```ignore
//! use brightsky::http::{ReqwlessClient, ReqwlessConfig};
//! use brightsky::BrightSkyClient;
//!
//! // tcp and dns implement embedded-nal-async traits
//! let http_client = ReqwlessClient::new(tcp, dns, ReqwlessConfig::default());
//! let client = BrightSkyClient::with_http_client(http_client);
//! ```

#[cfg(feature = "reqwest-client")]
mod reqwest_client;
#[cfg(feature = "reqwest-client")]
pub use reqwest_client::*;

#[cfg(feature = "reqwless-client")]
mod reqwless_client;
#[cfg(feature = "reqwless-client")]
pub use reqwless_client::{DEFAULT_BUFFER_SIZE, ReqwlessConfig, reqwless_get};

#[cfg(feature = "std")]
use std::vec::Vec;

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use core::fmt::Debug;

/// HTTP response from a client.
#[derive(Debug)]
pub struct HttpResponse {
    /// HTTP status code (e.g., 200, 404, 500)
    pub status: u16,
    /// Response body as bytes
    pub body: Vec<u8>,
}

impl HttpResponse {
    /// Create a new HTTP response.
    pub fn new(status: u16, body: Vec<u8>) -> Self {
        Self { status, body }
    }

    /// Check if the response status indicates success (2xx).
    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status)
    }

    /// Get the response body as a string slice.
    #[cfg(feature = "std")]
    pub fn body_str(&self) -> Result<&str, core::str::Utf8Error> {
        core::str::from_utf8(&self.body)
    }
}

/// Error type for HTTP operations.
#[derive(Debug)]
pub enum HttpClientError {
    /// Request failed to send
    Request(HttpRequestError),
    /// Response had an error status code
    Status {
        /// The HTTP status code
        code: u16,
        /// Optional error message from response body
        message: Option<Vec<u8>>,
    },
    /// Failed to read response body
    Body,
    /// Connection error
    Connection,
    /// TLS/SSL error
    Tls,
    /// Timeout
    Timeout,
    /// URL parsing error
    InvalidUrl,
    /// Other error
    Other,
}

/// Underlying request error details.
#[derive(Debug)]
pub enum HttpRequestError {
    #[cfg(feature = "reqwest-client")]
    Reqwest(reqwest::Error),
    #[cfg(feature = "reqwless-client")]
    Reqwless(reqwless::Error),
    /// Generic error for custom implementations
    Custom,
}

impl core::fmt::Display for HttpClientError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Request(_) => write!(f, "HTTP request failed"),
            Self::Status { code, .. } => write!(f, "HTTP error status: {}", code),
            Self::Body => write!(f, "Failed to read response body"),
            Self::Connection => write!(f, "Connection error"),
            Self::Tls => write!(f, "TLS/SSL error"),
            Self::Timeout => write!(f, "Request timeout"),
            Self::InvalidUrl => write!(f, "Invalid URL"),
            Self::Other => write!(f, "Unknown HTTP error"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for HttpClientError {}

/// Trait for HTTP clients that can be used with BrightSkyClient.
///
/// This trait allows different HTTP client implementations to be used
/// interchangeably, enabling support for both std and no_std environments.
///
/// # Implementation Notes
///
/// Implementations should:
/// - Handle HTTPS connections (the Bright Sky API uses HTTPS)
/// - Return the full response body as bytes
/// - Map backend-specific errors to `HttpClientError`
pub trait HttpClient {
    /// Perform a GET request to the given URL.
    ///
    /// # Arguments
    ///
    /// * `url` - The full URL to request (including query parameters)
    ///
    /// # Returns
    ///
    /// Returns `HttpResponse` on success, containing the status code and body.
    /// Returns `HttpClientError` on failure.
    fn get(
        &self,
        url: &str,
    ) -> impl core::future::Future<Output = Result<HttpResponse, HttpClientError>> + Send;
}
