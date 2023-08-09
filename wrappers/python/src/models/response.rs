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
pub struct SolrDocsResponseWrapper {
    #[pyo3(get)]
    pub num_found: usize,
    #[pyo3(get)]
    pub start: usize,
    #[pyo3(get)]
    pub num_found_exact: bool,
    #[pyo3(get)]
    pub docs: Vec<PyObject>,
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
    pub fn get_response(&self) -> PyResult<Option<SolrDocsResponseWrapper>> {
        match self.0.get_response() {
            Some(v) => Ok(Some(
                SolrDocsResponseWrapper::try_from(v.to_owned()).map_err(PyErrWrapper::from)?,
            )),
            None => Ok(None),
        }
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

    #[getter]
    pub fn get_next_cursor_mark(&self) -> Option<&str> {
        self.0.next_cursor_mark.as_deref()
    }
}

impl TryFrom<SolrDocsResponse> for SolrDocsResponseWrapper {
    type Error = PyErrWrapper;

    fn try_from(value: SolrDocsResponse) -> Result<Self, Self::Error> {
        Python::with_gil(|py| -> Result<Self, Self::Error> {
            let docs = value
                .get_docs::<serde_json::Value>()
                .map_err(PyErrWrapper::from)?;
            let docs = docs
                .iter()
                .map(|x| pythonize(py, x).map_err(PyErrWrapper::from))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(SolrDocsResponseWrapper {
                num_found: value.num_found,
                start: value.start,
                num_found_exact: value.num_found_exact,
                docs,
            })
        })
    }
}
