use ::curies::{Converter, Record};
use pyo3::{exceptions::PyException, prelude::*};
use pythonize::pythonize;
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

/// Python bindings
#[pymodule]
fn curies_rs(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
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
    record: Record,
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
            record: Record {
                prefix,
                uri_prefix,
                prefix_synonyms: prefix_synonyms.into_iter().collect(),
                uri_prefix_synonyms: uri_prefix_synonyms.into_iter().collect(),
                pattern: None,
            },
        })
    }

    // Return the Record as a python dictionary
    #[pyo3(text_signature = "($self)")]
    fn dict(&self, py: Python<'_>) -> PyResult<PyObject> {
        pythonize(py, &self.record).map_err(|e| {
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
            converter: Converter::default(),
        })
        // Handle errors:
        // Converter::default()
        //     .map(|converter| Self { converter })
        //     .map_err(|e| PyErr::new::<PyException, _>(format!("{e}")))
    }

    #[staticmethod]
    #[pyo3(text_signature = "(data)")]
    fn load_extended_prefix_map(data: &str) -> PyResult<Self> {
        // Use a tokio runtime to wait on the async operation
        let rt = Runtime::new().map_err(|e| {
            PyErr::new::<PyException, _>(format!("Failed to create Tokio runtime: {e}"))
        })?;
        let result = rt.block_on(async move {
            Converter::from_extended_prefix_map(data)
                .await
                .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))
        });
        result.map(|converter| Self { converter })
    }

    #[pyo3(text_signature = "($self, record)")]
    fn add_record(&mut self, record: RecordPy) -> PyResult<()> {
        self.converter
            .add_record(record.record)
            .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))
    }

    fn compress(&self, uri: String) -> PyResult<String> {
        self.converter
            .compress(&uri)
            .map_err(|e| PyErr::new::<PyException, _>(e.to_string()))
    }
}
