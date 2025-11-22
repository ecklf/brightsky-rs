use crate::{BlindSkyClientError, ToBrightSkyClientUrl, types::RadarCompressionFormat};
use chrono::NaiveDate;
use url::Url;

/// Query builder for the radar endpoint (`/radar`).
///
/// This builder constructs queries to retrieve radar rainfall data with 1km spatial
/// and 5-minute temporal resolution, including forecasts for the next two hours.
///
/// ## Important Notes
///
/// - **Large data sets**: Radar data covers a 1200×1100 pixel grid (1.2M pixels total)
/// - **Use bounding boxes**: Always use `lat`/`lon` or `bbox` when possible to reduce response size
/// - **Compression formats**: Use `Compressed` format for best performance and smallest responses
/// - **Data retention**: Past radar records are kept for 6 hours only
///
/// ## Data Format
///
/// Values represent **0.01 mm / 5 min** precipitation amounts. For example:
/// - Value `45` = 0.45 mm of precipitation in that 1km² area during 5 minutes
/// - The grid uses polar stereographic projection, different from mercator projection
///
/// ## Examples
///
/// ### Basic radar query near Münster
/// ```rust
/// use brightsky::{RadarWeatherQueryBuilder, types::RadarCompressionFormat};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let query = RadarWeatherQueryBuilder::new()
///         .with_lat_lon((52.0, 7.6))  // Near Münster
///         .with_compression_format(RadarCompressionFormat::Compressed)  // Recommended
///         .build()?;
///     Ok(())
/// }
/// ```
///
/// ### Custom bounding box for smaller area
/// ```rust
/// use brightsky::RadarWeatherQueryBuilder;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let query = RadarWeatherQueryBuilder::new()
///         .with_bbox(vec![100, 100, 300, 300])  // Custom 200×200 pixel area
///         .build()?;
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct RadarWeatherQueryBuilder {
    /// Bounding box in pixels (top, left, bottom, right)
    pub bbox: Option<Vec<i64>>,
    /// Distance in meters around lat/lon (used with lat/lon, default: 200,000)
    pub distance: Option<u64>,
    /// Latitude in decimal degrees (-90.0 to 90.0)
    pub lat: Option<String>,
    /// Longitude in decimal degrees (-180.0 to 180.0)
    pub lon: Option<String>,
    /// First timestamp to retrieve (defaults to 1 hour before latest)
    pub date: Option<NaiveDate>,
    /// Last timestamp to retrieve (defaults to 2 hours after date)
    pub last_date: Option<NaiveDate>,
    /// Precipitation data encoding format
    pub compression_format: Option<RadarCompressionFormat>,
    /// Timezone for timestamp presentation (tz database format)
    pub tz: Option<String>,
}

impl RadarWeatherQueryBuilder {
    /// Create a new radar query builder.
    ///
    /// Returns a builder with no parameters set. All parameters are optional,
    /// but using location parameters is highly recommended to reduce response size.
    pub fn new() -> Self {
        Self {
            lat: None,
            lon: None,
            bbox: None,
            distance: None,
            date: None,
            last_date: None,
            compression_format: None,
            tz: None,
        }
    }

    /// Set the geographic coordinates for the radar query.
    ///
    /// When using coordinates, the API will return data within `distance` meters
    /// of this location (default: 200km in each direction).
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
    /// use brightsky::RadarWeatherQueryBuilder;
    ///
    /// let query = RadarWeatherQueryBuilder::new()
    ///     .with_lat_lon((52.0, 7.6));  // Near Münster
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

    /// Set a custom bounding box for radar data in pixel coordinates.
    ///
    /// The full radar grid is 1200×1100 pixels. Using a smaller bounding box
    /// significantly reduces response size and processing time.
    ///
    /// # Parameters
    ///
    /// * `bbox` - Vector of [top, left, bottom, right] pixel coordinates (edges inclusive)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brightsky::RadarWeatherQueryBuilder;
    ///
    /// let query = RadarWeatherQueryBuilder::new()
    ///     .with_bbox(vec![100, 100, 300, 300]);  // 200×200 pixel area
    /// ```
    pub fn with_bbox(mut self, bbox: Vec<i64>) -> Self {
        self.bbox = Some(bbox);
        self
    }

    /// Set the distance radius when using lat/lon coordinates.
    ///
    /// Must be used together with `lat` and `lon`. Data will reach this distance
    /// in meters to each side of the specified location.
    ///
    /// # Parameters
    ///
    /// * `distance` - Distance in meters (default: 200,000 = 200km)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brightsky::RadarWeatherQueryBuilder;
    ///
    /// let query = RadarWeatherQueryBuilder::new()
    ///     .with_lat_lon((52.0, 7.6))
    ///     .with_distance(100_000);  // 100km radius
    /// ```
    pub fn with_distance(mut self, distance: u64) -> Self {
        self.distance = Some(distance);
        self
    }

    /// Set the start date for radar data retrieval (optional).
    ///
    /// This is the timestamp of the first radar record to retrieve.
    /// If not specified, defaults to 1 hour before the latest measurement.
    ///
    /// # Parameters
    ///
    /// * `date` - First date to retrieve radar data for
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brightsky::RadarWeatherQueryBuilder;
    /// use chrono::NaiveDate;
    ///
    /// let query = RadarWeatherQueryBuilder::new()
    ///     .with_date(NaiveDate::from_ymd_opt(2023, 8, 7).unwrap());
    /// ```
    pub fn with_date(mut self, date: NaiveDate) -> Self {
        self.date = Some(date);
        self
    }

    /// Set the end date for radar data retrieval (optional).
    ///
    /// This is the timestamp of the last radar record to retrieve.
    /// If not specified, defaults to 2 hours after `date`.
    ///
    /// # Parameters
    ///
    /// * `last_date` - Last date to retrieve radar data for
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brightsky::RadarWeatherQueryBuilder;
    /// use chrono::NaiveDate;
    ///
    /// let query = RadarWeatherQueryBuilder::new()
    ///     .with_date(NaiveDate::from_ymd_opt(2023, 8, 7).unwrap())
    ///     .with_last_date(NaiveDate::from_ymd_opt(2023, 8, 7).unwrap());  // Same day
    /// ```
    pub fn with_last_date(mut self, last_date: NaiveDate) -> Self {
        self.last_date = Some(last_date);
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
    /// use brightsky::RadarWeatherQueryBuilder;
    ///
    /// let query = RadarWeatherQueryBuilder::new()
    ///     .with_lat_lon((52.0, 7.6))
    ///     .with_tz("Europe/Berlin");
    /// ```
    pub fn with_tz(mut self, tz: &str) -> Self {
        self.tz = Some(tz.to_string());
        self
    }

    /// Set the compression format for precipitation data (**recommended**).
    ///
    /// Different formats offer trade-offs between response size and processing complexity:
    /// - `Compressed`: Smallest responses, best performance (recommended)
    /// - `Bytes`: Medium size, no decompression needed
    /// - `Plain`: Largest responses, easiest to process
    ///
    /// # Parameters
    ///
    /// * `format` - Compression format for precipitation data
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brightsky::{RadarWeatherQueryBuilder, types::RadarCompressionFormat};
    ///
    /// let query = RadarWeatherQueryBuilder::new()
    ///     .with_lat_lon((52.0, 7.6))
    ///     .with_compression_format(RadarCompressionFormat::Compressed);  // Recommended
    /// ```
    pub fn with_compression_format(mut self, format: RadarCompressionFormat) -> Self {
        self.compression_format = Some(format);
        self
    }

    /// Build and validate the query.
    ///
    /// Validates all parameters and returns the query ready for execution.
    /// All parameters are optional for radar queries, but location parameters
    /// are highly recommended to reduce response size.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Self)` if validation passes, otherwise returns a `BlindSkyClientError`.
    ///
    /// # Errors
    ///
    /// - `InvalidLatitude`/`InvalidLongitude` - Coordinates out of valid range
    /// - `ParseFloatError` - Invalid numeric values
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brightsky::{RadarWeatherQueryBuilder, types::RadarCompressionFormat};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let query = RadarWeatherQueryBuilder::new()
    ///         .with_lat_lon((52.0, 7.6))
    ///         .with_compression_format(RadarCompressionFormat::Compressed)
    ///         .build()?;  // Validates parameters
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
                        Err(BlindSkyClientError::InvalidLongitude(lat))
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

        Ok(self)
    }
}

impl ToBrightSkyClientUrl for RadarWeatherQueryBuilder {
    fn to_url(self, host: &str) -> Result<Url, BlindSkyClientError> {
        let base = Url::parse(host).map_err(BlindSkyClientError::UrlParseError)?; // Dummy error
        let mut url = base
            .join("radar")
            .map_err(BlindSkyClientError::UrlParseError)?; // Dummy error

        let mut query = url.query_pairs_mut();

        if let Some(lat) = self.lat {
            query.append_pair("lat", &lat);
        }
        if let Some(lon) = self.lon {
            query.append_pair("lon", &lon);
        }

        if let Some(bbox) = self.bbox {
            let bbox_str = bbox
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",");
            query.append_pair("bbox", &bbox_str);
        }

        if let Some(distance) = self.distance {
            query.append_pair("distance", &distance.to_string());
        }
        if let Some(date) = self.date {
            query.append_pair("date", &date.to_string());
        }
        if let Some(last_date) = self.last_date {
            query.append_pair("last_date", &last_date.to_string());
        }
        if let Some(format) = self.compression_format {
            let format_str = match format {
                RadarCompressionFormat::Compressed => "compressed",
                RadarCompressionFormat::Bytes => "bytes",
                RadarCompressionFormat::Plain => "plain",
            };
            query.append_pair("format", format_str);
        }

        if let Some(tz) = self.tz {
            query.append_pair("tz", &tz);
        }
        drop(query);
        Ok(url)
    }
}

impl Default for RadarWeatherQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}
