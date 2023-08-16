use crate::models::context::SolrServerContext;
use crate::models::error::{try_solr_error, SolrError};
use crate::models::response::SolrResponse;
use crate::queries::components::facetset::FacetSetComponentBuilder;
use crate::queries::components::grouping::GroupingComponentBuilder;
use crate::queries::components::json_facet::JsonFacetComponentBuilder;
use crate::queries::def_type::DefType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
struct PostQueryWrapper {
    pub params: SelectQueryBuilder,
}

/// Builder for a select query.
///
/// Also take a look at [AsyncSolrCloudClient::select](crate::clients::async_cloud_client::AsyncSolrCloudClient::select)
/// ```rust
///     use solrstice::queries::select::SelectQueryBuilder;
///     SelectQueryBuilder::new().fq(&["field1:val1", "field2:val2"]).q("*:*").rows(10).start(0);
/// ```
#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub struct SelectQueryBuilder {
    pub q: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fq: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fl: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<String>>,
    pub handle: String,
    pub rows: usize,
    pub start: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cursorMark")]
    pub cursor_mark: Option<String>,
    #[serde(flatten)]
    pub grouping: Option<GroupingComponentBuilder>,
    #[serde(flatten)]
    pub def_type: Option<DefType>,
    #[serde(flatten)]
    pub facetset: Option<FacetSetComponentBuilder>,
    #[serde(flatten)]
    pub json_facet: Option<JsonFacetComponentBuilder>,
}

impl SelectQueryBuilder {
    /// Builder for a select query.
    ///
    /// Also take a look at [AsyncSolrCloudClient::select](crate::clients::async_cloud_client::AsyncSolrCloudClient::select)
    /// ```rust
    ///     use solrstice::queries::select::SelectQueryBuilder;
    ///     SelectQueryBuilder::new().fq(&["field1:val1", "field2:val2"]).q("*:*").rows(10).start(0);
    /// ```
    pub fn new() -> Self {
        SelectQueryBuilder {
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
    pub fn q(mut self, q: &str) -> Self {
        self.q = q.to_string();
        self
    }

    /// A list of filter queries
    /// ```rust
    /// use solrstice::queries::select::SelectQueryBuilder;
    /// SelectQueryBuilder::new().fq(&["id:1"]);
    /// ```
    pub fn fq(mut self, queries: &[&str]) -> Self {
        self.fq = Some(queries.into_iter().map(|x| x.to_string()).collect());
        self
    }

    /// Set the fields to return
    /// ```rust
    /// use solrstice::queries::select::SelectQueryBuilder;
    /// SelectQueryBuilder::new().fl(&["field1", "field2"]);
    /// ```
    pub fn fl(mut self, fields: &[&str]) -> Self {
        self.fl = Some(fields.into_iter().map(|x| x.to_string()).collect());
        self
    }

    ///Set the sort order
    ///```rust
    /// use solrstice::queries::select::SelectQueryBuilder;
    /// SelectQueryBuilder::new().sort(&["id asc", "field1 desc"]);
    /// ```
    pub fn sort(mut self, sort: &[&str]) -> Self {
        self.sort = Some(sort.into_iter().map(|x| x.to_string()).collect());
        self
    }

    /// How many rows to return
    /// ```rust
    /// use solrstice::queries::select::SelectQueryBuilder;
    /// SelectQueryBuilder::new().rows(1000);
    /// ```
    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = rows;
        self
    }

    /// The offset to start from
    /// ```rust
    /// use solrstice::queries::select::SelectQueryBuilder;
    /// SelectQueryBuilder::new().start(10);
    /// ```
    pub fn start(mut self, start: usize) -> Self {
        self.start = start;
        self
    }

    /// Use a cursor mark to iterate over the results
    /// Default starts with "*", and which causes [SolrResponse::next_cursor_mark](crate::models::response::SolrResponse::next_cursor_mark) to be set. And can be provided for the next select.
    /// ```no_run
    /// use solrstice::queries::select::SelectQueryBuilder;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # use solrstice::clients::async_cloud_client;
    /// use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AsyncSolrCloudClient::new(SolrServerContextBuilder::new(SolrSingleServerHost::new("localhost:8983")).build());
    /// let mut builder = SelectQueryBuilder::new().cursor_mark("*");
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
    pub fn cursor_mark(mut self, cursor_mark: &str) -> Self {
        self.cursor_mark = Some(cursor_mark.to_string());
        self
    }

    /// Do a grouping query. Also take a look at [SolrGroupResult](crate::models::group::SolrGroupResult) and [SolrGroupFieldResult](crate::models::group::SolrGroupFieldResult)
    /// # Examples
    /// ```no_run
    /// # use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// use solrstice::queries::components::grouping::GroupingComponentBuilder;
    /// use solrstice::queries::select::SelectQueryBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AsyncSolrCloudClient::new(SolrServerContextBuilder::new(SolrSingleServerHost::new("localhost:8983")).build());
    /// let builder = SelectQueryBuilder::new()
    ///     .grouping(
    ///         &GroupingComponentBuilder::new()
    ///             .queries(&["age:[0 TO 59]", "age:[60 TO *]"])
    ///             .limit(10),
    ///     );
    /// let response = client.select(&builder, "collection").await?;
    /// let groups = response.get_groups().ok_or("No groups")?;
    /// let queries = groups.get("age:[0 TO 59]").ok_or("Missing group")?.get_query_result().ok_or("Missing query result")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn grouping(mut self, grouping: &GroupingComponentBuilder) -> Self {
        self.grouping = Some(grouping.clone());
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
    /// use solrstice::queries::components::grouping::GroupingComponentBuilder;
    /// use solrstice::queries::def_type::{DefType, EdismaxQueryBuilder};
    /// use solrstice::queries::select::SelectQueryBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AsyncSolrCloudClient::new(SolrServerContextBuilder::new(SolrSingleServerHost::new("localhost:8983")).build());
    /// let builder = SelectQueryBuilder::new()
    ///     .q("outdoors")
    ///     .def_type(&DefType::Edismax(EdismaxQueryBuilder::new().qf("interests^20").bq(&["interests:cars^20"])));
    /// let response = client.select(&builder, "collection").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn def_type(mut self, def_type: &DefType) -> Self {
        self.def_type = Some(def_type.clone());
        self
    }

    pub fn facetset(mut self, facetset: &FacetSetComponentBuilder) -> Self {
        self.facetset = Some(facetset.clone());
        self
    }

    pub fn json_facet(mut self, json_facet: &JsonFacetComponentBuilder) -> Self {
        self.json_facet = Some(json_facet.clone());
        self
    }

    pub async fn execute(
        &self,
        builder: &SolrServerContext,
        collection: &str,
    ) -> Result<SolrResponse, SolrError> {
        let solr_url = format!(
            "{}/solr/{}/{}",
            builder.host.get_solr_node().await?,
            collection,
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
impl SelectQueryBuilder {
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
    use crate::queries::components::grouping::GroupingComponentBuilder;
    use crate::queries::select::SelectQueryBuilder;

    #[test]
    pub fn serialize_select_query_builder_works() {
        let builder = SelectQueryBuilder::new().fq(&["id:1", "id:2"]).grouping(
            &GroupingComponentBuilder::new()
                .queries(&["id:1", "id:2"])
                .fields(&["id", "name"])
                .limit(10),
        );
        let serialized = serde_json::to_string(&builder).unwrap();
        let deserialized = serde_json::from_str::<SelectQueryBuilder>(&serialized).unwrap();
        assert_eq!(builder, deserialized);
    }
}
