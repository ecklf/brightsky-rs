#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{format, string::String, string::ToString};

use crate::{BrightSkyError, ToBrightSkyUrl};

#[cfg(feature = "std")]
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

    /// Set a specific municipality warn cell ID.
    pub fn with_warn_cell_id(mut self, warn_cell_id: i64) -> Self {
        self.warn_cell_id = Some(warn_cell_id.to_string());
        self
    }

    /// Set the timezone for timestamp presentation.
    pub fn with_tz(mut self, tz: &str) -> Self {
        self.tz = Some(tz.to_string());
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
                        Err(BrightSkyError::InvalidLatitude(lat))
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
        if let Some(warn_cell_id_str) = &self.warn_cell_id {
            warn_cell_id_str
                .parse::<i64>()
                .map_err(BrightSkyError::ParseIntError)?;
        }

        Ok(self)
    }
}

impl ToBrightSkyUrl for AlertsQueryBuilder {
    #[cfg(feature = "std")]
    fn to_url(self, host: &str) -> Result<Url, BrightSkyError> {
        let base = Url::parse(host)?;
        let mut url = base.join("alerts")?;

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

    fn to_url_string(self, host: &str) -> Result<String, BrightSkyError> {
        #[cfg(not(feature = "std"))]
        use alloc::vec::Vec;
        #[cfg(feature = "std")]
        use std::vec::Vec;

        let mut url = format!("{}/alerts", host.trim_end_matches('/'));
        let mut params = Vec::new();

        if let Some(lat) = self.lat {
            params.push(format!("lat={}", lat));
        }
        if let Some(lon) = self.lon {
            params.push(format!("lon={}", lon));
        }
        if let Some(warn_cell_id) = self.warn_cell_id {
            params.push(format!("warn_cell_id={}", warn_cell_id));
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

impl Default for AlertsQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}
