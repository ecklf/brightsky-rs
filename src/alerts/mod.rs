//! Alerts endpoint (`/alerts`) - official weather warnings and alerts.
//!
//! This module provides access to official weather alerts issued by the German Weather
//! Service (DWD). Alerts cover severe weather events like storms, heavy rain, snow,
//! heat waves, and other meteorological hazards.
//!
//! ## Key Features
//!
//! - **Official warnings**: Direct from DWD's alert system in CAP format
//! - **Geographic precision**: Alerts issued for ~11,000 municipality cells
//! - **Multilingual**: Alert text in both German and English
//! - **Severity levels**: From minor advisories to extreme warnings
//! - **Real-time updates**: Current active alerts and all-clear notifications
//!
//! ## Alert System Overview
//!
//! The DWD issues alerts for municipality-based cells (_Gemeinden_) across Germany:
//! - **~11,000 cells** covering all German municipalities
//! - **Cell-based warnings**: Most warnings apply to multiple cells
//! - **Precise timing**: Specific start and end times for weather events
//! - **Detailed information**: Descriptions, instructions, and severity levels
//!
//! ## Severity Levels
//!
//! Alerts are classified by severity:
//! - **Minor**: Limited impact expected
//! - **Moderate**: Some impact possible
//! - **Severe**: Significant impact likely
//! - **Extreme**: Widespread severe impact expected
//!
//! ## Usage Examples
//!
//! ### Get all current alerts
//! ```rust,no_run
//! use brightsky::{BrightSkyClient, AlertsQueryBuilder, types::AlertsResponse};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = BrightSkyClient::new();
//!
//!     let query = AlertsQueryBuilder::new().build()?;
//!     let response = client.get::<AlertsResponse>(query).await?;
//!
//!     println!("Found {} active alerts", response.alerts.len());
//!
//!     for alert in response.alerts {
//!         println!("⚠️ {}", alert.headline_en);
//!         println!("   Severity: {:?}", alert.severity);
//!         println!("   From {} to {}", alert.onset, alert.expires.unwrap_or_default());
//!         println!("   {}", alert.description_en);
//!         println!();
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Get alerts for specific location
//! ```rust
//! use brightsky::{BrightSkyClient, AlertsQueryBuilder, types::AlertsResponse};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = BrightSkyClient::new();
//!
//!     let query = AlertsQueryBuilder::new()
//!         .with_lat_lon((52.52, 13.4))  // Berlin coordinates
//!         .build()?;
//!
//!     let response = client.get::<AlertsResponse>(query).await?;
//!
//!     if let Some(location) = response.location {
//!         println!("Alerts for {}, {}", location.name_short, location.state_short);
//!         println!("Municipality cell ID: {}", location.warn_cell_id);
//!     }
//!
//!     if response.alerts.is_empty() {
//!         println!("No active weather alerts for this location");
//!     } else {
//!         println!("Active alerts:");
//!         for alert in response.alerts {
//!             let severity_emoji = match alert.severity {
//!                 Some(brightsky::types::AlertSeverity::Minor) => "🟡",
//!                 Some(brightsky::types::AlertSeverity::Moderate) => "🟠",
//!                 Some(brightsky::types::AlertSeverity::Severe) => "🔴",
//!                 Some(brightsky::types::AlertSeverity::Extreme) => "🟣",
//!                 None => "⚪",
//!             };
//!
//!             println!("{} {}", severity_emoji, alert.headline_en);
//!
//!             if let Some(instruction) = alert.instruction_en {
//!                 println!("   💡 {}", instruction);
//!             }
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Monitor alerts by warn cell ID
//! ```rust
//! use brightsky::{BrightSkyClient, AlertsQueryBuilder, types::AlertsResponse};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = BrightSkyClient::new();
//!
//!     let query = AlertsQueryBuilder::new()
//!         .with_warn_cell_id(803159016)  // Specific municipality cell
//!         .with_tz("Europe/Berlin")
//!         .build()?;
//!
//!     let response = client.get::<AlertsResponse>(query).await?;
//!
//!     for alert in response.alerts {
//!         println!("Alert ID: {}", alert.alert_id);
//!         println!("Event: {} / {}", alert.event_en.unwrap_or_default(), alert.event_de.unwrap_or_default());
//!         println!("Status: {:?}", alert.status);
//!         println!("Urgency: {:?}", alert.urgency);
//!         println!("Certainty: {:?}", alert.certainty);
//!
//!         println!("\nGerman: {}", alert.headline_de);
//!         println!("{}", alert.description_de);
//!
//!         println!("\nEnglish: {}", alert.headline_en);
//!         println!("{}", alert.description_en);
//!     }
//!
//!     Ok(())
//! }
//! ```

mod query_builder;
pub use query_builder::*;
