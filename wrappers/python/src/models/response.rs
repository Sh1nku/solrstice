use crate::models::error::PyErrWrapper;
use crate::models::facet_set::SolrFacetSetResultWrapper;
use crate::models::group::SolrGroupResultWrapper;
use crate::models::json_facet::SolrJsonFacetResponseWrapper;
use pyo3::prelude::*;
use pythonize::pythonize;
use solrstice::models::{SolrDocsResponse, SolrResponse};
use std::collections::HashMap;

#[derive(Clone)]
#[pyclass(name = "SolrDocsResponse", module = "solrstice.models", subclass)]
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
            docs.into_iter()
                .map(|doc| pythonize(py, &doc).map_err(PyErrWrapper::from))
                .collect::<Result<Vec<_>, _>>()
        })
        .map_err(|e| e.into())
    }
}

#[derive(Clone)]
#[pyclass(name = "SolrResponse", module = "solrstice.models", subclass)]
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

    pub fn get_groups(&self) -> HashMap<String, SolrGroupResultWrapper> {
        let groups = self.0.get_groups();
        match groups {
            None => HashMap::new(),
            Some(groups) => {
                let groups = groups
                    .iter()
                    .map(|(k, v)| (k.to_owned(), SolrGroupResultWrapper::from(v.to_owned())))
                    .collect::<HashMap<String, SolrGroupResultWrapper>>();
                groups
            }
        }
    }

    pub fn get_next_cursor_mark(&self) -> Option<&str> {
        self.0.next_cursor_mark.as_deref()
    }

    pub fn get_facet_set(&self) -> SolrFacetSetResultWrapper {
        let facet_set = self.0.get_facet_set();
        match facet_set {
            None => SolrFacetSetResultWrapper::default(),
            Some(facet_set) => facet_set.into(),
        }
    }

    pub fn get_json_facets(&self) -> Option<SolrJsonFacetResponseWrapper> {
        self.0.get_json_facets().map(|f| f.clone().into())
    }
}
