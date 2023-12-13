use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct DuplicateRecordError(pub String);

impl Error for DuplicateRecordError {}

impl fmt::Display for DuplicateRecordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Curies Duplicate Record: {}", self.0)
    }
}

// NOTE: In case we need a generic error that contains other errors

#[derive(Debug)]
pub enum CuriesError {
    NotFound(String),
    InvalidCurie(String),
    DuplicateRecordError(String),
}

impl Error for CuriesError {}

impl fmt::Display for CuriesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CuriesError::NotFound(ref prefix) => write!(f, "Prefix not found: {}", prefix),
            CuriesError::DuplicateRecordError(ref prefix) => {
                write!(f, "Duplicate record found for prefix: {}", prefix)
            }
            CuriesError::InvalidCurie(ref prefix) => write!(f, "Invalid CURIE: {}", prefix),
        }
    }
}

// pub struct CuriesError(pub String);
// impl fmt::Display for CuriesError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }
// // Add handling for errors from external dependencies
// // to be able to use ? more to handle errors
// impl From<DuplicateRecordError> for CuriesError {
//     fn from(err: DuplicateRecordError) -> Self {
//         CuriesError(err.to_string())
//     }
// }
