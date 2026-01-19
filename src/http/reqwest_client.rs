//! reqwest-based HTTP client implementation.
//!
//! This module provides the default HTTP client for std environments,
//! using the popular `reqwest` crate.

use super::{HttpClient, HttpClientError, HttpRequestError, HttpResponse};

/// HTTP client using reqwest for std environments.
///
/// This is the default client when the `reqwest-client` feature is enabled.
///
/// # Example
///
/// ```rust
/// use brightsky::http::ReqwestClient;
/// use brightsky::BrightSkyClient;
///
/// // Using default client
/// let client = BrightSkyClient::new();
///
/// // Or create with custom reqwest client
/// let reqwest_client = reqwest::Client::builder()
///     .timeout(std::time::Duration::from_secs(30))
///     .build()
///     .unwrap();
/// let http_client = ReqwestClient::with_client(reqwest_client);
/// let client = BrightSkyClient::with_http_client(http_client);
/// ```
#[derive(Debug, Clone)]
pub struct ReqwestClient {
    inner: reqwest::Client,
}

impl ReqwestClient {
    /// Create a new reqwest client with default settings.
    pub fn new() -> Self {
        Self {
            inner: reqwest::Client::new(),
        }
    }

    /// Create a reqwest client wrapper from an existing reqwest::Client.
    ///
    /// This allows you to configure the underlying client with custom
    /// timeouts, headers, proxies, etc.
    pub fn with_client(client: reqwest::Client) -> Self {
        Self { inner: client }
    }

    /// Get a reference to the underlying reqwest client.
    pub fn inner(&self) -> &reqwest::Client {
        &self.inner
    }
}

impl Default for ReqwestClient {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpClient for ReqwestClient {
    async fn get(&self, url: &str) -> Result<HttpResponse, HttpClientError> {
        let response = self
            .inner
            .get(url)
            .send()
            .await
            .map_err(|e| HttpClientError::Request(HttpRequestError::Reqwest(e)))?;

        let status = response.status().as_u16();
        let body = response
            .bytes()
            .await
            .map_err(|e| HttpClientError::Request(HttpRequestError::Reqwest(e)))?
            .to_vec();

        Ok(HttpResponse::new(status, body))
    }
}

impl From<reqwest::Error> for HttpClientError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            HttpClientError::Timeout
        } else if err.is_connect() {
            HttpClientError::Connection
        } else if err.is_body() {
            HttpClientError::Body
        } else if err.is_status() {
            HttpClientError::Status {
                code: err.status().map(|s| s.as_u16()).unwrap_or(0),
                message: None,
            }
        } else {
            HttpClientError::Request(HttpRequestError::Reqwest(err))
        }
    }
}
