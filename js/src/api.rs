use std::collections::HashSet;

use curies::{
    sources::{
        get_bioregistry_converter, get_go_converter, get_monarch_converter, get_obo_converter,
    },
    Converter, Record,
};
use js_sys::{Array, Promise};
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
                pattern: None,
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
            converter: Converter::default(),
        })
    }

    #[wasm_bindgen(static_method_of = ConverterJs, js_name = fromPrefixMap)]
    pub fn from_prefix_map(prefix_map: String) -> Promise {
        future_to_promise(async move {
            match Converter::from_prefix_map(&*prefix_map).await {
                Ok(converter) => Ok(JsValue::from(ConverterJs { converter })),
                Err(e) => Err(JsValue::from_str(&e.to_string())),
            }
        })
    }

    #[wasm_bindgen(static_method_of = ConverterJs, js_name = fromJsonld)]
    pub fn from_jsonld(jsonld: String) -> Promise {
        future_to_promise(async move {
            match Converter::from_jsonld(&*jsonld).await {
                Ok(converter) => Ok(JsValue::from(ConverterJs { converter })),
                Err(e) => Err(JsValue::from_str(&e.to_string())),
            }
        })
    }

    #[wasm_bindgen(static_method_of = ConverterJs, js_name = fromExtendedPrefixMap)]
    pub fn from_extended_prefix_map(prefix_map: String) -> Promise {
        future_to_promise(async move {
            match Converter::from_extended_prefix_map(&*prefix_map).await {
                Ok(converter) => Ok(JsValue::from(ConverterJs { converter })),
                Err(e) => Err(JsValue::from_str(&e.to_string())),
            }
        })
    }

    #[wasm_bindgen(js_name = addRecord)]
    pub fn add_record(&mut self, record: RecordJs) -> Result<(), JsValue> {
        self.converter
            .add_record(record.record)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = addCurie)]
    pub fn add_curie(&mut self, prefix: &str, uri_prefix: &str) -> Result<(), JsValue> {
        self.converter
            .add_curie(prefix, uri_prefix)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn chain(&self, converter: &ConverterJs) -> Result<ConverterJs, JsValue> {
        Converter::chain(vec![self.converter.clone(), converter.converter.clone()])
            .map(|converter| ConverterJs { converter })
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

    #[wasm_bindgen(js_name = expandList)]
    pub fn expand_list(&self, curies: JsValue) -> Result<JsValue, JsValue> {
        let curies_vec: Vec<String> = serde_wasm_bindgen::from_value(curies)
            .map_err(|e| JsValue::from_str(&format!("Error converting CURIEs list: {}", e)))?;
        let js_array = self
            .converter
            .expand_list(curies_vec.iter().map(String::as_str).collect())
            .into_iter()
            .map(JsValue::from)
            .collect::<Array>();
        Ok(JsValue::from(js_array))
    }

    #[wasm_bindgen(js_name = compressList)]
    pub fn compress_list(&self, curies: JsValue) -> Result<JsValue, JsValue> {
        let curies_vec: Vec<String> = serde_wasm_bindgen::from_value(curies)
            .map_err(|e| JsValue::from_str(&format!("Error converting URIs list: {}", e)))?;
        let js_array = self
            .converter
            .compress_list(curies_vec.iter().map(String::as_str).collect())
            .into_iter()
            .map(JsValue::from)
            .collect::<Array>();
        Ok(JsValue::from(js_array))
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.converter.to_string()
    }
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

/// Get Bioregistry converter
#[wasm_bindgen(js_name = getBioregistryConverter)]
pub fn get_bioregistry_converter_js() -> Promise {
    future_to_promise(async move {
        match get_bioregistry_converter().await {
            Ok(converter) => Ok(JsValue::from(ConverterJs { converter })),
            Err(e) => Err(JsValue::from_str(&format!(
                "Error getting Bioregistry converter: {e}"
            ))),
        }
    })
}

/// Get GO converter
#[wasm_bindgen(js_name = getGoConverter)]
pub fn get_go_converter_js() -> Promise {
    future_to_promise(async move {
        match get_go_converter().await {
            Ok(converter) => Ok(JsValue::from(ConverterJs { converter })),
            Err(e) => Err(JsValue::from_str(&format!(
                "Error getting GO converter: {e}"
            ))),
        }
    })
}

/// Get Monarch converter
#[wasm_bindgen(js_name = getMonarchConverter)]
pub fn get_monarch_converter_js() -> Promise {
    future_to_promise(async move {
        match get_monarch_converter().await {
            Ok(converter) => Ok(JsValue::from(ConverterJs { converter })),
            Err(e) => Err(JsValue::from_str(&format!(
                "Error getting Monarch converter: {e}"
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

// NOTE: we cannot easily convert a JS object to a string in Rust, it needs to be done in JS with JSON.stringify()
// fn get_str_from_obj(obj: JsValue) -> Result<String, JsValue> {
//     if obj.is_string() {
//         obj.as_string().ok_or_else(|| JsValue::from_str("String conversion failed"))
//     } else if obj.is_object() {
//         let str: String = serde_wasm_bindgen::from_value(obj)
//             .map_err(|e| JsValue::from_str(&format!("Failed to serialize JSON: {}", e)))?;
//         Ok(str)
//     } else {
//         return Err(JsValue::from_str("Expected a string or a JSON object"));
//     }
// }

// #[wasm_bindgen(static_method_of = ConverterJs)]
// pub fn chain(converters: &JsValue) -> Promise {
//     future_to_promise(async move {
//         let converters_vec: Vec<ConverterJs> = serde_wasm_bindgen::from_value(converters).map_err(|e| {
//             JsValue::from_str(&format!("Error converting converters list: {}", e))
//         })?;
//         let rust_converters: Vec<Converter> = converters_vec
//             .into_iter()
//             .map(|converter_js| converter_js.converter)
//             .collect();
//         match Converter::chain(rust_converters) {
//             Ok(converter) => Ok(JsValue::from(ConverterJs { converter })),
//             Err(e) => Err(JsValue::from_str(&e.to_string())),
//         }
//     })
// }
