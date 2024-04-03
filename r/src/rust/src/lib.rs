use extendr_api::prelude::*;

use ::curies::{sources::get_bioregistry_converter, Converter as RsConverter};
use tokio::runtime::Runtime;

/// Converter struct for R
/// @export
pub struct Converter {
    converter: RsConverter,
    pub name: String,
}

#[extendr]
impl Converter {
    fn new() -> Result<Self> {
        // Building from empty converter works
        // let mut converter = Converter::default();
        // let record1 = Record {
        //     prefix: "doid".to_string(),
        //     uri_prefix: "http://purl.obolibrary.org/obo/DOID_".to_string(),
        //     prefix_synonyms: HashSet::from(["DOID".to_string()]),
        //     uri_prefix_synonyms: HashSet::from(["https://identifiers.org/DOID/"].map(String::from)),
        //     pattern: None,
        // };
        // converter.add_record(record1).unwrap();
        // But import using the async function and rt.block_on fails to generate wrapper functions
        let converter = init_converter();
        Ok(Self {
            converter,
            name: "".to_string(),
        })
    }

    fn compress(&self, uri: &str) -> String {
        self.converter.compress(uri).unwrap()
    }

    fn expand(&self, curie: &str) -> String {
        self.converter.expand(curie).unwrap()
    }
}

/// Initialize converter
fn init_converter() -> RsConverter {
    let rt = Runtime::new().unwrap();
    rt.block_on(async { get_bioregistry_converter().await.unwrap() })
}

// Macro to generate exports. This ensures exported functions are registered with R. See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod curies;
    impl Converter;
    // fn hello_world;
}

// Return string `"Hello world!"` to R.
// @export
// #[extendr]
// fn hello_world() -> &'static str {
//     "Hello world!"
// }
