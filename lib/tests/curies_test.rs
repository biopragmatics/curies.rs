use curies::{Converter, Record, sources::get_obo_converter};
use std::collections::{HashSet, HashMap};

#[test]
fn new_empty_converter() -> Result<(), Box<dyn std::error::Error>> {
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
    let curie = converter.find_by_prefix("doid")?;
    assert_eq!(curie.prefix, "doid");
    println!("Found CURIE by prefix: {}", curie.prefix);

    let curie = converter.find_by_uri_prefix("http://purl.obolibrary.org/obo/DOID_")?;
    assert_eq!(curie.prefix, "doid");
    println!("Found CURIE by URI prefix: {}", curie.prefix);

    let curie = converter.find_by_uri("http://purl.obolibrary.org/obo/DOID_1234")?;
    assert_eq!(curie.prefix, "doid");
    println!("Found CURIE by URI: {}", curie.prefix);

    // Test expansion and compression
    let uri = converter.expand("doid:1234")?;
    println!("Expanded CURIE: {}", uri);
    assert_eq!(uri, "http://purl.obolibrary.org/obo/DOID_1234");

    let curie = converter.compress("http://purl.obolibrary.org/obo/DOID_1234")?;
    println!("Compressed URI: {}", curie);
    assert_eq!(curie, "doid:1234");
    Ok(())
}


#[test]
fn from_prefix_map_converter() -> Result<(), Box<dyn std::error::Error>> {
    let mut prefix_map: HashMap<String, String> = HashMap::new();
    prefix_map.insert("DOID".to_string(), "http://purl.obolibrary.org/obo/DOID_".to_string());
    prefix_map.insert("OBO".to_string(), "http://purl.obolibrary.org/obo/".to_string());
    let converter = Converter::from_prefix_map(prefix_map)?;

    let uri = converter.expand("DOID:1234")?;
    println!("Expanded CURIE: {}", uri);
    assert_eq!(uri, "http://purl.obolibrary.org/obo/DOID_1234");

    let curie = converter.compress("http://purl.obolibrary.org/obo/DOID_1234")?;
    println!("Compressed URI: {}", curie);
    assert_eq!(curie, "DOID:1234");
    Ok(())
}

#[tokio::test]
async fn from_jsonld_converter() -> Result<(), Box<dyn std::error::Error>> {
    // let url = "http://purl.obolibrary.org/meta/obo_context.jsonld";
    let converter = get_obo_converter().await?;

    let uri = converter.expand("DOID:1234")?;
    println!("Expanded CURIE: {}", uri);
    assert_eq!(uri, "http://purl.obolibrary.org/obo/DOID_1234");

    let curie = converter.compress("http://purl.obolibrary.org/obo/DOID_1234")?;
    println!("Compressed URI: {}", curie);
    assert_eq!(curie, "DOID:1234");
    Ok(())
}
