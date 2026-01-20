//! # BrightSky - German Weather Data API Client
//!
//! This crate provides a Rust client for the [Bright Sky API](https://brightsky.dev/),
//! a free and open-source weather API that serves as a gateway to weather data published
//! by Germany's meteorological service (DWD) on their [open data server](https://opendata.dwd.de/).
//!
//! ## Features
//!
//! - **Current Weather**: Get real-time weather conditions compiled from SYNOP observations
//! - **Historical & Forecast Weather**: Access hourly weather records and forecasts
//! - **Radar Data**: Retrieve high-resolution rainfall radar with 1km spatial and 5-minute temporal resolution
//! - **Weather Alerts**: Access official weather warnings and alerts from DWD
//! - **No API Key Required**: The public instance at `https://api.brightsky.dev/` is free to use
//! - **Embedded Support**: Works in no_std environments with the `reqwless-client` feature
//!
//! ## Geographical Coverage
//!
//! Due to its nature as Germany's meteorological service, the observations have a strong focus on Germany.
//! However, forecasts cover the whole world, albeit at much lower density outside of Germany.
//! Historical data is available going back to January 1st, 2010.
//!
//! ## Quick Start (std with reqwest)
//!
//! ```rust,no_run
//! use brightsky::{BrightSkyClient, CurrentWeatherQueryBuilder, WeatherQueryBuilder, types::{CurrentWeatherResponse, WeatherResponse}};
//! use chrono::NaiveDate;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = BrightSkyClient::new();
//!
//!     // Get current weather for Berlin
//!     let current_query = CurrentWeatherQueryBuilder::new()
//!         .with_lat_lon((52.52, 13.4))
//!         .build()?;
//!
//!     let current_weather = client.get::<CurrentWeatherResponse>(current_query).await?;
//!     println!("Current temperature: {:?}°C", current_weather.weather.temperature);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Embedded Usage (no_std with reqwless)
//!
//! For embedded systems, brightsky provides query builders and response types.
//! You handle HTTP yourself with reqwless or any other HTTP client:
//!
//! ```ignore
//! use brightsky::{CurrentWeatherQueryBuilder, ToBrightSkyClientUrl, types::CurrentWeatherResponse};
//! use reqwless::client::{HttpClient, TlsConfig, TlsVerify};
//! use reqwless::request::Method;
//!
//! // Build the URL using brightsky's query builder
//! let query = CurrentWeatherQueryBuilder::new()
//!     .with_lat_lon((52.52, 13.4))
//!     .build()?;
//! let url = query.to_url_string("https://api.brightsky.dev")?;
//!
//! // Make request with your own HTTP client
//! let mut request = http_client.request(Method::GET, &url).await?;
//! let response = request.send(&mut rx_buffer).await?;
//! let body = response.body().read_to_end().await?;
//!
//! // Deserialize using brightsky's types  
//! let weather: CurrentWeatherResponse = serde_json::from_slice(body)?;
//! ```
//!
//! ## Feature Flags
//!
//! - `std` (default): Enable std library support
//! - `reqwest-client` (default): Use reqwest HTTP client (requires std)
//!
//! ## Data Sources
//!
//! All data is taken from the DWD open data server:
//! - **Current weather/SYNOP**: Real-time observations from weather stations
//! - **Hourly weather**: Historical observations, current day data, and MOSMIX forecasts
//! - **Radar**: Composite rainfall radar (RV product)
//! - **Alerts**: Official weather warnings in CAP format
//!
//! ## Usage Guidelines
//!
//! Please note that the [DWD's Terms of Use](https://www.dwd.de/EN/service/copyright/copyright_artikel.html)
//! apply to all data retrieved through this API.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::string::String;

#[cfg(feature = "std")]
use serde::de::DeserializeOwned;

#[cfg(feature = "std")]
use url::Url;

pub mod http;
pub mod types;

mod weather;
pub use weather::WeatherQueryBuilder;

mod current_weather;
pub use current_weather::CurrentWeatherQueryBuilder;

mod radar;
pub use radar::RadarWeatherQueryBuilder;

mod alerts;
pub use alerts::AlertsQueryBuilder;

mod errors;
pub use errors::*;

// Re-export HTTP client types for convenience
#[cfg(feature = "reqwest-client")]
pub use http::ReqwestClient;
#[cfg(feature = "std")]
pub use http::{HttpClient, HttpClientError, HttpResponse};

/// Base URL for the Bright Sky API
#[cfg(feature = "std")]
const BRIGHT_SKY_API: &str = "https://api.brightsky.dev";

/// HTTP client for making requests to the Bright Sky API.
///
/// The client handles request formatting and response deserialization
/// for all Bright Sky API endpoints. It is generic over the HTTP backend,
/// allowing it to work in both std and no_std environments.
///
/// ## Examples (std with reqwest - default)
///
/// ```rust,no_run
/// use brightsky::{BrightSkyClient, CurrentWeatherQueryBuilder, types::CurrentWeatherResponse};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = BrightSkyClient::new();
///
///     let query = CurrentWeatherQueryBuilder::new()
///         .with_lat_lon((52.52, 13.4))  // Berlin coordinates
///         .build()?;
///
///     let response = client.get::<CurrentWeatherResponse>(query).await?;
///     println!("Temperature: {:?}°C", response.weather.temperature);
///     Ok(())
/// }
/// ```
///
/// ## Examples (embedded with reqwless)
///
/// For embedded systems, use the query builders and response types directly:
///
/// ```ignore
/// use brightsky::{CurrentWeatherQueryBuilder, ToBrightSkyClientUrl, types::CurrentWeatherResponse};
///
/// let query = CurrentWeatherQueryBuilder::new()
///     .with_lat_lon((52.52, 13.4))
///     .build()?;
/// let url = query.to_url_string("https://api.brightsky.dev")?;
/// // Then use your HTTP client to fetch and serde_json to deserialize
/// ```
#[cfg(feature = "std")]
pub struct BrightSkyClient<C: HttpClient> {
    host: &'static str,
    client: C,
}

/// Trait for converting query builders into Bright Sky API URLs.
///
/// This trait is implemented by all query builder types to convert
/// their parameters into properly formatted API URLs.
pub trait ToBrightSkyClientUrl {
    /// Convert the query builder into a URL for the Bright Sky API.
    ///
    /// # Parameters
    ///
    /// * `host` - The base URL of the Bright Sky API
    ///
    /// # Errors
    ///
    /// Returns `BrightSkyError` if URL construction fails due to:
    /// - Invalid parameter values
    /// - URL parsing errors
    /// - Missing required parameters
    #[cfg(feature = "std")]
    fn to_url(self, host: &str) -> Result<Url, BrightSkyError>;

    /// Convert the query builder into a URL string for the Bright Sky API.
    ///
    /// This is the no_std compatible version that returns a String instead of Url.
    /// Also available in std environments for convenience.
    fn to_url_string(self, host: &str) -> Result<String, BrightSkyError>;
}

// Default implementation using reqwest
#[cfg(all(feature = "std", feature = "reqwest-client"))]
impl BrightSkyClient<http::ReqwestClient> {
    /// Create a new Bright Sky API client with the default reqwest backend.
    ///
    /// Uses the default public API endpoint at `https://api.brightsky.dev`.
    /// No API key is required.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brightsky::BrightSkyClient;
    ///
    /// let client = BrightSkyClient::new();
    /// ```
    pub fn new() -> Self {
        Self::with_http_client(http::ReqwestClient::new())
    }
}

#[cfg(all(feature = "std", feature = "reqwest-client"))]
impl Default for BrightSkyClient<http::ReqwestClient> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "std")]
impl<C: HttpClient> BrightSkyClient<C> {
    /// Create a new Bright Sky API client with a custom HTTP client.
    ///
    /// This allows you to use any HTTP client that implements the `HttpClient` trait.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use brightsky::BrightSkyClient;
    /// use brightsky::http::ReqwestClient;
    ///
    /// let http_client = ReqwestClient::new();
    /// let client = BrightSkyClient::with_http_client(http_client);
    /// ```
    pub fn with_http_client(client: C) -> Self {
        BrightSkyClient {
            host: BRIGHT_SKY_API,
            client,
        }
    }

    /// Create a new client with a custom host URL.
    ///
    /// Useful for testing with mock servers or self-hosted instances.
    pub fn with_host(client: C, host: &'static str) -> Self {
        BrightSkyClient { host, client }
    }

    /// Get a reference to the underlying HTTP client.
    pub fn http_client(&self) -> &C {
        &self.client
    }

    /// Get the configured host URL.
    pub fn host(&self) -> &str {
        self.host
    }

    /// Send a GET request to the Bright Sky API and deserialize the response.
    ///
    /// This method handles the HTTP communication, error checking, and JSON
    /// deserialization for all API endpoints.
    ///
    /// # Type Parameters
    ///
    /// * `R` - The response type to deserialize into (e.g., `CurrentWeatherResponse`, `WeatherResponse`)
    ///
    /// # Parameters
    ///
    /// * `builder` - A query builder that implements `ToBrightSkyClientUrl`
    ///
    /// # Returns
    ///
    /// Returns the deserialized API response of type `R`.
    ///
    /// # Errors
    ///
    /// Returns `BrightSkyError` for:
    /// - Network errors
    /// - HTTP error status codes
    /// - JSON deserialization failures
    /// - Invalid query parameters
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use brightsky::{BrightSkyClient, CurrentWeatherQueryBuilder, types::CurrentWeatherResponse};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = BrightSkyClient::new();
    ///
    ///     let query = CurrentWeatherQueryBuilder::new()
    ///         .with_lat_lon((52.52, 13.4))
    ///         .build()?;
    ///
    ///     let response: CurrentWeatherResponse = client.get(query).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get<R: DeserializeOwned>(
        &self,
        builder: impl ToBrightSkyClientUrl,
    ) -> Result<R, BrightSkyError> {
        let url = builder.to_url(self.host)?;
        let response = self
            .client
            .get(url.as_ref())
            .await
            .map_err(BrightSkyError::from)?;

        if !response.is_success() {
            return Err(BrightSkyError::HttpError(HttpErrorKind::Status {
                code: response.status,
            }));
        }

        #[cfg(debug_assertions)]
        {
            if let Ok(text) = core::str::from_utf8(&response.body) {
                eprintln!("Response Text: {}", text);
            }
        }

        let json: R = serde_json::from_slice(&response.body).map_err(BrightSkyError::SerdeError)?;

        Ok(json)
    }
}

// Legacy type alias for backwards compatibility
#[cfg(feature = "reqwest-client")]
#[deprecated(since = "0.2.0", note = "Use BrightSkyClient<ReqwestClient> instead")]
pub type LegacyBrightSkyClient = BrightSkyClient<http::ReqwestClient>;

// Also keep BlindSkyClientError as deprecated alias
#[deprecated(since = "0.2.0", note = "Use BrightSkyError instead")]
pub type BlindSkyClientError = BrightSkyError;

#[cfg(all(test, feature = "reqwest-client"))]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[tokio::test]
    async fn test_brightsky_client_creation() {
        let _client = BrightSkyClient::new();
        // Test passes if client creation doesn't panic
    }

    #[tokio::test]
    async fn test_brightsky_client_default() {
        let _client = BrightSkyClient::default();
        // Test passes if client creation doesn't panic
    }

    #[tokio::test]
    async fn test_current_weather_query_build_valid() {
        let query = CurrentWeatherQueryBuilder::new()
            .with_lat_lon((52.52, 13.4))
            .build();

        assert!(query.is_ok());
        let query = query.unwrap();
        assert!(query.lat.is_some());
        assert!(query.lon.is_some());
    }

    #[tokio::test]
    async fn test_current_weather_query_build_invalid_lat() {
        let result = CurrentWeatherQueryBuilder::new()
            .with_lat_lon((91.0, 13.4))
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            BrightSkyError::InvalidLongitude(_) => (),
            _ => panic!("Expected InvalidLongitude error"),
        }
    }

    #[tokio::test]
    async fn test_current_weather_query_build_invalid_lon() {
        let result = CurrentWeatherQueryBuilder::new()
            .with_lat_lon((52.52, 181.0))
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            BrightSkyError::InvalidLongitude(_) => (),
            _ => panic!("Expected InvalidLongitude error"),
        }
    }

    #[tokio::test]
    async fn test_current_weather_query_with_max_dist() {
        let query = CurrentWeatherQueryBuilder::new()
            .with_lat_lon((52.52, 13.4))
            .with_max_dist(10000)
            .build();

        assert!(query.is_ok());
        let query = query.unwrap();
        assert_eq!(query.max_dist, Some("10000".to_string()));
    }

    #[tokio::test]
    async fn test_current_weather_query_invalid_max_dist() {
        let result = CurrentWeatherQueryBuilder::new()
            .with_lat_lon((52.52, 13.4))
            .with_max_dist(500001)
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            BrightSkyError::InvalidMaxDistance(_) => (),
            _ => panic!("Expected InvalidMaxDistance error"),
        }
    }

    #[tokio::test]
    async fn test_current_weather_query_with_station_ids() {
        let query = CurrentWeatherQueryBuilder::new()
            .with_dwd_station_id(vec!["01766".to_string()])
            .with_wmo_station_id(vec!["10315".to_string()])
            .build();

        assert!(query.is_ok());
        let query = query.unwrap();
        assert_eq!(query.dwd_station_id, Some(vec!["01766".to_string()]));
        assert_eq!(query.wmo_station_id, Some(vec!["10315".to_string()]));
    }

    #[tokio::test]
    async fn test_current_weather_query_with_source_ids() {
        let query = CurrentWeatherQueryBuilder::new()
            .with_source_id(vec![1234, 5678])
            .build();

        assert!(query.is_ok());
        let query = query.unwrap();
        assert_eq!(
            query.source_id,
            Some(vec!["1234".to_string(), "5678".to_string()])
        );
    }

    #[tokio::test]
    async fn test_current_weather_query_with_timezone() {
        let query = CurrentWeatherQueryBuilder::new()
            .with_lat_lon((52.52, 13.4))
            .with_tz("Europe/Berlin")
            .build();

        assert!(query.is_ok());
        let query = query.unwrap();
        assert_eq!(query.tz, Some("Europe/Berlin".to_string()));
    }

    #[tokio::test]
    async fn test_current_weather_query_with_units() {
        let query = CurrentWeatherQueryBuilder::new()
            .with_lat_lon((52.52, 13.4))
            .with_units(types::UnitType::Si)
            .build();

        assert!(query.is_ok());
        let query = query.unwrap();
        assert_eq!(query.units, Some(types::UnitType::Si));
    }

    #[tokio::test]
    async fn test_weather_query_build_valid() {
        let date = NaiveDate::from_ymd_opt(2023, 8, 7).unwrap();
        let query = WeatherQueryBuilder::new()
            .with_date(date)
            .with_lat_lon((52.52, 13.4))
            .build();

        assert!(query.is_ok());
        let query = query.unwrap();
        assert!(query.date.is_some());
        assert!(query.lat.is_some());
        assert!(query.lon.is_some());
    }

    #[tokio::test]
    async fn test_weather_query_build_missing_date() {
        let result = WeatherQueryBuilder::new()
            .with_lat_lon((52.52, 13.4))
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            BrightSkyError::DateNotSet => (),
            _ => panic!("Expected DateNotSet error"),
        }
    }

    #[tokio::test]
    async fn test_weather_query_with_date_range() {
        let start_date = NaiveDate::from_ymd_opt(2023, 8, 7).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2023, 8, 10).unwrap();

        let query = WeatherQueryBuilder::new()
            .with_date(start_date)
            .with_last_date(end_date)
            .with_lat_lon((52.52, 13.4))
            .build();

        assert!(query.is_ok());
        let query = query.unwrap();
        assert_eq!(query.date, Some(start_date));
        assert_eq!(query.last_date, Some(end_date));
    }

    #[tokio::test]
    async fn test_weather_query_with_dwd_station() {
        let date = NaiveDate::from_ymd_opt(2023, 8, 7).unwrap();
        let query = WeatherQueryBuilder::new()
            .with_date(date)
            .with_dwd_station_id(vec!["01766", "00420"])
            .build();

        assert!(query.is_ok());
        let query = query.unwrap();
        assert_eq!(query.dwd_station_id, Some(vec!["01766", "00420"]));
    }

    #[tokio::test]
    async fn test_weather_query_with_wmo_station() {
        let date = NaiveDate::from_ymd_opt(2023, 8, 7).unwrap();
        let query = WeatherQueryBuilder::new()
            .with_date(date)
            .with_wmo_station_id(vec!["10315"])
            .build();

        assert!(query.is_ok());
        let query = query.unwrap();
        assert_eq!(query.wmo_station_id, Some(vec!["10315"]));
    }

    #[tokio::test]
    async fn test_weather_query_with_source_ids() {
        let date = NaiveDate::from_ymd_opt(2023, 8, 7).unwrap();
        let query = WeatherQueryBuilder::new()
            .with_date(date)
            .with_source_id(vec![1234, 5678])
            .build();

        assert!(query.is_ok());
        let query = query.unwrap();
        assert_eq!(
            query.source_id,
            Some(vec!["1234".to_string(), "5678".to_string()])
        );
    }

    #[tokio::test]
    async fn test_weather_query_invalid_coordinates() {
        let date = NaiveDate::from_ymd_opt(2023, 8, 7).unwrap();
        let result = WeatherQueryBuilder::new()
            .with_date(date)
            .with_lat_lon((95.0, 200.0))
            .build();

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_url_generation_current_weather() {
        let query = CurrentWeatherQueryBuilder::new()
            .with_lat_lon((52.52, 13.4))
            .with_max_dist(10000)
            .with_tz("Europe/Berlin")
            .build()
            .unwrap();

        let url = query.to_url("https://api.brightsky.dev").unwrap();

        assert_eq!(url.path(), "/current_weather");
        assert!(url.query().unwrap().contains("lat=52.52"));
        assert!(url.query().unwrap().contains("lon=13.4"));
        assert!(url.query().unwrap().contains("max_dist=10000"));
        assert!(
            url.query().unwrap().contains("tz=Europe") && url.query().unwrap().contains("Berlin")
        );
    }

    #[tokio::test]
    async fn test_url_generation_weather() {
        let date = NaiveDate::from_ymd_opt(2023, 8, 7).unwrap();
        let query = WeatherQueryBuilder::new()
            .with_date(date)
            .with_lat_lon((52.52, 13.4))
            .with_units(types::UnitType::Dwd)
            .build()
            .unwrap();

        let url = query.to_url("https://api.brightsky.dev").unwrap();

        assert_eq!(url.path(), "/weather");
        assert!(url.query().unwrap().contains("date=2023-08-07"));
        assert!(url.query().unwrap().contains("lat=52.52"));
        assert!(url.query().unwrap().contains("lon=13.4"));
        assert!(url.query().unwrap().contains("units=dwd"));
    }

    #[tokio::test]
    async fn test_url_generation_with_multiple_station_ids() {
        let query = CurrentWeatherQueryBuilder::new()
            .with_dwd_station_id(vec!["01766".to_string(), "00420".to_string()])
            .build()
            .unwrap();

        let url = query.to_url("https://api.brightsky.dev").unwrap();
        let query_str = url.query().unwrap();

        assert!(query_str.contains("dwd_station_id=01766"));
        assert!(query_str.contains("dwd_station_id=00420"));
    }
}
