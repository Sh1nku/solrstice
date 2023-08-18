use crate::models::context::SolrServerContextWrapper;
use crate::models::error::PyErrWrapper;
use crate::models::response::SolrResponseWrapper;
use crate::queries::components::grouping::GroupingComponentWrapper;
use crate::queries::def_type::DefTypeWrapper;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use serde::{Deserialize, Serialize};
use solrstice::models::error::SolrError;
use solrstice::queries::components::grouping::GroupingComponent;
use solrstice::queries::select::SelectQuery;

#[pyclass(name = "SelectQuery", module = "solrstice.queries")]
#[derive(Clone, Serialize, Deserialize)]
pub struct SelectQueryWrapper(SelectQuery);

#[pymethods]
impl SelectQueryWrapper {
    #[new]
    fn new(
        q: Option<String>,
        fl: Option<Vec<&str>>,
        fq: Option<Vec<&str>>,
        rows: Option<usize>,
        start: Option<usize>,
        sort: Option<Vec<&str>>,
        cursor_mark: Option<String>,
        grouping: Option<GroupingComponentWrapper>,
        def_type: Option<DefTypeWrapper>,
    ) -> Self {
        let mut builder = SelectQuery::new();
        if let Some(q) = q {
            builder = builder.q(q);
        }
        if let Some(fl) = fl {
            builder = builder.fl(&fl);
        }
        if let Some(fq) = fq {
            builder = builder.fq(&fq);
        }
        if let Some(rows) = rows {
            builder = builder.rows(rows);
        }
        if let Some(start) = start {
            builder = builder.start(start);
        }
        if let Some(sort) = sort {
            builder = builder.sort(&sort);
        }
        if let Some(cursor_mark) = cursor_mark {
            builder = builder.cursor_mark(cursor_mark);
        }
        if let Some(grouping) = grouping {
            builder = builder.grouping::<GroupingComponent>(grouping.into());
        }
        if let Some(def_type) = def_type {
            builder = builder.def_type(def_type);
        }
        Self(builder)
    }

    pub fn execute<'a>(
        &self,
        py: Python<'a>,
        context: SolrServerContextWrapper,
        collection: String,
    ) -> PyResult<&'a PyAny> {
        let builder = self.0.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let result: SolrResponseWrapper = builder
                .execute(&context.into(), &collection)
                .await
                .map_err(PyErrWrapper::from)?
                .into();
            Ok(Python::with_gil(|_| result))
        })
    }

    pub fn execute_blocking(
        &self,
        py: Python,
        contect: SolrServerContextWrapper,
        collection: String,
    ) -> PyResult<SolrResponseWrapper> {
        let builder = self.0.clone();
        py.allow_threads(move || {
            let result: SolrResponseWrapper = builder
                .execute_blocking(&contect.into(), &collection)
                .map_err(PyErrWrapper::from)?
                .into();
            Ok(result)
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
