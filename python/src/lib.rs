mod api;

use crate::api::*;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// Python bindings
#[pymodule]
fn curies_rs(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("__package__", "curies-rs")?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__author__", env!("CARGO_PKG_AUTHORS").replace(':', "\n"))?;

    m.add_class::<RecordPy>()?;
    m.add_class::<ConverterPy>()?;
    m.add_wrapped(wrap_pyfunction!(get_obo_converter))?;
    m.add_wrapped(wrap_pyfunction!(get_bioregistry_converter))?;
    m.add_wrapped(wrap_pyfunction!(get_monarch_converter))?;
    m.add_wrapped(wrap_pyfunction!(get_go_converter))
}
