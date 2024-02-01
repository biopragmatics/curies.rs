use extendr_api::prelude::*;

use ::curies::Converter;
// use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

pub struct ConverterR {
    converter: Converter,
}

#[extendr]
impl ConverterR {
    fn new() -> Result<Self> {
        Ok(Self {
            converter: Converter::default(),
        })
        // Handle errors:
        // Converter::default()
        //     .map(|converter| Self { converter })
        //     .map_err(|e| PyErr::new::<PyException, _>(format!("{e}")))
    }

    fn load_extended_prefix_map(data: &str) -> Result<Self> {
        // Use a tokio runtime to wait on the async operation
        let rt = Runtime::new().unwrap();
        // map_err(|e| {
        //     Error::msg(format!("Failed to create Tokio runtime: {e}"))
        // })?;
        let result = rt.block_on(async move {
            Converter::from_extended_prefix_map(data).await.unwrap()
            // .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))
        });
        // result.map(|converter| Self { converter })
        Ok(Self { converter: result })
    }

    // fn add_record(&mut self, record: RecordR) -> Result<()> {
    //     self.converter
    //         .add_record(record.record)
    //         .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))
    // }

    fn compress(&self, uri: String) -> Result<String> {
        Ok(self.converter.compress(&uri).unwrap())
        // .map_err(|e| Err(Error::Other(e.to_string())))
    }
}

// #[extendr]
// #[derive(Debug, Clone)]
// pub struct RecordR {
//     record: Record,
// }

// #[extendr]
// impl RecordR {
//     fn new(
//         prefix: String,
//         uri_prefix: String,
//         prefix_synonyms: Vec<String>,
//         uri_prefix_synonyms: Vec<String>,
//     ) -> Result<Self> {
//         Ok(Self {
//             record: Record {
//                 prefix,
//                 uri_prefix,
//                 prefix_synonyms: prefix_synonyms.into_iter().collect(),
//                 uri_prefix_synonyms: uri_prefix_synonyms.into_iter().collect(),
//                 pattern: None,
//             },
//         })
//     }
// }

// /// Return string `"Hello world!"` to R.
// /// @export
// #[extendr]
// fn hello_world() -> &'static str {
//     "Hello world!"
// }

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod curiesr;
    impl ConverterR;
    // fn hello_world;
}
