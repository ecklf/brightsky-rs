#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{format, string::String, string::ToString, vec::Vec};

use crate::{BrightSkyError, ToBrightSkyClientUrl, types::UnitType};

#[cfg(feature = "std")]
use url::Url;

/// Query builder for the current weather endpoint (`/current_weather`).
///
/// This builder helps construct queries to get current weather conditions compiled
/// from SYNOP observations from the past 1.5 hours. Unlike other weather endpoints,
/// this provides a best-effort solution to reflect current weather conditions.
///
/// ## Location Requirements
///
/// You must supply **either**:
/// - Both `lat` and `lon` coordinates, OR
/// - One of `dwd_station_id`, `wmo_station_id`, or `source_id`
///
/// ## Examples
///
/// ### Using coordinates
/// ```rust
/// use brightsky::CurrentWeatherQueryBuilder;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let query = CurrentWeatherQueryBuilder::new()
///         .with_lat_lon((52.52, 13.4))  // Berlin coordinates
///         .with_max_dist(10000)         // Within 10km
///         .build()?;
///     Ok(())
/// }
/// ```
///
/// ### Using station ID
/// ```rust
/// use brightsky::CurrentWeatherQueryBuilder;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let query = CurrentWeatherQueryBuilder::new()
///         .with_dwd_station_id(vec!["01766".to_string()])  // Münster/Osnabrück
///         .build()?;
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct CurrentWeatherQueryBuilder {
    /// Latitude in decimal degrees (-90.0 to 90.0)
    pub lat: Option<String>,
    /// Longitude in decimal degrees (-180.0 to 180.0)
    pub lon: Option<String>,
    /// Maximum distance from lat/lon in meters (0 to 500,000)
    pub max_dist: Option<String>,
    /// DWD station IDs (5 alphanumeric characters each)
    pub dwd_station_id: Option<Vec<String>>,
    /// WMO station IDs (5 alphanumeric characters each)
    pub wmo_station_id: Option<Vec<String>>,
    /// Bright Sky source IDs
    pub source_id: Option<Vec<String>>,
    /// Timezone for timestamp presentation (tz database format)
    pub tz: Option<String>,
    /// Physical units system (DWD or SI)
    pub units: Option<UnitType>,
}

impl CurrentWeatherQueryBuilder {
    /// Create a new current weather query builder.
    ///
    /// Returns a builder with no parameters set. You must call location methods
    /// and `build()` before using with a client.
    pub fn new() -> Self {
        Self {
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
    /// use brightsky::CurrentWeatherQueryBuilder;
    ///
    /// let query = CurrentWeatherQueryBuilder::new()
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
    /// use brightsky::CurrentWeatherQueryBuilder;
    ///
    /// let query = CurrentWeatherQueryBuilder::new()
    ///     .with_lat_lon((52.52, 13.4))
    ///     .with_max_dist(10000);  // Within 10km
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
    /// * `ids` - Vector of DWD station ID strings
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brightsky::CurrentWeatherQueryBuilder;
    ///
    /// let query = CurrentWeatherQueryBuilder::new()
    ///     .with_dwd_station_id(vec![
    ///         "01766".to_string(),  // Münster/Osnabrück (primary)
    ///         "00420".to_string(),  // Fallback station
    ///     ]);
    /// ```
    pub fn with_dwd_station_id(mut self, ids: Vec<String>) -> Self {
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
    /// * `ids` - Vector of WMO station ID strings
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brightsky::CurrentWeatherQueryBuilder;
    ///
    /// let query = CurrentWeatherQueryBuilder::new()
    ///     .with_wmo_station_id(vec!["10315".to_string()]);
    /// ```
    pub fn with_wmo_station_id(mut self, ids: Vec<String>) -> Self {
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
    /// use brightsky::CurrentWeatherQueryBuilder;
    ///
    /// let query = CurrentWeatherQueryBuilder::new()
    ///     .with_source_id(vec![1234, 2345]);
    /// ```
    pub fn with_source_id(mut self, ids: Vec<i64>) -> Self {
        self.source_id = Some(ids.into_iter().map(|id| id.to_string()).collect());
        self
    }

    /// Set the timezone for timestamp presentation.
    ///
    /// Timestamps in the response will be presented in this timezone.
    /// Uses tz database names (e.g., "Europe/Berlin", "UTC").
    ///
    /// # Parameters
    ///
    /// * `tz` - Timezone name from the tz database
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brightsky::CurrentWeatherQueryBuilder;
    ///
    /// let query = CurrentWeatherQueryBuilder::new()
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
    /// use brightsky::{CurrentWeatherQueryBuilder, types::UnitType};
    ///
    /// let query = CurrentWeatherQueryBuilder::new()
    ///     .with_lat_lon((52.52, 13.4))
    ///     .with_units(UnitType::Si);  // Use SI units (Kelvin, m/s, etc.)
    /// ```
    pub fn with_units(mut self, units: UnitType) -> Self {
        self.units = Some(units);
        self
    }

    /// Build and validate the query.
    ///
    /// Validates all parameters and returns the query ready for execution.
    /// You must provide either coordinates or station IDs before calling build.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Self)` if validation passes, otherwise returns a `BrightSkyError`.
    ///
    /// # Errors
    ///
    /// - `InvalidLatitude`/`InvalidLongitude` - Coordinates out of valid range
    /// - `InvalidMaxDistance` - Distance greater than 500,000 meters
    /// - `ParseFloatError`/`ParseIntError` - Invalid numeric values
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brightsky::CurrentWeatherQueryBuilder;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let query = CurrentWeatherQueryBuilder::new()
    ///         .with_lat_lon((52.52, 13.4))
    ///         .build()?;  // Validates parameters
    ///     Ok(())
    /// }
    /// ```
    pub fn build(self) -> Result<Self, BrightSkyError> {
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

impl ToBrightSkyClientUrl for CurrentWeatherQueryBuilder {
    #[cfg(feature = "std")]
    fn to_url(self, host: &str) -> Result<Url, BrightSkyError> {
        let base = Url::parse(host)?;
        let mut url = base.join("current_weather")?;

        let mut query = url.query_pairs_mut();

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
                query.append_pair("dwd_station_id", &id);
            }
        }
        if let Some(wmo_station_id) = self.wmo_station_id {
            for id in wmo_station_id {
                query.append_pair("wmo_station_id", &id);
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
        let mut url = format!("{}/current_weather", host.trim_end_matches('/'));
        let mut params = alloc::vec::Vec::new();

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

impl Default for CurrentWeatherQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}
