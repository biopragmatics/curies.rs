//! Contains functions for getting pre-defined contexts

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
/// use curies::sources::{get_obo_converter};
/// use tokio::runtime;
///
/// let rt = runtime::Runtime::new().expect("Failed to create Tokio runtime");
/// let converter = rt.block_on(async {
///      get_obo_converter().await
/// }).expect("Failed to create the OBO converter");
///
/// let uri = converter.expand("DOID:1234").unwrap();
/// assert_eq!(uri, "http://purl.obolibrary.org/obo/DOID_1234");
///
/// let unregistered_uri = converter.expand("missing.prefix:0000001");
/// assert!(unregistered_uri.is_err());
///
/// let curie = converter.compress("http://purl.obolibrary.org/obo/DOID_1234").unwrap();
/// assert_eq!(curie, "DOID:1234");
///
/// let unregistered_curie = converter.compress("http://example.org/missing.prefix:0000001");
/// assert!(unregistered_curie.is_err());
/// ```
pub async fn get_obo_converter() -> Result<Converter, CuriesError> {
    Converter::from_jsonld("https://purl.obolibrary.org/meta/obo_context.jsonld").await
}

/// Get the Prefix Commons-maintained [Monarch Initiative
/// context](https://github.com/prefixcommons/prefixcommons-py/blob/master/prefixcommons/registry/monarch_context.jsonld)
///
/// The Monarch Initiative context is a simple prefix map stored in a JSON-LD file.
/// It contains a project-specific mix of prefixes from GO, OBO, and Identifiers.org.
///
/// Note, this is not a carefully constructed context, as there are overlapping entries
/// such as:
///
/// - TrEMBL and `http://purl.uniprot.org/uniprot/`
/// - SwissProt and `http://identifiers.org/SwissProt:`
/// - UniProtKB" and `http://identifiers.org/uniprot/`
///
/// # Examples
///
/// ```rust
/// use curies::sources::{get_monarch_converter};
/// use tokio::runtime;
///
/// let rt = runtime::Runtime::new().expect("Failed to create Tokio runtime");
/// let converter = rt.block_on(async {
///      get_monarch_converter().await
/// }).expect("Failed to create the GO converter");
///
/// let uri = converter.expand("CHEBI:24867").unwrap();
/// assert_eq!(uri, "http://purl.obolibrary.org/obo/CHEBI_24867");
///
/// let unregistered_uri = converter.expand("addgene:50943");
/// assert!(unregistered_uri.is_err(), "AddGene is not registered in the Monarch context");
///
/// let curie = converter.compress("http://purl.obolibrary.org/obo/CHEBI_24867").unwrap();
/// assert_eq!(curie, "CHEBI:24867");
///
/// let unregistered_curie = converter.compress("http://addgene.org/50943");
/// assert!(unregistered_curie.is_err(), "AddGene is not registered in the Monarch context");
/// ```
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
///
/// # Examples
///
/// ```rust
/// use curies::sources::{get_go_converter};
/// use tokio::runtime;
///
/// let rt = runtime::Runtime::new().expect("Failed to create Tokio runtime");
/// let converter = rt.block_on(async {
///      get_go_converter().await
/// }).expect("Failed to create the GO converter");
///
/// let uri = converter.expand("NCBIGene:100010").unwrap();
/// assert_eq!(uri, "http://identifiers.org/ncbigene/100010");
///
/// let unregistered_uri = converter.expand("DOID:1234");
/// assert!(unregistered_uri.is_err(), "DOID is not registered in the GO context");
///
/// let curie = converter.compress("http://identifiers.org/ncbigene/100010").unwrap();
/// assert_eq!(curie, "NCBIGene:100010");
///
/// let unregistered_curie = converter.compress("http://purl.obolibrary.org/obo/DOID_1234");
/// assert!(unregistered_curie.is_err(), "DOID is not registered in the GO context");
/// ```
pub async fn get_go_converter() -> Result<Converter, CuriesError> {
    Converter::from_jsonld("https://raw.githubusercontent.com/prefixcommons/prefixcommons-py/master/prefixcommons/registry/go_context.jsonld").await
}

/// Get the BioRegistry extended prefix map.
///
/// # Examples
///
/// ```rust
/// use curies::sources::get_bioregistry_converter;
/// use tokio::runtime;
///
/// let rt = runtime::Runtime::new().expect("Failed to create Tokio runtime");
/// let converter = rt.block_on(async {
///      get_bioregistry_converter().await
/// }).expect("Failed to create the GO converter");
///
/// let uri = converter.expand("NCBIGene:100010").unwrap();
/// assert_eq!(uri, "https://www.ncbi.nlm.nih.gov/gene/100010");
/// ```
pub async fn get_bioregistry_converter() -> Result<Converter, CuriesError> {
    Converter::from_extended_prefix_map("https://raw.githubusercontent.com/biopragmatics/bioregistry/main/exports/contexts/bioregistry.epm.json").await
}
