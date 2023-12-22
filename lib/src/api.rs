//! API for `Converter` and `Record`

use crate::error::CuriesError;
use crate::fetch::{ExtendedPrefixMapSource, PrefixMapSource};
use ptrie::Trie;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
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
    uri_map: HashMap<String, Arc<Record>>,
    trie: Trie<char, Arc<Record>>,
    delimiter: String,
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
            uri_map: HashMap::new(),
            // trie: TrieBuilder::new().build(),
            trie: Trie::<char, Arc<Record>>::new(),
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
    /// }).expect("Failed to create the GO converter");
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
    ///     let converter = Converter::from_jsonld("http://purl.obolibrary.org/meta/obo_context.jsonld");
    ///     Ok(())
    /// }
    /// ```
    pub async fn from_jsonld<T: PrefixMapSource>(data: T) -> Result<Self, CuriesError> {
        let prefix_map = data.fetch().await?;
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
    /// * `data` - The extended prefix map data, as URL, string, file, or `Vec<HashMap>`
    ///
    /// # Examples
    ///
    /// ```
    /// use curies::Converter;
    ///
    /// let converter = Converter::from_extended_prefix_map("https://raw.github.com/biopragmatics/bioregistry/main/exports/contexts/bioregistry.epm.json");
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

    /// Add a `Record` to the `Converter`.
    /// When adding a new record we create a reference to the `Record` (Arc)
    /// And we use this reference in the prefix and URI hashmaps
    pub fn add_record(&mut self, record: Record) -> Result<(), CuriesError> {
        let rec = Arc::new(record);
        if self.prefix_map.contains_key(&rec.prefix) {
            return Err(CuriesError::DuplicateRecord(rec.prefix.clone()));
        }
        if self.uri_map.contains_key(&rec.uri_prefix) {
            return Err(CuriesError::DuplicateRecord(rec.uri_prefix.clone()));
        }
        // Check if any of the synonyms are already present in the maps
        for prefix in &rec.prefix_synonyms {
            if self.prefix_map.contains_key(prefix) {
                return Err(CuriesError::DuplicateRecord(prefix.clone()));
            }
        }
        for uri_prefix in &rec.uri_prefix_synonyms {
            if self.uri_map.contains_key(uri_prefix) {
                return Err(CuriesError::DuplicateRecord(uri_prefix.clone()));
            }
        }
        self.records.push(rec.clone());
        self.prefix_map.insert(rec.prefix.clone(), rec.clone());
        for prefix in &rec.prefix_synonyms {
            self.prefix_map.insert(prefix.clone(), rec.clone());
        }
        self.uri_map.insert(rec.uri_prefix.clone(), rec.clone());
        self.trie
            .insert(rec.uri_prefix.clone().chars(), rec.clone());
        for uri_prefix in &rec.uri_prefix_synonyms {
            self.uri_map.insert(uri_prefix.clone(), rec.clone());
            self.trie.insert(uri_prefix.chars(), rec.clone());
        }
        Ok(())
    }

    /// Add a CURIE prefix and its prefix URI to the `Converter`
    pub fn add_curie(&mut self, prefix: &str, uri_prefix: &str) -> Result<(), CuriesError> {
        self.add_record(Record::new(prefix, uri_prefix))
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
        self.uri_map.insert(rec.uri_prefix.clone(), rec.clone());
        for prefix in &rec.prefix_synonyms {
            self.prefix_map.insert(prefix.clone(), rec.clone());
        }
        for uri_prefix in &rec.uri_prefix_synonyms {
            self.uri_map.insert(uri_prefix.clone(), rec.clone());
        }
        if self
            .trie
            .set_value(rec.uri_prefix.chars(), rec.clone())
            .is_err()
        {
            self.trie.insert(rec.uri_prefix.chars(), rec.clone());
        }
        for uri_prefix in &rec.uri_prefix_synonyms {
            if self
                .trie
                .set_value(uri_prefix.chars(), rec.clone())
                .is_err()
            {
                self.trie.insert(uri_prefix.chars(), rec.clone());
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
        match self.uri_map.get(uri_prefix) {
            Some(record) => Ok(record),
            None => Err(CuriesError::NotFound(uri_prefix.to_string())),
        }
    }

    /// Find corresponding CURIE `Record` given a complete URI
    pub fn find_by_uri(&self, uri: &str) -> Result<Arc<Record>, CuriesError> {
        match self.trie.find_longest_prefix(uri.chars()) {
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
                    .find_map(|synonym| uri.strip_prefix(synonym))
            })
            .ok_or_else(|| CuriesError::NotFound(uri.to_string()))?;
        self.validate_id(id, &record)?;
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
