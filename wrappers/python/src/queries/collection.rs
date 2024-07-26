use crate::models::context::SolrServerContextWrapper;
use crate::models::error::PyErrWrapper;
use pyo3::prelude::*;
use solrstice::queries::collection::{
    collection_exists as collection_exists_rs, create_collection as create_collection_rs,
    delete_collection as delete_collection_rs, get_collections as get_collections_rs,
};
use solrstice::queries::collection::{
    collection_exists_blocking as collection_exists_blocking_rs,
    create_collection_blocking as create_collection_blocking_rs,
    delete_collection_blocking as delete_collection_blocking_rs,
    get_collections_blocking as get_collections_blocking_rs,
};
use solrstice::SolrServerContext;

#[pymodule]
pub fn collection(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_collection, m)?)?;
    m.add_function(wrap_pyfunction!(get_collections, m)?)?;
    m.add_function(wrap_pyfunction!(collection_exists, m)?)?;
    m.add_function(wrap_pyfunction!(delete_collection, m)?)?;

    m.add_function(wrap_pyfunction!(create_collection_blocking, m)?)?;
    m.add_function(wrap_pyfunction!(get_collections_blocking, m)?)?;
    m.add_function(wrap_pyfunction!(collection_exists_blocking, m)?)?;
    m.add_function(wrap_pyfunction!(delete_collection_blocking, m)?)?;
    Ok(())
}

#[pyfunction]
pub fn create_collection(
    py: Python,
    context: SolrServerContextWrapper,
    name: String,
    config: String,
    shards: Option<usize>,
    replication_factor: Option<usize>,
) -> PyResult<Bound<PyAny>> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let context: SolrServerContext = context.into();
        create_collection_rs(
            &context,
            name.as_str(),
            config.as_str(),
            shards.unwrap_or(1),
            replication_factor.unwrap_or(1),
        )
        .await
        .map_err(PyErrWrapper::from)?;
        Ok(())
    })
}

#[pyfunction]
pub fn create_collection_blocking(
    py: Python,
    context: SolrServerContextWrapper,
    name: String,
    config: String,
    shards: Option<usize>,
    replication_factor: Option<usize>,
) -> PyResult<()> {
    py.allow_threads(move || {
        let context: SolrServerContext = context.into();
        create_collection_blocking_rs(
            &context,
            name.as_str(),
            config.as_str(),
            shards.unwrap_or(1),
            replication_factor.unwrap_or(1),
        )
        .map_err(PyErrWrapper::from)?;
        Ok(())
    })
}

#[pyfunction]
pub fn get_collections(py: Python, context: SolrServerContextWrapper) -> PyResult<Bound<PyAny>> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let context: SolrServerContext = context.into();
        let result = get_collections_rs(&context)
            .await
            .map_err(PyErrWrapper::from)?;
        Ok(Python::with_gil(|_| result))
    })
}

#[pyfunction]
pub fn get_collections_blocking(
    py: Python,
    context: SolrServerContextWrapper,
) -> PyResult<Vec<String>> {
    py.allow_threads(move || {
        let context: SolrServerContext = context.into();
        let result = get_collections_blocking_rs(&context).map_err(PyErrWrapper::from)?;
        Ok(result)
    })
}

#[pyfunction]
pub fn collection_exists(
    py: Python,
    context: SolrServerContextWrapper,
    name: String,
) -> PyResult<Bound<PyAny>> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let context: SolrServerContext = context.into();
        let result = collection_exists_rs(&context, name.as_str())
            .await
            .map_err(PyErrWrapper::from)?;
        Ok(Python::with_gil(|_| result))
    })
}

#[pyfunction]
pub fn collection_exists_blocking(
    py: Python,
    context: SolrServerContextWrapper,
    name: String,
) -> PyResult<bool> {
    py.allow_threads(move || {
        let context: SolrServerContext = context.into();
        let result =
            collection_exists_blocking_rs(&context, name.as_str()).map_err(PyErrWrapper::from)?;
        Ok(result)
    })
}

#[pyfunction]
pub fn delete_collection(
    py: Python,
    context: SolrServerContextWrapper,
    name: String,
) -> PyResult<Bound<PyAny>> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let context: SolrServerContext = context.into();
        delete_collection_rs(&context, name.as_str())
            .await
            .map_err(PyErrWrapper::from)?;
        Ok(())
    })
}

#[pyfunction]
pub fn delete_collection_blocking(
    py: Python,
    context: SolrServerContextWrapper,
    name: String,
) -> PyResult<()> {
    py.allow_threads(move || {
        let context: SolrServerContext = context.into();
        delete_collection_blocking_rs(&context, name.as_str()).map_err(PyErrWrapper::from)?;
        Ok(())
    })
}
