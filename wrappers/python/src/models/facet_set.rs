use crate::models::error::PyErrWrapper;
use pyo3::prelude::*;
use pythonize::pythonize;
use solrstice::models::{SolrFacetSetResult, SolrFieldFacetResult, SolrPivotFacetResult};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Default)]
#[pyclass(name = "SolrFacetSetResult", module = "solrstice.models", subclass)]
pub struct SolrFacetSetResultWrapper(SolrFacetSetResult);

#[pymethods]
impl SolrFacetSetResultWrapper {
    pub fn get_queries(&self) -> HashMap<String, usize> {
        self.0.get_queries().clone()
    }

    pub fn get_pivots(&self) -> HashMap<String, Vec<SolrPivotFacetResultWrapper>> {
        self.0
            .get_pivots()
            .iter()
            .map(|(k, v)| (k.clone(), v.iter().map(|x| x.into()).collect()))
            .collect()
    }

    pub fn get_fields(&self) -> HashMap<String, Vec<SolrFieldFacetResultWrapper>> {
        self.0
            .get_fields()
            .iter()
            .map(|(k, v)| (k.clone(), v.iter().map(|x| x.into()).collect()))
            .collect()
    }
}

impl From<SolrFacetSetResultWrapper> for SolrFacetSetResult {
    fn from(wrapper: SolrFacetSetResultWrapper) -> Self {
        wrapper.0
    }
}

impl From<&SolrFacetSetResultWrapper> for SolrFacetSetResult {
    fn from(wrapper: &SolrFacetSetResultWrapper) -> Self {
        wrapper.0.clone()
    }
}

impl From<SolrFacetSetResult> for SolrFacetSetResultWrapper {
    fn from(facet_set: SolrFacetSetResult) -> Self {
        SolrFacetSetResultWrapper(facet_set)
    }
}

impl From<&SolrFacetSetResult> for SolrFacetSetResultWrapper {
    fn from(facet_set: &SolrFacetSetResult) -> Self {
        SolrFacetSetResultWrapper(facet_set.clone())
    }
}

#[derive(Clone, Debug, PartialEq)]
#[pyclass(name = "SolrPivotFacetResult", module = "solrstice.models", subclass)]
pub struct SolrPivotFacetResultWrapper(SolrPivotFacetResult);

#[pymethods]
impl SolrPivotFacetResultWrapper {
    pub fn get_value(&self) -> PyResult<PyObject> {
        Python::with_gil(|py| -> PyResult<PyObject> {
            let value = self
                .0
                .get_value::<serde_json::Value>()
                .map_err(PyErrWrapper::from)?;
            Ok(pythonize(py, &value)?)
        })
    }

    pub fn get_pivots(&self) -> Vec<SolrPivotFacetResultWrapper> {
        self.0.get_pivots().iter().map(|x| x.into()).collect()
    }

    pub fn get_queries(&self) -> HashMap<String, usize> {
        self.0.get_queries().clone()
    }

    pub fn get_count(&self) -> usize {
        self.0.get_count()
    }
}

impl From<SolrPivotFacetResultWrapper> for SolrPivotFacetResult {
    fn from(wrapper: SolrPivotFacetResultWrapper) -> Self {
        wrapper.0
    }
}

impl From<&SolrPivotFacetResultWrapper> for SolrPivotFacetResult {
    fn from(wrapper: &SolrPivotFacetResultWrapper) -> Self {
        wrapper.0.clone()
    }
}

impl From<SolrPivotFacetResult> for SolrPivotFacetResultWrapper {
    fn from(pivot_facet_result: SolrPivotFacetResult) -> Self {
        SolrPivotFacetResultWrapper(pivot_facet_result)
    }
}

impl From<&SolrPivotFacetResult> for SolrPivotFacetResultWrapper {
    fn from(pivot_facet_result: &SolrPivotFacetResult) -> Self {
        SolrPivotFacetResultWrapper(pivot_facet_result.clone())
    }
}

#[derive(Clone, Debug, PartialEq)]
#[pyclass(name = "SolrFieldFacetResult", module = "solrstice.models", subclass)]
pub struct SolrFieldFacetResultWrapper(SolrFieldFacetResult);

#[pymethods]
impl SolrFieldFacetResultWrapper {
    pub fn get_key(&self) -> PyResult<PyObject> {
        Python::with_gil(|py| -> PyResult<PyObject> {
            let value = self
                .0
                .get_key::<serde_json::Value>()
                .map_err(PyErrWrapper::from)?;
            Ok(pythonize(py, &value)?)
        })
    }

    pub fn get_count(&self) -> usize {
        self.0.get_count()
    }
}

impl From<SolrFieldFacetResultWrapper> for SolrFieldFacetResult {
    fn from(wrapper: SolrFieldFacetResultWrapper) -> Self {
        wrapper.0
    }
}

impl From<&SolrFieldFacetResultWrapper> for SolrFieldFacetResult {
    fn from(wrapper: &SolrFieldFacetResultWrapper) -> Self {
        wrapper.0.clone()
    }
}

impl From<SolrFieldFacetResult> for SolrFieldFacetResultWrapper {
    fn from(field_facet_result: SolrFieldFacetResult) -> Self {
        SolrFieldFacetResultWrapper(field_facet_result)
    }
}

impl From<&SolrFieldFacetResult> for SolrFieldFacetResultWrapper {
    fn from(field_facet_result: &SolrFieldFacetResult) -> Self {
        SolrFieldFacetResultWrapper(field_facet_result.clone())
    }
}
