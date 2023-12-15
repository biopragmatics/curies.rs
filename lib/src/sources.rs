use crate::{error::CuriesError, Converter};

/// Get the latest [OBO Foundry context](http://purl.obolibrary.org/meta/obo_context.jsonld).
///
/// The OBO Foundry context is a simple prefix map stored in a JSON-LD file.
/// It contains OBO Foundry preferred prefixes and OBO PURL expansions,
/// but no synonyms.
///
/// # Examples
///
/// ```rust
/// use curies::{Converter, Record, get_obo_converter};
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

/// Get the Prefix Commons-maintained [Monarch Initiative
/// context](https://github.com/prefixcommons/prefixcommons-py/blob/master/prefixcommons/registry/monarch_context.jsonld)
pub async fn get_monarch_converter() -> Result<Converter, CuriesError> {
    Converter::from_jsonld("https://raw.githubusercontent.com/prefixcommons/prefixcommons-py/master/prefixcommons/registry/monarch_context.jsonld").await
}

/// Get the Prefix Commons-maintained [Gene Ontology (GO)
/// context](https://github.com/prefixcommons/prefixcommons-py/blob/master/prefixcommons/registry/go_context.jsonld)
///
/// The Gene Ontology context is a simple prefix map stored in a JSON-LD file.
/// It contains prefixes corresponding to semantic spaces that are useful for
/// modeling the molecular functions, cellular components, and biological processes
/// that genes take part in.
pub async fn get_go_converter() -> Result<Converter, CuriesError> {
    Converter::from_jsonld("https://raw.githubusercontent.com/prefixcommons/prefixcommons-py/master/prefixcommons/registry/go_context.jsonld").await
}
