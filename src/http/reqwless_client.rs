//! reqwless-based HTTP client implementation for embedded/no_std environments.
//!
//! This module provides an HTTP client for embedded systems using the `reqwless`
//! crate, which works without std and is designed for constrained environments.
//!
//! # Note
//!
//! Due to lifetime constraints in reqwless, this implementation requires the user
//! to manage the HTTP client lifecycle. See the examples for proper usage patterns.

extern crate alloc;

use alloc::vec::Vec;

use super::{HttpClientError, HttpRequestError, HttpResponse};

/// Default buffer size for HTTP responses (16KB).
///
/// This should be sufficient for most weather API responses.
/// Radar data may require larger buffers - consider using the
/// `radar` feature flag to exclude radar endpoints if memory is constrained.
pub const DEFAULT_BUFFER_SIZE: usize = 16 * 1024;

/// Configuration for the reqwless HTTP client.
#[derive(Debug, Clone, Copy)]
pub struct ReqwlessConfig {
    /// Buffer size for reading HTTP responses.
    /// Larger buffers can handle bigger responses but use more memory.
    pub buffer_size: usize,
}

impl Default for ReqwlessConfig {
    fn default() -> Self {
        Self {
            buffer_size: DEFAULT_BUFFER_SIZE,
        }
    }
}

impl ReqwlessConfig {
    /// Create a new configuration with custom buffer size.
    pub const fn with_buffer_size(buffer_size: usize) -> Self {
        Self { buffer_size }
    }
}

/// Helper function to perform a GET request using reqwless.
///
/// This function handles the HTTP request and returns the response.
/// It is designed to be used with embassy-net or similar embedded networking stacks.
///
/// # Example
///
/// ```ignore
/// use brightsky::http::{reqwless_get, ReqwlessConfig};
/// use embassy_net::tcp::TcpSocket;
/// use embassy_net::dns::DnsSocket;
///
/// async fn fetch_weather<'a>(
///     tcp: &TcpClient<'a, ...>,
///     dns: &DnsSocket<'a, ...>,
///     url: &str,
/// ) -> Result<HttpResponse, HttpClientError> {
///     let config = ReqwlessConfig::default();
///     reqwless_get(tcp, dns, url, config).await
/// }
/// ```
///
/// # Memory Considerations
///
/// The reqwless client uses a fixed-size buffer for responses. Weather API
/// responses vary in size:
/// - Current weather: ~2-4 KB
/// - Weather forecasts: ~10-50 KB (depends on date range)
/// - Radar data: Can be very large (100KB+) - consider excluding for embedded
/// - Alerts: ~5-20 KB (depends on number of alerts)
pub async fn reqwless_get<T, D>(
    tcp: &T,
    dns: &D,
    url: &str,
    config: ReqwlessConfig,
) -> Result<HttpResponse, HttpClientError>
where
    T: embedded_nal_async::TcpConnect,
    for<'a> <T as embedded_nal_async::TcpConnect>::Connection<'a>:
        embedded_io_async::Read + embedded_io_async::Write,
    D: embedded_nal_async::Dns,
{
    use embedded_io_async::Read;
    use reqwless::{client::HttpClient, request::Method};

    let mut client = HttpClient::new(tcp, dns);
    let mut buffer = alloc::vec![0u8; config.buffer_size];

    let mut request = client
        .request(Method::GET, url)
        .await
        .map_err(map_reqwless_error)?;

    let response = request
        .send(&mut buffer)
        .await
        .map_err(map_reqwless_error)?;

    // Extract status code
    let status = response.status.0;

    // Read the response body
    let mut body = Vec::new();
    let mut reader = response.body().reader();

    // Read in chunks until EOF
    let mut chunk_buf = [0u8; 1024];
    loop {
        match reader.read(&mut chunk_buf).await {
            Ok(0) => break, // EOF
            Ok(n) => body.extend_from_slice(&chunk_buf[..n]),
            Err(_) => return Err(HttpClientError::Body),
        }
    }

    Ok(HttpResponse::new(status, body))
}

/// Map reqwless errors to HttpClientError.
fn map_reqwless_error(err: reqwless::Error) -> HttpClientError {
    match err {
        reqwless::Error::Network(_) => HttpClientError::Connection,
        reqwless::Error::Dns => HttpClientError::Connection,
        reqwless::Error::BufferTooSmall => HttpClientError::Body,
        reqwless::Error::InvalidUrl(_) => HttpClientError::InvalidUrl,
        _ => HttpClientError::Request(HttpRequestError::Reqwless(err)),
    }
}

impl From<reqwless::Error> for HttpClientError {
    fn from(err: reqwless::Error) -> Self {
        map_reqwless_error(err)
    }
}
