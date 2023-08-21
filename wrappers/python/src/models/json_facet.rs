use crate::models::error::PyErrWrapper;
use crate::queries::components::json_facet::{
    JsonFacetComponentWrapper, JsonFacetTypeWrapper, JsonQueryFacetWrapper, JsonStatFacetWrapper,
    JsonTermsFacetWrapper,
};
use pyo3::prelude::*;
use pythonize::pythonize;
use solrstice::models::json_facet::SolrJsonFacetResponse;
use std::collections::HashMap;

#[pymodule]
pub fn json_facet(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SolrJsonFacetResponseWrapper>()?;
    m.add_class::<JsonFacetComponentWrapper>()?;
    m.add_class::<JsonFacetTypeWrapper>()?;
    m.add_class::<JsonQueryFacetWrapper>()?;
    m.add_class::<JsonTermsFacetWrapper>()?;
    m.add_class::<JsonStatFacetWrapper>()?;
    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
#[pyclass(name = "SolrJsonFacetResponse", module = "solrstice.json_facet")]
pub struct SolrJsonFacetResponseWrapper(SolrJsonFacetResponse);

#[pymethods]
impl SolrJsonFacetResponseWrapper {
    pub fn get_buckets(&self) -> Vec<SolrJsonFacetResponseWrapper> {
        self.0
            .get_buckets()
            .map(|bucket| SolrJsonFacetResponseWrapper::from(bucket))
            .collect()
    }

    pub fn get_flat_facets(&self) -> PyResult<HashMap<String, PyObject>> {
        Python::with_gil(|py| -> PyResult<HashMap<String, PyObject>> {
            self.0
                .get_flat_facets()
                .iter()
                .map(|(k, v)| {
                    let py_obj = pythonize(py, v).map_err(PyErrWrapper::from)?;
                    Ok((k.to_string(), py_obj))
                })
                .collect()
        })
    }

    pub fn get_nested_facets(&self) -> HashMap<String, SolrJsonFacetResponseWrapper> {
        self.0
            .get_nested_facets()
            .iter()
            .map(|(k, v)| (k.to_string(), SolrJsonFacetResponseWrapper::from(v)))
            .collect()
    }

    pub fn get_count(&self) -> Option<usize> {
        self.0.get_count()
    }

    pub fn get_val(&self) -> PyResult<Option<PyObject>> {
        Python::with_gil(|py| -> PyResult<Option<PyObject>> {
            Ok(self
                .0
                .get_val()
                .map(|v| pythonize(py, v).map_err(PyErrWrapper::from))
                .transpose()
                .map_err(PyErrWrapper::from)?)
        })
    }
}

impl From<SolrJsonFacetResponseWrapper> for SolrJsonFacetResponse {
    fn from(wrapper: SolrJsonFacetResponseWrapper) -> Self {
        wrapper.0
    }
}

impl From<&SolrJsonFacetResponseWrapper> for SolrJsonFacetResponse {
    fn from(wrapper: &SolrJsonFacetResponseWrapper) -> Self {
        wrapper.0.clone()
    }
}

impl From<SolrJsonFacetResponse> for SolrJsonFacetResponseWrapper {
    fn from(response: SolrJsonFacetResponse) -> Self {
        SolrJsonFacetResponseWrapper(response)
    }
}

impl From<&SolrJsonFacetResponse> for SolrJsonFacetResponseWrapper {
    fn from(response: &SolrJsonFacetResponse) -> Self {
        SolrJsonFacetResponseWrapper(response.clone())
    }
}
