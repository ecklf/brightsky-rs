//! Type definitions for Bright Sky API responses and parameters.
//!
//! This module contains all the data structures used for communicating with
//! the Bright Sky API, including request parameters, response types, and
//! various enumerations for weather data.

use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Deserializer, Serialize};
use std::io::Read;

/// Format options for radar precipitation data encoding.
///
/// Determines how the precipitation data is encoded in the `precipitation_5` field
/// of radar responses. Different formats offer trade-offs between response size
/// and processing complexity.
#[derive(Debug, PartialEq)]
pub enum RadarCompressionFormat {
    /// Base64-encoded, zlib-compressed bytestring of 2-byte integers.
    /// This is the most efficient format in terms of response size and should
    /// be used whenever possible.
    Compressed,
    /// Base64-encoded bytestring of 2-byte integers without compression.
    /// Use when you want to avoid decompression but still need binary efficiency.
    Bytes,
    /// Nested array of integers returned directly as JSON.
    /// Simplest to process but largest response size. Best for small bounding boxes.
    Plain,
}

/// Represents precipitation data that may be in different compressed formats.
///
/// This enum handles the different ways radar precipitation data can be encoded
/// in API responses, automatically detecting and parsing the appropriate format.
/// Values represent 0.01 mm / 5 min precipitation amounts.
#[derive(Debug, Clone, PartialEq)]
pub enum MaybeCompressedPrecipitation {
    /// Zlib-compressed precipitation data as 16-bit integers
    Compressed(Vec<u16>),
    /// Uncompressed precipitation data as 16-bit integers
    Bytes(Vec<u16>),
    /// Plain 2D array of precipitation values
    Plain(Vec<Vec<u16>>),
}

impl<'de> Deserialize<'de> for MaybeCompressedPrecipitation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;

        match value {
            // If it is a nested array, treat it as plain 2D array
            serde_json::Value::Array(outer) => {
                let mut result = Vec::new();
                for inner in outer {
                    if let serde_json::Value::Array(inner_array) = inner {
                        let mut row = Vec::new();
                        for v in inner_array {
                            let val = v
                                .as_u64()
                                .ok_or_else(|| serde::de::Error::custom("Invalid array element"))?
                                as u16;
                            row.push(val);
                        }
                        result.push(row);
                    } else {
                        return Err(serde::de::Error::custom("Expected nested array"));
                    }
                }
                Ok(MaybeCompressedPrecipitation::Plain(result))
            }
            // Otherwise treat it as base64 string (compressed or bytes format)
            serde_json::Value::String(s) => {
                let decoded = general_purpose::STANDARD
                    .decode(&s)
                    .map_err(|e| serde::de::Error::custom(format!("Base64 decode error: {}", e)))?;

                // Attempt to decompress using zlib (which is the default return format)
                let mut decoder = flate2::read::ZlibDecoder::new(&decoded[..]);
                let mut decompressed = Vec::new();
                if decoder.read_to_end(&mut decompressed).is_ok() {
                    let values: Vec<u16> = decompressed
                        .chunks_exact(2)
                        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                        .collect();
                    return Ok(MaybeCompressedPrecipitation::Compressed(values));
                }

                // If decompression fails, treat it as raw bytes
                let values: Vec<u16> = decoded
                    .chunks_exact(2)
                    .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                    .collect();
                Ok(MaybeCompressedPrecipitation::Bytes(values))
            }
            _ => Err(serde::de::Error::custom("Expected string or array")),
        }
    }
}

/// Weather condition icons suitable for display in weather applications.
///
/// Unlike numerical parameters, this field is calculated from different fields
/// in the raw data as a best effort approach. Not all values are available for
/// all source types.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum WeatherIcon {
    /// Clear sky during daytime
    ClearDay,
    /// Clear sky during nighttime
    ClearNight,
    /// Partly cloudy during daytime
    PartlyCloudyDay,
    /// Partly cloudy during nighttime
    PartlyCloudyNight,
    /// Overcast/cloudy conditions
    Cloudy,
    /// Foggy conditions with reduced visibility
    Fog,
    /// Windy conditions
    Wind,
    /// Rainy conditions
    Rain,
    /// Sleet (mixed rain and snow)
    Sleet,
    /// Snowy conditions
    Snow,
    /// Hail conditions
    Hail,
    /// Thunderstorm conditions
    Thunderstorm,
    /// Unknown or unrecognized weather condition
    Unknown,
}

impl<'de> Deserialize<'de> for WeatherIcon {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: String = String::deserialize(deserializer)?;
        match value.as_str() {
            "clear-day" => Ok(WeatherIcon::ClearDay),
            "clear-night" => Ok(WeatherIcon::ClearNight),
            "partly-cloudy-day" => Ok(WeatherIcon::PartlyCloudyDay),
            "partly-cloudy-night" => Ok(WeatherIcon::PartlyCloudyNight),
            "cloudy" => Ok(WeatherIcon::Cloudy),
            "fog" => Ok(WeatherIcon::Fog),
            "wind" => Ok(WeatherIcon::Wind),
            "rain" => Ok(WeatherIcon::Rain),
            "sleet" => Ok(WeatherIcon::Sleet),
            "snow" => Ok(WeatherIcon::Snow),
            "hail" => Ok(WeatherIcon::Hail),
            "thunderstorm" => Ok(WeatherIcon::Thunderstorm),
            // For null or unknown values
            _ => Ok(WeatherIcon::Unknown),
        }
    }
}

/// Current weather conditions derived from meteorological observations.
///
/// Unlike numerical parameters, this field is calculated from different fields
/// in the raw data as a best effort approach. Not all values are available for
/// all source types.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum WeatherCondition {
    /// Dry conditions with no precipitation
    Dry,
    /// Foggy conditions
    Fog,
    /// Rainy conditions
    Rain,
    /// Sleet (mixed rain and snow)
    Sleet,
    /// Snowy conditions
    Snow,
    /// Hail conditions
    Hail,
    /// Thunderstorm conditions
    Thunderstorm,
    /// Unknown or unrecognized condition
    Unknown,
}

impl<'de> Deserialize<'de> for WeatherCondition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: String = String::deserialize(deserializer)?;
        match value.as_str() {
            "dry" => Ok(WeatherCondition::Dry),
            "fog" => Ok(WeatherCondition::Fog),
            "rain" => Ok(WeatherCondition::Rain),
            "sleet" => Ok(WeatherCondition::Sleet),
            "snow" => Ok(WeatherCondition::Snow),
            "hail" => Ok(WeatherCondition::Hail),
            "thunderstorm" => Ok(WeatherCondition::Thunderstorm),
            _ => Ok(WeatherCondition::Unknown),
        }
    }
}

/// Type of meteorological observation or data source.
///
/// Indicates the nature and time characteristics of the weather data source.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum ObservationType {
    /// Historical weather observations from past measurements
    Historical,
    /// Current weather data from recent observations
    Current,
    /// SYNOP observations (Surface Synoptic Observations) - real-time station reports
    Synop,
    /// Weather forecast data
    Forecast,
}

impl<'de> Deserialize<'de> for ObservationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: String = String::deserialize(deserializer)?;
        match value.as_str() {
            "historical" => Ok(ObservationType::Historical),
            "current" => Ok(ObservationType::Current),
            "synop" => Ok(ObservationType::Synop),
            "forecast" => Ok(ObservationType::Forecast),
            _ => Err(serde::de::Error::custom("Invalid observation type")),
        }
    }
}

/// Physical units system for meteorological parameters.
///
/// Determines the unit system used for returned meteorological data.
/// The `dwd` system uses units common in meteorological applications,
/// while `si` uses International System of Units (with precipitation always in mm).
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum UnitType {
    /// International System of Units (SI)
    /// - Temperature: Kelvin (K)
    /// - Pressure: Pascal (Pa)
    /// - Wind speed: m/s
    /// - Solar irradiation: J/m²
    /// - Sunshine: seconds
    /// - Precipitation: mm (exception to SI)
    Si,
    /// DWD (German Weather Service) standard units
    /// - Temperature: Celsius (°C)
    /// - Pressure: hectopascal (hPa)
    /// - Wind speed: km/h
    /// - Solar irradiation: kWh/m²
    /// - Sunshine: minutes
    /// - Precipitation: mm
    Dwd,
}

impl<'de> Deserialize<'de> for UnitType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: String = String::deserialize(deserializer)?;
        match value.as_str() {
            "si" => Ok(UnitType::Si),
            "dwd" => Ok(UnitType::Dwd),
            _ => Err(serde::de::Error::custom("Invalid unit type")),
        }
    }
}

/// Response structure for data returned by the `/weather` endpoint.
///
/// Contains hourly weather records and/or forecasts for the requested time range,
/// along with information about the weather stations used as data sources.
///
/// ## Example
///
/// ```rust
/// use brightsky::{BrightSkyClient, WeatherQueryBuilder};
/// use brightsky::types::WeatherResponse;
/// use chrono::NaiveDate;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = BrightSkyClient::new();
///
///     let query = WeatherQueryBuilder::new()
///         .with_lat_lon((52.52, 13.4))  // Berlin
///         .with_date(NaiveDate::from_ymd_opt(2023, 8, 7).unwrap())
///         .build()?;
///
///     let response: WeatherResponse = client.get(query).await?;
///
///     for record in response.weather {
///         println!("Time: {}, Temp: {:?}°C", record.timestamp, record.temperature);
///     }
///
///     Ok(())
/// }
/// ```
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeatherResponse {
    /// List of hourly weather records/forecasts for the requested period
    pub weather: Vec<Weather>,
    /// Information about weather stations used as data sources
    pub sources: Vec<Source>,
}

/// A single hourly weather record containing meteorological measurements and/or forecasts.
///
/// Contains various weather parameters measured or forecasted for a specific hour.
/// Many fields may be `None` depending on the data source and measurement capabilities.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Weather {
    /// ISO 8601 formatted timestamp of this weather record
    pub timestamp: String,
    /// Bright Sky source ID for this record
    pub source_id: i64,
    /// Total cloud cover at timestamp (percentage)
    pub cloud_cover: Option<f64>,
    /// Current weather conditions (derived field)
    pub condition: Option<WeatherCondition>,
    /// Dew point at timestamp, 2m above ground (°C or K depending on units)
    pub dew_point: Option<f64>,
    /// Icon alias suitable for current weather conditions (derived field)
    pub icon: Option<WeatherIcon>,
    /// Atmospheric pressure at timestamp, reduced to mean sea level (hPa or Pa)
    pub pressure_msl: Option<f64>,
    /// Relative humidity at timestamp (percentage)
    pub relative_humidity: Option<i64>,
    /// Air temperature at timestamp, 2m above ground (°C or K)
    pub temperature: Option<f64>,
    /// Visibility at timestamp (meters)
    pub visibility: Option<i64>,
    /// Mapping of parameters to alternative source IDs used for missing values
    pub fallback_source_ids: Option<std::collections::HashMap<String, i64>>,
    /// Total precipitation during previous 60 minutes (mm)
    pub precipitation: Option<f64>,
    /// Solar irradiation during previous 60 minutes (kWh/m² or J/m²)
    pub solar: Option<f64>,
    /// Sunshine duration during previous 60 minutes (minutes or seconds)
    pub sunshine: Option<f64>,
    /// Mean wind direction during previous hour, 10m above ground (degrees)
    pub wind_direction: Option<i64>,
    /// Mean wind speed during previous hour, 10m above ground (km/h or m/s)
    pub wind_speed: Option<f64>,
    /// Direction of maximum wind gust during previous hour, 10m above ground (degrees)
    pub wind_gust_direction: Option<i64>,
    /// Speed of maximum wind gust during previous hour, 10m above ground (km/h or m/s)
    pub wind_gust_speed: Option<f64>,
    /// Probability of >0.1mm precipitation in previous hour (percentage, forecasts only)
    pub precipitation_probability: Option<i64>,
    /// Probability of >0.2mm precipitation in previous 6 hours (percentage, forecasts only, at 0/6/12/18 UTC)
    pub precipitation_probability_6h: Option<i64>,
}

/// Information about a weather data source (typically a weather station).
///
/// Contains metadata about weather stations or other data sources used
/// to provide weather measurements and forecasts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Source {
    /// Bright Sky source ID
    pub id: i64,
    /// DWD weather station ID (typically 5 alphanumeric characters)
    pub dwd_station_id: Option<String>,
    /// WMO weather station ID (typically 5 alphanumeric characters)
    pub wmo_station_id: Option<String>,
    /// Human-readable weather station name
    pub station_name: Option<String>,
    /// Type of observations provided by this source
    pub observation_type: ObservationType,
    /// ISO 8601 timestamp of first available record for this source
    pub first_record: String,
    /// ISO 8601 timestamp of latest available record for this source
    pub last_record: String,
    /// Station latitude in decimal degrees
    pub lat: f64,
    /// Station longitude in decimal degrees
    pub lon: f64,
    /// Station height above sea level in meters
    pub height: f64,
    /// Distance to requested lat/lon in meters (when applicable)
    pub distance: Option<f64>,
}

/// Response structure for data returned by the `/current_weather` endpoint.
///
/// Returns current weather conditions compiled from recent SYNOP observations
/// from the past 1.5 hours, providing a best-effort representation of current weather.
///
/// ## Example
///
/// ```rust
/// use brightsky::{BrightSkyClient, CurrentWeatherQueryBuilder};
/// use brightsky::types::CurrentWeatherResponse;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = BrightSkyClient::new();
///
///     let query = CurrentWeatherQueryBuilder::new()
///         .with_lat_lon((52.52, 13.4))  // Berlin coordinates
///         .build()?;
///
///     let response: CurrentWeatherResponse = client.get(query).await?;
///
///     println!("Current temperature: {:?}°C", response.weather.temperature);
///     println!("Conditions: {:?}", response.weather.condition);
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentWeatherResponse {
    /// Current weather conditions compiled from recent observations
    pub weather: CurrentWeather,
    /// Information about weather stations used as data sources
    pub sources: Vec<CurrentWeatherSource>,
}

/// Current weather conditions compiled from recent SYNOP observations.
///
/// Unlike regular weather records, current weather provides measurements
/// at multiple time intervals (10, 30, and 60 minutes) where available,
/// compiled from SYNOP observations from the past 1.5 hours.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CurrentWeather {
    /// ISO 8601 formatted timestamp of this weather record
    pub timestamp: String,
    /// Bright Sky source ID for this record
    pub source_id: i64,
    /// Total cloud cover at timestamp (percentage)
    pub cloud_cover: Option<f64>,
    /// Current weather conditions (derived field)
    pub condition: Option<WeatherCondition>,
    /// Dew point at timestamp, 2m above ground (°C or K)
    pub dew_point: Option<f64>,
    /// Icon alias suitable for current weather conditions (derived field)
    pub icon: Option<WeatherIcon>,
    /// Atmospheric pressure at timestamp, reduced to mean sea level (hPa or Pa)
    pub pressure_msl: Option<f64>,
    /// Relative humidity at timestamp (percentage)
    pub relative_humidity: Option<i64>,
    /// Air temperature at timestamp, 2m above ground (°C or K)
    pub temperature: Option<f64>,
    /// Visibility at timestamp (meters)
    pub visibility: Option<i64>,
    /// Mapping of meteorological parameters to alternative source IDs
    /// used to fill missing values in the main source
    pub fallback_source_ids: Option<std::collections::HashMap<String, i64>>,
    /// Total precipitation during previous 10 minutes (mm)
    pub precipitation_10: Option<f64>,
    /// Total precipitation during previous 30 minutes (mm)
    pub precipitation_30: Option<f64>,
    /// Total precipitation during previous 60 minutes (mm)
    pub precipitation_60: Option<f64>,
    /// Solar irradiation during previous 10 minutes (kWh/m² or J/m²)
    pub solar_10: Option<f64>,
    /// Solar irradiation during previous 30 minutes (kWh/m² or J/m²)
    pub solar_30: Option<f64>,
    /// Solar irradiation during previous 60 minutes (kWh/m² or J/m²)
    pub solar_60: Option<f64>,
    /// Sunshine duration during previous 30 minutes (minutes or seconds)
    pub sunshine_30: Option<f64>,
    /// Sunshine duration during previous 60 minutes (minutes or seconds)
    pub sunshine_60: Option<f64>,
    /// Mean wind direction during previous 10 minutes, 10m above ground (degrees)
    pub wind_direction_10: Option<i64>,
    /// Mean wind direction during previous 30 minutes, 10m above ground (degrees)
    pub wind_direction_30: Option<i64>,
    /// Mean wind direction during previous 60 minutes, 10m above ground (degrees)
    pub wind_direction_60: Option<i64>,
    /// Mean wind speed during previous 10 minutes, 10m above ground (km/h or m/s)
    pub wind_speed_10: Option<f64>,
    /// Mean wind speed during previous 30 minutes, 10m above ground (km/h or m/s)
    pub wind_speed_30: Option<f64>,
    /// Mean wind speed during previous 60 minutes, 10m above ground (km/h or m/s)
    pub wind_speed_60: Option<f64>,
    /// Direction of maximum wind gust during previous 10 minutes, 10m above ground (degrees)
    pub wind_gust_direction_10: Option<i64>,
    /// Direction of maximum wind gust during previous 30 minutes, 10m above ground (degrees)
    pub wind_gust_direction_30: Option<i64>,
    /// Direction of maximum wind gust during previous 60 minutes, 10m above ground (degrees)
    pub wind_gust_direction_60: Option<i64>,
    /// Speed of maximum wind gust during previous 10 minutes, 10m above ground (km/h or m/s)
    pub wind_gust_speed_10: Option<f64>,
    /// Speed of maximum wind gust during previous 30 minutes, 10m above ground (km/h or m/s)
    pub wind_gust_speed_30: Option<f64>,
    /// Speed of maximum wind gust during previous 60 minutes, 10m above ground (km/h or m/s)
    pub wind_gust_speed_60: Option<f64>,
}

/// Information about a current weather data source.
///
/// Similar to `Source` but with guaranteed non-optional station identification fields
/// for current weather endpoints that specifically work with SYNOP stations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CurrentWeatherSource {
    /// Bright Sky source ID
    pub id: i64,
    /// DWD weather station ID (always present for current weather sources)
    pub dwd_station_id: String,
    /// WMO weather station ID (always present for current weather sources)
    pub wmo_station_id: String,
    /// Human-readable weather station name (always present for current weather sources)
    pub station_name: String,
    /// Type of observations provided by this source
    pub observation_type: ObservationType,
    /// ISO 8601 timestamp of first available record for this source
    pub first_record: String,
    /// ISO 8601 timestamp of latest available record for this source
    pub last_record: String,
    /// Station latitude in decimal degrees
    pub lat: f64,
    /// Station longitude in decimal degrees
    pub lon: f64,
    /// Station height above sea level in meters
    pub height: f64,
    /// Distance to requested lat/lon in meters (when applicable)
    pub distance: Option<f64>,
}

/// Response structure for data returned by the `/radar` endpoint.
///
/// Contains radar rainfall data with 1km spatial and 5-minute temporal resolution,
/// including forecasts for the next two hours. Past radar records are kept for 6 hours.
///
/// Values in `precipitation_5` represent 0.01 mm / 5 min. For example, a value of 45
/// means 0.45 mm of precipitation fell in that square kilometer during those 5 minutes.
///
/// ## Example
///
/// ```rust
/// use brightsky::{BrightSkyClient, RadarWeatherQueryBuilder};
/// use brightsky::types::RadarResponse;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = BrightSkyClient::new();
///
///     let query = RadarWeatherQueryBuilder::new()
///         .with_lat_lon((52.0, 7.6))  // Near Münster
///         .build()?;
///
///     let response: RadarResponse = client.get(query).await?;
///
///     for record in response.radar {
///         println!("Radar timestamp: {}", record.timestamp);
///         // Process precipitation data...
///     }
///
///     Ok(())
/// }
/// ```
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RadarResponse {
    /// List of radar records with precipitation data
    pub radar: Vec<Radar>,
    /// GeoJSON-formatted bounding box showing lat/lon coordinates of the four corners
    pub geometry: Option<Geometry>,
    /// Bounding box (top, left, bottom, right) in pixels when lat/lon was supplied
    pub bbox: Option<Vec<i64>>,
    /// Exact x-y position of the supplied coordinates when lat/lon was supplied
    pub latlon_position: Option<LatlonPosition>,
}

/// A single radar measurement record with precipitation data.
///
/// Contains 5-minute precipitation data for a specific timestamp, with values
/// representing 0.01 mm / 5 min precipitation amounts.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Radar {
    /// ISO 8601 formatted timestamp of this radar record
    pub timestamp: String,
    /// Unique identifier for DWD radar product source (e.g., "RADOLAN::RV::2023-08-08T11:45:00+00:00")
    pub source: String,
    /// 5-minute precipitation data in various possible formats (compressed/bytes/plain)
    /// Values represent 0.01 mm / 5 min
    pub precipitation_5: MaybeCompressedPrecipitation,
}

/// GeoJSON geometry representing the bounding box of radar data.
///
/// Contains the geographic coordinates of the four corners of the returned radar data area.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Geometry {
    /// GeoJSON geometry type (typically "Polygon")
    #[serde(rename = "type")]
    pub geometry_type: String,
    /// Array of coordinate pairs [longitude, latitude] defining the polygon corners
    pub coordinates: Vec<Vec<f64>>,
}

/// Exact pixel position within the radar grid for a given lat/lon coordinate.
///
/// Returned when lat/lon coordinates are provided to indicate the precise
/// position within the radar data grid.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LatlonPosition {
    /// X coordinate within the radar grid
    pub x: f64,
    /// Y coordinate within the radar grid
    pub y: f64,
}

/// Status of a weather alert.
///
/// Indicates whether this is a real alert or a test message.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AlertStatus {
    /// Real, active weather alert
    Actual,
    /// Test alert message
    Test,
}

impl<'de> Deserialize<'de> for AlertStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: String = String::deserialize(deserializer)?;
        match value.as_str() {
            "actual" => Ok(AlertStatus::Actual),
            "test" => Ok(AlertStatus::Test),
            _ => Err(serde::de::Error::custom("Invalid alert status")),
        }
    }
}

/// Category of weather alert.
///
/// Classifies alerts by their primary domain.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AlertCategory {
    /// Meteorological alert (weather-related)
    Met,
    /// Public health related alert
    Health,
}

impl<'de> Deserialize<'de> for AlertCategory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: String = String::deserialize(deserializer)?;
        match value.as_str() {
            "met" => Ok(AlertCategory::Met),
            "health" => Ok(AlertCategory::Health),
            _ => Err(serde::de::Error::custom("Invalid alert category")),
        }
    }
}

/// Recommended response type for a weather alert.
///
/// Indicates what type of action is recommended for the target audience.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AlertResponseType {
    /// Take preparatory action
    Prepare,
    /// All clear - previous alert conditions have ended
    AllClear,
    /// No specific action recommended
    None,
    /// Monitor the situation
    Monitor,
}

impl<'de> Deserialize<'de> for AlertResponseType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: String = String::deserialize(deserializer)?;
        match value.as_str() {
            "prepare" => Ok(AlertResponseType::Prepare),
            "allclear" => Ok(AlertResponseType::AllClear),
            "none" => Ok(AlertResponseType::None),
            "monitor" => Ok(AlertResponseType::Monitor),
            _ => Err(serde::de::Error::custom("Invalid alert response type")),
        }
    }
}

/// Urgency level of a weather alert.
///
/// Indicates the time frame for the expected weather event.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AlertUrgency {
    /// Immediate threat or event in progress
    Immediate,
    /// Future threat, advance warning
    Future,
}

impl<'de> Deserialize<'de> for AlertUrgency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: String = String::deserialize(deserializer)?;
        match value.as_str() {
            "immediate" => Ok(AlertUrgency::Immediate),
            "future" => Ok(AlertUrgency::Future),
            _ => Err(serde::de::Error::custom("Invalid alert urgency")),
        }
    }
}

/// Severity level of a weather alert.
///
/// Indicates the expected intensity and potential impact of the weather event.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AlertSeverity {
    /// Minor impact expected
    Minor,
    /// Moderate impact possible
    Moderate,
    /// Severe impact likely
    Severe,
    /// Extreme impact expected
    Extreme,
}

impl<'de> Deserialize<'de> for AlertSeverity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: String = String::deserialize(deserializer)?;
        match value.as_str() {
            "minor" => Ok(AlertSeverity::Minor),
            "moderate" => Ok(AlertSeverity::Moderate),
            "severe" => Ok(AlertSeverity::Severe),
            "extreme" => Ok(AlertSeverity::Extreme),
            _ => Err(serde::de::Error::custom("Invalid alert severity")),
        }
    }
}

/// Certainty level of a weather alert.
///
/// Indicates the confidence in the occurrence of the forecasted event.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AlertCertainty {
    /// Event has been observed and is occurring
    Observed,
    /// Event is likely to occur (forecast)
    Likely,
}

impl<'de> Deserialize<'de> for AlertCertainty {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: String = String::deserialize(deserializer)?;
        match value.as_str() {
            "observed" => Ok(AlertCertainty::Observed),
            "likely" => Ok(AlertCertainty::Likely),
            _ => Err(serde::de::Error::custom("Invalid alert certainty")),
        }
    }
}

/// An individual weather alert issued by DWD.
///
/// Contains complete information about a weather warning, including severity,
/// timing, affected areas, and descriptive text in both German and English.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Alert {
    /// Bright Sky internal ID for this alert
    pub id: i64,
    /// Unique CAP (Common Alerting Protocol) message identifier
    pub alert_id: String,
    /// Alert status (actual warning or test)
    pub status: AlertStatus,
    /// ISO 8601 timestamp when alert was issued
    pub effective: String,
    /// ISO 8601 timestamp when weather event is expected to begin
    pub onset: String,
    /// ISO 8601 timestamp when weather event is expected to end
    pub expires: Option<String>,
    /// Alert category (meteorological or health-related)
    pub category: Option<AlertCategory>,
    /// Recommended response type for the target audience
    pub response_type: Option<AlertResponseType>,
    /// Urgency of the alert (immediate or future)
    pub urgency: Option<AlertUrgency>,
    /// Severity level of the expected weather event
    pub severity: Option<AlertSeverity>,
    /// Certainty level of the forecast
    pub certainty: Option<AlertCertainty>,
    /// DWD internal event code
    pub event_code: Option<i64>,
    /// English label for the DWD event code (e.g., "wind gusts")
    pub event_en: Option<String>,
    /// German label for the DWD event code (e.g., "WINDBÖEN")
    pub event_de: Option<String>,
    /// Alert headline in English
    pub headline_en: String,
    /// Alert headline in German
    pub headline_de: String,
    /// Detailed alert description in English
    pub description_en: String,
    /// Detailed alert description in German
    pub description_de: String,
    /// Additional safety instructions in English
    pub instruction_en: Option<String>,
    /// Additional safety instructions in German
    pub instruction_de: Option<String>,
}

/// Geographic location information for weather alerts.
///
/// Provides details about the municipality and administrative divisions
/// for a given location, used in conjunction with weather alerts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Location {
    /// Municipality warn cell ID (based on German _Gemeinden_)
    pub warn_cell_id: i64,
    /// Full municipality name (e.g., "Stadt Göttingen")
    pub name: String,
    /// Shortened municipality name (e.g., "Göttingen")
    pub name_short: String,
    /// District name (e.g., "Göttingen")
    pub district: String,
    /// Full federal state name (e.g., "Niedersachsen")
    pub state: String,
    /// Federal state abbreviation (e.g., "NI")
    pub state_short: String,
}

/// Response structure for data returned by the `/alerts` endpoint.
///
/// Contains weather alerts for the requested location or all alerts if no location specified.
/// When location parameters are provided, additional location information is included.
///
/// ## Example
///
/// ```rust
/// use brightsky::{BrightSkyClient, AlertsQueryBuilder};
/// use brightsky::types::AlertsResponse;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = BrightSkyClient::new();
///
///     let query = AlertsQueryBuilder::new()
///         .with_lat_lon((52.52, 13.4))  // Berlin coordinates
///         .build()?;
///
///     let response: AlertsResponse = client.get(query).await?;
///
///     for alert in response.alerts {
///         let severity_str = match alert.severity {
///             Some(s) => format!("{:?}", s),
///             None => "Unknown".to_string(),
///         };
///         println!("Alert: {} ({})", alert.headline_en, severity_str);
///     }
///
///     if let Some(location) = response.location {
///         println!("Location: {}, {}", location.name_short, location.state_short);
///     }
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlertsResponse {
    /// List of weather alerts
    pub alerts: Vec<Alert>,
    /// Location information when lat/lon or warn_cell_id was provided
    pub location: Option<Location>,
}
