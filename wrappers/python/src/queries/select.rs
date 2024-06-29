use crate::models::context::SolrServerContextWrapper;
use crate::models::error::PyErrWrapper;
use crate::models::response::SolrResponseWrapper;
use crate::queries::components::facet_set::FacetSetComponentWrapper;
use crate::queries::components::grouping::GroupingComponentWrapper;
use crate::queries::components::json_facet::JsonFacetComponentWrapper;
use crate::queries::def_type::DefTypeWrapper;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use serde::{Deserialize, Serialize};
use solrstice::models::context::SolrServerContext;
use solrstice::models::error::SolrError;
use solrstice::queries::components::facet_set::FacetSetComponent;
use solrstice::queries::components::grouping::GroupingComponent;
use solrstice::queries::components::json_facet::JsonFacetComponent;
use solrstice::queries::def_type::DefType;
use solrstice::queries::select::SelectQuery;

#[pyclass(name = "SelectQuery", module = "solrstice.queries", subclass)]
#[derive(Clone, Serialize, Deserialize)]
pub struct SelectQueryWrapper(SelectQuery);

#[pymethods]
impl SelectQueryWrapper {
    #[new]
    fn new(
        q: Option<String>,
        fl: Option<Vec<String>>,
        fq: Option<Vec<String>>,
        rows: Option<usize>,
        start: Option<usize>,
        sort: Option<Vec<String>>,
        cursor_mark: Option<String>,
        grouping: Option<GroupingComponentWrapper>,
        def_type: Option<DefTypeWrapper>,
        facet_set: Option<FacetSetComponentWrapper>,
        json_facet: Option<JsonFacetComponentWrapper>,
    ) -> Self {
        let mut builder = SelectQuery::new();
        if let Some(q) = q {
            builder = builder.q(q);
        }
        builder = builder.fl::<String, Vec<String>, Option<Vec<String>>>(fl);
        builder = builder.fq::<String, Vec<String>, Option<Vec<String>>>(fq);
        if let Some(rows) = rows {
            builder = builder.rows(rows);
        }
        if let Some(start) = start {
            builder = builder.start(start);
        }
        if let Some(sort) = sort {
            builder = builder.sort(sort);
        }
        if let Some(cursor_mark) = cursor_mark {
            builder = builder.cursor_mark(cursor_mark);
        }
        builder = builder
            .grouping::<GroupingComponent, Option<GroupingComponent>>(grouping.map(|x| x.into()));
        builder = builder.def_type::<DefType, Option<DefType>>(def_type.map(|x| x.into()));
        builder = builder
            .facet_set::<FacetSetComponent, Option<FacetSetComponent>>(facet_set.map(|x| x.into()));
        builder = builder.json_facet::<JsonFacetComponent, Option<JsonFacetComponent>>(
            json_facet.map(|x| x.into()),
        );
        Self(builder)
    }

    pub fn execute<'py>(
        &self,
        py: Python<'py>,
        context: SolrServerContextWrapper,
        collection: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let builder = self.0.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let context: SolrServerContext = context.into();
            let result: SolrResponseWrapper = builder
                .execute(&context, &collection)
                .await
                .map_err(PyErrWrapper::from)?
                .into();
            Ok(Python::with_gil(|_| result))
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
            let result: SolrResponseWrapper = builder
                .execute_blocking(&context, &collection)
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
        Ok(PyBytes::new_bound(
            py,
            serde_json::to_string(&self)
                .map_err(SolrError::from)
                .map_err(PyErrWrapper::from)?
                .as_bytes(),
        )
        .to_object(py))
    }
}
