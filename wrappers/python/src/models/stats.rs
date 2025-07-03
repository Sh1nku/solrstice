use crate::models::error::PyErrWrapper;
use pyo3::{pyclass, pymethods, PyObject, PyResult, Python};
use pythonize::pythonize;
use solrstice::models::{SolrStatsFieldResult, SolrStatsResult};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
#[pyclass(name = "SolrStatsResult", module = "solrstice.models", subclass)]
pub struct SolrStatsResultWrapper(SolrStatsResult);

#[pymethods]
impl SolrStatsResultWrapper {
    pub fn get_fields(&self) -> HashMap<String, SolrStatsFieldResultWrapper> {
        self.0
            .get_fields()
            .iter()
            .map(|(k, v)| (k.clone(), v.into()))
            .collect()
    }
}

impl From<SolrStatsResultWrapper> for SolrStatsResult {
    fn from(wrapper: SolrStatsResultWrapper) -> Self {
        wrapper.0
    }
}

impl From<&SolrStatsResultWrapper> for SolrStatsResult {
    fn from(wrapper: &SolrStatsResultWrapper) -> Self {
        wrapper.0.clone()
    }
}

impl From<SolrStatsResult> for SolrStatsResultWrapper {
    fn from(stats: SolrStatsResult) -> Self {
        SolrStatsResultWrapper(stats)
    }
}

impl From<&SolrStatsResult> for SolrStatsResultWrapper {
    fn from(stats: &SolrStatsResult) -> Self {
        SolrStatsResultWrapper(stats.clone())
    }
}

#[derive(Clone, Debug, PartialEq)]
#[pyclass(name = "SolrStatsFieldResult", module = "solrstice.models", subclass)]
pub struct SolrStatsFieldResultWrapper(SolrStatsFieldResult);

#[pymethods]
impl SolrStatsFieldResultWrapper {
    pub fn get_min(&self) -> PyResult<PyObject> {
        Python::with_gil(|py| -> PyResult<PyObject> {
            let value = self
                .0
                .get_min::<serde_json::Value>()
                .map_err(PyErrWrapper::from)?;
            Ok(pythonize(py, &value)?)
        })
    }

    pub fn get_max(&self) -> PyResult<PyObject> {
        Python::with_gil(|py| -> PyResult<PyObject> {
            let value = self
                .0
                .get_max::<serde_json::Value>()
                .map_err(PyErrWrapper::from)?;
            Ok(pythonize(py, &value)?)
        })
    }

    pub fn get_count(&self) -> u64 {
        self.0.get_count()
    }

    pub fn get_missing(&self) -> u64 {
        self.0.get_missing()
    }

    pub fn get_sum(&self) -> Option<f64> {
        self.0.get_sum()
    }

    pub fn get_mean(&self) -> PyResult<Option<PyObject>> {
        Python::with_gil(|py| -> PyResult<Option<PyObject>> {
            match self.0.get_mean::<serde_json::Value>() {
                Some(result) => match result {
                    Ok(value) => Ok(Some(pythonize(py, &value)?)),
                    Err(e) => Err(PyErrWrapper::from(e).into()),
                },
                None => Ok(None),
            }
        })
    }

    pub fn get_sum_of_squares(&self) -> Option<f64> {
        self.0.get_sum_of_squares()
    }

    pub fn get_stddev(&self) -> Option<f64> {
        self.0.get_stddev()
    }
}

impl From<SolrStatsFieldResultWrapper> for SolrStatsFieldResult {
    fn from(wrapper: SolrStatsFieldResultWrapper) -> Self {
        wrapper.0
    }
}

impl From<&SolrStatsFieldResultWrapper> for SolrStatsFieldResult {
    fn from(wrapper: &SolrStatsFieldResultWrapper) -> Self {
        wrapper.0.clone()
    }
}

impl From<SolrStatsFieldResult> for SolrStatsFieldResultWrapper {
    fn from(field: SolrStatsFieldResult) -> Self {
        SolrStatsFieldResultWrapper(field)
    }
}

impl From<&SolrStatsFieldResult> for SolrStatsFieldResultWrapper {
    fn from(field: &SolrStatsFieldResult) -> Self {
        SolrStatsFieldResultWrapper(field.clone())
    }
}
