# 🦀 Use from Rust

[![crates.io](https://img.shields.io/crates/v/curies.svg)](https://crates.io/crates/curies)

You can use the Rust crate to work with CURIEs:

```rust
use curies::{Converter, Record};
use std::collections::HashSet;

fn example() -> Result<(), Box<dyn std::error::Error>> {
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
    converter.build();

    let uri = converter.expand("doid:1234")?;
    println!("Expanded CURIE: {}", uri);

    let curie = converter.compress("http://purl.obolibrary.org/obo/DOID_1234")?;
    println!("Compressed URI: {}", curie);
    Ok(())
}
example().unwrap();
```

## 📖 API reference

Checkout the **[API documentation](https://docs.rs/curies)** for more details on how to use the different components and functions of the rust crate.
