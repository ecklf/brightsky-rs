use crate::{BlindSkyClientError, ToBrightSkyClientUrl};
use url::Url;

/// Query builder for the alerts endpoint (`/alerts`).
///
/// This builder constructs queries to retrieve weather alerts for a given location
/// or all weather alerts if no location is specified. The DWD issues warnings for
/// approximately 11,000 municipality-based cells across Germany.
///
/// ## Location Options
///
/// You can specify location in one of these ways:
/// - **Coordinates**: Both `lat` and `lon` together
/// - **Warn Cell ID**: A specific municipality cell ID
/// - **No location**: Returns all current alerts
///
/// ## Alert Coverage
///
/// - Alerts are issued for municipality cells (_Gemeinden_) across Germany
/// - Each alert covers specific geographic areas and time periods
/// - Alerts include severity levels from minor to extreme
/// - Text is provided in both German and English
///
/// ## Examples
///
/// ### Get alerts for specific coordinates
/// ```rust
/// use brightsky::AlertsQueryBuilder;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let query = AlertsQueryBuilder::new()
///         .with_lat_lon((52.52, 13.4))  // Berlin coordinates
///         .build()?;
///     Ok(())
/// }
/// ```
///
/// ### Get alerts for a specific warn cell
/// ```rust
/// use brightsky::AlertsQueryBuilder;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let query = AlertsQueryBuilder::new()
///         .with_warn_cell_id(803159016)  // Specific municipality cell
///         .build()?;
///     Ok(())
/// }
/// ```
///
/// ### Get all current alerts
/// ```rust
/// use brightsky::AlertsQueryBuilder;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let all_alerts = AlertsQueryBuilder::new().build()?;
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct AlertsQueryBuilder {
    /// Latitude in decimal degrees (-90.0 to 90.0)
    pub lat: Option<String>,
    /// Longitude in decimal degrees (-180.0 to 180.0)
    pub lon: Option<String>,
    /// Municipality warn cell ID
    pub warn_cell_id: Option<String>,
    /// Timezone for timestamp presentation (tz database format)
    pub tz: Option<String>,
}

impl AlertsQueryBuilder {
    /// Create a new alerts query builder.
    ///
    /// Returns a builder with no parameters set. All parameters are optional -
    /// if no location is specified, all current alerts will be returned.
    pub fn new() -> Self {
        Self {
            lat: None,
            lon: None,
            warn_cell_id: None,
            tz: None,
        }
    }

    /// Set the geographic coordinates for the alerts query.
    ///
    /// When coordinates are provided, the API will return alerts for the municipality
    /// cell containing that location, plus additional location information in the response.
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
    /// use brightsky::AlertsQueryBuilder;
    ///
    /// let query = AlertsQueryBuilder::new()
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

    /// Set a specific municipality warn cell ID.
    ///
    /// The DWD divides Germany into approximately 11,000 cells based on municipalities.
    /// When a warn cell ID is provided, alerts for that specific cell are returned
    /// along with location information.
    ///
    /// # Parameters
    ///
    /// * `warn_cell_id` - Municipality warn cell ID
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brightsky::AlertsQueryBuilder;
    ///
    /// let query = AlertsQueryBuilder::new()
    ///     .with_warn_cell_id(803159016);  // Specific municipality
    /// ```
    pub fn with_warn_cell_id(mut self, warn_cell_id: i64) -> Self {
        self.warn_cell_id = Some(warn_cell_id.to_string());
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
    /// use brightsky::AlertsQueryBuilder;
    ///
    /// let query = AlertsQueryBuilder::new()
    ///     .with_lat_lon((52.52, 13.4))
    ///     .with_tz("Europe/Berlin");
    /// ```
    pub fn with_tz(mut self, tz: &str) -> Self {
        self.tz = Some(tz.to_string());
        self
    }

    /// Build and validate the query.
    ///
    /// Validates all parameters and returns the query ready for execution.
    /// All parameters are optional - if no location is specified, all alerts are returned.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Self)` if validation passes, otherwise returns a `BlindSkyClientError`.
    ///
    /// # Errors
    ///
    /// - `InvalidLatitude`/`InvalidLongitude` - Coordinates out of valid range
    /// - `ParseFloatError`/`ParseIntError` - Invalid numeric values
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brightsky::AlertsQueryBuilder;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     // Get all alerts
    ///     let all_alerts = AlertsQueryBuilder::new().build()?;
    ///
    ///     // Get alerts for specific location
    ///     let location_alerts = AlertsQueryBuilder::new()
    ///         .with_lat_lon((52.52, 13.4))
    ///         .build()?;
    ///     Ok(())
    /// }
    /// ```
    pub fn build(self) -> Result<Self, BlindSkyClientError> {
        if let Some(lat_str) = &self.lat {
            lat_str
                .parse::<f64>()
                .map_err(BlindSkyClientError::ParseFloatError)
                .and_then(|lat| -> Result<(), BlindSkyClientError> {
                    if !(-90.0..=90.0).contains(&lat) {
                        Err(BlindSkyClientError::InvalidLatitude(lat))
                    } else {
                        Ok(())
                    }
                })?;
        }
        if let Some(lon_str) = &self.lon {
            lon_str
                .parse::<f64>()
                .map_err(BlindSkyClientError::ParseFloatError)
                .and_then(|lon| -> Result<(), BlindSkyClientError> {
                    if !(-180.0..=180.0).contains(&lon) {
                        Err(BlindSkyClientError::InvalidLongitude(lon))
                    } else {
                        Ok(())
                    }
                })?;
        }
        if let Some(warn_cell_id_str) = &self.warn_cell_id {
            warn_cell_id_str
                .parse::<i64>()
                .map_err(BlindSkyClientError::ParseIntError)?;
        }

        Ok(self)
    }
}

impl ToBrightSkyClientUrl for AlertsQueryBuilder {
    fn to_url(self, host: &str) -> Result<Url, BlindSkyClientError> {
        let base = Url::parse(host).map_err(BlindSkyClientError::UrlParseError)?;
        let mut url = base
            .join("alerts")
            .map_err(BlindSkyClientError::UrlParseError)?;

        let mut query = url.query_pairs_mut();

        if let Some(lat) = self.lat {
            query.append_pair("lat", &lat);
        }
        if let Some(lon) = self.lon {
            query.append_pair("lon", &lon);
        }
        if let Some(warn_cell_id) = self.warn_cell_id {
            query.append_pair("warn_cell_id", &warn_cell_id);
        }
        if let Some(tz) = self.tz {
            query.append_pair("tz", &tz);
        }

        drop(query);
        Ok(url)
    }
}

impl Default for AlertsQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}
