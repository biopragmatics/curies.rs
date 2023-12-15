use crate::{error::CuriesError, Converter};

/// Get the latest OBO Foundry JSON-LD context.
/// The resulting converter contains the OBO Foundry
/// preferred prefixes and OBO PURL expansions, but no
/// synonyms.
///
/// # Examples
///
/// ```rust
/// use curies::{Converter, Record, get_obo_converter};
/// use std::collections::HashSet;
///
/// let converter = get_obo_converter();
///
/// let uri = converter.expand("DOID:1234")?;
/// assert_eq!(uri, "http://purl.obolibrary.org/obo/DOID_1234");
///
/// let unregistered_uri = converter.expand("missing.prefix:0000001");
/// assert!(unregistered_uri.is_err());
///
/// let curie = converter.compress("http://purl.obolibrary.org/obo/DOID_1234")?;
/// assert_eq!(curie, "DOID:1234");
///
/// let unregistered_curie = converter.compress("http://example.org/missing.prefix:0000001");
/// assert!(unregistered_curie.is_err());
/// ```
pub async fn get_obo_converter() -> Result<Converter, CuriesError> {
    Converter::from_jsonld("http://purl.obolibrary.org/meta/obo_context.jsonld").await
}
