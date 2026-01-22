//! # BrightSky - German Weather Data API Client
//!
//! This crate provides type-safe query builders and response types for the
//! [Bright Sky API](https://brightsky.dev/), a free and open-source weather API
//! that serves weather data from Germany's meteorological service (DWD).
//!
//! ## Design Philosophy
//!
//! This crate focuses on **query building** and **response types** only.
//! You bring your own HTTP client (reqwest, reqwless, ureq, etc.).
//!
//! ## Features
//!
//! - **Current Weather**: Get real-time weather conditions from SYNOP observations
//! - **Historical & Forecast Weather**: Access hourly weather records and forecasts
//! - **Radar Data**: Retrieve precipitation radar with 1km spatial resolution
//! - **Weather Alerts**: Access official weather warnings from DWD
//! - **No API Key Required**: The public API at `https://api.brightsky.dev/` is free
//! - **no_std Compatible**: Works in embedded environments
//!
//! ## Quick Start (with reqwest)
//!
//! ```rust,no_run
//! use brightsky::{CurrentWeatherQueryBuilder, ToBrightSkyUrl, types::CurrentWeatherResponse};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Build the query
//!     let query = CurrentWeatherQueryBuilder::new()
//!         .with_lat_lon((52.52, 13.4))  // Berlin
//!         .build()?;
//!
//!     // Generate URL and fetch with your HTTP client
//!     let url = query.to_url("https://api.brightsky.dev")?;
//!     let response: CurrentWeatherResponse = reqwest::get(url).await?.json().await?;
//!
//!     println!("Temperature: {:?}°C", response.weather.temperature);
//!     Ok(())
//! }
//! ```
//!
//! ## Embedded Usage (no_std with reqwless)
//!
//! ```ignore
//! use brightsky::{CurrentWeatherQueryBuilder, ToBrightSkyUrl, types::CurrentWeatherResponse};
//!
//! let query = CurrentWeatherQueryBuilder::new()
//!     .with_lat_lon((52.52, 13.4))
//!     .build()?;
//!
//! // Generate URL string for embedded HTTP clients
//! let url = query.to_url_string("https://api.brightsky.dev")?;
//!
//! // Use your HTTP client to fetch, then deserialize
//! let response: CurrentWeatherResponse = serde_json::from_slice(&body)?;
//! ```
//!
//! ## Feature Flags
//!
//! - `std` (default): Enable std library support and `url::Url` generation
//! - `reqwest`: Enable `BrightSkyReqwestExt` trait for ergonomic reqwest usage
//! - Without `std`: Only string URL generation available (no_std compatible)
//!
//! ## With reqwest Extension Trait
//!
//! Enable the `reqwest` feature for the most ergonomic API:
//!
//! ```rust,ignore
//! use brightsky::{CurrentWeatherQueryBuilder, ext::BrightSkyReqwestExt, types::CurrentWeatherResponse};
//!
//! let client = reqwest::Client::new();
//! let query = CurrentWeatherQueryBuilder::new()
//!     .with_lat_lon((52.52, 13.4))
//!     .build()?;
//!
//! let response: CurrentWeatherResponse = client.get_brightsky(query).await?;
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::string::String;

#[cfg(feature = "std")]
use url::Url;

pub mod types;

#[cfg(feature = "reqwest")]
pub mod ext;

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

/// Base URL for the Bright Sky API
pub const BRIGHT_SKY_API: &str = "https://api.brightsky.dev";

/// Trait for converting query builders into Bright Sky API URLs.
///
/// This trait is implemented by all query builder types to convert
/// their parameters into properly formatted API URLs.
///
/// # Examples
///
/// ```rust
/// use brightsky::{CurrentWeatherQueryBuilder, ToBrightSkyUrl, BRIGHT_SKY_API};
///
/// // Build a query
/// let query = CurrentWeatherQueryBuilder::new()
///     .with_lat_lon((52.52, 13.4))
///     .build()
///     .unwrap();
///
/// // Convert to URL string (always available, including no_std)
/// let url_string = query.to_url_string(BRIGHT_SKY_API).unwrap();
/// assert!(url_string.contains("/current_weather"));
/// ```
///
/// With the `std` feature, you can also get a `url::Url`:
///
/// ```rust
/// # #[cfg(feature = "std")]
/// # fn main() {
/// use brightsky::{CurrentWeatherQueryBuilder, ToBrightSkyUrl, BRIGHT_SKY_API};
///
/// let query = CurrentWeatherQueryBuilder::new()
///     .with_lat_lon((52.52, 13.4))
///     .build()
///     .unwrap();
///
/// let url = query.to_url(BRIGHT_SKY_API).unwrap();
/// assert_eq!(url.path(), "/current_weather");
/// # }
/// # #[cfg(not(feature = "std"))]
/// # fn main() {}
/// ```
pub trait ToBrightSkyUrl {
    /// Convert the query builder into a `url::Url` for the Bright Sky API.
    ///
    /// Only available with the `std` feature.
    #[cfg(feature = "std")]
    fn to_url(self, host: &str) -> Result<Url, BrightSkyError>;

    /// Convert the query builder into a URL string for the Bright Sky API.
    ///
    /// This is always available and is the primary method for no_std environments.
    fn to_url_string(self, host: &str) -> Result<String, BrightSkyError>;
}

// Keep the old trait name as a deprecated alias for backwards compatibility
#[deprecated(since = "0.2.0", note = "Use ToBrightSkyUrl instead")]
pub trait ToBrightSkyClientUrl: ToBrightSkyUrl {}

#[allow(deprecated)]
impl<T: ToBrightSkyUrl> ToBrightSkyClientUrl for T {}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_current_weather_query_build_valid() {
        let query = CurrentWeatherQueryBuilder::new()
            .with_lat_lon((52.52, 13.4))
            .build();

        assert!(query.is_ok());
        let query = query.unwrap();
        assert!(query.lat.is_some());
        assert!(query.lon.is_some());
    }

    #[test]
    fn test_current_weather_query_build_invalid_lat() {
        let result = CurrentWeatherQueryBuilder::new()
            .with_lat_lon((91.0, 13.4))
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_current_weather_query_build_invalid_lon() {
        let result = CurrentWeatherQueryBuilder::new()
            .with_lat_lon((52.52, 181.0))
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_weather_query_build_valid() {
        let date = NaiveDate::from_ymd_opt(2023, 8, 7).unwrap();
        let query = WeatherQueryBuilder::new()
            .with_date(date)
            .with_lat_lon((52.52, 13.4))
            .build();

        assert!(query.is_ok());
    }

    #[test]
    fn test_weather_query_build_missing_date() {
        let result = WeatherQueryBuilder::new()
            .with_lat_lon((52.52, 13.4))
            .build();

        assert!(result.is_err());
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_url_generation() {
        let query = CurrentWeatherQueryBuilder::new()
            .with_lat_lon((52.52, 13.4))
            .build()
            .unwrap();

        let url = query.to_url(BRIGHT_SKY_API).unwrap();
        assert_eq!(url.path(), "/current_weather");
        assert!(url.query().unwrap().contains("lat=52.52"));
        assert!(url.query().unwrap().contains("lon=13.4"));
    }

    #[test]
    fn test_url_string_generation() {
        let query = CurrentWeatherQueryBuilder::new()
            .with_lat_lon((52.52, 13.4))
            .build()
            .unwrap();

        let url = query.to_url_string(BRIGHT_SKY_API).unwrap();
        assert!(url.contains("/current_weather"));
        assert!(url.contains("lat=52.52"));
        assert!(url.contains("lon=13.4"));
    }
}
