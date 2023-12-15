use std::error::Error;
use std::fmt;
use std::str::Utf8Error;

use serde::Deserialize;

/// Enum of errors returned by this library
#[derive(Debug, Deserialize)]
pub enum CuriesError {
    NotFound(String),
    InvalidCurie(String),
    InvalidFormat(String),
    DuplicateRecord(String),
    Utf8(String),
    SerdeJson(String),
    Reqwest(String),
    StdIo(String),
}

impl Error for CuriesError {}

impl fmt::Display for CuriesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CuriesError::NotFound(ref prefix) => write!(f, "Prefix not found: {}", prefix),
            CuriesError::DuplicateRecord(ref prefix) => {
                write!(f, "Duplicate record found for prefix: {}", prefix)
            }
            CuriesError::InvalidCurie(ref msg) => write!(f, "Invalid CURIE: {}", msg),
            CuriesError::InvalidFormat(ref msg) => write!(f, "Invalid format: {}", msg),
            CuriesError::Utf8(ref msg) => write!(f, "Error decoding UTF-8: {}", msg),
            CuriesError::SerdeJson(ref msg) => write!(f, "Error parsing JSON: {}", msg),
            CuriesError::Reqwest(ref msg) => write!(f, "Error sending request: {}", msg),
            CuriesError::StdIo(ref msg) => write!(f, "Error reading file: {}", msg),
        }
    }
}

// Add handling for errors from external dependencies to be able to use ? more to handle errors
impl From<Utf8Error> for CuriesError {
    fn from(err: Utf8Error) -> Self {
        CuriesError::Utf8(err.to_string())
    }
}
impl From<serde_json::Error> for CuriesError {
    fn from(err: serde_json::Error) -> Self {
        CuriesError::SerdeJson(err.to_string())
    }
}
impl From<reqwest::Error> for CuriesError {
    fn from(err: reqwest::Error) -> Self {
        CuriesError::Reqwest(err.to_string())
    }
}
impl From<std::io::Error> for CuriesError {
    fn from(err: std::io::Error) -> Self {
        CuriesError::StdIo(err.to_string())
    }
}
