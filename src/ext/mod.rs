//! Extension traits for HTTP client integration.
//!
//! This module provides extension traits that add `.get_brightsky()` methods
//! to HTTP clients, making it easy to fetch weather data.
//!
//! ## Feature Flags
//!
//! - `reqwest`: Enables `BrightSkyReqwestExt` trait for `reqwest::Client`
//!
//! ## Embedded Usage
//!
//! For embedded systems using reqwless or similar clients, use `to_url_string()`
//! directly and deserialize with `serde_json::from_slice()`. See the crate-level
//! documentation for examples.

#[cfg(feature = "reqwest")]
mod reqwest_ext;

#[cfg(feature = "reqwest")]
pub use reqwest_ext::*;
