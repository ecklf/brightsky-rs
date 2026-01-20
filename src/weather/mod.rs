//! Weather endpoint (`/weather`) - hourly weather records and forecasts.
//!
//! This module provides access to hourly weather data, including both historical
//! observations and forecasts. The endpoint returns detailed meteorological measurements
//! for specified time ranges and locations.
//!
//! ## Key Features
//!
//! - **Historical data**: Weather observations going back to January 1st, 2010
//! - **Current observations**: Recent hourly measurements from weather stations
//! - **Forecasts**: Weather predictions from DWD's MOSMIX system
//! - **Global coverage**: Forecasts available worldwide (denser coverage in Germany)
//! - **Hourly resolution**: Data points for each hour in the requested time range
//!
//! ## Data Types
//!
//! The endpoint combines different types of weather data:
//! - **Historical**: Past weather observations from measurement stations
//! - **Current**: Recent observations from the current day
//! - **Forecast**: Weather predictions for future dates
//!
//! ## Usage Examples
//!
//! ### Get historical weather data
//! ```rust,no_run
//! use brightsky::{WeatherQueryBuilder, ToBrightSkyUrl, BRIGHT_SKY_API, types::WeatherResponse};
//! use chrono::NaiveDate;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let query = WeatherQueryBuilder::new()
//!         .with_date(NaiveDate::from_ymd_opt(2023, 7, 1).unwrap())
//!         .with_last_date(NaiveDate::from_ymd_opt(2023, 7, 7).unwrap())
//!         .with_lat_lon((52.52, 13.4))  // Berlin
//!         .build()?;
//!
//!     let url = query.to_url(BRIGHT_SKY_API)?;
//!     let response: WeatherResponse = reqwest::get(url).await?.json().await?;
//!
//!     for record in response.weather {
//!         println!("{}: {}C, {}% humidity",
//!             record.timestamp,
//!             record.temperature.unwrap_or(0.0),
//!             record.relative_humidity.unwrap_or(0)
//!         );
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Get weather forecast
//! ```rust,no_run
//! use brightsky::{WeatherQueryBuilder, ToBrightSkyUrl, BRIGHT_SKY_API, types::WeatherResponse};
//! use chrono::{NaiveDate, Utc};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let tomorrow = Utc::now().date_naive() + chrono::Duration::days(1);
//!
//!     let query = WeatherQueryBuilder::new()
//!         .with_date(tomorrow)
//!         .with_dwd_station_id(vec!["01766"])  // Muenster/Osnabrueck
//!         .build()?;
//!
//!     let url = query.to_url(BRIGHT_SKY_API)?;
//!     let response: WeatherResponse = reqwest::get(url).await?.json().await?;
//!
//!     for record in response.weather {
//!         if let Some(prob) = record.precipitation_probability {
//!             println!("{}: {}% chance of rain", record.timestamp, prob);
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```

mod query_builder;
pub use query_builder::*;
