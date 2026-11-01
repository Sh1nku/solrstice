use crate::models::context::SolrServerContextWrapper;
use crate::models::error::PyErrWrapper;
use crate::models::response::SolrResponseWrapper;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pythonize::depythonize;
use serde::{Deserialize, Serialize};
use solrstice::CommitType;
use solrstice::Error;
use solrstice::SolrServerContext;
use solrstice::{DeleteQuery, UpdateQuery};

#[pyclass(name = "CommitType", eq, eq_int)]
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum CommitTypeWrapper {
    Hard,
    Soft,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[pyclass(name = "UpdateQuery", module = "solrstice", subclass)]
pub struct UpdateQueryWrapper(UpdateQuery);

#[pymethods]
impl UpdateQueryWrapper {
    #[new]
    #[pyo3(signature = (handler=None, commit_type=None))]
    pub fn new(handler: Option<String>, commit_type: Option<CommitTypeWrapper>) -> Self {
        let mut builder = UpdateQuery::new();
        if let Some(handler) = handler {
            builder = builder.handler(handler);
        }
        if let Some(commit_type) = commit_type {
            builder = builder.commit_type(commit_type.into());
        }
        Self(builder)
    }

    pub fn execute<'py>(
        &self,
        py: Python<'py>,
        context: SolrServerContextWrapper,
        collection: String,
        data: Vec<PyObject>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let builder = self.0.clone();
        let data: Result<Vec<serde_json::Value>, PyErrWrapper> = data
            .into_iter()
            .map(|x| depythonize(&x.into_bound(py)).map_err(PyErrWrapper::from))
            .collect();
        let data = data?;
        pyo3_async_runtimes::tokio::future_into_py::<_, SolrResponseWrapper>(py, async move {
            let context: SolrServerContext = context.into();
            let result = builder
                .execute(&context, collection.as_str(), data.as_slice())
                .await
                .map_err(PyErrWrapper::from)?;
            Ok(Python::with_gil(|_| result.into()))
        })
    }

    pub fn execute_blocking<'py>(
        &self,
        py: Python<'py>,
        context: SolrServerContextWrapper,
        collection: String,
        data: Vec<PyObject>,
    ) -> PyResult<SolrResponseWrapper> {
        let data: Result<Vec<serde_json::Value>, PyErrWrapper> = data
            .into_iter()
            .map(|x| depythonize(&x.into_bound(py)).map_err(PyErrWrapper::from))
            .collect();
        let data = data?;
        let builder = self.0.clone();
        py.allow_threads(move || {
            let context: SolrServerContext = context.into();
            let result = builder
                .execute_blocking(&context, collection.as_str(), data.as_slice())
                .map_err(PyErrWrapper::from)?;
            Ok(result.into())
        })
    }

    pub fn __setstate__(&mut self, py: Python, state: PyObject) -> PyResult<()> {
        match state.bind(py).extract::<&[u8]>() {
            Ok(s) => {
                *self = serde_json::from_slice(s)
                    .map_err(Error::from)
                    .map_err(PyErrWrapper::from)?;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn __getstate__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        Ok(PyBytes::new(
            py,
            serde_json::to_string(&self)
                .map_err(Error::from)
                .map_err(PyErrWrapper::from)?
                .as_bytes(),
        ))
    }
}

impl From<CommitTypeWrapper> for CommitType {
    fn from(value: CommitTypeWrapper) -> Self {
        match value {
            CommitTypeWrapper::Hard => CommitType::Hard,
            CommitTypeWrapper::Soft => CommitType::Soft,
        }
    }
}

impl From<CommitType> for CommitTypeWrapper {
    fn from(value: CommitType) -> Self {
        match value {
            CommitType::Hard => CommitTypeWrapper::Hard,
            CommitType::Soft => CommitTypeWrapper::Soft,
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[pyclass(name = "DeleteQuery", module = "solrstice", subclass)]
pub struct DeleteQueryWrapper(DeleteQuery);

#[pymethods]
impl DeleteQueryWrapper {
    #[new]
    #[pyo3(signature = (handler=None, commit_type=None, ids=None, queries=None))]
    pub fn new(
        handler: Option<String>,
        commit_type: Option<CommitTypeWrapper>,
        ids: Option<Vec<String>>,
        queries: Option<Vec<String>>,
    ) -> Self {
        let mut builder = DeleteQuery::new();
        if let Some(handler) = handler {
            builder = builder.handler(handler);
        }
        if let Some(commit_type) = commit_type {
            let commit_type: CommitType = commit_type.into();
            builder = builder.commit_type(commit_type);
        }
        if let Some(ids) = ids {
            builder = builder.ids(&ids);
        }
        if let Some(queries) = queries {
            builder = builder.queries(&queries);
        }
        Self(builder)
    }

    pub fn execute<'py>(
        &self,
        py: Python<'py>,
        context: SolrServerContextWrapper,
        collection: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let builder = self.0.clone();
        pyo3_async_runtimes::tokio::future_into_py::<_, SolrResponseWrapper>(py, async move {
            let context: SolrServerContext = context.into();
            let result = builder
                .execute(&context, collection.as_str())
                .await
                .map_err(PyErrWrapper::from)?;
            Ok(Python::with_gil(|_| result.into()))
        })
    }

    pub fn execute_blocking(
        &self,
        py: Python,
        context: SolrServerContextWrapper,
        collection: String,
    ) -> PyResult<SolrResponseWrapper> {
        let builder = self.0.clone();
        py.allow_threads(move || {
            let context: SolrServerContext = context.into();
            let result = builder
                .execute_blocking(&context, collection.as_str())
                .map_err(PyErrWrapper::from)?;
            Ok(result.into())
        })
    }

    pub fn __setstate__(&mut self, py: Python, state: PyObject) -> PyResult<()> {
        match state.bind(py).extract::<&[u8]>() {
            Ok(s) => {
                *self = serde_json::from_slice(s)
                    .map_err(Error::from)
                    .map_err(PyErrWrapper::from)?;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn __getstate__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        Ok(PyBytes::new(
            py,
            serde_json::to_string(&self)
                .map_err(Error::from)
                .map_err(PyErrWrapper::from)?
                .as_bytes(),
        ))
    }
}
