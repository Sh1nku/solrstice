use crate::models::commit_type::CommitType;
use crate::models::context::SolrServerContext;
use crate::models::error::{try_solr_error, SolrError};
use crate::models::response::SolrResponse;
use serde::{Deserialize, Serialize};

/// A builder for the update handler.
/// # Examples
/// ```no_run
/// # use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
/// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
/// # use solrstice::models::context::SolrServerContextBuilder;
/// # use solrstice::queries::index::UpdateQueryBuilder;
/// # use serde::Serialize;
/// # use solrstice::models::commit_type::CommitType;
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// #[derive(Serialize)]
/// struct Data {id: String}
///
/// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
/// let client = AsyncSolrCloudClient::new(context);
/// let response = client.index(&UpdateQueryBuilder::new().commit_type(CommitType::Soft), "collection_name", &[Data {id: "test".to_string()}]).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug)]
pub struct UpdateQueryBuilder {
    pub handler: String,
    pub commit_type: CommitType,
}

impl UpdateQueryBuilder {
    /// Create a new instance of UpdateQueryBuilder.
    /// # Examples
    /// ```no_run
    /// # use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # use solrstice::queries::index::UpdateQueryBuilder;
    /// # use serde::Serialize;
    /// # use solrstice::models::commit_type::CommitType;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// #[derive(Serialize)]
    /// struct Data {id: String}
    ///
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// let response = client.index(&UpdateQueryBuilder::new().commit_type(CommitType::Soft), "collection_name", &[Data {id: "test".to_string()}]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> Self {
        UpdateQueryBuilder {
            handler: "update".to_string(),
            commit_type: CommitType::Hard,
        }
    }

    /// Set the handler for the query. Default is "update".
    /// # Examples
    /// ```no_run
    /// use solrstice::queries::index::UpdateQueryBuilder;
    /// let builder = UpdateQueryBuilder::new().handler("custom_handler");
    /// ```
    pub fn handler(mut self, handler: &str) -> Self {
        self.handler = handler.to_string();
        self
    }

    /// Set the commit type for the query. Default is CommitType::Hard.
    /// # Examples
    /// ```no_run
    /// use solrstice::models::commit_type::CommitType;
    /// use solrstice::queries::index::UpdateQueryBuilder;
    /// let builder = UpdateQueryBuilder::new().commit_type(CommitType::Soft);
    /// ```
    pub fn commit_type(mut self, commit_type: CommitType) -> Self {
        self.commit_type = commit_type;
        self
    }

    /// Execute the query.
    ///
    /// This is not meant to be used directly. Use [AsyncSolrCloudClient::index](crate::clients::async_cloud_client::AsyncSolrCloudClient::index) instead.
    pub async fn execute<T: Serialize>(
        &self,
        builder: &SolrServerContext,
        collection: &str,
        data: &[T],
    ) -> Result<SolrResponse, SolrError> {
        let solr_url = format!(
            "{}/solr/{}/{}",
            &builder.host.get_solr_node().await?,
            &collection,
            &self.handler
        );

        let mut request = builder
            .client
            .post(solr_url)
            .query(&[("overwrite", "true"), ("wt", "json")])
            .json(data);
        if let Some(auth) = &builder.auth {
            request = auth.add_auth_to_request(request)
        }

        match self.commit_type {
            CommitType::Hard => request = request.query(&[("commit", "true")]),
            CommitType::Soft => request = request.query(&[("softCommit", "true")]),
        }
        let json = request.send().await?.json::<SolrResponse>().await?;
        try_solr_error(&json)?;
        Ok(json)
    }
}

/// A builder for deleting documents.
///
/// Since there is no way to delete properly with JSON, it uses XML.
/// # Examples
/// ```no_run
/// # use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
/// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
/// # use solrstice::models::context::SolrServerContextBuilder;
/// # use solrstice::queries::index::DeleteQueryBuilder;
/// # use serde::Serialize;
/// # use solrstice::models::commit_type::CommitType;
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// #[derive(Serialize)]
/// struct Data {id: String}
///
/// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
/// let client = AsyncSolrCloudClient::new(context);
/// let response = client.delete(&DeleteQueryBuilder::new().ids(&["document1", "document2"]), "collection_name").await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug)]
pub struct DeleteQueryBuilder {
    /// The handler for the query. Default is "update".
    pub handler: String,
    /// The commit type for the query. Default is CommitType::Hard.
    pub commit_type: CommitType,
    /// Ids to delete
    pub ids: Option<Vec<String>>,
    /// Queries to delete
    pub queries: Option<Vec<String>>,
}

impl DeleteQueryBuilder {
    /// Create a new instance of DeleteQueryBuilder.
    /// # Examples
    /// ```no_run
    /// # use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # use solrstice::queries::index::DeleteQueryBuilder;
    /// # use serde::Serialize;
    /// # use solrstice::models::commit_type::CommitType;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// #[derive(Serialize)]
    /// struct Data {id: String}
    ///
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// let response = client.delete(&DeleteQueryBuilder::new().ids(&["document1", "document2"]), "collection_name").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> Self {
        DeleteQueryBuilder {
            handler: "update".to_string(),
            commit_type: CommitType::Hard,
            ids: None,
            queries: None,
        }
    }

    /// Set the handler for the query. Default is "update".
    /// # Examples
    /// ```no_run
    /// use solrstice::queries::index::DeleteQueryBuilder;
    /// let builder = DeleteQueryBuilder::new().handler("custom_handler");
    /// ```
    pub fn handler(mut self, handler: &str) -> Self {
        self.handler = handler.to_string();
        self
    }

    /// Set the commit_type for the query. Default is CommitType::Hard.
    /// # Examples
    /// ```no_run
    /// use solrstice::models::commit_type::CommitType;
    /// use solrstice::queries::index::DeleteQueryBuilder;
    /// let builder = DeleteQueryBuilder::new().commit_type(CommitType::Soft);
    /// ```
    pub fn commit_type(mut self, commit_type: CommitType) -> Self {
        self.commit_type = commit_type;
        self
    }

    /// Set the ids to delete
    /// # Examples
    /// ```no_run
    /// use solrstice::queries::index::DeleteQueryBuilder;
    /// let builder = DeleteQueryBuilder::new().ids(&["document1", "document2"]);
    /// ```
    pub fn ids(mut self, ids: &[&str]) -> Self {
        self.ids = Some(ids.iter().map(|s| s.to_string()).collect());
        self
    }

    /// Set the queries to delete
    /// # Examples
    /// ```no_run
    /// use solrstice::queries::index::DeleteQueryBuilder;
    /// let builder = DeleteQueryBuilder::new().queries(&["age:[* TO *]"]);
    /// ```
    pub fn queries(mut self, queries: &[&str]) -> Self {
        self.queries = Some(queries.iter().map(|s| s.to_string()).collect());
        self
    }

    /// Execute the query.
    ///
    /// This is not meant to be used directly. Use [AsyncSolrCloudClient::delete](crate::clients::async_cloud_client::AsyncSolrCloudClient::delete) instead.
    pub async fn execute(
        &self,
        context: &SolrServerContext,
        collection: &str,
    ) -> Result<SolrResponse, SolrError> {
        let solr_url = format!(
            "{}/solr/{}/{}",
            &context.host.get_solr_node().await?,
            &collection,
            &self.handler
        );
        let ids = self.ids.as_ref().map(|ids| {
            ids.iter()
                .map(|id| format!("<id>{}</id>", id))
                .collect::<Vec<String>>()
                .join("")
        });
        let queries = self.queries.as_ref().map(|queries| {
            queries
                .iter()
                .map(|query| format!("<query>{}</query>", query))
                .collect::<Vec<String>>()
                .join("")
        });

        let mut request = context
            .client
            .post(solr_url)
            .query(&[("overwrite", "true"), ("wt", "json")])
            .header("Content-Type", "application/xml")
            .body(format!(
                "<delete>{}{}</delete>",
                ids.unwrap_or_default(),
                queries.unwrap_or_default()
            ));
        if let Some(auth) = &context.auth {
            request = auth.add_auth_to_request(request)
        }

        match self.commit_type {
            CommitType::Hard => request = request.query(&[("commit", "true")]),
            CommitType::Soft => request = request.query(&[("softCommit", "true")]),
        }
        let json = request.send().await?.json::<SolrResponse>().await?;
        try_solr_error(&json)?;
        Ok(json)
    }
}

#[cfg(feature = "blocking")]
use crate::runtime::RUNTIME;
#[cfg(feature = "blocking")]
impl UpdateQueryBuilder {
    /// Execute the query.
    ///
    /// This is not meant to be used directly. Use [BlockingSolrCloudClient::index](crate::clients::blocking_cloud_client::BlockingSolrCloudClient::index) instead.
    pub fn execute_blocking<T: Serialize>(
        &self,
        context: &SolrServerContext,
        collection: &str,
        data: &[T],
    ) -> Result<SolrResponse, SolrError> {
        RUNTIME
            .handle()
            .block_on(self.execute(context, collection, data))
    }
}
#[cfg(feature = "blocking")]
impl DeleteQueryBuilder {
    /// Execute the query.
    ///
    /// This is not meant to be used directly. Use [BlockingSolrCloudClient::delete](crate::clients::blocking_cloud_client::BlockingSolrCloudClient::delete) instead.
    pub fn execute_blocking(
        &self,
        context: &SolrServerContext,
        collection: &str,
    ) -> Result<SolrResponse, SolrError> {
        RUNTIME.handle().block_on(self.execute(context, collection))
    }
}
