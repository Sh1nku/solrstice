use crate::models::context::SolrServerContextWrapper;
use crate::models::error::PyErrWrapper;
use crate::models::response::SolrResponseWrapper;
use crate::queries::components::facet_set::FacetSetComponentWrapper;
use crate::queries::components::grouping::GroupingComponentWrapper;
use crate::queries::components::json_facet::JsonFacetComponentWrapper;
use crate::queries::components::stats::StatsComponentWrapper;
use crate::queries::def_type::DefTypeWrapper;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pythonize::pythonize;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use solrstice::FacetSetComponent;
use solrstice::GroupingComponent;
use solrstice::JsonFacetComponent;
use solrstice::SelectQuery;
use solrstice::SolrServerContext;
use solrstice::{DefType, StatsComponent};
use solrstice::{Error, SelectDestination};
use std::collections::HashMap;

#[pyclass(name = "SelectQuery", module = "solrstice", subclass, from_py_object)]
#[derive(Clone, Serialize, Deserialize)]
pub struct SelectQueryWrapper(SelectQuery);

#[pymethods]
impl SelectQueryWrapper {
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (q=None, fl=None, fq=None, rows=None, start=None, sort=None, cursor_mark=None, grouping=None, def_type=None, facet_set=None, json_facet=None, stats=None, additional_params=None))]
    fn new(
        q: Option<String>,
        fl: Option<Vec<String>>,
        fq: Option<Vec<String>>,
        rows: Option<usize>,
        start: Option<usize>,
        sort: Option<String>,
        cursor_mark: Option<String>,
        grouping: Option<GroupingComponentWrapper>,
        def_type: Option<DefTypeWrapper>,
        facet_set: Option<FacetSetComponentWrapper>,
        json_facet: Option<JsonFacetComponentWrapper>,
        stats: Option<StatsComponentWrapper>,
        additional_params: Option<HashMap<String, Py<PyAny>>>,
    ) -> PyResult<Self> {
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
        builder = builder.stats::<StatsComponent, Option<StatsComponent>>(stats.map(|x| x.into()));
        builder = builder.json_facet::<JsonFacetComponent, Option<JsonFacetComponent>>(
            json_facet.map(|x| x.into()),
        );
        if let Some(additional_params) = additional_params {
            // Import the json module and convert the PyObject to a string, then to a serde_json::Value
            let converted_params = Python::attach(|py| -> PyResult<HashMap<String, Value>> {
                let mut converted_params = HashMap::new();
                let json = py.import("json")?;
                for (key, value) in additional_params {
                    let value = json
                        .call_method1("dumps", (value,))
                        .map_err(PyErrWrapper::from)?
                        .extract::<String>()
                        .map_err(PyErrWrapper::from)?;
                    let value: Value =
                        serde_json::from_str(value.as_str()).map_err(PyErrWrapper::from)?;
                    converted_params.insert(key, value);
                }
                Ok(converted_params)
            })?;
            builder = builder.additional_params(converted_params);
        }
        Ok(Self(builder))
    }

    #[pyo3(signature = (context, collection, handler=None))]
    pub fn execute<'py>(
        &self,
        py: Python<'py>,
        context: SolrServerContextWrapper,
        collection: String,
        handler: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let builder = self.0.clone();
        let mut destination = SelectDestination::new(collection);
        if let Some(handler) = handler {
            destination = destination.handler(handler);
        }
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let context: SolrServerContext = context.into();
            let result: SolrResponseWrapper = builder
                .execute(&context, &destination)
                .await
                .map_err(PyErrWrapper::from)?
                .into();
            Ok(Python::attach(|_| result))
        })
    }

    #[pyo3(signature = (context, collection, handler=None))]
    pub fn execute_raw<'py>(
        &self,
        py: Python<'py>,
        context: SolrServerContextWrapper,
        collection: String,
        handler: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let builder = self.0.clone();
        let mut destination = SelectDestination::new(collection);
        if let Some(handler) = handler {
            destination = destination.handler(handler);
        }

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let context: SolrServerContext = context.into();
            let result: HashMap<String, Value> = builder
                .execute_raw(&context, &destination)
                .await
                .map_err(PyErrWrapper::from)?;

            Python::attach(|py| -> PyResult<Py<PyAny>> {
                let obj = pythonize(py, &result).map_err(PyErrWrapper::from)?;
                Ok(obj.unbind())
            })
        })
    }

    #[pyo3(signature = (context, collection, handler=None))]
    pub fn execute_blocking(
        &self,
        py: Python,
        context: SolrServerContextWrapper,
        collection: String,
        handler: Option<String>,
    ) -> PyResult<SolrResponseWrapper> {
        let builder = self.0.clone();
        let mut destination = SelectDestination::new(collection);
        if let Some(handler) = handler {
            destination = destination.handler(handler);
        }
        py.detach(move || {
            let context: SolrServerContext = context.into();
            let result: SolrResponseWrapper = builder
                .execute_blocking(&context, &destination)
                .map_err(PyErrWrapper::from)?
                .into();
            Ok(result)
        })
    }

    #[pyo3(signature = (context, collection, handler=None))]
    pub fn execute_blocking_raw(
        &self,
        py: Python,
        context: SolrServerContextWrapper,
        collection: String,
        handler: Option<String>,
    ) -> PyResult<Py<PyAny>> {
        let builder = self.0.clone();
        let mut destination = SelectDestination::new(collection);
        if let Some(handler) = handler {
            destination = destination.handler(handler);
        }
        let value: Result<HashMap<String, Value>, PyErrWrapper> = py.detach(move || {
            let context: SolrServerContext = context.into();
            let result: HashMap<String, Value> = builder
                .execute_blocking_raw(&context, &destination)
                .map_err(PyErrWrapper::from)?;
            Ok(result)
        });
        let value = value?;
        Python::attach(|py| -> PyResult<Py<PyAny>> { Ok(pythonize(py, &value)?.into()) })
    }

    pub fn __setstate__(&mut self, py: Python, state: Py<PyAny>) -> PyResult<()> {
        match state.bind(py).extract::<&[u8]>() {
            Ok(s) => {
                *self = serde_json::from_slice(s)
                    .map_err(Error::from)
                    .map_err(PyErrWrapper::from)?;
                Ok(())
            }
            Err(e) => Err(e.into()),
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
