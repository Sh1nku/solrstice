use crate::models::context::SolrServerContext;
use crate::models::error::SolrError;
use crate::models::response::SolrResponse;
use crate::queries::alias::{alias_exists, create_alias, delete_alias, get_aliases};
use crate::queries::collection::{
    collection_exists, create_collection, delete_collection, get_collections,
};
use crate::queries::config::{config_exists, delete_config, get_configs, upload_config};
use crate::queries::index::{DeleteQueryBuilder, UpdateQueryBuilder};
use crate::queries::select::SelectQueryBuilder;
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;

/// Async client for SolrCloud
/// # Examples
/// ```rust
/// use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
/// use solrstice::hosts::solr_server_host::SolrSingleServerHost;
/// use solrstice::models::context::SolrServerContextBuilder;
///
/// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
/// let client = AsyncSolrCloudClient::new(context);
/// ```
#[derive(Clone)]
pub struct AsyncSolrCloudClient {
    /// The solr server context used to specify how to connect to Solr
    pub context: SolrServerContext,
}

impl AsyncSolrCloudClient {
    /// Create a new instance of AsyncSolrCloudClient
    /// # Examples
    /// ```rust
    /// use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
    /// use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// use solrstice::models::context::SolrServerContextBuilder;
    ///
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// ```
    pub fn new(context: SolrServerContext) -> AsyncSolrCloudClient {
        AsyncSolrCloudClient { context }
    }

    /// Upload a config to SolrCloud
    /// # Examples
    /// ```no_run
    /// # use std::path::Path;
    /// # use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// client.upload_config("config_name", Path::new("/path/to/config")).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn upload_config(&self, name: &str, path: &Path) -> Result<(), SolrError> {
        upload_config(&self.context, name, path).await
    }

    /// Get the configs existing in SolrCloud
    /// # Examples
    /// ```no_run
    /// # use std::path::Path;
    /// # use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// let configs: Vec<String> = client.get_configs().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_configs(&self) -> Result<Vec<String>, SolrError> {
        get_configs(&self.context).await
    }

    /// Check if a config exists in SolrCloud
    /// # Examples
    /// ```no_run
    /// # use std::path::Path;
    /// # use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// let exists: bool = client.config_exists("config_name").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn config_exists(&self, name: &str) -> Result<bool, SolrError> {
        config_exists(&self.context, name).await
    }

    /// Delete a config from SolrCloud
    /// # Examples
    /// ```no_run
    /// # use std::path::Path;
    /// # use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// client.delete_config("config_name").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_config(&self, name: &str) -> Result<(), SolrError> {
        delete_config(&self.context, name).await
    }

    /// Create a collection in SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// client.create_collection("collection_name", "config_name", 1, 1).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_collection(
        &self,
        name: &str,
        config: &str,
        shards: usize,
        replication_factor: usize,
    ) -> Result<(), SolrError> {
        create_collection(&self.context, name, config, shards, replication_factor).await
    }

    /// Get collections from SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// let collections: Vec<String> = client.get_collections().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_collections(&self) -> Result<Vec<String>, SolrError> {
        get_collections(&self.context).await
    }

    /// Check if a collection exists in SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// let exists: bool = client.collection_exists("collection_name").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn collection_exists(&self, name: &str) -> Result<bool, SolrError> {
        collection_exists(&self.context, name).await
    }

    /// Delete a collection from SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// client.delete_collection("collection_name").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_collection(&self, name: &str) -> Result<(), SolrError> {
        delete_collection(&self.context, name).await
    }

    /// Create an alias in SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// client.create_alias("alias_name", &["collection1", "collection2"]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_alias(&self, alias: &str, collections: &[&str]) -> Result<(), SolrError> {
        create_alias(&self.context, alias, collections).await
    }

    /// Get aliases from SolrCloud
    /// # Examples
    /// ```no_run
    /// # use std::collections::HashMap;
    /// # use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// let aliases: HashMap<String, Vec<String>> = client.get_aliases().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_aliases(&self) -> Result<HashMap<String, Vec<String>>, SolrError> {
        get_aliases(&self.context).await
    }

    /// Check if an alias exists in SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// let exists: bool = client.alias_exists("alias_name").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn alias_exists(&self, name: &str) -> Result<bool, SolrError> {
        alias_exists(&self.context, name).await
    }

    /// Delete an alias from SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// client.delete_alias("alias_name").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_alias(&self, name: &str) -> Result<(), SolrError> {
        delete_alias(&self.context, name).await
    }

    /// Index some data into SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # use solrstice::queries::index::UpdateQueryBuilder;
    /// # use serde::Serialize;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// #[derive(Serialize)]
    /// struct Data {id: String}
    ///
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// let response = client.index(&UpdateQueryBuilder::new(), "collection_name", &[Data {id: "test".to_string()}]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn index<T: Serialize>(
        &self,
        builder: &UpdateQueryBuilder,
        collection: &str,
        data: &[T],
    ) -> Result<SolrResponse, SolrError> {
        builder.execute(&self.context, collection, data).await
    }

    /// Select some data from SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # use solrstice::queries::select::SelectQueryBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// let response = client.select(&SelectQueryBuilder::new().fq(&["age:[* TO *]"]), "collection_name").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn select(
        &self,
        builder: &SelectQueryBuilder,
        collection: &str,
    ) -> Result<SolrResponse, SolrError> {
        builder.execute(&self.context, collection).await
    }

    /// Delete some data from SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # use solrstice::queries::index::DeleteQueryBuilder;
    /// # use solrstice::queries::select::SelectQueryBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// let response = client.delete(&DeleteQueryBuilder::new().ids(&["document1"]).queries(&["age:[* TO *]"]), "collection_name").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(
        &self,
        builder: &DeleteQueryBuilder,
        collection: &str,
    ) -> Result<SolrResponse, SolrError> {
        builder.execute(&self.context, collection).await
    }
}
