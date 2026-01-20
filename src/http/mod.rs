//! HTTP client abstraction for pluggable backends.
//!
//! This module provides a trait-based abstraction over HTTP clients.
//!
//! # Feature Flags
//!
//! - `reqwest-client` (default): Enables the reqwest backend (requires std)
//! - `reqwless-client`: Enables types for embedded systems (no HTTP client, just types)
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
//! For embedded systems, brightsky provides query builders and response types.
//! You handle HTTP yourself with reqwless:
//!
//! ```ignore
//! use brightsky::{CurrentWeatherQueryBuilder, ToBrightSkyClientUrl, types::CurrentWeatherResponse};
//! use reqwless::client::{HttpClient, TlsConfig, TlsVerify};
//!
//! // Build the URL using brightsky's query builder
//! let url = CurrentWeatherQueryBuilder::new()
//!     .with_lat_lon((52.52, 13.4))
//!     .build()?
//!     .to_url_string("https://api.brightsky.dev")?;
//!
//! // Make request with your own HTTP client
//! let mut request = http_client.request(Method::GET, &url).await?;
//! let response = request.send(&mut rx_buffer).await?;
//! let body = response.body().read_to_end().await?;
//!
//! // Deserialize using brightsky's types
//! let weather: CurrentWeatherResponse = serde_json::from_slice(body)?;
//! ```

#[cfg(feature = "reqwest-client")]
mod reqwest_client;
#[cfg(feature = "reqwest-client")]
pub use reqwest_client::*;

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
#[cfg(feature = "std")]
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
