use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::{CuriesError, Record};

/// Trait to provide the prefix map as URL, HashMap, string, or Path to file
#[async_trait(?Send)]
pub trait PrefixMapSource: Send + Sync {
    async fn fetch(self) -> Result<HashMap<String, Value>, CuriesError>;
}
#[async_trait(?Send)]
impl PrefixMapSource for &str {
    async fn fetch(self) -> Result<HashMap<String, Value>, CuriesError> {
        Ok(serde_json::from_str(&fetch_url(self).await?)?)
    }
}
#[async_trait(?Send)]
impl PrefixMapSource for &Path {
    async fn fetch(self) -> Result<HashMap<String, Value>, CuriesError> {
        Ok(serde_json::from_str(&fetch_file(self).await?)?)
    }
}
#[async_trait(?Send)]
impl PrefixMapSource for HashMap<String, String> {
    async fn fetch(self) -> Result<HashMap<String, Value>, CuriesError> {
        Ok(self
            .into_iter()
            .map(|(key, value)| (key, Value::String(value)))
            .collect())
    }
}
#[async_trait(?Send)]
impl PrefixMapSource for HashMap<String, Value> {
    async fn fetch(self) -> Result<HashMap<String, Value>, CuriesError> {
        Ok(self)
    }
}

/// Trait to provide the extended prefix map as URL, `Vec<Record>`, string, or Path to file
#[async_trait(?Send)]
pub trait ExtendedPrefixMapSource: Send + Sync {
    async fn fetch(self) -> Result<Vec<Record>, CuriesError>;
}
#[async_trait(?Send)]
impl ExtendedPrefixMapSource for Vec<Record> {
    async fn fetch(self) -> Result<Vec<Record>, CuriesError> {
        Ok(self)
    }
}
#[async_trait(?Send)]
impl ExtendedPrefixMapSource for &str {
    async fn fetch(self) -> Result<Vec<Record>, CuriesError> {
        Ok(serde_json::from_str(&fetch_url(self).await?)?)
    }
}
#[async_trait(?Send)]
impl ExtendedPrefixMapSource for &Path {
    async fn fetch(self) -> Result<Vec<Record>, CuriesError> {
        Ok(serde_json::from_str(&fetch_file(self).await?)?)
    }
}

/// Given a string, fetch data as string if it is a URL, otherwise return the string
async fn fetch_url(url: &str) -> Result<String, CuriesError> {
    if url.starts_with("https://") || url.starts_with("http://") || url.starts_with("ftp://") {
        // Get URL content with HTTP request
        let client = reqwest::Client::new();
        Ok(client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .send()
            .await?
            .text()
            .await?)
    } else {
        Ok(url.to_owned())
    }
}

/// Given a `Path` get the file content if it exists
async fn fetch_file(path: &Path) -> Result<String, CuriesError> {
    if path.exists() {
        // Read from a file path
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    } else {
        Err(CuriesError::NotFound(format!("{:?}", path.to_str())))
    }
}
