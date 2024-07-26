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
use crate::queries::index::{DeleteQueryWrapper, UpdateQueryWrapper};
use crate::queries::select::SelectQueryWrapper;
use pyo3::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;

#[pyclass(name = "AsyncSolrCloudClient", module = "solrstice", subclass)]
#[derive(Clone)]
pub struct AsyncSolrCloudClientWrapper(SolrServerContextWrapper);

#[pymethods]
impl AsyncSolrCloudClientWrapper {
    #[new]
    fn new(context: SolrServerContextWrapper) -> Self {
        AsyncSolrCloudClientWrapper(context)
    }

    pub fn upload_config<'py>(
        &self,
        py: Python<'py>,
        name: String,
        path: PathBuf,
    ) -> PyResult<Bound<'py, PyAny>> {
        let context = self.0.clone();
        upload_config(py, context, name, path)
    }

    pub fn get_configs<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let context = self.0.clone();
        get_configs(py, context)
    }

    pub fn config_exists<'py>(&self, py: Python<'py>, name: String) -> PyResult<Bound<'py, PyAny>> {
        let context = self.0.clone();
        config_exists(py, context, name)
    }

    pub fn delete_config<'py>(&self, py: Python<'py>, name: String) -> PyResult<Bound<'py, PyAny>> {
        let context = self.0.clone();
        delete_config(py, context, name)
    }

    pub fn create_collection<'py>(
        &self,
        py: Python<'py>,
        name: String,
        config: String,
        shards: Option<usize>,
        replication_factor: Option<usize>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let context = self.0.clone();
        create_collection(py, context, name, config, shards, replication_factor)
    }

    pub fn get_collections<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let context = self.0.clone();
        get_collections(py, context)
    }

    pub fn collection_exists<'py>(
        &self,
        py: Python<'py>,
        name: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let context = self.0.clone();
        collection_exists(py, context, name)
    }

    pub fn delete_collection<'py>(
        &self,
        py: Python<'py>,
        name: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let context = self.0.clone();
        delete_collection(py, context, name)
    }

    pub fn get_aliases<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let context = self.0.clone();
        get_aliases(py, context)
    }

    pub fn create_alias<'py>(
        &self,
        py: Python<'py>,
        name: String,
        collections: Vec<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let context = self.0.clone();
        create_alias(py, context, name, collections)
    }

    pub fn alias_exists<'py>(&self, py: Python<'py>, name: String) -> PyResult<Bound<'py, PyAny>> {
        let context = self.0.clone();
        alias_exists(py, context, name)
    }

    pub fn index<'py>(
        &self,
        py: Python<'py>,
        builder: UpdateQueryWrapper,
        collection: String,
        data: Vec<PyObject>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let context = self.0.clone();
        builder.execute(py, context, collection, data)
    }

    pub fn select<'py>(
        &self,
        py: Python<'py>,
        builder: &SelectQueryWrapper,
        collection: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let context = self.0.clone();
        builder.execute(py, context, collection)
    }

    pub fn delete<'py>(
        &self,
        py: Python<'py>,
        builder: &DeleteQueryWrapper,
        collection: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let context = self.0.clone();
        builder.execute(py, context, collection)
    }
}

#[pyclass(name = "BlockingSolrCloudClient", module = "solrstice", subclass)]
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
        builder: UpdateQueryWrapper,
        collection: String,
        data: Vec<PyObject>,
    ) -> PyResult<SolrResponseWrapper> {
        let context = self.0.clone();
        builder.execute_blocking(py, context, collection, data)
    }

    pub fn select(
        &self,
        py: Python,
        builder: &SelectQueryWrapper,
        collection: String,
    ) -> PyResult<SolrResponseWrapper> {
        let context = self.0.clone();
        builder.execute_blocking(py, context, collection)
    }

    pub fn delete(
        &self,
        py: Python,
        builder: &DeleteQueryWrapper,
        collection: String,
    ) -> PyResult<SolrResponseWrapper> {
        let context = self.0.clone();
        builder.execute_blocking(py, context, collection)
    }
}
