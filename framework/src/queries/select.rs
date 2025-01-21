use crate::error::Error;
use crate::models::context::SolrServerContext;
use crate::models::response::SolrResponse;
use crate::queries::components::facet_set::FacetSetComponent;
use crate::queries::components::grouping::GroupingComponent;
use crate::queries::components::json_facet::JsonFacetComponent;
use crate::queries::def_type::DefType;
use crate::queries::request_builder::SolrRequestBuilder;
#[cfg(feature = "blocking")]
use crate::runtime::RUNTIME;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
struct PostQueryWrapper {
    pub params: SelectQuery,
}

fn deserialize_empty_map_as_none<'de, D>(
    deserializer: D,
) -> Result<Option<HashMap<String, Value>>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.filter(|map: &HashMap<String, Value>| !map.is_empty()))
}

/// Builder for a select query.
///
/// Also take a look at [AsyncSolrCloudClient::select](crate::AsyncSolrCloudClient::select)
/// ```rust
///     use solrstice::SelectQuery;
///     SelectQuery::new().fq(["field1:val1", "field2:val2"]).q("*:*").rows(10).start(0);
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
    facet_set: Option<FacetSetComponent>,
    #[serde(flatten)]
    json_facet: Option<JsonFacetComponent>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        flatten,
        deserialize_with = "deserialize_empty_map_as_none"
    )]
    additional_params: Option<HashMap<String, Value>>,
}

impl From<&SelectQuery> for SelectQuery {
    fn from(query: &SelectQuery) -> Self {
        query.clone()
    }
}

impl AsRef<SelectQuery> for SelectQuery {
    fn as_ref(&self) -> &SelectQuery {
        self
    }
}

impl SelectQuery {
    /// Builder for a select query.
    ///
    /// Also take a look at [AsyncSolrCloudClient::select](crate::select)
    /// ```rust
    ///     use solrstice::SelectQuery;
    ///     SelectQuery::new().fq(["field1:val1", "field2:val2"]).q("*:*").rows(10).start(0);
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
            facet_set: None,
            json_facet: None,
            additional_params: None,
        }
    }

    /// Set the q parameter. Default is "*:*"
    pub fn q<S: Into<String>>(mut self, q: S) -> Self {
        self.q = q.into();
        self
    }

    /// A list of filter queries
    /// ```rust
    /// use solrstice::SelectQuery;
    /// SelectQuery::new().fq(["id:1"]);
    /// ```
    pub fn fq<S: Into<String>, V: IntoIterator<Item = S>, O: Into<Option<V>>>(
        mut self,
        queries: O,
    ) -> Self {
        self.fq = queries
            .into()
            .map(|x| x.into_iter().map(|x| x.into()).collect());
        self
    }

    /// Set the fields to return
    /// ```rust
    /// use solrstice::SelectQuery;
    /// SelectQuery::new().fl(["field1", "field2"]);
    /// ```
    pub fn fl<S: Into<String>, V: IntoIterator<Item = S>, O: Into<Option<V>>>(
        mut self,
        fields: O,
    ) -> Self {
        self.fl = fields
            .into()
            .map(|x| x.into_iter().map(|x| x.into()).collect());
        self
    }

    ///Set the sort order
    ///```rust
    /// use solrstice::SelectQuery;
    /// SelectQuery::new().sort(["id asc", "field1 desc"]);
    /// ```
    pub fn sort<S: Into<String>, V: IntoIterator<Item = S>, O: Into<Option<V>>>(
        mut self,
        sort: O,
    ) -> Self {
        self.sort = sort
            .into()
            .map(|x| x.into_iter().map(|x| x.into()).collect());
        self
    }

    /// How many rows to return
    /// ```rust
    /// use solrstice::SelectQuery;
    /// SelectQuery::new().rows(1000);
    /// ```
    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = rows;
        self
    }

    /// The offset to start from
    /// ```rust
    /// use solrstice::SelectQuery;
    /// SelectQuery::new().start(10);
    /// ```
    pub fn start(mut self, start: usize) -> Self {
        self.start = start;
        self
    }

    /// Use a cursor mark to iterate over the results
    /// Default starts with "*", and which causes [SolrResponse::next_cursor_mark](crate::models::response::SolrResponse::next_cursor_mark) to be set. And can be provided for the next select.
    /// ```no_run
    /// use solrstice::{AsyncSolrCloudClient, SelectQuery, SolrSingleServerHost};
    /// # use solrstice::SolrServerContextBuilder;
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
    pub fn cursor_mark<S: Into<String>, O: Into<Option<S>>>(mut self, cursor_mark: O) -> Self {
        self.cursor_mark = cursor_mark.into().map(|x| x.into());
        self
    }

    /// Do a grouping query. Also take a look at [SolrGroupResult](crate::models::group::SolrGroupResult) and [SolrGroupFieldResult](crate::models::group::SolrGroupFieldResult)
    /// # Examples
    /// ```no_run
    /// use solrstice::{GroupingComponent, SelectQuery, SolrSingleServerHost};
    /// # use solrstice::AsyncSolrCloudClient;
    /// # use solrstice::SolrServerContextBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AsyncSolrCloudClient::new(SolrServerContextBuilder::new(SolrSingleServerHost::new("localhost:8983")).build());
    /// let builder = SelectQuery::new()
    ///     .grouping(
    ///         &GroupingComponent::new()
    ///             .queries(["age:[0 TO 59]", "age:[60 TO *]"])
    ///             .limit(10),
    ///     );
    /// let response = client.select(&builder, "collection").await?;
    /// let groups = response.get_groups().ok_or("No groups")?;
    /// let queries = groups.get("age:[0 TO 59]").ok_or("Missing group")?.get_query_result().ok_or("Missing query result")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn grouping<G: Into<GroupingComponent>, O: Into<Option<G>>>(mut self, grouping: O) -> Self {
        self.grouping = grouping.into().map(|x| x.into());
        self
    }

    /// Specify an alternate query parser. Default is "lucene", but can also be "dismax" or "edismax"
    ///
    /// Note. The default q parameter is *:*, which will not work on `dismax` or `edismax`. So you need to specify a query.
    /// # Examples
    /// ```no_run
    /// use solrstice::{DefType, EdismaxQuery, SelectQuery, SolrSingleServerHost};
    /// # use solrstice::AsyncSolrCloudClient;
    /// # use solrstice::SolrServerContextBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AsyncSolrCloudClient::new(SolrServerContextBuilder::new(SolrSingleServerHost::new("localhost:8983")).build());
    /// let builder = SelectQuery::new()
    ///     .q("outdoors")
    ///     .def_type(&DefType::Edismax(EdismaxQuery::new().qf("interests^20").bq(["interests:cars^20"])));
    /// let response = client.select(&builder, "collection").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn def_type<T: Into<DefType>, O: Into<Option<T>>>(mut self, def_type: O) -> Self {
        self.def_type = def_type.into().map(|x| x.into());
        self
    }

    pub fn facet_set<T: Into<FacetSetComponent>, O: Into<Option<T>>>(
        mut self,
        facet_set: O,
    ) -> Self {
        self.facet_set = facet_set.into().map(|x| x.into());
        self
    }

    pub fn json_facet<T: Into<JsonFacetComponent>, O: Into<Option<T>>>(
        mut self,
        json_facet: O,
    ) -> Self {
        self.json_facet = json_facet.into().map(|x| x.into());
        self
    }

    /// Additional parameters to send to Solr in the `params` field of the JSON Query DSL.
    /// ```json
    /// {
    ///     "params": {
    ///         "child.q": "{!term f=_nest_parent_ v=$row.id}",
    ///         "child.fl": "id,age,count,interests"
    ///     }
    /// }
    /// ```
    pub fn additional_params<T, K, V>(mut self, params: T) -> Self
    where
        T: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<Value>,
    {
        self.additional_params = Some(
            params
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        );
        self
    }

    pub async fn execute<T: AsRef<str>, C: AsRef<SolrServerContext>>(
        &self,
        context: C,
        collection: T,
    ) -> Result<SolrResponse, Error> {
        let solr_url = format!("/solr/{}/{}", collection.as_ref(), &self.handle);
        let wrapper = PostQueryWrapper {
            params: self.clone(),
        };
        let data = SolrRequestBuilder::new(context.as_ref(), solr_url.as_str())
            .send_post_with_json::<PostQueryWrapper>(&wrapper)
            .await?;
        Ok(data)
    }
}

#[cfg(feature = "blocking")]
impl SelectQuery {
    pub fn execute_blocking<C: AsRef<SolrServerContext>, S: AsRef<str>>(
        &self,
        context: C,
        collection: S,
    ) -> Result<SolrResponse, Error> {
        RUNTIME.handle().block_on(self.execute(context, collection))
    }
}

#[cfg(test)]
pub mod tests {
    use crate::queries::components::grouping::GroupingComponent;
    use crate::queries::select::SelectQuery;

    #[test]
    pub fn serialize_select_arguments_work() {
        let _ = SelectQuery::new()
            .fq(["id:1"])
            .fq(vec!["id:1"])
            .fq(vec![String::from("id:1")])
            .fq(&[String::from("id:1")]);
    }

    #[test]
    pub fn serialize_select_query_builder_works() {
        let builder = SelectQuery::new().fq(["id:1", "id:2"]).grouping(
            GroupingComponent::new()
                .queries(["id:1", "id:2"])
                .fields(["id", "name"])
                .limit(10),
        );
        let serialized = serde_json::to_string(&builder).unwrap();
        let deserialized = serde_json::from_str::<SelectQuery>(&serialized).unwrap();
        assert_eq!(builder, deserialized);
    }
}
