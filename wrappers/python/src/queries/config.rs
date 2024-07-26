use crate::models::context::SolrServerContextWrapper;
use crate::models::error::PyErrWrapper;
use pyo3::prelude::*;
use solrstice::queries::config::{
    config_exists as config_exists_rs, delete_config as delete_config_rs,
    get_configs as get_configs_rs, upload_config as upload_config_rs,
};
use solrstice::queries::config::{
    config_exists_blocking as config_exists_blocking_rs,
    delete_config_blocking as delete_config_blocking_rs,
    get_configs_blocking as get_configs_blocking_rs,
    upload_config_blocking as upload_config_blocking_rs,
};
use solrstice::SolrServerContext;
use std::path::PathBuf;

#[pymodule]
pub fn config(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(upload_config, m)?)?;
    m.add_function(wrap_pyfunction!(get_configs, m)?)?;
    m.add_function(wrap_pyfunction!(config_exists, m)?)?;
    m.add_function(wrap_pyfunction!(delete_config, m)?)?;

    m.add_function(wrap_pyfunction!(upload_config_blocking, m)?)?;
    m.add_function(wrap_pyfunction!(get_configs_blocking, m)?)?;
    m.add_function(wrap_pyfunction!(config_exists_blocking, m)?)?;
    m.add_function(wrap_pyfunction!(delete_config_blocking, m)?)?;
    Ok(())
}

#[pyfunction]
pub fn upload_config(
    py: Python,
    context: SolrServerContextWrapper,
    name: String,
    path: PathBuf,
) -> PyResult<Bound<PyAny>> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let context: SolrServerContext = context.into();
        upload_config_rs(&context, name.as_str(), path.as_path())
            .await
            .map_err(PyErrWrapper::from)?;
        Ok(())
    })
}

#[pyfunction]
pub fn upload_config_blocking(
    py: Python,
    context: SolrServerContextWrapper,
    name: String,
    path: PathBuf,
) -> PyResult<()> {
    py.allow_threads(move || {
        let context: SolrServerContext = context.into();
        upload_config_blocking_rs(&context, name.as_str(), path.as_path())
            .map_err(PyErrWrapper::from)?;
        Ok(())
    })
}

#[pyfunction]
pub fn get_configs(py: Python, context: SolrServerContextWrapper) -> PyResult<Bound<PyAny>> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let context: SolrServerContext = context.into();
        let result = get_configs_rs(&context).await.map_err(PyErrWrapper::from)?;
        Ok(Python::with_gil(|_| result))
    })
}

#[pyfunction]
pub fn get_configs_blocking(
    py: Python,
    context: SolrServerContextWrapper,
) -> PyResult<Vec<String>> {
    py.allow_threads(move || {
        let context: SolrServerContext = context.into();
        let result = get_configs_blocking_rs(&context).map_err(PyErrWrapper::from)?;
        Ok(result)
    })
}

#[pyfunction]
pub fn config_exists(
    py: Python,
    context: SolrServerContextWrapper,
    name: String,
) -> PyResult<Bound<PyAny>> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let context: SolrServerContext = context.into();
        let result = config_exists_rs(&context, name.as_str())
            .await
            .map_err(PyErrWrapper::from)?;
        Ok(Python::with_gil(|_| result))
    })
}

#[pyfunction]
pub fn config_exists_blocking(
    py: Python,
    context: SolrServerContextWrapper,
    name: String,
) -> PyResult<bool> {
    py.allow_threads(move || {
        let context: SolrServerContext = context.into();
        let result =
            config_exists_blocking_rs(&context, name.as_str()).map_err(PyErrWrapper::from)?;
        Ok(result)
    })
}

#[pyfunction]
pub fn delete_config(
    py: Python,
    context: SolrServerContextWrapper,
    name: String,
) -> PyResult<Bound<PyAny>> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let context: SolrServerContext = context.into();
        delete_config_rs(&context, name.as_str())
            .await
            .map_err(PyErrWrapper::from)?;
        Ok(())
    })
}

#[pyfunction]
pub fn delete_config_blocking(
    py: Python,
    context: SolrServerContextWrapper,
    name: String,
) -> PyResult<()> {
    py.allow_threads(move || {
        let context: SolrServerContext = context.into();
        delete_config_blocking_rs(&context, name.as_str()).map_err(PyErrWrapper::from)?;
        Ok(())
    })
}
