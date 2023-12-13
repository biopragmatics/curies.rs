use curies::{Converter, Record};
use std::{collections::HashSet, error::Error, fs};

#[test]
fn main_tests() -> Result<(), Box<dyn std::error::Error>> {
    let mut converter = Converter::new();

    let record1 = Record {
        prefix: "doid".to_string(),
        uri_prefix: "http://purl.obolibrary.org/obo/DOID_".to_string(),
        prefix_synonyms: HashSet::from(["DOID".to_string()]),
        uri_prefix_synonyms: HashSet::from(["https://identifiers.org/DOID/"].map(String::from)),
    };
    let record2 = Record {
        prefix: "obo".to_string(),
        uri_prefix: "http://purl.obolibrary.org/obo/".to_string(),
        prefix_synonyms: HashSet::from(["OBO".to_string()]),
        uri_prefix_synonyms: HashSet::from(["https://identifiers.org/obo/"].map(String::from)),
    };
    converter.add_record(record1)?;
    converter.add_record(record2)?;

    // Find Record by prefix or URI
    let curie = converter.find_by_prefix("doid").unwrap();
    assert_eq!(curie.prefix, "doid");
    println!("Found CURIE by prefix: {}", curie.prefix);

    let curie = converter
        .find_by_uri_prefix("http://purl.obolibrary.org/obo/DOID_")
        .unwrap();
    assert_eq!(curie.prefix, "doid");
    println!("Found CURIE by URI prefix: {}", curie.prefix);

    let curie = converter
        .find_by_uri("http://purl.obolibrary.org/obo/DOID_1234")
        .unwrap();
    assert_eq!(curie.prefix, "doid");
    println!("Found CURIE by URI: {}", curie.prefix);

    // Test expansion and compression
    let uri = converter.expand("doid:1234").unwrap();
    println!("Expanded CURIE: {}", uri);
    assert_eq!(uri, "http://purl.obolibrary.org/obo/DOID_1234");

    let curie = converter
        .compress("http://purl.obolibrary.org/obo/DOID_1234")
        .unwrap();
    println!("Compressed URI: {}", curie);
    assert_eq!(curie, "doid:1234");
    Ok(())
}
