use crate::models::context::SolrServerContext;
use crate::models::error::SolrError;
use crate::models::response::SolrResponse;
use crate::queries::alias::{
    alias_exists_blocking, create_alias_blocking, delete_alias_blocking, get_aliases_blocking,
};
use crate::queries::collection::{
    collection_exists_blocking, create_collection_blocking, delete_collection_blocking,
    get_collections_blocking,
};
use crate::queries::config::{
    config_exists_blocking, delete_config_blocking, get_configs_blocking, upload_config_blocking,
};
use crate::queries::index::{DeleteQuery, UpdateQuery};
use crate::queries::select::SelectQuery;
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;

/// A blocking client for SolrCloud.
/// # Examples
/// ```rust
/// use solrstice::clients::blocking_cloud_client::BlockingSolrCloudClient;
/// use solrstice::hosts::solr_server_host::SolrSingleServerHost;
/// use solrstice::models::context::SolrServerContextBuilder;
///
/// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
/// let client = BlockingSolrCloudClient::new(context);
/// ```
#[derive(Clone)]
pub struct BlockingSolrCloudClient {
    /// The solr server context used to specify how to connect to Solr
    pub context: SolrServerContext,
}

impl BlockingSolrCloudClient {
    /// Create a new instance of BlockingSolrCloudClient
    /// # Examples
    /// ```rust
    /// use solrstice::clients::blocking_cloud_client::BlockingSolrCloudClient;
    /// use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// use solrstice::models::context::SolrServerContextBuilder;
    ///
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = BlockingSolrCloudClient::new(context);
    /// ```
    pub fn new(context: SolrServerContext) -> BlockingSolrCloudClient {
        BlockingSolrCloudClient { context }
    }

    /// Upload a config to SolrCloud
    /// # Examples
    /// ```no_run
    /// # use std::path::Path;
    /// # use solrstice::clients::blocking_cloud_client::BlockingSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = BlockingSolrCloudClient::new(context);
    /// client.upload_config("config_name", Path::new("/path/to/config"))?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn upload_config(&self, name: &str, path: &Path) -> Result<(), SolrError> {
        upload_config_blocking(&self.context, name, path)
    }

    /// Get the configs existing in SolrCloud
    /// # Examples
    /// ```no_run
    /// # use std::path::Path;
    /// # use solrstice::clients::blocking_cloud_client::BlockingSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = BlockingSolrCloudClient::new(context);
    /// let configs: Vec<String> = client.get_configs()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_configs(&self) -> Result<Vec<String>, SolrError> {
        get_configs_blocking(&self.context)
    }

    /// Check if a config exists in SolrCloud
    /// # Examples
    /// ```no_run
    /// # use std::path::Path;
    /// # use solrstice::clients::blocking_cloud_client::BlockingSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = BlockingSolrCloudClient::new(context);
    /// let exists: bool = client.config_exists("config_name")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn config_exists(&self, name: &str) -> Result<bool, SolrError> {
        config_exists_blocking(&self.context, name)
    }

    /// Delete a config from SolrCloud
    /// # Examples
    /// ```no_run
    /// # use std::path::Path;
    /// # use solrstice::clients::blocking_cloud_client::BlockingSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = BlockingSolrCloudClient::new(context);
    /// client.delete_config("config_name")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete_config(&self, name: &str) -> Result<(), SolrError> {
        delete_config_blocking(&self.context, name)
    }

    /// Create a collection in SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::clients::blocking_cloud_client::BlockingSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = BlockingSolrCloudClient::new(context);
    /// client.create_collection("collection_name", "config_name", 1, 1)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_collection(
        &self,
        name: &str,
        config: &str,
        shards: usize,
        replication_factor: usize,
    ) -> Result<(), SolrError> {
        create_collection_blocking(&self.context, name, config, shards, replication_factor)
    }

    /// Get collections from SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
    /// use solrstice::clients::blocking_cloud_client::BlockingSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = BlockingSolrCloudClient::new(context);
    /// let collections: Vec<String> = client.get_collections()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_collections(&self) -> Result<Vec<String>, SolrError> {
        get_collections_blocking(&self.context)
    }

    /// Check if a collection exists in SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::clients::blocking_cloud_client::BlockingSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = BlockingSolrCloudClient::new(context);
    /// let exists: bool = client.collection_exists("collection_name")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn collection_exists(&self, name: &str) -> Result<bool, SolrError> {
        collection_exists_blocking(&self.context, name)
    }

    /// Delete a collection from SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::clients::blocking_cloud_client::BlockingSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = BlockingSolrCloudClient::new(context);
    /// client.delete_collection("collection_name")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete_collection(&self, name: &str) -> Result<(), SolrError> {
        delete_collection_blocking(&self.context, name)
    }

    /// Create an alias in SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::clients::blocking_cloud_client::BlockingSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = BlockingSolrCloudClient::new(context);
    /// client.create_alias("alias_name", &["collection1", "collection2"])?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_alias(&self, alias: &str, collections: &[&str]) -> Result<(), SolrError> {
        create_alias_blocking(&self.context, alias, collections)
    }

    /// Get aliases from SolrCloud
    /// # Examples
    /// ```no_run
    /// # use std::collections::HashMap;
    /// # use solrstice::clients::blocking_cloud_client::BlockingSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = BlockingSolrCloudClient::new(context);
    /// let aliases: HashMap<String, Vec<String>> = client.get_aliases()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_aliases(&self) -> Result<HashMap<String, Vec<String>>, SolrError> {
        get_aliases_blocking(&self.context)
    }

    /// Check if an alias exists in SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::clients::blocking_cloud_client::BlockingSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = BlockingSolrCloudClient::new(context);
    /// let exists: bool = client.alias_exists("alias_name")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn alias_exists(&self, name: &str) -> Result<bool, SolrError> {
        alias_exists_blocking(&self.context, name)
    }

    /// Delete an alias from SolrCloud
    /// # Examples
    /// ```no_run
    /// use solrstice::clients::blocking_cloud_client::BlockingSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = BlockingSolrCloudClient::new(context);
    /// client.delete_alias("alias_name")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete_alias(&self, name: &str) -> Result<(), SolrError> {
        delete_alias_blocking(&self.context, name)
    }

    /// Index some data into SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # use solrstice::queries::index::UpdateQuery;
    /// # use serde::Serialize;
    /// # use solrstice::clients::blocking_cloud_client::BlockingSolrCloudClient;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// #[derive(Serialize)]
    /// struct Data {id: String}
    ///
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = BlockingSolrCloudClient::new(context);
    /// let response = client.index(&UpdateQuery::new(), "collection_name", &[Data {id: "test".to_string()}])?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn index<T: Serialize>(
        &self,
        builder: &UpdateQuery,
        collection: &str,
        data: &[T],
    ) -> Result<SolrResponse, SolrError> {
        builder.execute_blocking(&self.context, collection, data)
    }

    /// Select some data from SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::clients::blocking_cloud_client::BlockingSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # use solrstice::queries::select::SelectQuery;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = BlockingSolrCloudClient::new(context);
    /// let response = client.select(&SelectQuery::new().fq(&["age:[* TO *]"]), "collection_name")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn select(
        &self,
        builder: &SelectQuery,
        collection: &str,
    ) -> Result<SolrResponse, SolrError> {
        builder.execute_blocking(&self.context, collection)
    }

    /// Delete some data from SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::clients::blocking_cloud_client::BlockingSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # use solrstice::queries::index::DeleteQuery;
    /// # use solrstice::queries::select::SelectQuery;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = BlockingSolrCloudClient::new(context);
    /// let response = client.delete(&DeleteQuery::new().ids(&["document1"]).queries(&["age:[* TO *]"]), "collection_name")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete(
        &self,
        builder: &DeleteQuery,
        collection: &str,
    ) -> Result<SolrResponse, SolrError> {
        builder.execute_blocking(&self.context, collection)
    }
}
