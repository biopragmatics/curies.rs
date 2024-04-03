use curies::{sources::get_bioregistry_converter, Converter, Record};
use serde_json::Value;
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

#[test]
fn new_empty_converter() -> Result<(), Box<dyn std::error::Error>> {
    let mut converter = Converter::new(":");
    let record1 = Record {
        prefix: "doid".to_string(),
        uri_prefix: "http://purl.obolibrary.org/obo/DOID_".to_string(),
        prefix_synonyms: HashSet::from(["DOID".to_string()]),
        uri_prefix_synonyms: HashSet::from(["https://identifiers.org/DOID/"].map(String::from)),
        pattern: None,
    };
    let record2 = Record {
        prefix: "obo".to_string(),
        uri_prefix: "http://purl.obolibrary.org/obo/".to_string(),
        prefix_synonyms: HashSet::from(["OBO".to_string()]),
        uri_prefix_synonyms: HashSet::from(["https://identifiers.org/obo/"].map(String::from)),
        pattern: None,
    };
    let record3 = Record {
        prefix: "wrongpattern".to_string(),
        uri_prefix: "http://purl.obolibrary.org/wrongpattern/".to_string(),
        prefix_synonyms: HashSet::new(),
        uri_prefix_synonyms: HashSet::new(),
        pattern: Some("\\".to_string()), // Wrong pattern for test
    };
    assert!(format!("{}", record1).starts_with("Prefix: doid"));
    assert!(format!("{}", converter).starts_with("Converter contains"));
    converter.add_record(record1.clone())?;
    converter.add_record(record2)?;
    converter.add_record(record3)?;
    assert_eq!(converter.len(), 3);
    assert!(!converter.is_empty());
    assert!(converter.get_prefixes(true).len() > converter.get_prefixes(false).len());
    assert!(converter.get_uri_prefixes(true).len() > converter.get_uri_prefixes(false).len());
    assert!(converter.write_extended_prefix_map()?.starts_with("[{"));
    assert!(converter.write_prefix_map().len() == 3);
    assert!(converter.write_jsonld()["@context"]
        .to_string()
        .starts_with("{"));
    println!("{:?}", converter.write_jsonld());
    // println!("{:?}", converter.write_extended_prefix_map());

    // Find Record by prefix or URI
    assert_eq!(converter.find_by_prefix("doid")?.prefix, "doid");
    assert_eq!(
        converter
            .find_by_uri_prefix("http://purl.obolibrary.org/obo/DOID_")?
            .prefix,
        "doid"
    );
    assert_eq!(
        converter
            .find_by_uri("http://purl.obolibrary.org/obo/DOID_1234")?
            .prefix,
        "doid"
    );

    // Test expansion and compression
    assert_eq!(
        converter.expand("doid:1234")?,
        "http://purl.obolibrary.org/obo/DOID_1234"
    );
    assert_eq!(
        converter.compress("http://purl.obolibrary.org/obo/DOID_1234")?,
        "doid:1234"
    );
    assert_eq!(
        converter.expand("DOID:1234")?,
        "http://purl.obolibrary.org/obo/DOID_1234"
    );
    assert_eq!(
        converter.compress("https://identifiers.org/DOID/1234")?,
        "doid:1234"
    );
    assert_eq!(
        converter.compress("http://purl.obolibrary.org/obo/1234")?,
        "obo:1234"
    );

    assert_eq!(
        converter
            .compress_list(["http://wrong/1234", "https://identifiers.org/DOID/1234"].to_vec()),
        [None, Some("doid:1234".to_string())].to_vec()
    );
    assert_eq!(
        converter.expand_list(["doid:1234", "wrong:1234"].to_vec()),
        [
            Some("http://purl.obolibrary.org/obo/DOID_1234".to_string()),
            None
        ]
        .to_vec()
    );

    // Test wrong calls
    assert!(converter
        .add_prefix("doid", "http://purl.obolibrary.org/obo/DOID_")
        .map_err(|e| assert!(e.to_string().starts_with("Duplicate record")))
        .is_err());
    assert!(converter
        .add_record(Record::new("wrong", "http://purl.obolibrary.org/obo/DOID_"))
        .is_err());
    assert!(converter
        .expand("wrong:1234")
        .map_err(|e| assert!(e.to_string().starts_with("Not found")))
        .is_err());
    assert!(converter
        .expand("wrong")
        .map_err(|e| assert!(e.to_string().starts_with("Invalid CURIE")))
        .is_err());
    assert!(converter.find_by_uri_prefix("wrong").is_err());
    assert!(converter.expand("wrongpattern:1234").is_err());
    let record4 = Record {
        prefix: "wrong".to_string(),
        uri_prefix: "http://wrong.org/".to_string(),
        prefix_synonyms: HashSet::new(),
        uri_prefix_synonyms: HashSet::from(["https://identifiers.org/obo/"].map(String::from)),
        pattern: None,
    };
    assert!(converter.add_record(record4).is_err());
    let record5 = Record {
        prefix: "wrong".to_string(),
        uri_prefix: "http://wrong.org/".to_string(),
        prefix_synonyms: HashSet::from(["OBO".to_string()]),
        uri_prefix_synonyms: HashSet::new(),
        pattern: None,
    };
    assert!(converter.add_record(record5).is_err());
    Ok(())
}

#[tokio::test]
async fn from_prefix_map_converter() -> Result<(), Box<dyn std::error::Error>> {
    let mut prefix_map: HashMap<String, String> = HashMap::new();
    prefix_map.insert(
        "DOID".to_string(),
        "http://purl.obolibrary.org/obo/DOID_".to_string(),
    );
    prefix_map.insert(
        "OBO".to_string(),
        "http://purl.obolibrary.org/obo/".to_string(),
    );
    let converter = Converter::from_prefix_map(prefix_map.clone()).await?;
    assert_eq!(
        converter.expand("DOID:1234")?,
        "http://purl.obolibrary.org/obo/DOID_1234"
    );
    assert_eq!(
        converter.compress("http://purl.obolibrary.org/obo/DOID_1234")?,
        "DOID:1234"
    );
    assert_eq!(
        converter.compress("http://purl.obolibrary.org/obo/1234")?,
        "OBO:1234"
    );
    assert!(Converter::from_jsonld(prefix_map).await.is_err());
    let prefix_map: HashMap<String, Value> = HashMap::new();
    assert!(Converter::from_jsonld(prefix_map).await.is_err());
    Ok(())
}

#[tokio::test]
async fn from_jsonld_context_file() -> Result<(), Box<dyn std::error::Error>> {
    let converter = Converter::from_jsonld(Path::new("tests/resources/context.jsonld")).await?;
    assert_eq!(
        converter.expand("DOID:1234")?,
        "http://purl.obolibrary.org/obo/DOID_1234"
    );
    assert_eq!(
        converter.compress("http://purl.obolibrary.org/obo/DOID_1234")?,
        "DOID:1234"
    );
    Ok(())
}

#[tokio::test]
async fn from_extended_map_file() -> Result<(), Box<dyn std::error::Error>> {
    let converter =
        Converter::from_extended_prefix_map(Path::new("tests/resources/extended_map.json")).await?;
    assert_eq!(
        converter.expand("doid:1234")?,
        "http://purl.obolibrary.org/obo/SPECIAL_DOID_1234"
    );
    assert_eq!(
        converter.compress("https://purl.obolibrary.org/obo/DOID_1234")?,
        "doid:1234"
    );
    assert!(converter
        .expand("doid:AAAA") // Test pattern
        .map_err(|e| assert!(e.to_string().starts_with("Invalid format")))
        .is_err());
    Ok(())
}

#[tokio::test]
async fn from_extended_map_vec() -> Result<(), Box<dyn std::error::Error>> {
    let records: Vec<Record> = [
        Record::new("doid", "http://purl.obolibrary.org/obo/DOID_"),
        Record::new("obo", "http://purl.obolibrary.org/obo/"),
    ]
    .to_vec();
    let converter = Converter::from_extended_prefix_map(records).await?;
    assert_eq!(
        converter.expand("doid:1234")?,
        "http://purl.obolibrary.org/obo/DOID_1234"
    );
    assert_eq!(
        converter.compress("http://purl.obolibrary.org/obo/DOID_1234")?,
        "doid:1234"
    );
    Ok(())
}

#[tokio::test]
async fn from_converter_errors() -> Result<(), Box<dyn std::error::Error>> {
    assert!(Converter::from_jsonld("{}").await.is_err());
    assert!(Converter::from_jsonld("wrong")
        .await
        .map_err(|e| assert!(e.to_string().starts_with("Error parsing")))
        .is_err());
    assert!(Converter::from_jsonld("https://wrong")
        .await
        .map_err(|e| assert!(e.to_string().starts_with("Error sending")))
        .is_err());
    assert!(Converter::from_jsonld(Path::new("wrong"))
        .await
        .map_err(|e| assert!(e.to_string().starts_with("Error reading")))
        .is_err());
    Ok(())
}

#[tokio::test]
async fn chain_converters() -> Result<(), Box<dyn std::error::Error>> {
    let converter = Converter::chain(vec![
        get_bioregistry_converter().await?,
        Converter::from_extended_prefix_map(Path::new("tests/resources/extended_map.json")).await?,
    ])?;
    assert_eq!(
        converter.compress("http://purl.obolibrary.org/obo/SPECIAL_DOID_1234")?,
        "doid:1234"
    );
    assert_eq!(
        converter.expand("specialgo:1234567")?,
        "http://purl.obolibrary.org/obo/GO_1234567"
    );
    assert!(Converter::chain(vec![]).is_err());
    // assert!(converter.delete_record("Wrong").is_err());
    Ok(())
}
