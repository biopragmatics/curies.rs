use ::curies::{
    sources::{
        get_bioregistry_converter as get_bioregistry_converter_rs,
        get_go_converter as get_go_converter_rs, get_monarch_converter as get_monarch_converter_rs,
        get_obo_converter as get_obo_converter_rs,
    },
    Converter, Record,
};
use pyo3::{exceptions::PyException, prelude::*};
use pythonize::pythonize;
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

#[pyclass(name = "Record", module = "curies_rs")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordPy {
    record: Record,
}

#[pymethods]
impl RecordPy {
    #[new]
    #[pyo3(text_signature = "(prefix, uri_prefix, prefix_synonyms=[], uri_prefix_synonyms=[])")]
    fn new(
        prefix: String,
        uri_prefix: String,
        prefix_synonyms: Option<Vec<String>>,
        uri_prefix_synonyms: Option<Vec<String>>,
    ) -> PyResult<Self> {
        Ok(Self {
            record: Record {
                prefix,
                uri_prefix,
                prefix_synonyms: prefix_synonyms.unwrap_or_default().into_iter().collect(),
                uri_prefix_synonyms: uri_prefix_synonyms
                    .unwrap_or_default()
                    .into_iter()
                    .collect(),
                pattern: None,
            },
        })
    }

    // Return the Record as a python dictionary
    #[pyo3(text_signature = "()")]
    fn dict(&self, py: Python<'_>) -> PyResult<PyObject> {
        pythonize(py, &self.record).map_err(|e| {
            PyErr::new::<PyException, _>(format!("Error converting struct Record to dict: {e}"))
        })
    }
}

/// Python bindings for the CURIE/URI Converter struct
#[pyclass(name = "Converter", module = "curies_rs", sequence)]
pub struct ConverterPy {
    converter: Converter,
}

#[pymethods]
impl ConverterPy {
    #[new]
    #[pyo3(text_signature = "()")]
    fn new() -> PyResult<Self> {
        Ok(Self {
            converter: Converter::default(),
        })
        // Handle errors:
        // Converter::default()
        //     .map(|converter| Self { converter })
        //     .map_err(|e| PyErr::new::<PyException, _>(format!("{e}")))
    }

    /// Load a `Converter` from an extended prefix map JSON string or URL
    #[staticmethod]
    #[pyo3(text_signature = "(data)")]
    fn from_extended_prefix_map(data: &str) -> PyResult<Self> {
        // Use a tokio runtime to wait on the async operation
        let rt = Runtime::new().map_err(|e| {
            PyErr::new::<PyException, _>(format!("Failed to create Tokio runtime: {e}"))
        })?;
        rt.block_on(async move {
            Converter::from_extended_prefix_map(data)
                .await
                .map(|converter| Self { converter })
                .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))
        })
    }

    // #[staticmethod]
    // #[pyo3(text_signature = "(data)")]
    // fn from_extended_prefix_map(py: Python, data: &PyAny) -> PyResult<Self> {
    //     let rt = Runtime::new().map_err(|e| PyErr::new::<PyException, _>(format!("Failed to create Tokio runtime: {e}")))?;
    //     rt.block_on(async move {
    //         let converter_result = if let Ok(s) = data.downcast::<PyString>() {
    //             // Handle the case where the input is a string
    //             Converter::from_extended_prefix_map(s.to_str()?)
    //                 .await
    //                 .map_err(|e| PyErr::new::<PyException, _>(format!("Error processing string input: {e}")))
    //         } else if let Ok(dict) = data.downcast::<PyDict>() {
    //             // Handle the case where the input is a dictionary
    //             // let json_str = dict_to_json(py, dict)?;
    //             let hashmap = dict;
    //             Converter::from_extended_prefix_map(&hashmap)
    //                 .await
    //                 .map_err(|e| PyErr::new::<PyException, _>(format!("Error processing dictionary input: {e}")))
    //         } else {
    //             // Return an error if neither type matches
    //             Err(PyErr::new::<PyTypeError, _>("Expected a string or a dictionary"))
    //         };
    //         converter_result.map(|converter| Self { converter })
    //     })
    // }

    /// Load a `Converter` from a prefix map JSON string or URL
    #[staticmethod]
    #[pyo3(text_signature = "(data)")]
    fn from_prefix_map(data: &str) -> PyResult<Self> {
        let rt = Runtime::new().map_err(|e| {
            PyErr::new::<PyException, _>(format!("Failed to create Tokio runtime: {e}"))
        })?;
        rt.block_on(async move {
            Converter::from_prefix_map(data)
                .await
                .map(|converter| Self { converter })
                .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))
        })
    }

    /// Load a `Converter` from a JSON-LD context string or URL
    #[staticmethod]
    #[pyo3(text_signature = "(data)")]
    fn from_jsonld(data: &str) -> PyResult<Self> {
        let rt = Runtime::new().map_err(|e| {
            PyErr::new::<PyException, _>(format!("Failed to create Tokio runtime: {e}"))
        })?;
        rt.block_on(async move {
            Converter::from_jsonld(data)
                .await
                .map(|converter| Self { converter })
                .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))
        })
    }

    /// Load a `Converter` from a SHACL prefix definition string or URL
    #[staticmethod]
    #[pyo3(text_signature = "(data)")]
    fn from_shacl(data: &str) -> PyResult<Self> {
        let rt = Runtime::new().map_err(|e| {
            PyErr::new::<PyException, _>(format!("Failed to create Tokio runtime: {e}"))
        })?;
        rt.block_on(async move {
            Converter::from_shacl(data)
                .await
                .map(|converter| Self { converter })
                .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))
        })
    }

    /// Add a record to the `Converter`
    #[pyo3(text_signature = "(record)")]
    fn add_record(&mut self, record: RecordPy) -> PyResult<()> {
        self.converter
            .add_record(record.record)
            .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))
    }

    /// Add a prefix/namespace to the `Converter`
    #[pyo3(text_signature = "(prefix, namespace)")]
    fn add_prefix(&mut self, prefix: String, namespace: String) -> PyResult<()> {
        self.converter
            .add_prefix(&prefix, &namespace)
            .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))
    }

    /// Compress a URI
    #[pyo3(text_signature = "(uri)")]
    fn compress(&self, uri: String) -> PyResult<String> {
        self.converter
            .compress(&uri)
            .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))
    }

    /// Expand a CURIE
    #[pyo3(text_signature = "(curie)")]
    fn expand(&self, curie: String) -> PyResult<String> {
        self.converter
            .expand(&curie)
            .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))
    }

    /// Expand a list of CURIEs
    #[pyo3(text_signature = "(curies)")]
    fn expand_list(&self, curies: Vec<String>, passthrough: Option<bool>) -> Vec<Option<String>> {
        self.converter.expand_list(
            curies.iter().map(|s| s.as_str()).collect(),
            passthrough.unwrap_or(true),
        )
    }

    /// Compress a list of URIs
    #[pyo3(text_signature = "(uris)")]
    fn compress_list(&self, uris: Vec<String>, passthrough: Option<bool>) -> Vec<Option<String>> {
        self.converter.compress_list(
            uris.iter().map(|s| s.as_str()).collect(),
            passthrough.unwrap_or(true),
        )
    }

    /// Standardize prefix
    #[pyo3(text_signature = "(prefix)")]
    fn standardize_prefix(&self, prefix: String) -> PyResult<String> {
        self.converter
            .standardize_prefix(&prefix)
            .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))
    }

    /// Standardize a CURIE
    #[pyo3(text_signature = "(curie)")]
    fn standardize_curie(&self, curie: String) -> PyResult<String> {
        self.converter
            .standardize_curie(&curie)
            .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))
    }

    /// Standardize a URI
    #[pyo3(text_signature = "(uri)")]
    fn standardize_uri(&self, uri: String) -> PyResult<String> {
        self.converter
            .standardize_uri(&uri)
            .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))
    }

    /// Checks if a given string is a valid URI according to the current `Converter`
    #[pyo3(text_signature = "(uri)")]
    fn is_uri(&self, uri: String) -> bool {
        self.converter.is_uri(&uri)
    }

    /// Checks if a given string is a valid CURIE according to the current `Converter`
    #[pyo3(text_signature = "(curie)")]
    fn is_curie(&self, curie: String) -> bool {
        self.converter.is_curie(&curie)
    }

    /// Attempts to compress a URI to a CURIE, or standardize it if it's already a CURIE.
    #[pyo3(text_signature = "(input)")]
    fn compress_or_standardize(&self, input: String) -> PyResult<String> {
        self.converter
            .compress_or_standardize(&input)
            .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))
    }

    /// Attempts to expand a CURIE to a URI, or standardize it if it's already a URI.
    #[pyo3(text_signature = "(input)")]
    fn expand_or_standardize(&self, input: String) -> PyResult<String> {
        self.converter
            .expand_or_standardize(&input)
            .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))
    }

    #[pyo3(text_signature = "(include_synonyms=False)")]
    fn get_prefixes(&self, include_synonyms: Option<bool>) -> Vec<String> {
        self.converter
            .get_prefixes(include_synonyms.unwrap_or(false))
    }

    #[pyo3(text_signature = "(include_synonyms=False)")]
    fn get_uri_prefixes(&self, include_synonyms: Option<bool>) -> Vec<String> {
        self.converter
            .get_uri_prefixes(include_synonyms.unwrap_or(false))
    }

    /// Chain with another `Converter`
    #[pyo3(text_signature = "(converter)")]
    fn chain(&self, converter: &ConverterPy) -> PyResult<Self> {
        Converter::chain(vec![self.converter.clone(), converter.converter.clone()])
            .map(|converter| ConverterPy { converter })
            .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))
    }

    /// Write the `Converter` as a simple prefix map JSON
    #[pyo3(text_signature = "()")]
    fn write_prefix_map(&self) -> String {
        format!("{:?}", self.converter.write_prefix_map())
    }

    /// Write the `Converter` as a extended prefix map JSON
    #[pyo3(text_signature = "()")]
    fn write_extended_prefix_map(&self) -> PyResult<String> {
        Ok((self
            .converter
            .write_extended_prefix_map()
            .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))?)
        .to_string())
    }

    /// Write the `Converter` prefix map as JSON-LD context
    #[pyo3(text_signature = "()")]
    fn write_jsonld(&self) -> String {
        format!("{}", self.converter.write_jsonld())
    }

    #[pyo3(text_signature = "()")]
    fn write_shacl(&self) -> PyResult<String> {
        self.converter
            .write_shacl()
            .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))
    }

    // NOTE: could there be a way to pass a list of converters?
    // #[staticmethod]
    // #[pyo3(text_signature = "(converters)")]
    // fn chain(converters: Vec<PyRef<ConverterPy>>) -> PyResult<Self> {
    //     Converter::chain(converters.into_iter().map(|c| c.converter).collect())
    //         .map(|converter| ConverterPy { converter.clone() })
    //         .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))
    // }

    /// Support for python `len()`
    fn __len__(&self) -> usize {
        self.converter.len()
    }
}

#[pyfunction]
pub fn get_obo_converter() -> PyResult<ConverterPy> {
    let rt = Runtime::new().map_err(|e| {
        PyErr::new::<PyException, _>(format!("Failed to create Tokio runtime: {e}"))
    })?;
    rt.block_on(async {
        get_obo_converter_rs()
            .await
            .map(|converter| ConverterPy { converter })
            .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))
    })
}

#[pyfunction]
pub fn get_bioregistry_converter() -> PyResult<ConverterPy> {
    // TODO: https://pyo3.rs/v0.21.1/ecosystem/async-await py: Python<'_>
    let rt = Runtime::new().map_err(|e| {
        PyErr::new::<PyException, _>(format!("Failed to create Tokio runtime: {e}"))
    })?;
    rt.block_on(async {
        get_bioregistry_converter_rs()
            .await
            .map(|converter| ConverterPy { converter })
            .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))
    })
    // pyo3_asyncio::tokio::future_into_py(py, async {
    //     get_bioregistry_converter_rs()
    //         .await
    //         .map(|converter| ConverterPy { converter })
    //         .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))
    // })
    // pyo3_asyncio::tokio::future_into_py(py, async {
    //     let py_converter = get_bioregistry_converter_rs().await.map_err(|e| PyErr::new::<PyException, _>(e.to_string()))?;
    //     let converter = py_converter.try_into()?;
    //     Ok(ConverterPy { converter })
    // })
}

// Maybe we need to implement IntoPy?
// impl IntoPy<Py<PyAny>> for ConverterPy {
//     fn into_py(self, py: Python<'_>) -> Py<PyAny> {
//         self.0
//     }
// }

#[pyfunction]
pub fn get_monarch_converter() -> PyResult<ConverterPy> {
    let rt = Runtime::new().map_err(|e| {
        PyErr::new::<PyException, _>(format!("Failed to create Tokio runtime: {e}"))
    })?;
    rt.block_on(async {
        get_monarch_converter_rs()
            .await
            .map(|converter| ConverterPy { converter })
            .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))
    })
}

#[pyfunction]
pub fn get_go_converter() -> PyResult<ConverterPy> {
    let rt = Runtime::new().map_err(|e| {
        PyErr::new::<PyException, _>(format!("Failed to create Tokio runtime: {e}"))
    })?;
    rt.block_on(async {
        get_go_converter_rs()
            .await
            .map(|converter| ConverterPy { converter })
            .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))
    })
}
