use std::collections::HashSet;

use ::curies::{Converter, Record};
use pyo3::{exceptions::PyException, prelude::*};
use pythonize::pythonize;
use serde::{Deserialize, Serialize};

/// Python bindings
#[pymodule]
fn curies(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("__package__", "curies-rs")?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__author__", env!("CARGO_PKG_AUTHORS").replace(':', "\n"))?;

    m.add_class::<RecordPy>()?;
    m.add_class::<ConverterPy>()
}

#[pyclass(name = "Record", module = "curies_rs")]
// #[pyclass(extends=Record, name = "Record", module = "curies_rs")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordPy {
    prefix: String,
    uri_prefix: String,
    prefix_synonyms: HashSet<String>,
    uri_prefix_synonyms: HashSet<String>,
}

impl RecordPy {
    #[allow(clippy::wrong_self_convention)]
    fn into_record(&self) -> Record {
        Record {
            prefix: self.prefix.clone(),
            uri_prefix: self.uri_prefix.clone(),
            prefix_synonyms: self.prefix_synonyms.clone(),
            uri_prefix_synonyms: self.uri_prefix_synonyms.clone(),
        }
    }
}

#[pymethods]
impl RecordPy {
    #[new]
    #[pyo3(text_signature = "(prefix, uri_prefix, prefix_synonyms, uri_prefix_synonyms)")]
    fn new(
        prefix: String,
        uri_prefix: String,
        prefix_synonyms: Vec<String>,
        uri_prefix_synonyms: Vec<String>,
    ) -> PyResult<Self> {
        Ok(Self {
            prefix,
            uri_prefix,
            prefix_synonyms: prefix_synonyms.into_iter().collect(),
            uri_prefix_synonyms: uri_prefix_synonyms.into_iter().collect(),
        })
    }

    // Return the Record as a python dictionary
    #[pyo3(text_signature = "($self)")]
    fn dict(&self, py: Python<'_>) -> PyResult<PyObject> {
        pythonize(py, &self).map_err(|e| {
            PyErr::new::<PyException, _>(format!("Error converting struct Record to dict: {e}"))
        })
    }
}

#[pyclass(name = "Converter", module = "curies_rs")]
pub struct ConverterPy {
    converter: Converter,
}

#[pymethods]
impl ConverterPy {
    #[new]
    #[pyo3(text_signature = "()")]
    fn new() -> PyResult<Self> {
        Ok(Self {
            converter: Converter::new(),
        })
        // Handle errors:
        // Converter::new()
        //     .map(|converter| Self { converter })
        //     .map_err(|e| PyErr::new::<PyException, _>(format!("{e}")))
    }

    #[pyo3(text_signature = "($self, record)")]
    fn add_record(&mut self, record: &RecordPy) -> PyResult<()> {
        self.converter
            .add_record(record.into_record())
            .map_err(|e| PyErr::new::<PyException, _>(format!("Error Checking: {e}")))
    }

    fn compress(&self, uri: String) -> PyResult<String> {
        self.converter
            .compress(&uri)
            .map_err(|e| PyErr::new::<PyException, _>(format!("Error Checking: {e}")))
    }
}
