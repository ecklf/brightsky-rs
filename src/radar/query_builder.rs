#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{format, string::String, string::ToString, vec::Vec};

use crate::{BrightSkyError, ToBrightSkyClientUrl, types::RadarCompressionFormat};
use chrono::NaiveDate;

#[cfg(feature = "std")]
use url::Url;

/// Query builder for the radar endpoint (`/radar`).
///
/// This builder constructs queries to retrieve radar rainfall data with 1km spatial
/// and 5-minute temporal resolution, including forecasts for the next two hours.
///
/// ## Important Notes
///
/// - **Large data sets**: Radar data covers a 1200x1100 pixel grid (1.2M pixels total)
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
///         .with_bbox(vec![100, 100, 300, 300])  // Custom 200x200 pixel area
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
    pub fn with_bbox(mut self, bbox: Vec<i64>) -> Self {
        self.bbox = Some(bbox);
        self
    }

    /// Set the distance radius when using lat/lon coordinates.
    pub fn with_distance(mut self, distance: u64) -> Self {
        self.distance = Some(distance);
        self
    }

    /// Set the start date for radar data retrieval (optional).
    pub fn with_date(mut self, date: NaiveDate) -> Self {
        self.date = Some(date);
        self
    }

    /// Set the end date for radar data retrieval (optional).
    pub fn with_last_date(mut self, last_date: NaiveDate) -> Self {
        self.last_date = Some(last_date);
        self
    }

    /// Set the timezone for timestamp presentation.
    pub fn with_tz(mut self, tz: &str) -> Self {
        self.tz = Some(tz.to_string());
        self
    }

    /// Set the compression format for precipitation data (**recommended**).
    pub fn with_compression_format(mut self, format: RadarCompressionFormat) -> Self {
        self.compression_format = Some(format);
        self
    }

    /// Build and validate the query.
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

        Ok(self)
    }
}

impl ToBrightSkyClientUrl for RadarWeatherQueryBuilder {
    #[cfg(feature = "std")]
    fn to_url(self, host: &str) -> Result<Url, BrightSkyError> {
        let base = Url::parse(host)?;
        let mut url = base.join("radar")?;

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

    #[cfg(not(feature = "std"))]
    fn to_url_string(self, host: &str) -> Result<String, BrightSkyError> {
        let mut url = format!("{}/radar", host.trim_end_matches('/'));
        let mut params = alloc::vec::Vec::new();

        if let Some(lat) = self.lat {
            params.push(format!("lat={}", lat));
        }
        if let Some(lon) = self.lon {
            params.push(format!("lon={}", lon));
        }
        if let Some(bbox) = self.bbox {
            let bbox_str = bbox
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",");
            params.push(format!("bbox={}", bbox_str));
        }
        if let Some(distance) = self.distance {
            params.push(format!("distance={}", distance));
        }
        if let Some(date) = self.date {
            params.push(format!("date={}", date));
        }
        if let Some(last_date) = self.last_date {
            params.push(format!("last_date={}", last_date));
        }
        if let Some(format) = self.compression_format {
            let format_str = match format {
                RadarCompressionFormat::Compressed => "compressed",
                RadarCompressionFormat::Bytes => "bytes",
                RadarCompressionFormat::Plain => "plain",
            };
            params.push(format!("format={}", format_str));
        }
        if let Some(tz) = self.tz {
            params.push(format!("tz={}", tz));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        Ok(url)
    }
}

impl Default for RadarWeatherQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}
