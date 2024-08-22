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

/// JavaScript binding for a `Record` struct
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

/// JavaScript binding for a `Converter` struct
#[wasm_bindgen(js_name = Converter)]
pub struct ConverterJs {
    converter: Converter,
}

// Optional arguments: https://docs.rs/wasm-bindgen-derive/latest/wasm_bindgen_derive/#optional-arguments
// Maybe try https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html
#[allow(unused_variables, clippy::inherent_to_string)]
#[wasm_bindgen(js_class = Converter)]
impl ConverterJs {
    /// Create blank `Converter`
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<ConverterJs, JsValue> {
        Ok(Self {
            converter: Converter::default(),
        })
    }

    /// Load `Converter` from prefix map JSON string or URL
    #[wasm_bindgen(static_method_of = ConverterJs, js_name = fromPrefixMap)]
    pub fn from_prefix_map(prefix_map: String) -> Promise {
        future_to_promise(async move {
            match Converter::from_prefix_map(&*prefix_map).await {
                Ok(converter) => Ok(JsValue::from(ConverterJs { converter })),
                Err(e) => Err(JsValue::from_str(&e.to_string())),
            }
        })
    }

    /// Load `Converter` from extended prefix map JSON string or URL
    #[wasm_bindgen(static_method_of = ConverterJs, js_name = fromExtendedPrefixMap)]
    pub fn from_extended_prefix_map(prefix_map: String) -> Promise {
        future_to_promise(async move {
            match Converter::from_extended_prefix_map(&*prefix_map).await {
                Ok(converter) => Ok(JsValue::from(ConverterJs { converter })),
                Err(e) => Err(JsValue::from_str(&e.to_string())),
            }
        })
    }

    /// Load `Converter` from JSON-LD string or URL
    #[wasm_bindgen(static_method_of = ConverterJs, js_name = fromJsonld)]
    pub fn from_jsonld(jsonld: String) -> Promise {
        future_to_promise(async move {
            match Converter::from_jsonld(&*jsonld).await {
                Ok(converter) => Ok(JsValue::from(ConverterJs { converter })),
                Err(e) => Err(JsValue::from_str(&e.to_string())),
            }
        })
    }

    /// Load `Converter` from JSON-LD string or URL
    #[wasm_bindgen(static_method_of = ConverterJs, js_name = fromShacl)]
    pub fn from_shacl(shacl: String) -> Promise {
        future_to_promise(async move {
            match Converter::from_shacl(&*shacl).await {
                Ok(converter) => Ok(JsValue::from(ConverterJs { converter })),
                Err(e) => Err(JsValue::from_str(&e.to_string())),
            }
        })
    }

    /// Add `Record` to the `Converter`
    #[wasm_bindgen(js_name = addRecord)]
    pub fn add_record(&mut self, record: RecordJs) -> Result<(), JsValue> {
        self.converter
            .add_record(record.record)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Add a CURIE as `Record` to the `Converter`
    #[wasm_bindgen(js_name = addCurie)]
    pub fn add_prefix(&mut self, prefix: &str, uri_prefix: &str) -> Result<(), JsValue> {
        self.converter
            .add_prefix(prefix, uri_prefix)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Chain with another `Converter`
    pub fn chain(&self, converter: &ConverterJs) -> Result<ConverterJs, JsValue> {
        Converter::chain(vec![self.converter.clone(), converter.converter.clone()])
            .map(|converter| ConverterJs { converter })
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Expand a CURIE to URI
    pub fn expand(&self, curie: String) -> Result<String, JsValue> {
        self.converter
            .expand(&curie)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Compress a URI to CURIE
    pub fn compress(&self, uri: String) -> Result<String, JsValue> {
        self.converter
            .compress(&uri)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    // TODO: Use Vec<String> instead of JsValue possible?
    /// Expand a list of CURIEs to URIs
    #[wasm_bindgen(js_name = expandList)]
    pub fn expand_list(
        &self,
        curies: JsValue,
        passthrough: Option<bool>,
    ) -> Result<JsValue, JsValue> {
        let curies_vec: Vec<String> = serde_wasm_bindgen::from_value(curies)
            .map_err(|e| JsValue::from_str(&format!("Error converting CURIEs list: {}", e)))?;
        let js_array = self
            .converter
            .expand_list(
                curies_vec.iter().map(String::as_str).collect(),
                passthrough.unwrap_or(true),
            )
            .into_iter()
            .map(JsValue::from)
            .collect::<Array>();
        Ok(JsValue::from(js_array))
    }

    /// Compress a list of URIs to CURIEs
    #[wasm_bindgen(js_name = compressList)]
    pub fn compress_list(
        &self,
        curies: JsValue,
        passthrough: Option<bool>,
    ) -> Result<JsValue, JsValue> {
        let curies_vec: Vec<String> = serde_wasm_bindgen::from_value(curies)
            .map_err(|e| JsValue::from_str(&format!("Error converting URIs list: {}", e)))?;
        let js_array = self
            .converter
            .compress_list(
                curies_vec.iter().map(String::as_str).collect(),
                passthrough.unwrap_or(true),
            )
            .into_iter()
            .map(JsValue::from)
            .collect::<Array>();
        Ok(JsValue::from(js_array))
    }

    /// Standardize prefix
    #[wasm_bindgen(js_name = standardizePrefix)]
    pub fn standardize_prefix(&self, prefix: String) -> Result<String, JsValue> {
        self.converter
            .standardize_prefix(&prefix)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Standardize a CURIE
    #[wasm_bindgen(js_name = standardizeCurie)]
    pub fn standardize_curie(&self, curie: String) -> Result<String, JsValue> {
        self.converter
            .standardize_curie(&curie)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Standardize a URI
    #[wasm_bindgen(js_name = standardizeUri)]
    pub fn standardize_uri(&self, uri: String) -> Result<String, JsValue> {
        self.converter
            .standardize_uri(&uri)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Checks if a given string is a valid URI according to the current `Converter`
    #[wasm_bindgen(js_name = isUri)]
    pub fn is_uri(&self, uri: String) -> bool {
        self.converter.is_uri(&uri)
    }

    /// Checks if a given string is a valid CURIE according to the current `Converter`
    #[wasm_bindgen(js_name = isCurie)]
    pub fn is_curie(&self, curie: String) -> bool {
        self.converter.is_curie(&curie)
    }

    /// Attempts to compress a URI to a CURIE, or standardize it if it's already a CURIE.
    #[wasm_bindgen(js_name = compressOrStandardize)]
    pub fn compress_or_standardize(&self, input: String) -> Result<String, JsValue> {
        self.converter
            .compress_or_standardize(&input)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Attempts to expand a CURIE to a URI, or standardize it if it's already a URI.
    #[wasm_bindgen(js_name = expandOrStandardize)]
    pub fn expand_or_standardize(&self, input: String) -> Result<String, JsValue> {
        self.converter
            .expand_or_standardize(&input)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = getPrefixes)]
    pub fn get_prefixes(&self, include_synonyms: Option<bool>) -> Vec<String> {
        self.converter
            .get_prefixes(include_synonyms.unwrap_or(false))
    }

    #[wasm_bindgen(js_name = getUriPrefixes)]
    pub fn get_uri_prefixes(&self, include_synonyms: Option<bool>) -> Vec<String> {
        self.converter
            .get_uri_prefixes(include_synonyms.unwrap_or(false))
    }

    /// Write the `Converter` as a simple prefix map JSON
    #[wasm_bindgen(js_name = writePrefixMap)]
    pub fn write_prefix_map(&self) -> String {
        format!("{:?}", self.converter.write_prefix_map())
    }

    /// Write the `Converter` as a extended prefix map JSON
    #[wasm_bindgen(js_name = writeExtendedPrefixMap)]
    pub fn write_extended_prefix_map(&self) -> Result<String, JsValue> {
        Ok((self
            .converter
            .write_extended_prefix_map()
            .map_err(|e| JsValue::from_str(&e.to_string()))?)
        .to_string())
    }

    /// Write the `Converter` prefix map as JSON-LD context
    #[wasm_bindgen(js_name = writeJsonld)]
    pub fn write_jsonld(&self) -> String {
        format!("{}", self.converter.write_jsonld())
    }

    /// Write the `Converter` prefix map as SHACL prefixes definition
    #[wasm_bindgen(js_name = writeShacl)]
    pub fn write_shacl(&self) -> Result<String, JsValue> {
        self.converter
            .write_shacl()
            .map_err(|e| JsValue::from_str(&e.to_string()))
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
