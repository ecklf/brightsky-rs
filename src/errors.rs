//! Error types for Bright Sky query building.

use core::num::{ParseFloatError, ParseIntError};

/// Error type for Bright Sky query building operations.
#[derive(Debug)]
pub enum BrightSkyError {
    /// Date parameter is required but was not set.
    DateNotSet,
    /// Latitude value is out of valid range (-90 to 90).
    InvalidLatitude(f64),
    /// Longitude value is out of valid range (-180 to 180).
    InvalidLongitude(f64),
    /// Max distance value is out of valid range (0 to 500000 meters).
    InvalidMaxDistance(u32),
    /// Failed to parse an integer value.
    ParseIntError(ParseIntError),
    /// Failed to parse a float value.
    ParseFloatError(ParseFloatError),
    /// URL parsing error.
    #[cfg(feature = "std")]
    UrlParseError(url::ParseError),
    /// URL parsing error (no_std).
    #[cfg(not(feature = "std"))]
    UrlParseError,
}

impl core::fmt::Display for BrightSkyError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DateNotSet => write!(f, "Date is required, but not set"),
            Self::InvalidLatitude(lat) => {
                write!(f, "Latitude must be between -90 and 90, got {}", lat)
            }
            Self::InvalidLongitude(lon) => {
                write!(f, "Longitude must be between -180 and 180, got {}", lon)
            }
            Self::InvalidMaxDistance(dist) => {
                write!(f, "Max distance must be between 0 and 500000, got {}", dist)
            }
            Self::ParseIntError(e) => write!(f, "Parse int failed: {}", e),
            Self::ParseFloatError(e) => write!(f, "Parse float failed: {}", e),
            #[cfg(feature = "std")]
            Self::UrlParseError(e) => write!(f, "URL parse error: {}", e),
            #[cfg(not(feature = "std"))]
            Self::UrlParseError => write!(f, "URL parse error"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for BrightSkyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ParseIntError(e) => Some(e),
            Self::ParseFloatError(e) => Some(e),
            Self::UrlParseError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<ParseIntError> for BrightSkyError {
    fn from(err: ParseIntError) -> Self {
        Self::ParseIntError(err)
    }
}

impl From<ParseFloatError> for BrightSkyError {
    fn from(err: ParseFloatError) -> Self {
        Self::ParseFloatError(err)
    }
}

#[cfg(feature = "std")]
impl From<url::ParseError> for BrightSkyError {
    fn from(err: url::ParseError) -> Self {
        Self::UrlParseError(err)
    }
}
