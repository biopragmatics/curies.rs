use crate::error::CuriesError;
use crate::fetch::{ExtendedPrefixMapSource, PrefixMapSource};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::Arc;
use trie_rs::{Trie, TrieBuilder};

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
///     converter.build();
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
pub struct Converter {
    records: Vec<Arc<Record>>,
    prefix_map: HashMap<String, Arc<Record>>,
    uri_map: HashMap<String, Arc<Record>>,
    trie_builder: TrieBuilder<u8>,
    trie: Trie<u8>,
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
            trie_builder: TrieBuilder::new(),
            trie: TrieBuilder::new().build(),
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
        converter.build();
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
        converter.build();
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
        data: T,
    ) -> Result<Self, CuriesError> {
        let records = data.fetch().await?;
        let mut converter = Converter::default();
        for record in records {
            converter.add_record(record)?;
        }
        converter.build();
        Ok(converter)
    }

    /// Add a `Record` to the `Converter`
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
        // TODO: check if synonyms are unique?

        self.records.push(rec.clone());
        self.prefix_map.insert(rec.prefix.clone(), rec.clone());
        self.uri_map.insert(rec.uri_prefix.clone(), rec.clone());
        self.trie_builder.push(&rec.uri_prefix);
        for prefix in &rec.prefix_synonyms {
            self.prefix_map.insert(prefix.clone(), rec.clone());
        }
        for uri_prefix in &rec.uri_prefix_synonyms {
            self.uri_map.insert(uri_prefix.clone(), rec.clone());
            self.trie_builder.push(uri_prefix);
        }
        // self.trie = self.trie_builder.build();
        Ok(())
    }

    /// Add a CURIE prefix and its prefix URI to the `Converter`
    pub fn add_curie(&mut self, prefix: &str, uri_prefix: &str) -> Result<(), CuriesError> {
        self.add_record(Record::new(prefix, uri_prefix))
    }

    /// Build trie search once all `Records` have been added
    pub fn build(&mut self) {
        self.trie = self.trie_builder.build();
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
    pub fn find_by_uri(&self, uri: &str) -> Result<&Arc<Record>, CuriesError> {
        let matching_uris = self.trie.common_prefix_search(uri);
        let utf8_uri = match matching_uris.last() {
            Some(u) => Ok(u),
            None => Err(CuriesError::NotFound(uri.to_string())),
        };
        self.find_by_uri_prefix(std::str::from_utf8(utf8_uri?)?)
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
