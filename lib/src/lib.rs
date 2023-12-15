#![doc = include_str!("../docs/use_rust.md")]
use async_trait::async_trait;
use error::CuriesError;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use trie_rs::{Trie, TrieBuilder};

pub mod error;
pub mod sources;

/// A CURIE `Record`, containing its prefixes and URI prefixes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub prefix: String,
    pub uri_prefix: String,
    pub prefix_synonyms: HashSet<String>,
    pub uri_prefix_synonyms: HashSet<String>,
    // TODO: pattern: Option<String>,
}

impl Record {
    pub fn new(prefix: &str, uri_prefix: &str) -> Self {
        Record {
            prefix: prefix.to_string(),
            uri_prefix: uri_prefix.to_string(),
            prefix_synonyms: HashSet::from([]),
            uri_prefix_synonyms: HashSet::from([]),
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

/// A `Converter` is composed of 2 HashMaps (one for prefixes, one for URIs),
/// and a trie search to find the longest URI
/// # Examples
///
/// ```
/// use curies::{Converter, Record};
/// use std::collections::HashSet;
///
/// fn use_converter() -> Result<(), Box<dyn std::error::Error>> {
///     let mut converter = Converter::new();
///     let record1 = Record {
///         prefix: "doid".to_string(),
///         uri_prefix: "http://purl.obolibrary.org/obo/DOID_".to_string(),
///         prefix_synonyms: HashSet::from(["DOID".to_string()]),
///         uri_prefix_synonyms: HashSet::from(["https://identifiers.org/DOID/"].map(String::from)),
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
pub struct Converter {
    records: Vec<Arc<Record>>,
    prefix_map: HashMap<String, Arc<Record>>,
    uri_map: HashMap<String, Arc<Record>>,
    trie_builder: TrieBuilder<u8>,
    trie: Trie<u8>,
    // TODO: pattern_map: HashMap<String, String>
    // delimiter: char
}

impl Converter {
    /// Create an empty `Converter`
    ///
    /// # Examples
    ///
    /// ```
    /// use curies::{Converter, Record};
    /// use std::collections::HashSet;
    ///
    /// let mut converter = Converter::new();
    /// let record1 = Record {
    ///     prefix: "doid".to_string(),
    ///     uri_prefix: "http://purl.obolibrary.org/obo/DOID_".to_string(),
    ///     prefix_synonyms: HashSet::from(["DOID".to_string()]),
    ///     uri_prefix_synonyms: HashSet::from(["https://identifiers.org/DOID/"].map(String::from)),
    /// };
    /// converter.add_record(record1).unwrap();
    /// ```
    pub fn new() -> Self {
        Converter {
            records: Vec::new(),
            prefix_map: HashMap::new(),
            uri_map: HashMap::new(),
            trie_builder: TrieBuilder::new(),
            trie: TrieBuilder::new().build(),
        }
    }

    /// Create a `Converter` from a prefix `HashMap`
    ///
    /// # Examples
    ///
    /// ```
    /// use curies::{Converter, Record};
    /// use std::collections::HashMap;
    ///
    /// let mut prefix_map: HashMap<String, String> = HashMap::new();
    /// prefix_map.insert("DOID".to_string(), "http://purl.obolibrary.org/obo/DOID_".to_string());
    /// prefix_map.insert("OBO".to_string(), "http://purl.obolibrary.org/obo/".to_string());
    /// let converter = Converter::from_prefix_map(prefix_map).unwrap();
    ///
    /// let curie = converter.compress("http://purl.obolibrary.org/obo/DOID_1234").unwrap();
    /// assert_eq!(curie, "DOID:1234");
    /// ```
    pub fn from_prefix_map(prefix_map: HashMap<String, String>) -> Result<Self, CuriesError> {
        let mut converter = Converter::default();
        for (prefix, uri_prefix) in prefix_map {
            converter.add_record(Record::new(&prefix, &uri_prefix))?;
        }
        Ok(converter)
    }

    /// Create a `Converter` from a JSON-LD file context
    ///
    /// # Examples
    ///
    /// ```
    /// use curies::{Converter, Record, error::CuriesError};
    ///
    /// fn test_from_jsonld() -> Result<(), CuriesError> {
    ///     let converter = Converter::from_jsonld("http://purl.obolibrary.org/meta/obo_context.jsonld");
    ///     Ok(())
    /// }
    /// ```
    pub async fn from_jsonld<T: DataSource>(data: T) -> Result<Self, CuriesError> {
        let prefix_map = data.fetch().await?;
        let mut converter = Converter::default();
        let context = match prefix_map.get("@context") {
            Some(Value::Object(map)) => map,
            _ => return Err(CuriesError::InvalidFormat("JSON-LD".to_string())),
        };
        for (key, value) in context {
            if key.starts_with('@') {
                continue;
            }
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
        self.trie = self.trie_builder.build();
        Ok(())
    }

    // TODO: fn add_curie()

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

        Ok(format!("{}:{}", &record.prefix, id))
    }

    /// Expands a CURIE to a URI
    pub fn expand(&self, curie: &str) -> Result<String, CuriesError> {
        let parts: Vec<&str> = curie.split(':').collect();
        if parts.len() != 2 {
            return Err(CuriesError::InvalidCurie(curie.to_string()));
        }
        let (prefix, id) = (parts[0], parts[1]);
        self.find_by_prefix(prefix)
            .map(|record| format!("{}{}", record.uri_prefix, id))
    }
}

/// Implement the `Default` trait since we have a constructor that does not need arguments
impl Default for Converter {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Converter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Converter contains {} records", self.records.len())?;
        Ok(())
    }
}

/// Trait to provide the data as URL, HashMap, string, or Path to file
// FutureExt::shared
#[async_trait(?Send)]
pub trait DataSource: Send + Sync {
    async fn fetch(self) -> Result<HashMap<String, Value>, CuriesError>;
}
#[async_trait(?Send)]
impl DataSource for HashMap<String, Value> {
    async fn fetch(self) -> Result<HashMap<String, Value>, CuriesError> {
        Ok(self)
    }
}
#[async_trait(?Send)]
impl DataSource for HashMap<String, String> {
    async fn fetch(self) -> Result<HashMap<String, Value>, CuriesError> {
        Ok(self
            .into_iter()
            .map(|(key, value)| (key, Value::String(value)))
            .collect())
    }
}
#[async_trait(?Send)]
impl DataSource for &str {
    async fn fetch(self) -> Result<HashMap<String, Value>, CuriesError> {
        if self.starts_with("https://") || self.starts_with("http://") || self.starts_with("ftp://")
        {
            // Get URL content with HTTP request
            let client = reqwest::Client::new();
            Ok(client
                .get(self)
                .header(reqwest::header::ACCEPT, "application/json")
                .send()
                .await?
                .json()
                .await?)
        } else {
            // Directly parsing the provided string as JSON
            Ok(serde_json::from_str(self)?)
        }
    }
}
#[async_trait(?Send)]
impl DataSource for &Path {
    async fn fetch(self) -> Result<HashMap<String, Value>, CuriesError> {
        if self.exists() {
            // Read from a file path
            let mut file = File::open(self)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            Ok(serde_json::from_str(&contents)?)
        } else {
            return Err(CuriesError::NotFound(format!("{:?}", self.to_str())));
        }
    }
}

// Python API: https://github.com/cthoyt/curies/blob/main/src/curies/api.py#L1099
// HashSet more efficient than Vec: https://stackoverflow.com/questions/3185226/huge-performance-difference-between-vector-and-hashset
// But HashSet are not ordered, while Vec are ordered.
// This choice was made since Pydantic is more convenient to use with lists since they
// are directly JSON-serializable.

// /// Stores the prefix and local unique identifier
// /// for a compact URI (CURIE)
// pub struct Reference {
//     prefix: String,
//     identifier: String,
// }
