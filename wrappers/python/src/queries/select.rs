use crate::models::context::SolrServerContextWrapper;
use crate::models::error::PyErrWrapper;
use crate::models::response::SolrResponseWrapper;
use crate::queries::components::grouping::GroupingComponentWrapper;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use serde::{Deserialize, Serialize};
use solrstice::models::error::SolrError;
use solrstice::queries::select::SelectQueryBuilder;
use crate::queries::def_type::DefTypeWrapper;

#[pyclass(name = "SelectQueryBuilder", module = "solrstice.queries")]
#[derive(Clone, Serialize, Deserialize)]
pub struct SelectQueryBuilderWrapper(SelectQueryBuilder);

#[pymethods]
impl SelectQueryBuilderWrapper {
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
        let builder = SelectQueryBuilder::new();
        let mut s = Self(builder);
        if let Some(q) = q {
            s.set_q(q);
        }
        s.set_fl(fl);
        s.set_fq(fq);
        if let Some(rows) = rows {
            s.set_rows(rows);
        }
        if let Some(start) = start {
            s.set_start(start);
        }
        s.set_sort(sort);
        s.set_cursor_mark(cursor_mark);
        s.set_grouping(grouping);
        s.set_def_type(def_type);
        s
    }

    #[getter]
    fn get_q(&self) -> &str {
        &self.0.q
    }

    #[setter]
    fn set_q(&mut self, q: String) {
        self.0.q = q
    }

    #[getter]
    fn get_fl(&self) -> Option<Vec<String>> {
        self.0.fl.clone()
    }

    #[setter]
    fn set_fl(&mut self, fl: Option<Vec<&str>>) {
        self.0.fl = fl.map(|f| f.into_iter().map(|x| x.to_string()).collect())
    }

    #[getter]
    fn get_fq(&self) -> Option<Vec<String>> {
        self.0.fq.clone()
    }

    #[setter]
    fn set_fq(&mut self, fq: Option<Vec<&str>>) {
        self.0.fq = fq.map(|f| f.into_iter().map(|x| x.to_string()).collect())
    }

    #[getter]
    fn get_rows(&self) -> usize {
        self.0.rows
    }

    #[setter]
    fn set_rows(&mut self, rows: usize) {
        self.0.rows = rows
    }

    #[getter]
    fn get_start(&self) -> usize {
        self.0.start
    }

    #[setter]
    fn set_start(&mut self, start: usize) {
        self.0.start = start
    }

    #[getter]
    fn get_sort(&self) -> Option<Vec<String>> {
        self.0.sort.clone()
    }

    #[setter]
    fn set_sort(&mut self, sort: Option<Vec<&str>>) {
        self.0.sort = sort.map(|f| f.into_iter().map(|x| x.to_string()).collect())
    }

    #[getter]
    fn get_cursor_mark(&self) -> Option<String> {
        self.0.cursor_mark.clone()
    }

    #[setter]
    fn set_cursor_mark(&mut self, cursor_mark: Option<String>) {
        self.0.cursor_mark = cursor_mark
    }

    #[getter]
    fn get_grouping(&self) -> Option<GroupingComponentWrapper> {
        self.0.grouping.clone().map(|g| g.into())
    }

    #[setter]
    fn set_grouping(&mut self, grouping: Option<GroupingComponentWrapper>) {
        self.0.grouping = grouping.map(|g| g.into())
    }

    #[getter]
    fn get_def_type(&self) -> Option<DefTypeWrapper> {
        self.0.def_type.clone().map(|g| g.into())
    }

    #[setter]
    fn set_def_type(&mut self, def_type: Option<DefTypeWrapper>) {
        self.0.def_type = def_type.map(|g| g.into())
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
