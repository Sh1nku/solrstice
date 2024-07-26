use crate::error::Error;
use crate::models::context::SolrServerContext;
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
/// use solrstice::{BlockingSolrCloudClient, SolrServerContextBuilder, SolrSingleServerHost};
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
    /// use solrstice::{BlockingSolrCloudClient, SolrServerContextBuilder, SolrSingleServerHost};
    ///
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = BlockingSolrCloudClient::new(context);
    /// ```
    pub fn new<C: Into<SolrServerContext>>(context: C) -> BlockingSolrCloudClient {
        BlockingSolrCloudClient {
            context: context.into(),
        }
    }

    /// Upload a config to SolrCloud
    /// # Examples
    /// ```no_run
    /// # use std::path::Path;
    /// # use solrstice::BlockingSolrCloudClient;
    /// # use solrstice::SolrSingleServerHost;
    /// # use solrstice::SolrServerContextBuilder;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = BlockingSolrCloudClient::new(context);
    /// client.upload_config("config_name", Path::new("/path/to/config"))?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn upload_config<S: AsRef<str>, P: AsRef<Path>>(
        &self,
        name: S,
        path: P,
    ) -> Result<(), Error> {
        upload_config_blocking(&self.context, name, path)
    }

    /// Get the configs existing in SolrCloud
    /// # Examples
    /// ```no_run
    /// # use std::path::Path;
    /// # use solrstice::BlockingSolrCloudClient;
    /// # use solrstice::SolrSingleServerHost;
    /// # use solrstice::SolrServerContextBuilder;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = BlockingSolrCloudClient::new(context);
    /// let configs: Vec<String> = client.get_configs()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_configs(&self) -> Result<Vec<String>, Error> {
        get_configs_blocking(&self.context)
    }

    /// Check if a config exists in SolrCloud
    /// # Examples
    /// ```no_run
    /// # use std::path::Path;
    /// # use solrstice::BlockingSolrCloudClient;
    /// # use solrstice::SolrSingleServerHost;
    /// # use solrstice::SolrServerContextBuilder;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = BlockingSolrCloudClient::new(context);
    /// let exists: bool = client.config_exists("config_name")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn config_exists<S: AsRef<str>>(&self, name: S) -> Result<bool, Error> {
        config_exists_blocking(&self.context, name)
    }

    /// Delete a config from SolrCloud
    /// # Examples
    /// ```no_run
    /// # use std::path::Path;
    /// # use solrstice::BlockingSolrCloudClient;
    /// # use solrstice::SolrSingleServerHost;
    /// # use solrstice::SolrServerContextBuilder;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = BlockingSolrCloudClient::new(context);
    /// client.delete_config("config_name")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete_config<S: AsRef<str>>(&self, name: S) -> Result<(), Error> {
        delete_config_blocking(&self.context, name)
    }

    /// Create a collection in SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::BlockingSolrCloudClient;
    /// # use solrstice::SolrSingleServerHost;
    /// # use solrstice::SolrServerContextBuilder;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = BlockingSolrCloudClient::new(context);
    /// client.create_collection("collection_name", "config_name", 1, 1)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_collection<S: AsRef<str>>(
        &self,
        name: S,
        config: S,
        shards: usize,
        replication_factor: usize,
    ) -> Result<(), Error> {
        create_collection_blocking(&self.context, name, config, shards, replication_factor)
    }

    /// Get collections from SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::AsyncSolrCloudClient;
    /// use solrstice::BlockingSolrCloudClient;
    /// # use solrstice::SolrSingleServerHost;
    /// # use solrstice::SolrServerContextBuilder;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = BlockingSolrCloudClient::new(context);
    /// let collections: Vec<String> = client.get_collections()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_collections(&self) -> Result<Vec<String>, Error> {
        get_collections_blocking(&self.context)
    }

    /// Check if a collection exists in SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::BlockingSolrCloudClient;
    /// # use solrstice::SolrSingleServerHost;
    /// # use solrstice::SolrServerContextBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = BlockingSolrCloudClient::new(context);
    /// let exists: bool = client.collection_exists("collection_name")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn collection_exists<S: AsRef<str>>(&self, name: S) -> Result<bool, Error> {
        collection_exists_blocking(&self.context, name)
    }

    /// Delete a collection from SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::BlockingSolrCloudClient;
    /// # use solrstice::SolrSingleServerHost;
    /// # use solrstice::SolrServerContextBuilder;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = BlockingSolrCloudClient::new(context);
    /// client.delete_collection("collection_name")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete_collection<S: AsRef<str>>(&self, name: S) -> Result<(), Error> {
        delete_collection_blocking(&self.context, name)
    }

    /// Create an alias in SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::BlockingSolrCloudClient;
    /// # use solrstice::SolrSingleServerHost;
    /// # use solrstice::SolrServerContextBuilder;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = BlockingSolrCloudClient::new(context);
    /// client.create_alias("alias_name", &["collection1", "collection2"])?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_alias<S: AsRef<str>>(&self, alias: S, collections: &[S]) -> Result<(), Error> {
        create_alias_blocking(&self.context, alias, collections)
    }

    /// Get aliases from SolrCloud
    /// # Examples
    /// ```no_run
    /// # use std::collections::HashMap;
    /// # use solrstice::BlockingSolrCloudClient;
    /// # use solrstice::SolrSingleServerHost;
    /// # use solrstice::SolrServerContextBuilder;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = BlockingSolrCloudClient::new(context);
    /// let aliases: HashMap<String, Vec<String>> = client.get_aliases()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_aliases(&self) -> Result<HashMap<String, Vec<String>>, Error> {
        get_aliases_blocking(&self.context)
    }

    /// Check if an alias exists in SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::BlockingSolrCloudClient;
    /// # use solrstice::SolrSingleServerHost;
    /// # use solrstice::SolrServerContextBuilder;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = BlockingSolrCloudClient::new(context);
    /// let exists: bool = client.alias_exists("alias_name")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn alias_exists<S: AsRef<str>>(&self, name: S) -> Result<bool, Error> {
        alias_exists_blocking(&self.context, name)
    }

    /// Delete an alias from SolrCloud
    /// # Examples
    /// ```no_run
    /// use solrstice::BlockingSolrCloudClient;
    /// # use solrstice::SolrSingleServerHost;
    /// # use solrstice::SolrServerContextBuilder;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = BlockingSolrCloudClient::new(context);
    /// client.delete_alias("alias_name")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete_alias<S: AsRef<str>>(&self, name: S) -> Result<(), Error> {
        delete_alias_blocking(&self.context, name)
    }

    /// Index some data into SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::SolrSingleServerHost;
    /// # use solrstice::SolrServerContextBuilder;
    /// # use solrstice::UpdateQuery;
    /// # use serde::Serialize;
    /// # use solrstice::BlockingSolrCloudClient;
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
    pub fn index<T: Serialize, S: AsRef<str>, B: AsRef<UpdateQuery>>(
        &self,
        builder: B,
        collection: S,
        data: &[T],
    ) -> Result<SolrResponse, Error> {
        builder
            .as_ref()
            .execute_blocking(&self.context, collection, data)
    }

    /// Select some data from SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::BlockingSolrCloudClient;
    /// # use solrstice::SolrSingleServerHost;
    /// # use solrstice::SolrServerContextBuilder;
    /// # use solrstice::SelectQuery;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = BlockingSolrCloudClient::new(context);
    /// let response = client.select(&SelectQuery::new().fq(["age:[* TO *]"]), "collection_name")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn select<S: AsRef<str>, B: AsRef<SelectQuery>>(
        &self,
        builder: B,
        collection: S,
    ) -> Result<SolrResponse, Error> {
        builder.as_ref().execute_blocking(&self.context, collection)
    }

    /// Delete some data from SolrCloud
    /// # Examples
    /// ```no_run
    /// # use solrstice::BlockingSolrCloudClient;
    /// # use solrstice::SolrSingleServerHost;
    /// # use solrstice::SolrServerContextBuilder;
    /// # use solrstice::DeleteQuery;
    /// # use solrstice::SelectQuery;
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = BlockingSolrCloudClient::new(context);
    /// let response = client.delete(&DeleteQuery::new().ids(["document1"]).queries(["age:[* TO *]"]), "collection_name")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete<S: AsRef<str>, B: AsRef<DeleteQuery>>(
        &self,
        builder: B,
        collection: S,
    ) -> Result<SolrResponse, Error> {
        builder.as_ref().execute_blocking(&self.context, collection)
    }
}
