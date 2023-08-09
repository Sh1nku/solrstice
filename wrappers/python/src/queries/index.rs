use crate::models::context::SolrServerContextWrapper;
use crate::models::error::PyErrWrapper;
use crate::models::response::SolrResponseWrapper;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pythonize::depythonize;
use serde::{Deserialize, Serialize};
use solrstice::models::commit_type::CommitType;
use solrstice::models::error::SolrError;
use solrstice::queries::index::{DeleteQueryBuilder, UpdateQueryBuilder};

#[pyclass(name = "CommitType")]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum CommitTypeWrapper {
    Hard,
    Soft,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[pyclass(name = "UpdateQueryBuilder", module = "solrstice.queries")]
pub struct UpdateQueryBuilderWrapper(UpdateQueryBuilder);

#[pymethods]
impl UpdateQueryBuilderWrapper {
    #[new]
    pub fn new(handler: Option<String>, commit_type: Option<CommitTypeWrapper>) -> Self {
        let mut s = Self(UpdateQueryBuilder::new());
        if let Some(handler) = handler {
            s.set_handler(handler);
        }
        if let Some(commit_type) = commit_type {
            s.set_commit_type(commit_type);
        }
        s
    }

    #[getter]
    pub fn get_handler(&self) -> &str {
        self.0.handler.as_str()
    }

    #[setter]
    pub fn set_handler(&mut self, handler: String) {
        self.0.handler = handler;
    }

    #[getter]
    pub fn get_commit_type(&self) -> CommitTypeWrapper {
        self.0.commit_type.into()
    }

    #[setter]
    pub fn set_commit_type(&mut self, commit_type: CommitTypeWrapper) {
        self.0.commit_type = commit_type.into();
    }

    pub fn execute<'a>(
        &self,
        py: Python<'a>,
        context: SolrServerContextWrapper,
        collection: String,
        data: Vec<PyObject>,
    ) -> PyResult<&'a PyAny> {
        let builder = self.0.clone();
        let data: Result<Vec<serde_json::Value>, PyErrWrapper> = data
            .into_iter()
            .map(|x| {
                let as_any = x.downcast::<PyAny>(py).map_err(PyErrWrapper::from)?;
                depythonize::<serde_json::Value>(as_any).map_err(PyErrWrapper::from)
            })
            .collect();
        let data = data?;
        pyo3_asyncio::tokio::future_into_py::<_, SolrResponseWrapper>(py, async move {
            let result = builder
                .execute(&context.into(), collection.as_str(), data.as_slice())
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
        data: Vec<PyObject>,
    ) -> PyResult<SolrResponseWrapper> {
        let data: Result<Vec<serde_json::Value>, PyErrWrapper> = data
            .into_iter()
            .map(|x| {
                let as_any = x.downcast::<PyAny>(py).map_err(PyErrWrapper::from)?;
                depythonize::<serde_json::Value>(as_any).map_err(PyErrWrapper::from)
            })
            .collect();
        let data = data?;
        let builder = self.0.clone();
        py.allow_threads(move || {
            let result = builder
                .execute_blocking(&context.into(), collection.as_str(), data.as_slice())
                .map_err(PyErrWrapper::from)?;
            Ok(result.into())
        })
    }

    pub fn __setstate__(&mut self, py: Python, state: PyObject) -> PyResult<()> {
        match state.extract::<&PyBytes>(py) {
            Ok(s) => {
                *self = serde_json::from_slice(s.as_bytes())
                    .map_err(SolrError::from)
                    .map_err(PyErrWrapper::from)?;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn __getstate__(&self, py: Python) -> PyResult<PyObject> {
        Ok(PyBytes::new(
            py,
            serde_json::to_string(&self)
                .map_err(SolrError::from)
                .map_err(PyErrWrapper::from)?
                .as_bytes(),
        )
        .to_object(py))
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
#[pyclass(name = "DeleteQueryBuilder", module = "solrstice.queries")]
pub struct DeleteQueryBuilderWrapper(DeleteQueryBuilder);

#[pymethods]
impl DeleteQueryBuilderWrapper {
    #[new]
    pub fn new(
        handler: Option<String>,
        commit_type: Option<CommitTypeWrapper>,
        ids: Option<Vec<&str>>,
        queries: Option<Vec<&str>>,
    ) -> Self {
        let mut s = Self(DeleteQueryBuilder::new());
        if let Some(handler) = handler {
            s.set_handler(handler);
        }
        if let Some(commit_type) = commit_type {
            s.set_commit_type(commit_type);
        }
        if let Some(ids) = ids {
            s.set_ids(Some(ids));
        }
        if let Some(queries) = queries {
            s.set_queries(Some(queries));
        }
        s
    }

    #[getter]
    pub fn get_handler(&self) -> &str {
        self.0.handler.as_str()
    }

    #[setter]
    pub fn set_handler(&mut self, handler: String) {
        self.0.handler = handler;
    }

    #[getter]
    pub fn get_commit_type(&self) -> CommitTypeWrapper {
        self.0.commit_type.into()
    }

    #[setter]
    pub fn set_commit_type(&mut self, commit_type: CommitTypeWrapper) {
        self.0.commit_type = commit_type.into();
    }

    #[getter]
    pub fn get_ids(&self) -> Option<Vec<&str>> {
        self.0
            .ids
            .as_ref()
            .map(|f| f.iter().map(|x| x.as_str()).collect())
    }

    #[setter]
    pub fn set_ids(&mut self, ids: Option<Vec<&str>>) {
        self.0.ids = ids.map(|f| f.into_iter().map(|x| x.to_string()).collect())
    }

    #[getter]
    pub fn get_queries(&self) -> Option<Vec<&str>> {
        self.0
            .queries
            .as_ref()
            .map(|f| f.iter().map(|x| x.as_str()).collect())
    }

    #[setter]
    pub fn set_queries(&mut self, queries: Option<Vec<&str>>) {
        self.0.queries = queries.map(|f| f.into_iter().map(|x| x.to_string()).collect())
    }

    pub fn execute<'a>(
        &self,
        py: Python<'a>,
        context: SolrServerContextWrapper,
        collection: String,
    ) -> PyResult<&'a PyAny> {
        let builder = self.0.clone();
        pyo3_asyncio::tokio::future_into_py::<_, SolrResponseWrapper>(py, async move {
            let result = builder
                .execute(&context.into(), collection.as_str())
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
            let result = builder
                .execute_blocking(&context.into(), collection.as_str())
                .map_err(PyErrWrapper::from)?;
            Ok(result.into())
        })
    }

    pub fn __setstate__(&mut self, py: Python, state: PyObject) -> PyResult<()> {
        match state.extract::<&PyBytes>(py) {
            Ok(s) => {
                *self = serde_json::from_slice(s.as_bytes())
                    .map_err(SolrError::from)
                    .map_err(PyErrWrapper::from)?;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn __getstate__(&self, py: Python) -> PyResult<PyObject> {
        Ok(PyBytes::new(
            py,
            serde_json::to_string(&self)
                .map_err(SolrError::from)
                .map_err(PyErrWrapper::from)?
                .as_bytes(),
        )
        .to_object(py))
    }
}
