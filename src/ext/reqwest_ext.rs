//! Extension trait for reqwest::Client integration.
//!
//! This module provides the `BrightSkyReqwestExt` trait which adds a
//! `.get_brightsky()` method to `reqwest::Client`.
//!
//! # Example
//!
//! ```rust,no_run
//! use brightsky::{CurrentWeatherQueryBuilder, ext::BrightSkyReqwestExt, types::CurrentWeatherResponse};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = reqwest::Client::new();
//!
//!     let query = CurrentWeatherQueryBuilder::new()
//!         .with_lat_lon((52.52, 13.4))
//!         .build()?;
//!
//!     let response: CurrentWeatherResponse = client.get_brightsky(query).await?;
//!     println!("Temperature: {:?}°C", response.weather.temperature);
//!     Ok(())
//! }
//! ```

use crate::{BRIGHT_SKY_API, BrightSkyError, ToBrightSkyUrl};
use serde::de::DeserializeOwned;

/// Error type for reqwest-based Bright Sky requests.
#[derive(Debug)]
pub enum ReqwestBrightSkyError {
    /// Error building the query or URL
    Query(BrightSkyError),
    /// HTTP request failed
    Request(reqwest::Error),
    /// JSON deserialization failed
    Json(reqwest::Error),
}

impl std::fmt::Display for ReqwestBrightSkyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Query(e) => write!(f, "Query error: {}", e),
            Self::Request(e) => write!(f, "Request error: {}", e),
            Self::Json(e) => write!(f, "JSON error: {}", e),
        }
    }
}

impl std::error::Error for ReqwestBrightSkyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Query(e) => Some(e),
            Self::Request(e) => Some(e),
            Self::Json(e) => Some(e),
        }
    }
}

impl From<BrightSkyError> for ReqwestBrightSkyError {
    fn from(err: BrightSkyError) -> Self {
        Self::Query(err)
    }
}

/// Extension trait that adds Bright Sky API methods to `reqwest::Client`.
///
/// Import this trait to use `.get_brightsky()` on any reqwest Client.
///
/// # Example
///
/// ```rust,no_run
/// use brightsky::{CurrentWeatherQueryBuilder, ext::BrightSkyReqwestExt, types::CurrentWeatherResponse};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = reqwest::Client::new();
///
///     let query = CurrentWeatherQueryBuilder::new()
///         .with_lat_lon((52.52, 13.4))
///         .build()?;
///
///     let response: CurrentWeatherResponse = client.get_brightsky(query).await?;
///     println!("Temperature: {:?}°C", response.weather.temperature);
///     Ok(())
/// }
/// ```
pub trait BrightSkyReqwestExt {
    /// Fetch data from the Bright Sky API using the given query builder.
    ///
    /// This method:
    /// 1. Converts the query to a URL using the default Bright Sky API endpoint
    /// 2. Sends a GET request
    /// 3. Deserializes the JSON response into the specified type
    ///
    /// # Type Parameters
    ///
    /// * `Q` - A query builder implementing `ToBrightSkyUrl`
    /// * `R` - The response type to deserialize (e.g., `CurrentWeatherResponse`)
    ///
    /// # Errors
    ///
    /// Returns `ReqwestBrightSkyError` if:
    /// - Query building/URL generation fails
    /// - The HTTP request fails
    /// - JSON deserialization fails
    fn get_brightsky<Q, R>(
        &self,
        query: Q,
    ) -> impl std::future::Future<Output = Result<R, ReqwestBrightSkyError>> + Send
    where
        Q: ToBrightSkyUrl + Send,
        R: DeserializeOwned;

    /// Fetch data from the Bright Sky API using a custom host URL.
    ///
    /// Same as `get_brightsky` but allows specifying a custom API endpoint,
    /// useful for testing with mock servers or self-hosted instances.
    fn get_brightsky_with_host<Q, R>(
        &self,
        query: Q,
        host: &str,
    ) -> impl std::future::Future<Output = Result<R, ReqwestBrightSkyError>> + Send
    where
        Q: ToBrightSkyUrl + Send,
        R: DeserializeOwned;
}

impl BrightSkyReqwestExt for reqwest::Client {
    async fn get_brightsky<Q, R>(&self, query: Q) -> Result<R, ReqwestBrightSkyError>
    where
        Q: ToBrightSkyUrl + Send,
        R: DeserializeOwned,
    {
        self.get_brightsky_with_host(query, BRIGHT_SKY_API).await
    }

    async fn get_brightsky_with_host<Q, R>(
        &self,
        query: Q,
        host: &str,
    ) -> Result<R, ReqwestBrightSkyError>
    where
        Q: ToBrightSkyUrl + Send,
        R: DeserializeOwned,
    {
        let url = query.to_url(host)?;

        let response = self
            .get(url)
            .send()
            .await
            .map_err(ReqwestBrightSkyError::Request)?;

        response.json().await.map_err(ReqwestBrightSkyError::Json)
    }
}
