use crate::models::error::PyErrWrapper;
use crate::models::response::SolrDocsResponseWrapper;
use pyo3::prelude::*;
use pythonize::pythonize;
use solrstice::models::{SolrGroupFieldResult, SolrGroupResult};

#[derive(Clone)]
#[pyclass(name = "SolrGroupResult", module = "solrstice.models", subclass)]
pub struct SolrGroupResultWrapper(SolrGroupResult);

#[pymethods]
impl SolrGroupResultWrapper {
    pub fn get_matches(&self) -> usize {
        self.0.get_matches()
    }

    pub fn get_n_groups(&self) -> Option<usize> {
        self.0.get_n_groups()
    }

    pub fn get_field_result(&self) -> Option<Vec<SolrGroupFieldResultWrapper>> {
        self.0
            .get_field_result()
            .map(|v| v.iter().map(|v| v.to_owned().into()).collect())
    }

    pub fn get_query_result(&self) -> PyResult<Option<SolrDocsResponseWrapper>> {
        match self.0.get_query_result() {
            Some(v) => Ok(Some(SolrDocsResponseWrapper::from(v.to_owned()))),
            None => Ok(None),
        }
    }

    pub fn get_simple_result(&self) -> PyResult<Option<SolrDocsResponseWrapper>> {
        match self.0.get_simple_result() {
            Some(v) => Ok(Some(SolrDocsResponseWrapper::from(v.to_owned()))),
            None => Ok(None),
        }
    }
}

impl From<SolrGroupResult> for SolrGroupResultWrapper {
    fn from(value: SolrGroupResult) -> Self {
        SolrGroupResultWrapper(value)
    }
}

impl From<SolrGroupResultWrapper> for SolrGroupResult {
    fn from(value: SolrGroupResultWrapper) -> Self {
        value.0
    }
}

#[derive(Clone)]
#[pyclass(name = "SolrGroupFieldResult", module = "solrstice.models", subclass)]
pub struct SolrGroupFieldResultWrapper(SolrGroupFieldResult);

#[pymethods]
impl SolrGroupFieldResultWrapper {
    pub fn get_group_value(&self) -> PyResult<PyObject> {
        Python::with_gil(|py| -> PyResult<PyObject> {
            let value = self
                .0
                .get_group_value::<serde_json::Value>()
                .map_err(PyErrWrapper::from)?;
            Ok(pythonize(py, &value)?)
        })
    }

    pub fn get_doc_list(&self) -> SolrDocsResponseWrapper {
        self.0.get_doc_list().into()
    }
}

impl From<SolrGroupFieldResult> for SolrGroupFieldResultWrapper {
    fn from(value: SolrGroupFieldResult) -> Self {
        SolrGroupFieldResultWrapper(value)
    }
}

impl From<SolrGroupFieldResultWrapper> for SolrGroupFieldResult {
    fn from(value: SolrGroupFieldResultWrapper) -> Self {
        value.0
    }
}
