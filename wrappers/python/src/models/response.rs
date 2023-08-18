use crate::models::error::PyErrWrapper;
use crate::models::group::{SolrGroupFieldResultWrapper, SolrGroupResultWrapper};
use pyo3::prelude::*;
use pythonize::pythonize;
use solrstice::models::response::{SolrDocsResponse, SolrResponse};
use std::collections::HashMap;

#[pymodule]
pub fn response(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SolrResponseWrapper>()?;
    m.add_class::<SolrDocsResponseWrapper>()?;
    m.add_class::<SolrGroupResultWrapper>()?;
    m.add_class::<SolrGroupFieldResultWrapper>()?;
    Ok(())
}

#[derive(Clone)]
#[pyclass(name = "SolrDocsResponse", module = "solrstice.response")]
pub struct SolrDocsResponseWrapper(SolrDocsResponse);

impl From<SolrDocsResponse> for SolrDocsResponseWrapper {
    fn from(value: SolrDocsResponse) -> Self {
        SolrDocsResponseWrapper(value)
    }
}

impl From<SolrDocsResponseWrapper> for SolrDocsResponse {
    fn from(value: SolrDocsResponseWrapper) -> Self {
        value.0
    }
}

impl From<&SolrDocsResponse> for SolrDocsResponseWrapper {
    fn from(value: &SolrDocsResponse) -> Self {
        SolrDocsResponseWrapper(value.to_owned())
    }
}

impl From<&SolrDocsResponseWrapper> for SolrDocsResponse {
    fn from(value: &SolrDocsResponseWrapper) -> Self {
        value.0.to_owned()
    }
}

#[pymethods]
impl SolrDocsResponseWrapper {
    pub fn get_num_found(&self) -> usize {
        self.0.get_num_found()
    }

    pub fn get_start(&self) -> usize {
        self.0.get_start()
    }

    pub fn get_num_found_exact(&self) -> bool {
        self.0.get_num_found_exact()
    }

    pub fn get_docs(&self) -> PyResult<Vec<PyObject>> {
        Python::with_gil(|py| -> Result<Vec<PyObject>, PyErrWrapper> {
            let docs = self
                .0
                .get_docs::<serde_json::Value>()
                .map_err(PyErrWrapper::from)?;
            let vec = docs
                .into_iter()
                .map(|doc| pythonize(py, &doc).map_err(PyErrWrapper::from))
                .collect::<Result<Vec<_>, _>>();
            vec
        })
        .map_err(|e| e.into())
    }
}

#[derive(Clone)]
#[pyclass(name = "SolrResponse", module = "solrstice.response")]
pub struct SolrResponseWrapper(SolrResponse);

impl From<SolrResponse> for SolrResponseWrapper {
    fn from(value: SolrResponse) -> Self {
        SolrResponseWrapper(value)
    }
}

#[pymethods]
impl SolrResponseWrapper {
    pub fn get_docs_response(&self) -> Option<SolrDocsResponseWrapper> {
        self.0
            .get_docs_response()
            .map(SolrDocsResponseWrapper::from)
    }

    pub fn get_groups(&self) -> Option<HashMap<String, SolrGroupResultWrapper>> {
        let groups = self.0.get_groups();
        match groups {
            None => None,
            Some(groups) => {
                let groups = groups
                    .iter()
                    .map(|(k, v)| (k.to_owned(), SolrGroupResultWrapper::from(v.to_owned())))
                    .collect::<HashMap<String, SolrGroupResultWrapper>>();
                Some(groups)
            }
        }
    }

    pub fn get_next_cursor_mark(&self) -> Option<&str> {
        self.0.next_cursor_mark.as_deref()
    }
}
