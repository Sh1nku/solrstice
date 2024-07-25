use crate::error::Error;
use crate::models::context::SolrServerContext;
use crate::models::response::SolrResponse;
use crate::queries::alias::{alias_exists, create_alias, delete_alias, get_aliases};
use crate::queries::collection::{
    collection_exists, create_collection, delete_collection, get_collections,
};
use crate::queries::config::{config_exists, delete_config, get_configs, upload_config};
use crate::queries::index::{DeleteQuery, UpdateQuery};
use crate::queries::select::SelectQuery;
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;

/// Async client for SolrCloud
/// # Examples
/// ```rust
/// use solrstice::{AsyncSolrCloudClient, SolrServerContextBuilder, SolrSingleServerHost};
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
    /// use solrstice::{AsyncSolrCloudClient, SolrServerContextBuilder, SolrSingleServerHost};
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// ```
    pub fn new<C: Into<SolrServerContext>>(context: C) -> AsyncSolrCloudClient {
        AsyncSolrCloudClient {
            context: context.into(),
        }
    }

    /// Upload a config to SolrCloud
    /// # Examples
    /// ```no_run
    /// # use std::path::Path;
    /// # use solrstice::{AsyncSolrCloudClient, SolrServerContextBuilder, SolrSingleServerHost};
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// client.upload_config("config_name", Path::new("/path/to/config")).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn upload_config<N: AsRef<str>, P: AsRef<Path>>(
        &self,
        name: N,
        path: P,
    ) -> Result<(), Error> {
        upload_config(&self.context, name, path).await
    }

    /// Get the configs existing in SolrCloud
    /// # Examples
    /// ```no_run
    /// # use std::path::Path;
    /// # use solrstice::AsyncSolrCloudClient;
    /// # use solrstice::SolrSingleServerHost;
    /// # use solrstice::SolrServerContextBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// let configs: Vec<String> = client.get_configs().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_configs(&self) -> Result<Vec<String>, Error> {
        get_configs(&self.context).await
    }

    /// Check if a config exists in SolrCloud
    /// # Examples
    /// ```no_run
    /// # use std::path::Path;
    /// # use solrstice::AsyncSolrCloudClient;
    /// # use solrstice::SolrSingleServerHost;
    /// # use solrstice::SolrServerContextBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// let exists: bool = client.config_exists("config_name").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn config_exists<S: AsRef<str>>(&self, name: S) -> Result<bool, Error> {
        config_exists(&self.context, name).await
    }

    /// Delete a config from SolrCloud
    /// # Examples
    /// ```no_run
    /// # use std::path::Path;
    /// # use solrstice::AsyncSolrCloudClient;
    /// # use solrstice::SolrSingleServerHost;
    /// # use solrstice::SolrServerContextBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// client.delete_config("config_name").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_config<N: AsRef<str>>(&self, name: N) -> Result<(), Error> {
        delete_config(&self.context, name).await
    }

    /// Create a collection in SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::AsyncSolrCloudClient;
    /// # use solrstice::SolrSingleServerHost;
    /// # use solrstice::SolrServerContextBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// client.create_collection("collection_name", "config_name", 1, 1).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_collection<S: AsRef<str>>(
        &self,
        name: S,
        config: S,
        shards: usize,
        replication_factor: usize,
    ) -> Result<(), Error> {
        create_collection(&self.context, name, config, shards, replication_factor).await
    }

    /// Get collections from SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::AsyncSolrCloudClient;
    /// # use solrstice::SolrSingleServerHost;
    /// # use solrstice::SolrServerContextBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// let collections: Vec<String> = client.get_collections().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_collections(&self) -> Result<Vec<String>, Error> {
        get_collections(&self.context).await
    }

    /// Check if a collection exists in SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::AsyncSolrCloudClient;
    /// # use solrstice::SolrSingleServerHost;
    /// # use solrstice::SolrServerContextBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// let exists: bool = client.collection_exists("collection_name").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn collection_exists<'a, S: AsRef<str>>(&self, name: S) -> Result<bool, Error> {
        collection_exists(&self.context, name).await
    }

    /// Delete a collection from SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::AsyncSolrCloudClient;
    /// # use solrstice::SolrSingleServerHost;
    /// # use solrstice::SolrServerContextBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// client.delete_collection("collection_name").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_collection<N: AsRef<str>>(&self, name: N) -> Result<(), Error> {
        delete_collection(&self.context, name).await
    }

    /// Create an alias in SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::AsyncSolrCloudClient;
    /// # use solrstice::SolrSingleServerHost;
    /// # use solrstice::SolrServerContextBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// client.create_alias("alias_name", &["collection1", "collection2"]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_alias<S: AsRef<str>>(
        &self,
        alias: S,
        collections: &[S],
    ) -> Result<(), Error> {
        create_alias(&self.context, alias, collections).await
    }

    /// Get aliases from SolrCloud
    /// # Examples
    /// ```no_run
    /// # use std::collections::HashMap;
    /// # use solrstice::AsyncSolrCloudClient;
    /// # use solrstice::SolrSingleServerHost;
    /// # use solrstice::SolrServerContextBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// let aliases: HashMap<String, Vec<String>> = client.get_aliases().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_aliases(&self) -> Result<HashMap<String, Vec<String>>, Error> {
        get_aliases(&self.context).await
    }

    /// Check if an alias exists in SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::AsyncSolrCloudClient;
    /// # use solrstice::SolrSingleServerHost;
    /// # use solrstice::SolrServerContextBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// let exists: bool = client.alias_exists("alias_name").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn alias_exists<N: AsRef<str>>(&self, name: N) -> Result<bool, Error> {
        alias_exists(&self.context, name).await
    }

    /// Delete an alias from SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::AsyncSolrCloudClient;
    /// # use solrstice::SolrSingleServerHost;
    /// # use solrstice::SolrServerContextBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// client.delete_alias("alias_name").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_alias<S: AsRef<str>>(&self, name: S) -> Result<(), Error> {
        delete_alias(&self.context, name).await
    }

    /// Index some data into SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::AsyncSolrCloudClient;
    /// # use solrstice::SolrSingleServerHost;
    /// # use solrstice::SolrServerContextBuilder;
    /// # use solrstice::UpdateQuery;
    /// # use serde::Serialize;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// #[derive(Serialize)]
    /// struct Data {id: String}
    ///
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// let response = client.index(&UpdateQuery::new(), "collection_name", &[Data {id: "test".to_string()}]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn index<T: Serialize, B: AsRef<UpdateQuery>, C: AsRef<str>>(
        &self,
        builder: B,
        collection: C,
        data: &[T],
    ) -> Result<SolrResponse, Error> {
        builder
            .as_ref()
            .execute(&self.context, collection, data)
            .await
    }

    /// Select some data from SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::AsyncSolrCloudClient;
    /// # use solrstice::SolrSingleServerHost;
    /// # use solrstice::SolrServerContextBuilder;
    /// # use solrstice::SelectQuery;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// let response = client.select(&SelectQuery::new().fq(["age:[* TO *]"]), "collection_name").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn select<B: AsRef<SelectQuery>, C: AsRef<str>>(
        &self,
        builder: B,
        collection: C,
    ) -> Result<SolrResponse, Error> {
        builder.as_ref().execute(&self.context, collection).await
    }

    /// Delete some data from SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::AsyncSolrCloudClient;
    /// # use solrstice::SolrSingleServerHost;
    /// # use solrstice::SolrServerContextBuilder;
    /// # use solrstice::DeleteQuery;
    /// # use solrstice::SelectQuery;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// let response = client.delete(&DeleteQuery::new().ids(["document1"]).queries(["age:[* TO *]"]), "collection_name").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete<B: AsRef<DeleteQuery>, C: AsRef<str>>(
        &self,
        builder: B,
        collection: C,
    ) -> Result<SolrResponse, Error> {
        builder.as_ref().execute(&self.context, collection).await
    }
}
