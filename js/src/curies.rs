use std::collections::HashSet;

use curies::{sources::get_obo_converter, Converter, Record};
use js_sys::Promise;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen(js_name = Record )]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordJs {
    record: Record,
}

#[allow(clippy::inherent_to_string, clippy::wrong_self_convention)]
#[wasm_bindgen(js_class = Record)]
impl RecordJs {
    #[wasm_bindgen(constructor)]
    pub fn new(
        prefix: String,
        uri_prefix: String,
        prefix_synonyms: Vec<String>,
        uri_prefix_synonyms: Vec<String>,
    ) -> Result<RecordJs, JsValue> {
        let prefix_synonyms_set: HashSet<String> = prefix_synonyms.into_iter().collect();
        let uri_prefix_synonyms_set: HashSet<String> = uri_prefix_synonyms.into_iter().collect();
        Ok(Self {
            record: Record {
                prefix,
                uri_prefix,
                prefix_synonyms: prefix_synonyms_set,
                uri_prefix_synonyms: uri_prefix_synonyms_set,
            },
        })
    }

    #[wasm_bindgen(js_name = toJs)]
    pub fn to_js(&self) -> Result<JsValue, JsValue> {
        to_value(&self.record).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.record.to_string()
    }
}

#[wasm_bindgen(js_name = Converter)]
pub struct ConverterJs {
    converter: Converter,
}

// Optional arguments: https://docs.rs/wasm-bindgen-derive/latest/wasm_bindgen_derive/#optional-arguments
// Maybe try https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html
#[allow(unused_variables, clippy::inherent_to_string)]
#[wasm_bindgen(js_class = Converter)]
impl ConverterJs {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<ConverterJs, JsValue> {
        Ok(Self {
            converter: Converter::new(),
        })
    }

    #[wasm_bindgen(js_name = addRecord)]
    pub fn add_record(&mut self, record: RecordJs) -> Result<(), JsValue> {
        self.converter
            .add_record(record.record)
            .map(|_| ())
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn expand(&self, curie: String) -> Result<String, JsValue> {
        self.converter
            .expand(&curie)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn compress(&self, uri: String) -> Result<String, JsValue> {
        self.converter
            .compress(&uri)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.converter.to_string()
    }

    // #[wasm_bindgen(js_name = prefixMap)]
    // pub fn prefix_map(&self) -> Result<JsValue, JsValue> {
    //     serde_wasm_bindgen::to_value(&self.converter.prefix_map).map_err(|e| e.into())
    // }
}

/// Get OBO converter
#[wasm_bindgen(js_name = getOboConverter)]
pub fn get_obo_converter_js() -> Promise {
    future_to_promise(async move {
        match get_obo_converter().await {
            Ok(converter) => Ok(JsValue::from(ConverterJs { converter })),
            Err(e) => Err(JsValue::from_str(&format!(
                "Error getting OBO converter: {e}"
            ))),
        }
    })
}

// impl Into<JsValue> for RecordJs {
//     fn into(self) -> JsValue {
//         // JsValue::from_serde(&self).unwrap()
//         self.to_js()
//     }
// }
