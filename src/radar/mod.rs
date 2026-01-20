//! Radar endpoint (`/radar`) - high-resolution rainfall radar data.
//!
//! This module provides access to DWD's rainfall radar data with 1km spatial and
//! 5-minute temporal resolution, including forecasts for the next two hours.
//! The data covers Germany and surrounding regions with exceptional detail.
//!
//! ## Key Features
//!
//! - **High resolution**: 1km² pixels, 5-minute updates
//! - **Large coverage**: 1200×1100 pixel grid covering Germany and borders
//! - **Real-time**: Current precipitation measurements from radar network
//! - **Short-term forecasts**: 2-hour precipitation forecasts
//! - **Multiple formats**: Compressed, bytes, or plain JSON formats
//!
//! ## Important Considerations
//!
//! - **Data size**: Full grid contains 1.32 million pixels - use bounding boxes!
//! - **Projection**: Uses polar stereographic projection (not mercator)
//! - **Units**: Values represent 0.01 mm / 5 min (value 45 = 0.45mm in 5 minutes)
//! - **Retention**: Past records kept for only 6 hours
//!
//! ## Coordinate System
//!
//! The radar grid uses a polar stereographic projection with these properties:
//! - **Proj string**: `+proj=stere +lat_0=90 +lat_ts=60 +lon_0=10 +a=6378137 +b=6356752.3142451802 +no_defs +x_0=543196.83521776402 +y_0=3622588.8619310018`
//! - **X range**: -500 to 1,099,500 (left to right)
//! - **Y range**: 500 to -1,199,500 (top to bottom)
//! - **Pixel size**: 1000×1000 meters (1 km²)
//!
//! ## Usage Examples
//!
//! ### Basic radar query with compressed data (recommended)
//! ```rust,no_run
//! use brightsky::{RadarWeatherQueryBuilder, ToBrightSkyUrl, BRIGHT_SKY_API, types::{RadarCompressionFormat, RadarResponse}};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let query = RadarWeatherQueryBuilder::new()
//!         .with_lat_lon((52.0, 7.6))  // Near Münster
//!         .with_distance(50_000)      // 50km radius
//!         .with_compression_format(RadarCompressionFormat::Compressed)
//!         .build()?;
//!
//!     let url = query.to_url(BRIGHT_SKY_API)?;
//!     let response: RadarResponse = reqwest::get(url).await?.json().await?;
//!
//!     for record in response.radar {
//!         println!("Radar data at {}: {} pixels",
//!             record.timestamp,
//!             match &record.precipitation_5 {
//!                 brightsky::types::MaybeCompressedPrecipitation::Compressed(data) => data.len(),
//!                 brightsky::types::MaybeCompressedPrecipitation::Bytes(data) => data.len(),
//!                 brightsky::types::MaybeCompressedPrecipitation::Plain(data) =>
//!                     data.iter().map(|row| row.len()).sum::<usize>(),
//!             }
//!         );
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Custom bounding box for specific area
//! ```rust,no_run
//! use brightsky::{RadarWeatherQueryBuilder, ToBrightSkyUrl, BRIGHT_SKY_API, types::{RadarCompressionFormat, RadarResponse}};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let query = RadarWeatherQueryBuilder::new()
//!         .with_bbox(vec![100, 100, 300, 300])  // 200x200 pixel area
//!         .with_compression_format(RadarCompressionFormat::Plain)  // For easy processing
//!         .build()?;
//!
//!     let url = query.to_url(BRIGHT_SKY_API)?;
//!     let response: RadarResponse = reqwest::get(url).await?.json().await?;
//!
//!     // Process precipitation data
//!     for record in response.radar {
//!         if let brightsky::types::MaybeCompressedPrecipitation::Plain(grid) = record.precipitation_5 {
//!             println!("Processing {}x{} precipitation grid for {}",
//!                 grid.len(),
//!                 grid.first().map_or(0, |row| row.len()),
//!                 record.timestamp
//!             );
//!
//!             // Find maximum precipitation in the grid
//!             let max_precip = grid.iter()
//!                 .flat_map(|row| row.iter())
//!                 .max()
//!                 .unwrap_or(&0);
//!             println!("Max precipitation: {} (0.01mm/5min)", max_precip);
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Data Processing Notes
//!
//! When working with compressed radar data, you'll need to:
//! 1. Decode the base64 string
//! 2. Decompress using zlib (for compressed format)
//! 3. Convert bytes to 16-bit integers (little-endian)
//! 4. Reshape into 2D grid based on your bounding box dimensions

mod query_builder;
pub use query_builder::*;
