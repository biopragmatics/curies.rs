//! API for `Converter` and `Record`

use crate::error::CuriesError;
use crate::fetch::{ExtendedPrefixMapSource, PrefixMapSource, ShaclSource};
use ptrie::Trie;
use regex::Regex;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::{json, Value};
use sophia::api::dataset::Dataset as _;
use sophia::api::graph::MutableGraph as _;
use sophia::api::ns::{xsd, Namespace};
use sophia::api::prefix::Prefix;
use sophia::api::quad::Quad as _;
use sophia::api::serializer::{Stringifier as _, TripleSerializer as _};
use sophia::api::source::QuadSource as _;
use sophia::api::term::matcher::Any;
use sophia::api::term::BnodeId;
use sophia::api::term::Term;
use sophia::inmem::dataset::LightDataset;
use sophia::inmem::graph::LightGraph;
use sophia::iri::Iri;
use sophia::turtle::parser::trig;
use sophia::turtle::serializer::turtle::{TurtleConfig, TurtleSerializer};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::Arc;

/// A CURIE `Record`, containing its prefixes and URI prefixes,
/// used by `Converters` to resolve CURIEs and URIs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub prefix: String,
    pub uri_prefix: String,
    #[serde(default)]
    pub prefix_synonyms: HashSet<String>,
    #[serde(default)]
    pub uri_prefix_synonyms: HashSet<String>,
    pub pattern: Option<String>,
}

impl Record {
    /// Create a new `Record` from a prefix and URI prefix
    ///
    /// ```
    /// use curies::Record;
    ///
    /// let record = Record::new("doid", "http://purl.obolibrary.org/obo/DOID_");
    /// ```
    pub fn new(prefix: &str, uri_prefix: &str) -> Self {
        Record {
            prefix: prefix.to_string(),
            uri_prefix: uri_prefix.to_string(),
            prefix_synonyms: HashSet::from([]),
            uri_prefix_synonyms: HashSet::from([]),
            pattern: None,
        }
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Prefix: {}", self.prefix)?;
        writeln!(f, "URI prefix: {}", self.uri_prefix)?;
        writeln!(f, "Prefix synonyms: {:?}", self.prefix_synonyms)?;
        writeln!(f, "URI prefix synonyms: {:?}", self.uri_prefix_synonyms)?;
        Ok(())
    }
}

/// A `Converter` loads CURIEs `Records` (prefix, uri_prefix, synonyms, pattern),
/// and enable to `compress` URIs or `expand` CURIEs.
///
/// It is composed of 2 HashMaps (one for prefixes, one for URIs),
/// and a trie search to find the longest URI
///
/// # Examples
///
/// ```
/// use curies::{Converter, Record};
/// use std::collections::HashSet;
///
/// fn use_converter() -> Result<(), Box<dyn std::error::Error>> {
///     let mut converter = Converter::default();
///     let record1 = Record {
///         prefix: "doid".to_string(),
///         uri_prefix: "http://purl.obolibrary.org/obo/DOID_".to_string(),
///         prefix_synonyms: HashSet::from(["DOID".to_string()]),
///         uri_prefix_synonyms: HashSet::from(["https://identifiers.org/DOID/"].map(String::from)),
///         pattern: None,
///     };
///     converter.add_record(record1)?;
///
///     let uri = converter.expand("doid:1234")?;
///     assert_eq!(uri, "http://purl.obolibrary.org/obo/DOID_1234");
///
///     let curie = converter.compress("http://purl.obolibrary.org/obo/DOID_1234")?;
///     assert_eq!(curie, "doid:1234");
///     Ok(())
/// }
/// use_converter().unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct Converter {
    records: Vec<Arc<Record>>,
    prefix_map: HashMap<String, Arc<Record>>,
    trie: Trie<u8, Arc<Record>>,
    delimiter: String,
}

impl Serialize for Converter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let records: Vec<&Record> = self.records.iter().map(|r| &**r).collect();
        records.serialize(serializer)
    }
}

impl Converter {
    /// Create an empty `Converter`
    ///
    /// ```
    /// use curies::{Converter, Record};
    /// use std::collections::HashSet;
    ///
    /// let mut converter = Converter::new(":");
    /// let record1 = Record::new("doid", "http://purl.obolibrary.org/obo/DOID_");
    /// converter.add_record(record1).unwrap();
    /// ```
    pub fn new(delimiter: &str) -> Self {
        Converter {
            records: Vec::new(),
            prefix_map: HashMap::new(),
            trie: Trie::<u8, Arc<Record>>::new(),
            delimiter: delimiter.to_string(),
        }
    }

    /// Create a `Converter` from a prefix map (`HashMap[prefix] = uri_prefix`)
    ///
    /// ```
    /// use curies::{Converter, Record};
    /// use std::collections::HashMap;
    /// use tokio::{runtime};
    ///
    /// let mut prefix_map: HashMap<String, String> = HashMap::new();
    /// prefix_map.insert("DOID".to_string(), "http://purl.obolibrary.org/obo/DOID_".to_string());
    /// prefix_map.insert("OBO".to_string(), "http://purl.obolibrary.org/obo/".to_string());
    ///
    /// let rt = runtime::Runtime::new().expect("Failed to create Tokio runtime");
    /// let converter = rt.block_on(async {
    ///      Converter::from_prefix_map(prefix_map).await
    /// }).expect("Failed to create the converter");
    ///
    /// let curie = converter.compress("http://purl.obolibrary.org/obo/DOID_1234").unwrap();
    /// assert_eq!(curie, "DOID:1234");
    /// ```
    pub async fn from_prefix_map<T: PrefixMapSource>(prefix_map: T) -> Result<Self, CuriesError> {
        let prefix_map: HashMap<String, Value> = prefix_map.fetch().await?;
        let mut converter = Converter::default();
        for (prefix, uri_prefix) in prefix_map {
            if let Value::String(uri_prefix_str) = uri_prefix {
                converter.add_record(Record::new(&prefix, &uri_prefix_str))?;
            }
        }
        Ok(converter)
    }

    /// Create a `Converter` from a JSON-LD file context
    ///
    /// ```
    /// use curies::{Converter, Record, error::CuriesError};
    ///
    /// fn test_from_jsonld() -> Result<(), CuriesError> {
    ///     let converter = Converter::from_jsonld("https://purl.obolibrary.org/meta/obo_context.jsonld");
    ///     Ok(())
    /// }
    /// ```
    pub async fn from_jsonld<T: PrefixMapSource>(jsonld: T) -> Result<Self, CuriesError> {
        let prefix_map = jsonld.fetch().await?;
        let mut converter = Converter::default();
        let context = match prefix_map.get("@context") {
            Some(Value::Object(map)) => map,
            _ => return Err(CuriesError::InvalidFormat("JSON-LD".to_string())),
        };
        for (key, value) in context {
            match value {
                Value::String(uri) => {
                    converter.add_record(Record::new(key, uri))?;
                }
                Value::Object(map) if map.get("@prefix") == Some(&Value::Bool(true)) => {
                    if let Some(Value::String(uri)) = map.get("@id") {
                        converter.add_record(Record::new(key, uri))?;
                    }
                }
                _ => continue,
            }
        }
        Ok(converter)
    }

    /// Create a `Converter` from an extended prefix map (list of records objects)
    ///
    /// # Arguments
    ///
    /// * `prefix_map` - The extended prefix map data, as URL, string, file, or `Vec<HashMap>`
    ///
    /// # Examples
    ///
    /// ```
    /// use curies::Converter;
    ///
    /// let converter = Converter::from_extended_prefix_map("https://w3id.org/biopragmatics/bioregistry.epm.json");
    /// ```
    pub async fn from_extended_prefix_map<T: ExtendedPrefixMapSource>(
        prefix_map: T,
    ) -> Result<Self, CuriesError> {
        let records = prefix_map.fetch().await?;
        let mut converter = Converter::default();
        for record in records {
            converter.add_record(record)?;
        }
        Ok(converter)
    }

    /// Create a `Converter` from a SHACL shape prefixes definition
    ///
    /// # Arguments
    ///
    /// * `data` - The SHACL shapes data, as URL, string, file, or `Vec<HashMap>`
    ///
    /// # Examples
    ///
    /// ```
    /// use curies::{Converter, Record};
    /// use std::collections::HashMap;
    /// use tokio::{runtime};
    ///
    /// let rt = runtime::Runtime::new().expect("Failed to create Tokio runtime");
    /// let converter = rt.block_on(async {
    ///      Converter::from_shacl("https://raw.githubusercontent.com/biopragmatics/bioregistry/main/exports/contexts/semweb.context.ttl").await
    /// }).expect("Failed to create the converter");
    ///
    /// let uri = converter.expand("foaf:name").unwrap();
    /// assert_eq!(uri, "http://xmlns.com/foaf/0.1/name");
    /// ```
    pub async fn from_shacl<T: ShaclSource>(shacl: T) -> Result<Self, CuriesError> {
        let rdf_str = shacl.fetch().await?;
        let mut converter = Converter::default();
        // Parse the RDF string
        let graph: LightDataset = trig::parse_str(&rdf_str)
            .collect_quads()
            .map_err(|e| CuriesError::InvalidFormat(format!("Error parsing TriG: {e}")))?;
        let shacl_ns = Namespace::new("http://www.w3.org/ns/shacl#")?;
        // Iterate over triples that match the SHACL prefix and namespace pattern
        for q_prefix in graph.quads_matching(Any, [shacl_ns.get("prefix")?], Any, Any) {
            for q_ns in
                graph.quads_matching([q_prefix?.s()], [shacl_ns.get("namespace")?], Any, Any)
            {
                converter.add_prefix(
                    q_prefix?
                        .o()
                        .lexical_form()
                        .ok_or(CuriesError::InvalidFormat(format!(
                            "Prefix term in SHACL graph {:?}",
                            q_prefix?.o()
                        )))?
                        .as_ref(),
                    q_ns?
                        .o()
                        .lexical_form()
                        .ok_or(CuriesError::InvalidFormat(format!(
                            "Namespace term in SHACL graph {:?}",
                            q_ns?.o()
                        )))?
                        .as_ref(),
                )?;
            }
        }
        Ok(converter)
    }

    /// Add a `Record` to the `Converter`.
    /// When adding a new record we create a reference to the `Record` (Arc)
    /// And we use this reference in the prefix and URI hashmaps
    pub fn add_record(&mut self, record: Record) -> Result<(), CuriesError> {
        let rec = Arc::new(record);
        if self.prefix_map.contains_key(&rec.prefix) {
            return Err(CuriesError::DuplicateRecord(rec.prefix.clone()));
        }
        if self.trie.contains_key(rec.uri_prefix.bytes()) {
            return Err(CuriesError::DuplicateRecord(rec.uri_prefix.clone()));
        }
        // Check if any of the synonyms are already present in the maps
        for prefix in &rec.prefix_synonyms {
            if self.prefix_map.contains_key(prefix) {
                return Err(CuriesError::DuplicateRecord(prefix.clone()));
            }
        }
        for uri_prefix in &rec.uri_prefix_synonyms {
            if self.trie.contains_key(uri_prefix.bytes()) {
                return Err(CuriesError::DuplicateRecord(uri_prefix.clone()));
            }
        }
        self.records.push(rec.clone());
        self.prefix_map.insert(rec.prefix.clone(), rec.clone());
        for prefix in &rec.prefix_synonyms {
            self.prefix_map.insert(prefix.clone(), rec.clone());
        }
        self.trie
            .insert(rec.uri_prefix.clone().bytes(), rec.clone());
        for uri_prefix in &rec.uri_prefix_synonyms {
            self.trie.insert(uri_prefix.bytes(), rec.clone());
        }
        Ok(())
    }

    /// Add a CURIE prefix and its prefix URI to the `Converter`
    pub fn add_prefix(&mut self, prefix: &str, uri_prefix: &str) -> Result<(), CuriesError> {
        self.add_record(Record::new(prefix, uri_prefix))
    }

    /// Get all prefixes in the `Converter` as a list
    pub fn get_prefixes(&self, include_synonyms: bool) -> Vec<String> {
        if include_synonyms {
            self.prefix_map.keys().cloned().collect()
        } else {
            self.records.iter().map(|r| r.prefix.clone()).collect()
        }
    }

    /// Get all URI prefixes in the `Converter` as a list
    pub fn get_uri_prefixes(&self, include_synonyms: bool) -> Vec<String> {
        if include_synonyms {
            let mut prefixes: Vec<String> = Vec::new();
            for record in &self.records {
                prefixes.push(record.uri_prefix.clone());
                for synonym in &record.uri_prefix_synonyms {
                    prefixes.push(synonym.clone());
                }
            }
            prefixes
        } else {
            self.records.iter().map(|r| r.uri_prefix.clone()).collect()
        }
    }

    /// Write the extended prefix map as a JSON string
    pub fn write_extended_prefix_map(&self) -> Result<String, CuriesError> {
        Ok(serde_json::to_string(&self)?)
    }

    /// Write the prefix map as a `HashMap` where keys are prefixes and values are URI prefixes.
    pub fn write_prefix_map(&self) -> HashMap<String, String> {
        self.records
            .iter()
            .map(|record| (record.prefix.clone(), record.uri_prefix.clone()))
            .collect()
    }

    /// Write the `Converter` prefix map as SHACL prefixes definition in the Turtle format.
    pub fn write_shacl(&self) -> Result<String, CuriesError> {
        let mut graph = LightGraph::new();
        let shacl_ns = Namespace::new("http://www.w3.org/ns/shacl#")?;
        let declare_subject = BnodeId::new_unchecked("declareNode".to_string());
        for (i, arc_record) in self.records.iter().enumerate() {
            let record = Arc::clone(arc_record);
            let subject = BnodeId::new_unchecked(format!("{}", i));
            graph.insert(&declare_subject, shacl_ns.get("declare")?, &subject)?;
            graph.insert(&subject, shacl_ns.get("prefix")?, record.prefix.as_str())?;
            graph.insert(
                &subject,
                shacl_ns.get("namespace")?,
                record.uri_prefix.as_str() * xsd::anyURI,
            )?;
        }
        let ttl_prefixes = [
            (
                Prefix::new_unchecked("xsd".to_string()),
                Iri::new_unchecked("http://www.w3.org/2001/XMLSchema#".to_string()),
            ),
            (
                Prefix::new_unchecked("sh".to_string()),
                Iri::new_unchecked("http://www.w3.org/ns/shacl#".to_string()),
            ),
        ];
        let ttl_config = TurtleConfig::new()
            .with_pretty(true)
            .with_prefix_map(&ttl_prefixes[..]);
        let mut ttl_stringifier = TurtleSerializer::new_stringifier_with_config(ttl_config);
        Ok(ttl_stringifier.serialize_graph(&graph)?.to_string())
    }

    /// Write the JSON-LD representation of the prefix map as serde JSON (can be cast to string easily)
    ///
    /// # Examples
    ///
    /// ```
    /// use curies::Converter;
    ///
    /// let mut converter = Converter::default();
    /// converter.add_prefix("doid", "http://purl.obolibrary.org/obo/DOID_").unwrap();
    ///
    /// assert!(converter.write_jsonld()["@context"]
    ///     .to_string()
    ///     .starts_with('{'));
    /// println!("{:?}", converter.write_jsonld());
    /// ```
    pub fn write_jsonld(&self) -> serde_json::Value {
        let mut context = json!({});
        for record in &self.records {
            context[record.prefix.clone()] = record.uri_prefix.clone().into();
            // TODO: do we add prefix synonyms to the context?
            for synonym in &record.prefix_synonyms {
                context[synonym.clone()] = record.uri_prefix.clone().into();
            }
        }
        json!({"@context": context})
    }

    /// Chain multiple `Converters` into a single `Converter`. The first `Converter` in the list is used as the base.
    /// If the same prefix is found in multiple converters, the first occurrence is kept,
    /// but the `uri_prefix` and synonyms are added as synonyms if they are different.
    ///
    /// ```
    /// use curies::{sources::{get_go_converter, get_obo_converter}, Converter};
    /// use std::path::Path;
    ///
    /// let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    /// let converter = rt.block_on(async {
    ///      Converter::chain(vec![
    ///         get_obo_converter().await.unwrap(),
    ///         get_go_converter().await.unwrap(),
    ///     ])
    /// }).expect("Failed to create the chained converter");
    /// ```
    pub fn chain(mut converters: Vec<Converter>) -> Result<Converter, CuriesError> {
        if converters.is_empty() {
            return Err(CuriesError::InvalidFormat(
                "The list of converters is empty".to_string(),
            ));
        }
        let mut base_converter = converters.remove(0);
        for converter in converters {
            for arc_record in converter.records {
                let record = Arc::try_unwrap(arc_record).unwrap_or_else(|arc| (*arc).clone());
                // Function to check if the record or its synonyms already exist in the base converter
                let find_record = |r: &Record| -> Option<Arc<Record>> {
                    base_converter
                        .prefix_map
                        .get(&r.prefix)
                        .cloned()
                        .or_else(|| {
                            r.prefix_synonyms
                                .iter()
                                .find_map(|synonym| base_converter.prefix_map.get(synonym).cloned())
                        })
                };
                if let Some(existing_arc) = find_record(&record) {
                    if existing_arc.uri_prefix != record.uri_prefix {
                        // Add the uri_prefix of the record as a synonym to the existing record
                        let mut updated_record = Arc::try_unwrap(existing_arc.clone())
                            .unwrap_or_else(|arc| (*arc).clone());
                        // Merge synonyms
                        updated_record
                            .uri_prefix_synonyms
                            .insert(record.uri_prefix.clone());
                        updated_record
                            .uri_prefix_synonyms
                            .extend(record.uri_prefix_synonyms.clone());
                        updated_record
                            .prefix_synonyms
                            .extend(record.prefix_synonyms.clone());
                        base_converter.update_record(updated_record)?;
                    }
                } else {
                    // If the prefix does not exist, add the record
                    base_converter.add_record(record)?;
                }
            }
        }
        Ok(base_converter)
    }

    /// Update a `Record` in the `Converter` by adding its new prefixes and URI prefixes to the maps
    ///
    /// ```
    /// use curies::{Converter, Record};
    ///
    /// let mut converter = Converter::default();
    /// converter.add_record(Record::new("doid", "http://purl.obolibrary.org/obo/DOID_")).unwrap();
    /// let record = Record::new("doid", "https://identifiers.org/doid:");
    /// let record2 = Record::new("notthere", "http://notthere");
    /// assert!(converter.update_record(record).is_ok());
    /// assert!(converter.update_record(record2).is_err());
    /// ```
    pub fn update_record(&mut self, record: Record) -> Result<(), CuriesError> {
        let rec = Arc::new(record);
        // Update the record in the records vector
        if let Some(pos) = self.records.iter().position(|r| r.prefix == rec.prefix) {
            self.records[pos] = rec.clone();
        } else {
            return Err(CuriesError::NotFound(rec.prefix.clone()));
        }
        // Update the maps and trie
        self.prefix_map.insert(rec.prefix.clone(), rec.clone());
        for prefix in &rec.prefix_synonyms {
            self.prefix_map.insert(prefix.clone(), rec.clone());
        }
        if self
            .trie
            .set_value(rec.uri_prefix.bytes(), rec.clone())
            .is_err()
        {
            self.trie.insert(rec.uri_prefix.bytes(), rec.clone());
        }
        for uri_prefix in &rec.uri_prefix_synonyms {
            if self
                .trie
                .set_value(uri_prefix.bytes(), rec.clone())
                .is_err()
            {
                self.trie.insert(uri_prefix.bytes(), rec.clone());
            }
        }
        Ok(())
    }

    /// Find corresponding CURIE `Record` given a prefix
    pub fn find_by_prefix(&self, prefix: &str) -> Result<&Arc<Record>, CuriesError> {
        match self.prefix_map.get(prefix) {
            Some(record) => Ok(record),
            None => Err(CuriesError::NotFound(prefix.to_string())),
        }
    }

    /// Find corresponding CURIE `Record` given a URI prefix
    pub fn find_by_uri_prefix(&self, uri_prefix: &str) -> Result<&Arc<Record>, CuriesError> {
        match self.trie.get(uri_prefix.bytes()) {
            Some(record) => Ok(record),
            None => Err(CuriesError::NotFound(uri_prefix.to_string())),
        }
    }

    /// Find corresponding CURIE `Record` given a complete URI
    pub fn find_by_uri(&self, uri: &str) -> Result<&Arc<Record>, CuriesError> {
        match self.trie.find_longest_prefix(uri.bytes()) {
            Some(rec) => Ok(rec),
            None => Err(CuriesError::NotFound(uri.to_string())),
        }
    }

    /// Validate an id against a `Record` regex pattern if it exists
    fn validate_id(&self, id: &str, record: &Arc<Record>) -> Result<(), CuriesError> {
        if let Some(pattern) = &record.pattern {
            let regex = Regex::new(pattern).map_err(|_| {
                CuriesError::InvalidFormat(format!("Invalid regex pattern {pattern}"))
            })?;
            if !regex.is_match(id) {
                return Err(CuriesError::InvalidFormat(format!(
                    "ID {id} does not match the pattern {pattern}"
                )));
            }
        }
        Ok(())
    }

    /// Compresses a URI to a CURIE
    pub fn compress(&self, uri: &str) -> Result<String, CuriesError> {
        let record = self.find_by_uri(uri)?;
        // Check for main prefix, if not match check in synonyms
        let id = uri
            .strip_prefix(&record.uri_prefix)
            .or_else(|| {
                record
                    .uri_prefix_synonyms
                    .iter()
                    .filter(|synonym| uri.starts_with(&**synonym))
                    .max_by_key(|synonym| synonym.len()) // Get longest first
                    .and_then(|synonym| uri.strip_prefix(synonym))
            })
            .ok_or_else(|| CuriesError::NotFound(uri.to_string()))?;
        self.validate_id(id, record)?;
        Ok(format!("{}{}{}", &record.prefix, self.delimiter, id))
    }

    /// Expands a CURIE to a URI
    pub fn expand(&self, curie: &str) -> Result<String, CuriesError> {
        let parts: Vec<&str> = curie.split(&self.delimiter).collect();
        if parts.len() != 2 {
            return Err(CuriesError::InvalidCurie(curie.to_string()));
        }
        let (prefix, id) = (parts[0], parts[1]);
        let record = self.find_by_prefix(prefix)?;
        self.validate_id(id, record)?;
        Ok(format!("{}{}", record.uri_prefix, id))
    }

    /// Compresses a list of URIs to CURIEs
    pub fn compress_list(&self, uris: Vec<&str>) -> Vec<Option<String>> {
        uris.into_iter()
            .map(|uri| match self.compress(uri) {
                Ok(curie) => Some(curie),
                Err(_) => None,
            })
            .collect()
    }

    /// Expands a list of CURIESs to URIs
    pub fn expand_list(&self, curies: Vec<&str>) -> Vec<Option<String>> {
        curies
            .into_iter()
            .map(|curie| match self.expand(curie) {
                Ok(uri) => Some(uri),
                Err(_) => None,
            })
            .collect()
    }

    /// Checks if a given string is a valid CURIE according to the current `Converter`
    ///
    /// # Examples
    ///
    /// ```
    /// use curies::Converter;
    ///
    /// let mut converter = Converter::default();
    /// converter.add_prefix("doid", "http://purl.obolibrary.org/obo/DOID_").unwrap();
    ///
    /// assert_eq!(converter.is_curie("doid:1234"), true);
    /// assert_eq!(converter.is_curie("go:0001"), false);
    /// ```
    pub fn is_curie(&self, curie: &str) -> bool {
        self.expand(curie).is_ok()
    }

    /// Checks if a given string is a valid URI according to the current `Converter`
    ///
    /// # Examples
    ///
    /// ```
    /// use curies::Converter;
    ///
    /// let mut converter = Converter::default();
    /// converter.add_prefix("doid", "http://purl.obolibrary.org/obo/DOID_").unwrap();
    ///
    /// assert_eq!(converter.is_uri("http://purl.obolibrary.org/obo/DOID_1234"), true);
    /// assert_eq!(converter.is_uri("http://purl.obolibrary.org/obo/GO_0001"), false);
    /// ```
    pub fn is_uri(&self, uri: &str) -> bool {
        self.compress(uri).is_ok()
    }

    // TODO: Error for GO because those 2 synonyms are added: http://amigo.geneontology.org/amigo/term/GO: and http://amigo.geneontology.org/amigo/term/
    // And sometime compress picks the shorter one
    // So we need to make sure the synonyms are not added if they are already in the trie

    /// Attempts to compress a URI to a CURIE, or standardize it if it's already a CURIE.
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
    /// }).expect("Failed to create the converter");
    ///
    /// assert_eq!(converter.compress_or_standardize("http://amigo.geneontology.org/amigo/term/GO:0032571").unwrap(), "go:0032571".to_string());
    /// assert_eq!(converter.compress_or_standardize("gomf:0032571").unwrap(), "go:0032571".to_string());
    /// assert!(converter.compress_or_standardize("http://purl.obolibrary.org/UNKNOWN_12345").is_err());
    /// ```
    pub fn compress_or_standardize(&self, input: &str) -> Result<String, CuriesError> {
        if self.is_curie(input) {
            self.standardize_curie(input)
        } else {
            self.compress(input)
        }
    }

    /// Attempts to expand a CURIE to a URI, or standardize it if it's already a URI.
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
    /// }).expect("Failed to create the converter");
    ///
    /// assert_eq!(converter.expand_or_standardize("http://amigo.geneontology.org/amigo/term/GO:0032571").unwrap(), "http://purl.obolibrary.org/obo/GO_0032571".to_string());
    /// assert_eq!(converter.expand_or_standardize("gomf:0032571").unwrap(), "http://purl.obolibrary.org/obo/GO_0032571".to_string());
    /// assert!(converter.expand_or_standardize("http://purl.obolibrary.org/UNKNOWN_12345").is_err());
    /// ```
    pub fn expand_or_standardize(&self, input: &str) -> Result<String, CuriesError> {
        if self.is_curie(input) {
            Ok(self.expand(input)?)
        } else {
            Ok(self.standardize_uri(input)?)
        }
    }

    /// Get the standard prefix for a given prefix
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
    /// }).expect("Failed to create the converter");
    /// assert_eq!(converter.standardize_prefix("gomf").unwrap(), "go");
    /// ```
    pub fn standardize_prefix(&self, prefix: &str) -> Result<String, CuriesError> {
        Ok(self.find_by_prefix(prefix)?.prefix.to_string())
    }

    /// Standardize a CURIE
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
    /// }).expect("Failed to create the converter");
    /// assert_eq!(converter.standardize_curie("gomf:0032571").unwrap(), "go:0032571");
    /// ```
    pub fn standardize_curie(&self, curie: &str) -> Result<String, CuriesError> {
        let parts: Vec<&str> = curie.split(':').collect();
        if parts.len() == 2 {
            Ok(format!(
                "{}:{}",
                self.standardize_prefix(parts[0])?,
                parts[1]
            ))
        } else {
            Ok(curie.to_string())
        }
    }

    /// Standardize a URI
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
    /// }).expect("Failed to create the converter");
    /// assert_eq!(
    ///     converter.standardize_uri("http://amigo.geneontology.org/amigo/term/GO:0032571").unwrap(),
    ///     "http://purl.obolibrary.org/obo/GO_0032571",
    /// );
    /// ```
    pub fn standardize_uri(&self, uri: &str) -> Result<String, CuriesError> {
        let rec = self.find_by_uri(uri)?;
        if uri.starts_with(&rec.uri_prefix) {
            Ok(uri.to_string())
        } else {
            let (_new_prefix, id) = rec
                .uri_prefix_synonyms
                .iter()
                .filter(|synonym| uri.starts_with(&**synonym))
                .max_by_key(|synonym| synonym.len()) // Get longest first
                .and_then(|synonym| uri.strip_prefix(synonym).map(|id| (synonym, id)))
                .ok_or_else(|| CuriesError::NotFound(uri.to_string()))?;
            Ok(format!("{}{}", rec.uri_prefix, id))
        }
    }

    /// Returns the number of `Records` in the `Converter`
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Returns true if there are no `Records` in the `Converter`
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}

/// Implement the `Default` trait since we have a constructor that does not need arguments
impl Default for Converter {
    fn default() -> Self {
        Self::new(":")
    }
}

impl fmt::Display for Converter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Converter contains {} records", self.records.len())?;
        Ok(())
    }
}

// Python API: https://github.com/cthoyt/curies/blob/main/src/curies/api.py#L1099

// /// Stores the prefix and local unique identifier
// /// for a compact URI (CURIE)
// pub struct Reference {
//     prefix: String,
//     identifier: String,
// }
