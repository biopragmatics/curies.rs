use error::CuriesError;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use trie_rs::{Trie, TrieBuilder};

use crate::error::DuplicateRecordError;
pub mod error;

/// A CURIE `Record`, containing its prefixes and URI prefixes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub prefix: String,
    pub uri_prefix: String,
    pub prefix_synonyms: HashSet<String>,
    pub uri_prefix_synonyms: HashSet<String>,
    // TODO: pattern: Option<String>,
}

/// A `Converter` is composed of 2 HashMaps (one for prefixes, one for URIs),
/// and a trie search to find the longest URI
pub struct Converter {
    prefix_map: HashMap<String, Arc<Record>>,
    uri_map: HashMap<String, Arc<Record>>,
    trie_builder: TrieBuilder<u8>,
    trie: Trie<u8>,
    // TODO: pattern_map: HashMap<String, String>
}

impl Converter {
    /// Create an empty `Converter`
    pub fn new() -> Self {
        Converter {
            prefix_map: HashMap::new(),
            uri_map: HashMap::new(),
            trie_builder: TrieBuilder::new(),
            trie: TrieBuilder::new().build(),
        }
    }

    /// When adding a new CURIE we create a reference to the `Record` (Arc)
    /// And we use this reference in the prefix and URI hashmaps
    pub fn add_record(&mut self, record: Record) -> Result<(), DuplicateRecordError> {
        let rec = Arc::new(record);
        if self.prefix_map.contains_key(&rec.prefix) {
            return Err(DuplicateRecordError(rec.prefix.clone()));
        }
        if self.uri_map.contains_key(&rec.uri_prefix) {
            return Err(DuplicateRecordError(rec.uri_prefix.clone()));
        }
        // TODO: check if synonyms are unique?

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
        // Ok(self.prefix_map.get(prefix))
        match self.prefix_map.get(prefix) {
            Some(record) => Ok(record),
            None => Err(CuriesError::NotFound(prefix.to_string())),
        }
    }

    /// Find corresponding CURIE `Record` given a URI prefix
    pub fn find_by_uri_prefix(&self, uri_prefix: &str) -> Option<&Arc<Record>> {
        self.uri_map.get(uri_prefix)
    }

    /// Find corresponding CURIE `Record` given a complete URI
    pub fn find_by_uri(&self, uri: &str) -> Option<&Arc<Record>> {
        let uri_in_u8s = self.trie.common_prefix_search(uri);
        let longest_uri = match std::str::from_utf8(uri_in_u8s.last()?) {
            Ok(valid_str) => valid_str,
            Err(_) => return None, // If UTF-8 conversion fails, return None
        };
        self.find_by_uri_prefix(longest_uri)
    }

    /// Compresses a URI to a CURIE
    pub fn compress(&self, uri: &str) -> Option<String> {
        self.find_by_uri(uri).and_then(|record| {
            let prefix = &record.prefix;
            let id = uri.strip_prefix(&record.uri_prefix).or_else(|| {
                record
                    .uri_prefix_synonyms
                    .iter()
                    .find_map(|synonym| uri.strip_prefix(synonym))
            })?;
            Some(format!("{}:{}", prefix, id))
        })
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

// Python API: https://github.com/cthoyt/curies/blob/main/src/curies/api.py#L1099
// HashSet lookup more efficient than Vec: O(1) vs O(n). But HashSet are not ordered, while Vec are ordered

// /// Stores the prefix and local unique identifier
// /// for a compact URI (CURIE)
// pub struct Reference {
//     prefix: String,
//     identifier: String,
// }

// pub struct Record {
//     curie_prefix: String,
//     uri_prefix: String,
//     curie_prefix_synonyms: Vec<String>,
//     uri_prefix_synonyms: Vec<String>,
// }
