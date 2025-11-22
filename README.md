# BrightSky 🌤️

A Rust client library for the [Bright Sky API](https://brightsky.dev/), providing access to German weather data from the Deutscher Wetterdienst (DWD).

[![Crates.io](https://img.shields.io/crates/v/brightsky.svg)](https://crates.io/crates/brightsky)
[![Documentation](https://docs.rs/brightsky/badge.svg)](https://docs.rs/brightsky)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
brightsky = "0.1.0"
tokio = { version = "1", features = ["full"] }
chrono = { version = "0.4", features = ["serde"] }
```

## Examples

### Current Weather

```rust
use brightsky::{BrightSkyClient, CurrentWeatherQueryBuilder, types::CurrentWeatherResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = BrightSkyClient::new();

    // Get current weather for Berlin
    let query = CurrentWeatherQueryBuilder::new()
        .with_lat_lon((52.52, 13.4))
        .build()?;

    let response = client.get::<CurrentWeatherResponse>(query).await?;
    println!("Temperature: {:?}°C", response.weather.temperature);
    println!("Condition: {:?}", response.weather.condition);

    Ok(())
}
```

### Weather Forecast/History

```rust
use brightsky::{BrightSkyClient, WeatherQueryBuilder, types::UnitType};
use chrono::NaiveDate;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = BrightSkyClient::new();

    let query = WeatherQueryBuilder::new()
        .with_lat_lon((52.52, 13.4))
        .with_date(NaiveDate::from_ymd_opt(2025, 1, 15).unwrap())
        .with_units(UnitType::Si)
        .build()?;

    let response = client.get(query).await?;
    println!("Found {} hourly records", response.weather.len());

    Ok(())
}
```

### Radar Data

```rust
use brightsky::{BrightSkyClient, RadarWeatherQueryBuilder, types::RadarCompressionFormat};
use chrono::{NaiveDateTime, NaiveDate};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = BrightSkyClient::new();

    let datetime = NaiveDate::from_ymd_opt(2025, 1, 15)
        .unwrap()
        .and_hms_opt(12, 0, 0)
        .unwrap();

    let query = RadarWeatherQueryBuilder::new()
        .with_datetime(datetime)
        .with_bbox((47.0, 5.0, 55.0, 16.0)) // Southern Germany to Northern Germany
        .with_format(RadarCompressionFormat::Plain)
        .build()?;

    let response = client.get(query).await?;
    println!("Radar data shape: {}x{}", response.height, response.width);

    Ok(())
}
```

### Alerts

```rust
use brightsky::{BrightSkyClient, AlertsQueryBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = BrightSkyClient::new();

    let query = AlertsQueryBuilder::new()
        .with_lat_lon((52.52, 13.4))
        .build()?;

    let response = client.get(query).await?;
    println!("Found {} active alerts", response.alerts.len());

    for alert in response.alerts {
        println!("Alert: {} - {}", alert.event, alert.headline);
    }

    Ok(())
}
```

## Query Builder Options

### Location Parameters
- **Coordinates**: `.with_lat_lon((lat, lon))`
- **Station IDs**: `.with_dwd_station_id(vec!["01766"])` or `.with_wmo_station_id(vec!["10315"])`
- **Source IDs**: `.with_source_id(vec![1234, 5678])`

### Temporal Parameters
- **Date**: `.with_date(date)` and optionally `.with_last_date(end_date)`
- **DateTime**: `.with_datetime(datetime)` (for radar data)
- **Timezone**: `.with_tz("Europe/Berlin")`

### Data Options
- **Units**: `.with_units(UnitType::Si)` or `.with_units(UnitType::Dwd)`
- **Max Distance**: `.with_max_dist(10000)` (meters, for station queries)
- **Radar Format**: `.with_format(RadarCompressionFormat::Compressed)`

## API Endpoints

| Endpoint | Builder | Response Type | Description |
|----------|---------|---------------|-------------|
| `/current_weather` | `CurrentWeatherQueryBuilder` | `CurrentWeatherResponse` | Current conditions |
| `/weather` | `WeatherQueryBuilder` | `WeatherResponse` | Historical/forecast data |
| `/radar` | `RadarWeatherQueryBuilder` | `RadarResponse` | Precipitation radar |
| `/alerts` | `AlertsQueryBuilder` | `AlertsResponse` | Weather warnings |

## Error Handling

The library uses the `BlindSkyClientError` enum for comprehensive error handling:

```rust
match client.get(query).await {
    Ok(response) => println!("Success: {:?}", response),
    Err(brightsky::BlindSkyClientError::InvalidLatitude(lat)) => {
        eprintln!("Invalid latitude: {}", lat);
    }
    Err(brightsky::BlindSkyClientError::InvalidLongitude(lon)) => {
        eprintln!("Invalid longitude: {}", lon);
    }
    Err(brightsky::BlindSkyClientError::ReqwestError(e)) => {
        eprintln!("Network error: {}", e);
    }
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Data Sources & Attribution

All data is sourced from the DWD (Deutscher Wetterdienst) open data server:
- **SYNOP observations**: Real-time weather station data
- **MOSMIX forecasts**: Numerical weather prediction
- **Radar composites**: RV product precipitation data
- **CAP alerts**: Official weather warnings

Please refer to the [DWD Terms of Use](https://www.dwd.de/EN/service/copyright/copyright_artikel.html) for data usage guidelines.

## Acknowledgements

BrightSky is provided by [Jakob de Maeyer](https://github.com/sponsors/jdemaeyer). Consider sponsoring his work! 

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Links

- [Bright Sky API Documentation](https://brightsky.dev/docs/)
- [DWD Open Data Portal](https://opendata.dwd.de/)
- [Crates.io Package](https://crates.io/crates/brightsky)
- [Documentation](https://docs.rs/brightsky)
