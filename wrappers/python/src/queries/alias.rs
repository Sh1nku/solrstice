use crate::models::context::SolrServerContextWrapper;
use crate::models::error::PyErrWrapper;
use pyo3::prelude::*;
use solrstice::queries::alias::{
    alias_exists as alias_exists_rs, create_alias as create_alias_rs,
    delete_alias as delete_alias_rs, get_aliases as get_aliases_rs,
};
use solrstice::queries::alias::{
    alias_exists_blocking as alias_exists_blocking_rs,
    create_alias_blocking as create_alias_blocking_rs,
    delete_alias_blocking as delete_alias_blocking_rs,
    get_aliases_blocking as get_aliases_blocking_rs,
};
use solrstice::SolrServerContext;
use std::collections::HashMap;

#[pymodule]
pub fn alias(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_aliases, m)?)?;
    m.add_function(wrap_pyfunction!(create_alias, m)?)?;
    m.add_function(wrap_pyfunction!(alias_exists, m)?)?;
    m.add_function(wrap_pyfunction!(delete_alias, m)?)?;

    m.add_function(wrap_pyfunction!(get_aliases_blocking, m)?)?;
    m.add_function(wrap_pyfunction!(create_alias_blocking, m)?)?;
    m.add_function(wrap_pyfunction!(alias_exists_blocking, m)?)?;
    m.add_function(wrap_pyfunction!(delete_alias_blocking, m)?)?;
    Ok(())
}

#[pyfunction]
pub fn get_aliases(py: Python, context: SolrServerContextWrapper) -> PyResult<Bound<PyAny>> {
    let context: SolrServerContext = context.into();
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let result = get_aliases_rs(context).await.map_err(PyErrWrapper::from)?;
        Ok(Python::with_gil(|_| result))
    })
}

#[pyfunction]
pub fn get_aliases_blocking(
    py: Python,
    context: SolrServerContextWrapper,
) -> PyResult<HashMap<String, Vec<String>>> {
    py.allow_threads(move || {
        let context: SolrServerContext = context.into();
        let result = get_aliases_blocking_rs(context).map_err(PyErrWrapper::from)?;
        Ok(result)
    })
}

#[pyfunction]
pub fn create_alias(
    py: Python,
    context: SolrServerContextWrapper,
    name: String,
    collections: Vec<String>,
) -> PyResult<Bound<PyAny>> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let context: SolrServerContext = context.into();
        Ok(create_alias_rs(
            &context,
            name.as_str(),
            collections
                .iter()
                .map(|x| x.as_str())
                .collect::<Vec<_>>()
                .as_slice(),
        )
        .await
        .map_err(PyErrWrapper::from)?)
    })
}

#[pyfunction]
pub fn create_alias_blocking(
    py: Python,
    context: SolrServerContextWrapper,
    name: String,
    collections: Vec<String>,
) -> PyResult<()> {
    py.allow_threads(move || {
        let context: SolrServerContext = context.into();
        Ok(create_alias_blocking_rs(
            &context,
            name.as_str(),
            collections
                .iter()
                .map(|x| x.as_str())
                .collect::<Vec<_>>()
                .as_slice(),
        )
        .map_err(PyErrWrapper::from)?)
    })
}

#[pyfunction]
pub fn alias_exists(
    py: Python,
    context: SolrServerContextWrapper,
    name: String,
) -> PyResult<Bound<PyAny>> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let context: SolrServerContext = context.into();
        let result = alias_exists_rs(&context, name.as_str())
            .await
            .map_err(PyErrWrapper::from)?;
        Ok(Python::with_gil(|_| result))
    })
}

#[pyfunction]
pub fn alias_exists_blocking(
    py: Python,
    context: SolrServerContextWrapper,
    name: String,
) -> PyResult<bool> {
    py.allow_threads(move || {
        let context: SolrServerContext = context.into();
        let result =
            alias_exists_blocking_rs(&context, name.as_str()).map_err(PyErrWrapper::from)?;
        Ok(result)
    })
}

#[pyfunction]
pub fn delete_alias(
    py: Python,
    context: SolrServerContextWrapper,
    name: String,
) -> PyResult<Bound<PyAny>> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let context: SolrServerContext = context.into();
        delete_alias_rs(&context, name.as_str())
            .await
            .map_err(PyErrWrapper::from)?;
        Ok(())
    })
}

#[pyfunction]
pub fn delete_alias_blocking(
    py: Python,
    context: SolrServerContextWrapper,
    name: String,
) -> PyResult<()> {
    py.allow_threads(move || {
        let context: SolrServerContext = context.into();
        delete_alias_blocking_rs(&context, name.as_str()).map_err(PyErrWrapper::from)?;
        Ok(())
    })
}
