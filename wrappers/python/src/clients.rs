use crate::models::context::SolrServerContextWrapper;
use crate::models::response::SolrResponseWrapper;
use crate::queries::alias::{
    alias_exists, alias_exists_blocking, create_alias, create_alias_blocking,
    delete_alias_blocking, get_aliases, get_aliases_blocking,
};
use crate::queries::collection::{
    collection_exists, collection_exists_blocking, create_collection, create_collection_blocking,
    delete_collection, delete_collection_blocking, get_collections, get_collections_blocking,
};
use crate::queries::config::{
    config_exists, config_exists_blocking, delete_config, delete_config_blocking, get_configs,
    get_configs_blocking, upload_config, upload_config_blocking,
};
use crate::queries::index::{DeleteQueryBuilderWrapper, UpdateQueryBuilderWrapper};
use crate::queries::select::SelectQueryBuilderWrapper;
use pyo3::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;

#[pymodule]
pub fn clients(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<AsyncSolrCloudClientWrapper>()?;
    m.add_class::<BlockingSolrCloudClientWrapper>()?;
    Ok(())
}

#[pyclass(name = "AsyncSolrCloudClient", module = "solrstice.clients")]
#[derive(Clone)]
pub struct AsyncSolrCloudClientWrapper(SolrServerContextWrapper);

#[pymethods]
impl AsyncSolrCloudClientWrapper {
    #[new]
    fn new(context: SolrServerContextWrapper) -> Self {
        AsyncSolrCloudClientWrapper(context)
    }

    pub fn upload_config<'a>(
        &self,
        py: Python<'a>,
        name: String,
        path: PathBuf,
    ) -> PyResult<&'a PyAny> {
        let context = self.0.clone();
        upload_config(py, context, name, path)
    }

    pub fn get_configs<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let context = self.0.clone();
        get_configs(py, context)
    }

    pub fn config_exists<'a>(&self, py: Python<'a>, name: String) -> PyResult<&'a PyAny> {
        let context = self.0.clone();
        config_exists(py, context, name)
    }

    pub fn delete_config<'a>(&self, py: Python<'a>, name: String) -> PyResult<&'a PyAny> {
        let context = self.0.clone();
        delete_config(py, context, name)
    }

    pub fn create_collection<'a>(
        &self,
        py: Python<'a>,
        name: String,
        config: String,
        shards: Option<usize>,
        replication_factor: Option<usize>,
    ) -> PyResult<&'a PyAny> {
        let context = self.0.clone();
        create_collection(py, context, name, config, shards, replication_factor)
    }

    pub fn get_collections<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let context = self.0.clone();
        get_collections(py, context)
    }

    pub fn collection_exists<'a>(&self, py: Python<'a>, name: String) -> PyResult<&'a PyAny> {
        let context = self.0.clone();
        collection_exists(py, context, name)
    }

    pub fn delete_collection<'a>(&self, py: Python<'a>, name: String) -> PyResult<&'a PyAny> {
        let context = self.0.clone();
        delete_collection(py, context, name)
    }

    pub fn get_aliases<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        let context = self.0.clone();
        get_aliases(py, context)
    }

    pub fn create_alias<'a>(
        &self,
        py: Python<'a>,
        name: String,
        collections: Vec<String>,
    ) -> PyResult<&'a PyAny> {
        let context = self.0.clone();
        create_alias(py, context, name, collections)
    }

    pub fn alias_exists<'a>(&self, py: Python<'a>, name: String) -> PyResult<&'a PyAny> {
        let context = self.0.clone();
        alias_exists(py, context, name)
    }

    pub fn index<'a>(
        &self,
        py: Python<'a>,
        builder: UpdateQueryBuilderWrapper,
        collection: String,
        data: Vec<PyObject>,
    ) -> PyResult<&'a PyAny> {
        let context = self.0.clone();
        builder.execute(py, context, collection, data)
    }

    pub fn select<'a>(
        &self,
        py: Python<'a>,
        builder: &SelectQueryBuilderWrapper,
        collection: String,
    ) -> PyResult<&'a PyAny> {
        let context = self.0.clone();
        builder.execute(py, context, collection)
    }

    pub fn delete<'a>(
        &self,
        py: Python<'a>,
        builder: &DeleteQueryBuilderWrapper,
        collection: String,
    ) -> PyResult<&'a PyAny> {
        let context = self.0.clone();
        builder.execute(py, context, collection)
    }
}

#[pyclass(name = "BlockingSolrCloudClient", module = "solrstice.clients")]
#[derive(Clone)]
pub struct BlockingSolrCloudClientWrapper(SolrServerContextWrapper);

#[pymethods]
impl BlockingSolrCloudClientWrapper {
    #[new]
    fn new(context: SolrServerContextWrapper) -> Self {
        BlockingSolrCloudClientWrapper(context)
    }

    pub fn upload_config(&self, py: Python, name: String, path: PathBuf) -> PyResult<()> {
        let context = self.0.clone();
        upload_config_blocking(py, context, name, path)
    }

    pub fn get_configs(&self, py: Python) -> PyResult<Vec<String>> {
        let context = self.0.clone();
        get_configs_blocking(py, context)
    }

    pub fn config_exists(&self, py: Python, name: String) -> PyResult<bool> {
        let context = self.0.clone();
        config_exists_blocking(py, context, name)
    }

    pub fn delete_config(&self, py: Python, name: String) -> PyResult<()> {
        let context = self.0.clone();
        delete_config_blocking(py, context, name)
    }

    pub fn create_collection(
        &self,
        py: Python,
        name: String,
        config: String,
        shards: Option<usize>,
        replication_factor: Option<usize>,
    ) -> PyResult<()> {
        let context = self.0.clone();
        create_collection_blocking(py, context, name, config, shards, replication_factor)
    }

    pub fn get_collections(&self, py: Python) -> PyResult<Vec<String>> {
        let context = self.0.clone();
        get_collections_blocking(py, context)
    }

    pub fn collection_exists(&self, py: Python, name: String) -> PyResult<bool> {
        let context = self.0.clone();
        collection_exists_blocking(py, context, name)
    }

    pub fn delete_collection(&self, py: Python, name: String) -> PyResult<()> {
        let context = self.0.clone();
        delete_collection_blocking(py, context, name)
    }

    pub fn create_alias(&self, py: Python, name: String, collections: Vec<String>) -> PyResult<()> {
        let context = self.0.clone();
        create_alias_blocking(py, context, name, collections)
    }

    pub fn get_aliases(&self, py: Python) -> PyResult<HashMap<String, Vec<String>>> {
        let context = self.0.clone();
        get_aliases_blocking(py, context)
    }

    pub fn alias_exists(&self, py: Python, name: String) -> PyResult<bool> {
        let context = self.0.clone();
        alias_exists_blocking(py, context, name)
    }

    pub fn delete_alias(&self, py: Python, name: String) -> PyResult<()> {
        let context = self.0.clone();
        delete_alias_blocking(py, context, name)
    }

    pub fn index(
        &self,
        py: Python,
        builder: UpdateQueryBuilderWrapper,
        collection: String,
        data: Vec<PyObject>,
    ) -> PyResult<SolrResponseWrapper> {
        let context = self.0.clone();
        builder.execute_blocking(py, context, collection, data)
    }

    pub fn select(
        &self,
        py: Python,
        builder: &SelectQueryBuilderWrapper,
        collection: String,
    ) -> PyResult<SolrResponseWrapper> {
        let context = self.0.clone();
        builder.execute_blocking(py, context, collection)
    }

    pub fn delete(
        &self,
        py: Python,
        builder: &DeleteQueryBuilderWrapper,
        collection: String,
    ) -> PyResult<SolrResponseWrapper> {
        let context = self.0.clone();
        builder.execute_blocking(py, context, collection)
    }
}
