#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{format, string::String, string::ToString, vec::Vec};

use crate::{BrightSkyError, ToBrightSkyClientUrl, types::UnitType};
use chrono::NaiveDate;

#[cfg(feature = "std")]
use url::Url;

/// Query builder for the weather endpoint (`/weather`).
///
/// This builder constructs queries to retrieve hourly weather records and/or forecasts
/// for a specified time range. The endpoint returns both historical observations and
/// weather forecasts depending on the requested date range.
///
/// ## Required Parameters
///
/// - **Date**: You must specify a `date` (timestamp of first record to retrieve)
///
/// ## Location Requirements
///
/// You must supply **either**:
/// - Both `lat` and `lon` coordinates, OR
/// - One of `dwd_station_id`, `wmo_station_id`, or `source_id`
///
/// ## Examples
///
/// ### Get weather for specific date and location
/// ```rust
/// use brightsky::WeatherQueryBuilder;
/// use chrono::NaiveDate;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let query = WeatherQueryBuilder::new()
///         .with_date(NaiveDate::from_ymd_opt(2023, 8, 7).unwrap())
///         .with_last_date(NaiveDate::from_ymd_opt(2023, 8, 8).unwrap())
///         .with_lat_lon((52.52, 13.4))  // Berlin
///         .build()?;
///     Ok(())
/// }
/// ```
///
/// ### Get forecast using station ID
/// ```rust
/// use brightsky::WeatherQueryBuilder;
/// use chrono::NaiveDate;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let query = WeatherQueryBuilder::new()
///         .with_date(NaiveDate::from_ymd_opt(2023, 8, 10).unwrap())
///         .with_dwd_station_id(vec!["01766"])  // Münster/Osnabrück
///         .build()?;
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct WeatherQueryBuilder<'a> {
    /// First timestamp to retrieve (required)
    pub date: Option<NaiveDate>,
    /// Last timestamp to retrieve (defaults to date + 1 day)
    pub last_date: Option<NaiveDate>,
    /// Latitude in decimal degrees (-90.0 to 90.0)
    pub lat: Option<String>,
    /// Longitude in decimal degrees (-180.0 to 180.0)
    pub lon: Option<String>,
    /// Maximum distance from lat/lon in meters (0 to 500,000)
    pub max_dist: Option<String>,
    /// DWD station IDs (5 alphanumeric characters each)
    pub dwd_station_id: Option<Vec<&'a str>>,
    /// WMO station IDs (5 alphanumeric characters each)
    pub wmo_station_id: Option<Vec<&'a str>>,
    /// Bright Sky source IDs
    pub source_id: Option<Vec<String>>,
    /// Timezone for timestamp presentation (tz database format)
    pub tz: Option<String>,
    /// Physical units system (DWD or SI)
    pub units: Option<UnitType>,
}

impl<'a> WeatherQueryBuilder<'a> {
    /// Create a new weather query builder.
    ///
    /// Returns a builder with no parameters set. You must set a date and location
    /// parameters before calling `build()`.
    pub fn new() -> Self {
        Self {
            date: None,
            last_date: None,
            lat: None,
            lon: None,
            max_dist: None,
            dwd_station_id: None,
            wmo_station_id: None,
            source_id: None,
            tz: None,
            units: None,
        }
    }

    /// Set the start date for weather data retrieval (**required**).
    ///
    /// This is the timestamp of the first weather record to retrieve.
    /// The date can contain time information and UTC offset in ISO 8601 format.
    ///
    /// # Parameters
    ///
    /// * `date` - First date to retrieve weather data for
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brightsky::WeatherQueryBuilder;
    /// use chrono::NaiveDate;
    ///
    /// let query = WeatherQueryBuilder::new()
    ///     .with_date(NaiveDate::from_ymd_opt(2023, 8, 7).unwrap());
    /// ```
    pub fn with_date(mut self, date: NaiveDate) -> Self {
        self.date = Some(date);
        self
    }

    /// Set the end date for weather data retrieval (optional).
    ///
    /// This is the timestamp of the last weather record to retrieve.
    /// If not specified, defaults to `date + 1 day`.
    ///
    /// # Parameters
    ///
    /// * `last_date` - Last date to retrieve weather data for
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brightsky::WeatherQueryBuilder;
    /// use chrono::NaiveDate;
    ///
    /// let query = WeatherQueryBuilder::new()
    ///     .with_date(NaiveDate::from_ymd_opt(2023, 8, 7).unwrap())
    ///     .with_last_date(NaiveDate::from_ymd_opt(2023, 8, 10).unwrap());  // 3-day range
    /// ```
    pub fn with_last_date(mut self, last_date: NaiveDate) -> Self {
        self.last_date = Some(last_date);
        self
    }

    /// Set the geographic coordinates for the weather query.
    ///
    /// # Parameters
    ///
    /// * `lat_lon` - Tuple of (latitude, longitude) in decimal degrees
    ///
    /// # Constraints
    ///
    /// - Latitude must be between -90.0 and 90.0
    /// - Longitude must be between -180.0 and 180.0
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brightsky::WeatherQueryBuilder;
    /// use chrono::NaiveDate;
    ///
    /// let query = WeatherQueryBuilder::new()
    ///     .with_date(NaiveDate::from_ymd_opt(2023, 8, 7).unwrap())
    ///     .with_lat_lon((52.52, 13.4));  // Berlin coordinates
    /// ```
    pub fn with_lat_lon(mut self, lat_lon: (f64, f64)) -> Self {
        // Format coordinates preserving all decimal precision
        // For whole numbers, ensure at least one decimal place is shown
        let lat_str = format!("{}", lat_lon.0);
        let lon_str = format!("{}", lat_lon.1);

        self.lat = Some(if !lat_str.contains('.') {
            format!("{}.0", lat_str)
        } else {
            lat_str
        });

        self.lon = Some(if !lon_str.contains('.') {
            format!("{}.0", lon_str)
        } else {
            lon_str
        });

        self
    }

    /// Set the maximum distance for station selection when using coordinates.
    ///
    /// Only has an effect when using `lat` and `lon`. Stations further than this
    /// distance will not be considered for weather data.
    ///
    /// # Parameters
    ///
    /// * `max_dist` - Maximum distance in meters (0 to 500,000, default: 50,000)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brightsky::WeatherQueryBuilder;
    /// use chrono::NaiveDate;
    ///
    /// let query = WeatherQueryBuilder::new()
    ///     .with_date(NaiveDate::from_ymd_opt(2023, 8, 7).unwrap())
    ///     .with_lat_lon((52.52, 13.4))
    ///     .with_max_dist(25000);  // Within 25km
    /// ```
    pub fn with_max_dist(mut self, max_dist: u32) -> Self {
        self.max_dist = Some(max_dist.to_string());
        self
    }

    /// Set DWD (German Weather Service) station IDs.
    ///
    /// You can supply multiple station IDs ordered from highest to lowest priority.
    /// Each ID is typically 5 alphanumeric characters.
    ///
    /// # Parameters
    ///
    /// * `ids` - Vector of DWD station ID string references
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brightsky::WeatherQueryBuilder;
    /// use chrono::NaiveDate;
    ///
    /// let query = WeatherQueryBuilder::new()
    ///     .with_date(NaiveDate::from_ymd_opt(2023, 8, 7).unwrap())
    ///     .with_dwd_station_id(vec!["01766", "00420"]);  // Multiple stations
    /// ```
    pub fn with_dwd_station_id(mut self, ids: Vec<&'a str>) -> Self {
        self.dwd_station_id = Some(ids);
        self
    }

    /// Set WMO (World Meteorological Organization) station IDs.
    ///
    /// You can supply multiple station IDs ordered from highest to lowest priority.
    /// Each ID is typically 5 alphanumeric characters.
    ///
    /// # Parameters
    ///
    /// * `ids` - Vector of WMO station ID string references
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brightsky::WeatherQueryBuilder;
    /// use chrono::NaiveDate;
    ///
    /// let query = WeatherQueryBuilder::new()
    ///     .with_date(NaiveDate::from_ymd_opt(2023, 8, 7).unwrap())
    ///     .with_wmo_station_id(vec!["10315"]);
    /// ```
    pub fn with_wmo_station_id(mut self, ids: Vec<&'a str>) -> Self {
        self.wmo_station_id = Some(ids);
        self
    }

    /// Set Bright Sky source IDs.
    ///
    /// You can supply multiple source IDs ordered from highest to lowest priority.
    /// These IDs are retrieved from the `/sources` endpoint.
    ///
    /// # Parameters
    ///
    /// * `ids` - Vector of Bright Sky source IDs
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brightsky::WeatherQueryBuilder;
    /// use chrono::NaiveDate;
    ///
    /// let query = WeatherQueryBuilder::new()
    ///     .with_date(NaiveDate::from_ymd_opt(2023, 8, 7).unwrap())
    ///     .with_source_id(vec![1234, 2345]);
    /// ```
    pub fn with_source_id(mut self, ids: Vec<i64>) -> Self {
        self.source_id = Some(ids.into_iter().map(|id| id.to_string()).collect());
        self
    }

    /// Set the timezone for timestamp presentation.
    ///
    /// Timestamps in the response will be presented in this timezone.
    /// Also used as timezone when parsing `date` and `last_date` unless they have explicit UTC offsets.
    /// Uses tz database names (e.g., "Europe/Berlin", "UTC").
    ///
    /// # Parameters
    ///
    /// * `tz` - Timezone name from the tz database
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brightsky::WeatherQueryBuilder;
    /// use chrono::NaiveDate;
    ///
    /// let query = WeatherQueryBuilder::new()
    ///     .with_date(NaiveDate::from_ymd_opt(2023, 8, 7).unwrap())
    ///     .with_lat_lon((52.52, 13.4))
    ///     .with_tz("Europe/Berlin");
    /// ```
    pub fn with_tz(mut self, tz: &str) -> Self {
        self.tz = Some(tz.to_string());
        self
    }

    /// Set the physical units system for meteorological parameters.
    ///
    /// # Parameters
    ///
    /// * `units` - Unit system (`UnitType::Dwd` for meteorological units, `UnitType::Si` for SI units)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brightsky::{WeatherQueryBuilder, types::UnitType};
    /// use chrono::NaiveDate;
    ///
    /// let query = WeatherQueryBuilder::new()
    ///     .with_date(NaiveDate::from_ymd_opt(2023, 8, 7).unwrap())
    ///     .with_lat_lon((52.52, 13.4))
    ///     .with_units(UnitType::Si);  // Use SI units
    /// ```
    pub fn with_units(mut self, units: UnitType) -> Self {
        self.units = Some(units);
        self
    }

    /// Build and validate the query.
    ///
    /// Validates all parameters and returns the query ready for execution.
    /// You must provide a date and either coordinates or station IDs before calling build.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Self)` if validation passes, otherwise returns a `BrightSkyError`.
    ///
    /// # Errors
    ///
    /// - `DateNotSet` - No date was provided (required parameter)
    /// - `InvalidLatitude`/`InvalidLongitude` - Coordinates out of valid range
    /// - `InvalidMaxDistance` - Distance greater than 500,000 meters
    /// - `ParseFloatError`/`ParseIntError` - Invalid numeric values
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brightsky::WeatherQueryBuilder;
    /// use chrono::NaiveDate;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let query = WeatherQueryBuilder::new()
    ///         .with_date(NaiveDate::from_ymd_opt(2023, 8, 7).unwrap())
    ///         .with_lat_lon((52.52, 13.4))
    ///         .build()?;  // Validates parameters
    ///     Ok(())
    /// }
    /// ```
    pub fn build(self) -> Result<Self, BrightSkyError> {
        if self.date.is_none() {
            return Err(BrightSkyError::DateNotSet);
        }
        if let Some(lat_str) = &self.lat {
            lat_str
                .parse::<f64>()
                .map_err(BrightSkyError::ParseFloatError)
                .and_then(|lat| -> Result<(), BrightSkyError> {
                    if !(-90.0..=90.0).contains(&lat) {
                        Err(BrightSkyError::InvalidLongitude(lat))
                    } else {
                        Ok(())
                    }
                })?;
        }
        if let Some(lon_str) = &self.lon {
            lon_str
                .parse::<f64>()
                .map_err(BrightSkyError::ParseFloatError)
                .and_then(|lon| -> Result<(), BrightSkyError> {
                    if !(-180.0..=180.0).contains(&lon) {
                        Err(BrightSkyError::InvalidLongitude(lon))
                    } else {
                        Ok(())
                    }
                })?;
        }
        if let Some(max_dist_str) = &self.max_dist {
            max_dist_str
                .parse::<u32>()
                .map_err(BrightSkyError::ParseIntError)
                .and_then(|max_dist| {
                    if max_dist > 500000 {
                        Err(BrightSkyError::InvalidMaxDistance(max_dist))
                    } else {
                        Ok(())
                    }
                })?;
        }

        Ok(self)
    }
}

impl<'a> ToBrightSkyClientUrl for WeatherQueryBuilder<'a> {
    #[cfg(feature = "std")]
    fn to_url(self, host: &str) -> Result<Url, BrightSkyError> {
        let base = Url::parse(host)?;
        let mut url = base.join("weather")?;

        let mut query = url.query_pairs_mut();

        if let Some(date) = self.date {
            query.append_pair("date", &date.to_string());
        }
        if let Some(last_date) = self.last_date {
            query.append_pair("last_date", &last_date.to_string());
        }
        if let Some(lat) = self.lat {
            query.append_pair("lat", &lat);
        }
        if let Some(lon) = self.lon {
            query.append_pair("lon", &lon);
        }
        if let Some(max_dist) = self.max_dist {
            query.append_pair("max_dist", &max_dist);
        }
        if let Some(dwd_station_id) = self.dwd_station_id {
            for id in dwd_station_id {
                query.append_pair("dwd_station_id", id);
            }
        }
        if let Some(wmo_station_id) = self.wmo_station_id {
            for id in wmo_station_id {
                query.append_pair("wmo_station_id", id);
            }
        }
        if let Some(source_id) = self.source_id {
            for id in source_id {
                query.append_pair("source_id", &id);
            }
        }
        if let Some(tz) = self.tz {
            query.append_pair("tz", &tz);
        }
        if let Some(units) = self.units {
            let unit_string = serde_json::to_string(&units).unwrap();
            query.append_pair("units", unit_string.trim_matches('"'));
        }
        drop(query);
        Ok(url)
    }

    #[cfg(not(feature = "std"))]
    fn to_url_string(self, host: &str) -> Result<String, BrightSkyError> {
        let mut url = format!("{}/weather", host.trim_end_matches('/'));
        let mut params = alloc::vec::Vec::new();

        if let Some(date) = self.date {
            params.push(format!("date={}", date));
        }
        if let Some(last_date) = self.last_date {
            params.push(format!("last_date={}", last_date));
        }
        if let Some(lat) = self.lat {
            params.push(format!("lat={}", lat));
        }
        if let Some(lon) = self.lon {
            params.push(format!("lon={}", lon));
        }
        if let Some(max_dist) = self.max_dist {
            params.push(format!("max_dist={}", max_dist));
        }
        if let Some(dwd_station_id) = self.dwd_station_id {
            for id in dwd_station_id {
                params.push(format!("dwd_station_id={}", id));
            }
        }
        if let Some(wmo_station_id) = self.wmo_station_id {
            for id in wmo_station_id {
                params.push(format!("wmo_station_id={}", id));
            }
        }
        if let Some(source_id) = self.source_id {
            for id in source_id {
                params.push(format!("source_id={}", id));
            }
        }
        if let Some(tz) = self.tz {
            params.push(format!("tz={}", tz));
        }
        if let Some(units) = self.units {
            let unit_string = serde_json::to_string(&units).unwrap();
            params.push(format!("units={}", unit_string.trim_matches('"')));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        Ok(url)
    }
}

impl<'a> Default for WeatherQueryBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}
