use extendr_api::prelude::*;

use ::curies::Converter;
use tokio::runtime::Runtime;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

pub struct ConverterR {
    // pub converter: Converter,
    // Getting error when using the converter struct
    // "Error in `value[[3L]]()`: Failed to generate wrapper functions"
    pub name: String,
}

#[extendr]
impl ConverterR {
    fn new() -> Result<Self> {
        Ok(Self {
            // converter: Converter::default(),
            name: "".to_string(),
        })
        // Handle errors:
        // Converter::default()
        //     .map(|converter| Self { converter })
        //     .map_err(|e| PyErr::new::<PyException, _>(format!("{e}")))
    }

    fn compress(&self, uri: &str) -> String {
        format!("{uri}{}", self.name)
    }

    // fn load_extended_prefix_map(data: &str) -> Result<Self> {
    //     // Use a tokio runtime to wait on the async operation
    //     let rt = Runtime::new().unwrap();
    //     // map_err(|e| {
    //     //     Error::msg(format!("Failed to create Tokio runtime: {e}"))
    //     // })?;
    //     let result = rt.block_on(async move {
    //         Converter::from_extended_prefix_map(data).await.unwrap()
    //         // .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))
    //     });
    //     // result.map(|converter| Self { converter })
    //     Ok(Self { converter: result })
    // }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod helloextendr;
    fn hello_world;
    impl ConverterR;
}
