#![doc = include_str!("../docs/use_rust.md")]
// #![warn(missing_docs)]
// #![doc(issue_tracker_base_url = "https://github.com/biopragmatics/curies.rs/issues")]

pub mod api;
pub mod error;
pub mod fetch;
pub mod sources;

pub use api::{Converter, Record};
pub use error::CuriesError;
