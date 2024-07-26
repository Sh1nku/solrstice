use crate::error::Error;
use crate::models::commit_type::CommitType;
use crate::models::context::SolrServerContext;
use crate::models::response::SolrResponse;
use crate::queries::request_builder::SolrRequestBuilder;
use serde::{Deserialize, Serialize};

/// A builder for the update handler.
/// # Examples
/// ```no_run
/// use serde::Serialize;
/// use solrstice::{AsyncSolrCloudClient, CommitType, SolrServerContextBuilder, SolrSingleServerHost, UpdateQuery};
///
///  async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// #[derive(Serialize)]
/// struct Data {id: String}
///
/// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
/// let client = AsyncSolrCloudClient::new(context);
/// let response = client.index(&UpdateQuery::new().commit_type(CommitType::Soft), "collection_name", &[Data {id: "test".to_string()}]).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug)]
pub struct UpdateQuery {
    handler: String,
    commit_type: CommitType,
}

impl From<&UpdateQuery> for UpdateQuery {
    fn from(query: &UpdateQuery) -> Self {
        query.clone()
    }
}

impl AsRef<UpdateQuery> for UpdateQuery {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl UpdateQuery {
    /// Create a new instance of UpdateQuery.
    /// # Examples
    /// ```no_run
    /// # use serde::Serialize;    ///
    ///
    /// use solrstice::{AsyncSolrCloudClient, CommitType, SolrServerContextBuilder, SolrSingleServerHost, UpdateQuery};
    ///
    ///  async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// #[derive(Serialize)]
    /// struct Data {id: String}
    ///
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// let response = client.index(&UpdateQuery::new().commit_type(CommitType::Soft), "collection_name", &[Data {id: "test".to_string()}]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> Self {
        UpdateQuery {
            handler: "update".to_string(),
            commit_type: CommitType::Hard,
        }
    }

    /// Set the handler for the query. Default is "update".
    /// # Examples
    /// ```no_run
    /// use solrstice::UpdateQuery;
    /// let builder = UpdateQuery::new().handler("custom_handler");
    /// ```
    pub fn handler<S: Into<String>>(mut self, handler: S) -> Self {
        self.handler = handler.into();
        self
    }

    /// Set the commit type for the query. Default is CommitType::Hard.
    /// # Examples
    /// ```no_run
    /// use solrstice::CommitType;
    /// use solrstice::UpdateQuery;
    /// let builder = UpdateQuery::new().commit_type(CommitType::Soft);
    /// ```
    pub fn commit_type(mut self, commit_type: CommitType) -> Self {
        self.commit_type = commit_type;
        self
    }

    /// Execute the query.
    ///
    /// This is not meant to be used directly. Use [AsyncSolrCloudClient::index](crate::clients::async_cloud_client::AsyncSolrCloudClient::index) instead.
    pub async fn execute<C: AsRef<SolrServerContext>, D: Serialize, S: AsRef<str>>(
        &self,
        context: C,
        collection: S,
        data: &[D],
    ) -> Result<SolrResponse, Error> {
        let mut query_params = vec![("overwrite", "true")];
        match self.commit_type {
            CommitType::Hard => query_params.push(("commit", "true")),
            CommitType::Soft => query_params.push(("softCommit", "true")),
        }

        SolrRequestBuilder::new(
            context.as_ref(),
            format!("/solr/{}/{}", collection.as_ref(), self.handler.as_str()).as_str(),
        )
        .with_query_params(query_params.as_ref())
        .send_post_with_json(data)
        .await
    }
}

/// A builder for deleting documents.
///
/// Since there is no way to delete properly with JSON, it uses XML.
/// # Examples
/// ```no_run
/// use serde::Serialize;
/// use solrstice::{AsyncSolrCloudClient, DeleteQuery, SolrServerContextBuilder, SolrSingleServerHost};
///
///  async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// #[derive(Serialize)]
/// struct Data {id: String}
///
/// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
/// let client = AsyncSolrCloudClient::new(context);
/// let response = client.delete(&DeleteQuery::new().ids(["document1", "document2"]), "collection_name").await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug)]
pub struct DeleteQuery {
    /// The handler for the query. Default is "update".
    handler: String,
    /// The commit type for the query. Default is CommitType::Hard.
    commit_type: CommitType,
    /// Ids to delete
    ids: Option<Vec<String>>,
    /// Queries to delete
    queries: Option<Vec<String>>,
}

impl From<&DeleteQuery> for DeleteQuery {
    fn from(query: &DeleteQuery) -> Self {
        query.clone()
    }
}

impl AsRef<DeleteQuery> for DeleteQuery {
    fn as_ref(&self) -> &DeleteQuery {
        self
    }
}

impl DeleteQuery {
    /// Create a new instance of DeleteQuery.
    /// # Examples
    /// ```no_run
    /// use serde::Serialize;
    /// use solrstice::{AsyncSolrCloudClient, DeleteQuery, SolrServerContextBuilder, SolrSingleServerHost};
    ///
    ///  async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// #[derive(Serialize)]
    /// struct Data {id: String}
    ///
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// let response = client.delete(&DeleteQuery::new().ids(["document1", "document2"]), "collection_name").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> Self {
        DeleteQuery {
            handler: "update".to_string(),
            commit_type: CommitType::Hard,
            ids: None,
            queries: None,
        }
    }

    /// Set the handler for the query. Default is "update".
    /// # Examples
    /// ```no_run
    /// use solrstice::DeleteQuery;
    /// let builder = DeleteQuery::new().handler("custom_handler");
    /// ```
    pub fn handler<S: Into<String>>(mut self, handler: S) -> Self {
        self.handler = handler.into();
        self
    }

    /// Set the commit_type for the query. Default is CommitType::Hard.
    /// # Examples
    /// ```no_run
    /// use solrstice::CommitType;
    /// use solrstice::DeleteQuery;
    /// let builder = DeleteQuery::new().commit_type(CommitType::Soft);
    /// ```
    pub fn commit_type(mut self, commit_type: CommitType) -> Self {
        self.commit_type = commit_type;
        self
    }

    /// Set the ids to delete
    /// # Examples
    /// ```no_run
    /// use solrstice::DeleteQuery;
    /// let builder = DeleteQuery::new().ids(["document1", "document2"]);
    /// ```
    pub fn ids<S: Into<String>, V: IntoIterator<Item = S>, O: Into<Option<V>>>(
        mut self,
        ids: O,
    ) -> Self {
        self.ids = ids
            .into()
            .map(|x| x.into_iter().map(|x| x.into()).collect());
        self
    }

    /// Set the queries to delete
    /// # Examples
    /// ```no_run
    /// use solrstice::DeleteQuery;
    /// let builder = DeleteQuery::new().queries(["age:[* TO *]"]);
    /// ```
    pub fn queries<S: Into<String>, V: IntoIterator<Item = S>, O: Into<Option<V>>>(
        mut self,
        queries: O,
    ) -> Self {
        self.queries = queries
            .into()
            .map(|x| x.into_iter().map(|x| x.into()).collect());
        self
    }

    /// Execute the query.
    ///
    /// This is not meant to be used directly. Use [AsyncSolrCloudClient::delete](crate::clients::async_cloud_client::AsyncSolrCloudClient::delete) instead.
    pub async fn execute<C: AsRef<SolrServerContext>, S: AsRef<str>>(
        &self,
        context: C,
        collection: S,
    ) -> Result<SolrResponse, Error> {
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

        let mut query_params = vec![("overwrite", "true")];
        match self.commit_type {
            CommitType::Hard => query_params.push(("commit", "true")),
            CommitType::Soft => query_params.push(("softCommit", "true")),
        }

        SolrRequestBuilder::new(
            context.as_ref(),
            format!("/solr/{}/{}", &collection.as_ref(), &self.handler).as_str(),
        )
        .with_query_params(query_params.as_ref())
        .with_headers(vec![("Content-Type", "application/xml")])
        .send_post_with_body(format!(
            "<delete>{}{}</delete>",
            ids.unwrap_or_default(),
            queries.unwrap_or_default()
        ))
        .await
    }
}

#[cfg(feature = "blocking")]
use crate::runtime::RUNTIME;
#[cfg(feature = "blocking")]
impl UpdateQuery {
    /// Execute the query.
    ///
    /// This is not meant to be used directly. Use [BlockingSolrCloudClient::index](crate::clients::blocking_cloud_client::BlockingSolrCloudClient::index) instead.
    pub fn execute_blocking<D: Serialize, C: AsRef<SolrServerContext>, S: AsRef<str>>(
        &self,
        context: C,
        collection: S,
        data: &[D],
    ) -> Result<SolrResponse, Error> {
        RUNTIME
            .handle()
            .block_on(self.execute(context, collection, data))
    }
}
#[cfg(feature = "blocking")]
impl DeleteQuery {
    /// Execute the query.
    ///
    /// This is not meant to be used directly. Use [BlockingSolrCloudClient::delete](crate::clients::blocking_cloud_client::BlockingSolrCloudClient::delete) instead.
    pub fn execute_blocking<C: AsRef<SolrServerContext>, S: AsRef<str>>(
        &self,
        context: C,
        collection: S,
    ) -> Result<SolrResponse, Error> {
        RUNTIME.handle().block_on(self.execute(context, collection))
    }
}
