//! Current weather endpoint (`/current_weather`) - real-time weather conditions.
//!
//! This module provides access to current weather data compiled from SYNOP observations
//! from the past 1.5 hours. Unlike other endpoints, this provides a best-effort solution
//! to reflect current weather conditions by aggregating recent observations.
//!
//! ## Key Features
//!
//! - **Real-time data**: Compiled from SYNOP observations within the last 1.5 hours
//! - **Multiple time intervals**: Provides measurements at 10, 30, and 60-minute intervals where available
//! - **Fallback sources**: Uses alternative weather stations to fill missing data gaps
//! - **Derived fields**: Includes calculated `condition` and `icon` fields for easy display
//!
//! ## Data Sources
//!
//! Data comes from SYNOP (Surface Synoptic Observations) stations that report weather
//! conditions every 10 minutes. The current weather endpoint compiles these recent
//! observations into a single comprehensive record.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use brightsky::{CurrentWeatherQueryBuilder, ToBrightSkyUrl, BRIGHT_SKY_API, types::CurrentWeatherResponse};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let query = CurrentWeatherQueryBuilder::new()
//!         .with_lat_lon((52.52, 13.4))  // Berlin coordinates
//!         .build()?;
//!
//!     let url = query.to_url(BRIGHT_SKY_API)?;
//!     let response: CurrentWeatherResponse = reqwest::get(url).await?.json().await?;
//!
//!     println!("Current temperature: {:?}°C", response.weather.temperature);
//!     println!("Conditions: {:?}", response.weather.condition);
//!     println!("Wind speed (10min): {:?} km/h", response.weather.wind_speed_10);
//!
//!     Ok(())
//! }
//! ```

mod query_builder;
pub use query_builder::*;
