# BrightSky

Type-safe query builders and response types for the [Bright Sky API](https://brightsky.dev/), providing access to German weather data from the Deutscher Wetterdienst (DWD).

[![Crates.io](https://img.shields.io/crates/v/brightsky.svg)](https://crates.io/crates/brightsky)
[![Documentation](https://docs.rs/brightsky/badge.svg)](https://docs.rs/brightsky)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Design Philosophy

This crate provides **query builders** and **response types** only. You bring your own HTTP client (reqwest, ureq, reqwless, etc.). This keeps the crate lightweight and works in both std and no_std environments.

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
brightsky = "0.2"
reqwest = { version = "0.13", features = ["json"] }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

## Examples

### Current Weather

```rust
use brightsky::{CurrentWeatherQueryBuilder, ToBrightSkyUrl, BRIGHT_SKY_API, types::CurrentWeatherResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build the query
    let query = CurrentWeatherQueryBuilder::new()
        .with_lat_lon((52.52, 13.4))  // Berlin
        .build()?;

    // Generate URL and fetch with reqwest
    let url = query.to_url(BRIGHT_SKY_API)?;
    let response: CurrentWeatherResponse = reqwest::get(url).await?.json().await?;

    println!("Temperature: {:?}C", response.weather.temperature);
    println!("Condition: {:?}", response.weather.condition);
    Ok(())
}
```

### Weather Forecast/History

```rust
use brightsky::{WeatherQueryBuilder, ToBrightSkyUrl, BRIGHT_SKY_API, types::{WeatherResponse, UnitType}};
use chrono::NaiveDate;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let query = WeatherQueryBuilder::new()
        .with_lat_lon((52.52, 13.4))
        .with_date(NaiveDate::from_ymd_opt(2025, 1, 15).unwrap())
        .with_units(UnitType::Si)
        .build()?;

    let url = query.to_url(BRIGHT_SKY_API)?;
    let response: WeatherResponse = reqwest::get(url).await?.json().await?;

    println!("Found {} hourly records", response.weather.len());
    Ok(())
}
```

### Embedded Usage (no_std)

For embedded systems, use `to_url_string()` instead of `to_url()`:

```rust,ignore
use brightsky::{CurrentWeatherQueryBuilder, ToBrightSkyUrl, BRIGHT_SKY_API, types::CurrentWeatherResponse};

let query = CurrentWeatherQueryBuilder::new()
    .with_lat_lon((52.52, 13.4))
    .build()?;

// Get URL as String (works in no_std)
let url = query.to_url_string(BRIGHT_SKY_API)?;

// Use your HTTP client (reqwless, etc.) to fetch
// Then deserialize with serde_json
let response: CurrentWeatherResponse = serde_json::from_slice(&body)?;
```

## Query Builders

| Endpoint | Builder | Response Type |
|----------|---------|---------------|
| `/current_weather` | `CurrentWeatherQueryBuilder` | `CurrentWeatherResponse` |
| `/weather` | `WeatherQueryBuilder` | `WeatherResponse` |
| `/radar` | `RadarWeatherQueryBuilder` | `RadarResponse` |
| `/alerts` | `AlertsQueryBuilder` | `AlertsResponse` |

### Common Options

- **Location**: `.with_lat_lon((lat, lon))` or `.with_dwd_station_id(vec!["01766"])`
- **Date**: `.with_date(date)` and `.with_last_date(end_date)`
- **Timezone**: `.with_tz("Europe/Berlin")`
- **Units**: `.with_units(UnitType::Si)` or `.with_units(UnitType::Dwd)`

## Feature Flags

- `std` (default): Enables `url::Url` support via `to_url()` method
- Without `std`: Only `to_url_string()` available (no_std compatible)

## Data Sources

All data is sourced from the DWD open data server:
- **SYNOP observations**: Real-time weather station data
- **MOSMIX forecasts**: Numerical weather prediction
- **Radar composites**: RV product precipitation data
- **CAP alerts**: Official weather warnings

Please refer to the [DWD Terms of Use](https://www.dwd.de/EN/service/copyright/copyright_artikel.html) for data usage guidelines.

## Acknowledgements

BrightSky API is provided by [Jakob de Maeyer](https://github.com/sponsors/jdemaeyer). Consider sponsoring his work!

## License

MIT License - see [LICENSE](LICENSE) for details.
