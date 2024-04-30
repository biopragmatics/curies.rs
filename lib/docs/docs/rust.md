# 🦀 Use from Rust

[![crates.io](https://img.shields.io/crates/v/curies.svg)](https://crates.io/crates/curies)

## 📥️ Installation

```bash
cargo add curies
```

## 🚀 Usage

You can use the Rust crate to work with CURIEs: import converters, compress URIs, expand CURIEs.

```rust
use curies::{Converter, Record, sources::get_bioregistry_converter};
use std::collections::HashSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Load from a prefix map json (string or URI)
    let converterFromMap = Converter::from_prefix_map(r#"{
  "doid": "http://purl.obolibrary.org/obo/MY_DOID_"
}"#).await?;

    // Load from an extended prefix map (string or URI)
    let converterFromUrl = Converter::from_extended_prefix_map("https://w3id.org/biopragmatics/bioregistry.epm.json").await?;

    // Load from a JSON-LD context (string or URI)
    let converterFromJsonld = Converter::from_jsonld("https://purl.obolibrary.org/meta/obo_context.jsonld").await?;

    // Load from one of the predefined source
    let converterFromSource = get_bioregistry_converter().await?;

    // Chain multiple converters in one
    let converter = Converter::chain(vec![converterFromMap, converterFromUrl, converterFromSource])?;

    let uri = converter.expand("doid:1234")?;
    println!("Expanded CURIE: {}", uri);

    let curie = converter.compress("http://purl.obolibrary.org/obo/DOID_1234")?;
    println!("Compressed URI: {}", curie);
    Ok(())
}
```

And

```rust
use curies::sources::get_bioregistry_converter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let converter = get_bioregistry_converter().await?;

    let epm = converter.write_extended_prefix_map()?;
    let pm = converter.write_prefix_map();
    let jsonld = converter.write_jsonld();
    let shacl = converter.write_shacl()?;
    Ok(())
}
```

## 🛠️ Manipulate converters and records

You can also build a `Converter` programmatically from `Record` structs:

```rust
extern crate curies;
use curies::{Converter, Record};
use std::collections::HashSet;

fn build_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut converter = Converter::default();

    let record1 = Record {
        prefix: "doid".to_string(),
        uri_prefix: "http://purl.obolibrary.org/obo/DOID_".to_string(),
        prefix_synonyms: HashSet::from(["DOID".to_string()]),
        uri_prefix_synonyms: HashSet::from(["https://identifiers.org/DOID/"].map(String::from)),
        pattern: None,
    };
    let record2 = Record::new("obo", "http://purl.obolibrary.org/obo/");
    converter.add_record(record1)?;
    converter.add_record(record2)?;

    let uri = converter.expand("doid:1234")?;
    println!("Expanded CURIE: {}", uri);

    let curie = converter.compress("http://purl.obolibrary.org/obo/DOID_1234")?;
    println!("Compressed URI: {}", curie);
    Ok(())
}
build_example().unwrap();
```

## 📖 API reference

Checkout the **[API documentation](https://docs.rs/curies)** for more details on how to use the different components and functions of the rust crate.
