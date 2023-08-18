use crate::models::context::SolrServerContext;
use crate::models::error::{try_solr_error, SolrError};
use crate::models::response::SolrResponse;
use crate::queries::components::facetset::FacetSetComponent;
use crate::queries::components::grouping::GroupingComponent;
use crate::queries::components::json_facet::JsonFacetComponent;
use crate::queries::def_type::DefType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
struct PostQueryWrapper {
    pub params: SelectQuery,
}

/// Builder for a select query.
///
/// Also take a look at [AsyncSolrCloudClient::select](crate::clients::async_cloud_client::AsyncSolrCloudClient::select)
/// ```rust
///     use solrstice::queries::select::SelectQuery;
///     SelectQuery::new().fq(&["field1:val1", "field2:val2"]).q("*:*").rows(10).start(0);
/// ```
#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub struct SelectQuery {
    q: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    fq: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fl: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<Vec<String>>,
    handle: String,
    rows: usize,
    start: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cursorMark")]
    cursor_mark: Option<String>,
    #[serde(flatten)]
    grouping: Option<GroupingComponent>,
    #[serde(flatten)]
    def_type: Option<DefType>,
    #[serde(flatten)]
    facetset: Option<FacetSetComponent>,
    #[serde(flatten)]
    json_facet: Option<JsonFacetComponent>,
}

impl SelectQuery {
    /// Builder for a select query.
    ///
    /// Also take a look at [AsyncSolrCloudClient::select](crate::clients::async_cloud_client::AsyncSolrCloudClient::select)
    /// ```rust
    ///     use solrstice::queries::select::SelectQuery;
    ///     SelectQuery::new().fq(&["field1:val1", "field2:val2"]).q("*:*").rows(10).start(0);
    /// ```
    pub fn new() -> Self {
        SelectQuery {
            q: "*:*".to_string(),
            fq: None,
            fl: None,
            sort: None,
            handle: "select".to_string(),
            rows: 10,
            start: 0,
            cursor_mark: None,
            grouping: None,
            def_type: None,
            facetset: None,
            json_facet: None,
        }
    }

    /// Set the q parameter. Default is "*:*"
    pub fn q<T: AsRef<str>>(mut self, q: T) -> Self {
        self.q = q.as_ref().to_string();
        self
    }

    /// A list of filter queries
    /// ```rust
    /// use solrstice::queries::select::SelectQuery;
    /// SelectQuery::new().fq(&["id:1"]);
    /// ```
    pub fn fq<T: AsRef<str>>(mut self, queries: &[T]) -> Self {
        self.fq = Some(queries.iter().map(|x| x.as_ref().to_string()).collect());
        self
    }

    /// Set the fields to return
    /// ```rust
    /// use solrstice::queries::select::SelectQuery;
    /// SelectQuery::new().fl(&["field1", "field2"]);
    /// ```
    pub fn fl<T: AsRef<str>>(mut self, fields: &[T]) -> Self {
        self.fl = Some(fields.iter().map(|x| x.as_ref().to_string()).collect());
        self
    }

    ///Set the sort order
    ///```rust
    /// use solrstice::queries::select::SelectQuery;
    /// SelectQuery::new().sort(&["id asc", "field1 desc"]);
    /// ```
    pub fn sort<T: AsRef<str>>(mut self, sort: &[T]) -> Self {
        self.sort = Some(sort.iter().map(|x| x.as_ref().to_string()).collect());
        self
    }

    /// How many rows to return
    /// ```rust
    /// use solrstice::queries::select::SelectQuery;
    /// SelectQuery::new().rows(1000);
    /// ```
    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = rows;
        self
    }

    /// The offset to start from
    /// ```rust
    /// use solrstice::queries::select::SelectQuery;
    /// SelectQuery::new().start(10);
    /// ```
    pub fn start(mut self, start: usize) -> Self {
        self.start = start;
        self
    }

    /// Use a cursor mark to iterate over the results
    /// Default starts with "*", and which causes [SolrResponse::next_cursor_mark](crate::models::response::SolrResponse::next_cursor_mark) to be set. And can be provided for the next select.
    /// ```no_run
    /// use solrstice::queries::select::SelectQuery;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # use solrstice::clients::async_cloud_client;
    /// use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AsyncSolrCloudClient::new(SolrServerContextBuilder::new(SolrSingleServerHost::new("localhost:8983")).build());
    /// let mut builder = SelectQuery::new().cursor_mark("*");
    /// let response = client.select(&builder, "collection").await?;
    /// let mut cursor_mark = response.next_cursor_mark.ok_or("No cursor mark")?;
    /// loop {
    ///     if cursor_mark == "*" {
    ///         break;
    ///     }
    ///     else {
    ///         builder = builder.cursor_mark(&cursor_mark);
    ///         let response = client.select(&builder, "collection").await?;
    ///         cursor_mark = response.next_cursor_mark.ok_or("No cursor mark")?;
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn cursor_mark<T: AsRef<str>>(mut self, cursor_mark: T) -> Self {
        self.cursor_mark = Some(cursor_mark.as_ref().to_string());
        self
    }

    /// Do a grouping query. Also take a look at [SolrGroupResult](crate::models::group::SolrGroupResult) and [SolrGroupFieldResult](crate::models::group::SolrGroupFieldResult)
    /// # Examples
    /// ```no_run
    /// # use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// use solrstice::queries::components::grouping::GroupingComponent;
    /// use solrstice::queries::select::SelectQuery;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AsyncSolrCloudClient::new(SolrServerContextBuilder::new(SolrSingleServerHost::new("localhost:8983")).build());
    /// let builder = SelectQuery::new()
    ///     .grouping(
    ///         &GroupingComponent::new()
    ///             .queries(&["age:[0 TO 59]", "age:[60 TO *]"])
    ///             .limit(10),
    ///     );
    /// let response = client.select(&builder, "collection").await?;
    /// let groups = response.get_groups().ok_or("No groups")?;
    /// let queries = groups.get("age:[0 TO 59]").ok_or("Missing group")?.get_query_result().ok_or("Missing query result")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn grouping<T: AsRef<GroupingComponent>>(mut self, grouping: T) -> Self {
        self.grouping = Some(grouping.as_ref().clone());
        self
    }

    /// Specify an alternate query parser. Default is "lucene", but can also be "dismax" or "edismax"
    ///
    /// Note. The default q parameter is *:*, which will not work on `dismax` or `edismax`. So you need to specify a query.
    /// # Examples
    /// ```no_run
    /// # use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// use solrstice::queries::components::grouping::GroupingComponent;
    /// use solrstice::queries::def_type::{DefType, EdismaxQuery};
    /// use solrstice::queries::select::SelectQuery;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AsyncSolrCloudClient::new(SolrServerContextBuilder::new(SolrSingleServerHost::new("localhost:8983")).build());
    /// let builder = SelectQuery::new()
    ///     .q("outdoors")
    ///     .def_type(&DefType::Edismax(EdismaxQuery::new().qf("interests^20").bq(&["interests:cars^20"])));
    /// let response = client.select(&builder, "collection").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn def_type<T: Into<DefType>>(mut self, def_type: T) -> Self {
        self.def_type = Some(def_type.into());
        self
    }

    pub fn facetset<T: AsRef<FacetSetComponent>>(mut self, facetset: T) -> Self {
        self.facetset = Some(facetset.as_ref().clone());
        self
    }

    pub fn json_facet<T: AsRef<JsonFacetComponent>>(mut self, json_facet: T) -> Self {
        self.json_facet = Some(json_facet.as_ref().clone());
        self
    }

    pub async fn execute<T: AsRef<str>>(
        &self,
        builder: &SolrServerContext,
        collection: T,
    ) -> Result<SolrResponse, SolrError> {
        let solr_url = format!(
            "{}/solr/{}/{}",
            builder.host.get_solr_node().await?,
            collection.as_ref(),
            &self.handle
        );
        let wrapper = PostQueryWrapper {
            params: self.clone(),
        };
        let mut request = builder
            .client
            .post(&solr_url)
            .json::<PostQueryWrapper>(&wrapper);
        if let Some(auth) = &builder.auth {
            request = auth.add_auth_to_request(request);
        }
        let data = request.send().await?.json::<SolrResponse>().await?;
        try_solr_error(&data)?;
        Ok(data)
    }
}

#[cfg(feature = "blocking")]
use crate::runtime::RUNTIME;
#[cfg(feature = "blocking")]
impl SelectQuery {
    pub fn execute_blocking(
        &self,
        builder: &SolrServerContext,
        collection: &str,
    ) -> Result<SolrResponse, SolrError> {
        RUNTIME.handle().block_on(self.execute(builder, collection))
    }
}

#[cfg(test)]
pub mod tests {
    use crate::queries::components::grouping::GroupingComponent;
    use crate::queries::select::SelectQuery;

    #[test]
    pub fn serialize_select_query_builder_works() {
        let builder = SelectQuery::new().fq(&["id:1", "id:2"]).grouping(
            &GroupingComponent::new()
                .queries(&["id:1", "id:2"])
                .fields(&["id", "name"])
                .limit(10),
        );
        let serialized = serde_json::to_string(&builder).unwrap();
        let deserialized = serde_json::from_str::<SelectQuery>(&serialized).unwrap();
        assert_eq!(builder, deserialized);
    }
}
