use crate::error::Error;
use crate::models::facet_set::SolrFacetSetResult;
use crate::models::group::SolrGroupResult;
use crate::models::json_facet::SolrJsonFacetResponse;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::value::RawValue;
use std::collections::HashMap;

/// Response header given by solr, if not `responseHeader=false` is passed.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SolrResponseHeader {
    #[serde(rename = "zkConnected")]
    /// Whether or not the request was made to a Zookeeper managed Solr instance.
    pub zk_connected: Option<bool>,
    /// The status of the request. 0 if successful.
    pub status: usize,
    #[serde(rename = "QTime")]
    /// The time in milliseconds that the request took to process.
    pub q_time: usize,
}

/// If the request was not successful, this will be populated.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SolrResponseError {
    /// The message of the error.
    pub msg: Option<String>,
    /// The trace of the error.
    pub trace: Option<String>,
    /// The code of the error.
    pub code: u16,
}

fn default_true() -> bool {
    true
}

/// Documentation response from Solr. The docs are not immediately deserialized to allow for reading the other fields first.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SolrDocsResponse {
    /// The number of documents found.
    #[serde(rename = "numFound")]
    pub(crate) num_found: usize,
    /// The start index of the documents.
    pub(crate) start: usize,
    #[serde(rename = "numFoundExact")]
    /// Whether or not the number of documents found is exact. This field only exists on Solr 8.6+. On older versions, this will always be true.
    #[serde(default = "default_true")]
    pub(crate) num_found_exact: bool,
    /// The documents returned by the query. Use [`SolrDocsResponse::get_docs`] to deserialize.
    docs: Box<RawValue>,
}

impl SolrDocsResponse {
    pub fn get_num_found(&self) -> usize {
        self.num_found
    }

    pub fn get_start(&self) -> usize {
        self.start
    }

    pub fn get_num_found_exact(&self) -> bool {
        self.num_found_exact
    }

    /// Deserialize the docs returned by a select request.
    ///
    /// # Examples
    /// ```no_run
    /// use solrstice::{AsyncSolrCloudClient, SelectQuery, SolrBasicAuth, SolrServerContextBuilder, SolrSingleServerHost};
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).with_auth(SolrBasicAuth::new("solr", Some("SolrRocks"))).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// let response = client.select(&SelectQuery::new(), "collection").await?;
    /// let docs = response.get_docs_response().unwrap().get_docs::<serde_json::Value>()?;
    /// Ok(())
    /// # }
    /// ```
    pub fn get_docs<V: DeserializeOwned>(&self) -> Result<Vec<V>, Error> {
        serde_json::from_str::<Vec<V>>(self.docs.get()).map_err(|e| e.into())
    }
}

/// Represents any response Solr can give. This is the top level response.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SolrResponse {
    /// The response header given by Solr if not `responseHeader=false` is passed.
    #[serde(rename = "responseHeader")]
    pub(crate) response_header: Option<SolrResponseHeader>,
    /// The error given by Solr if the request was not successful.
    pub(crate) error: Option<SolrResponseError>,
    /// Aliases given by solr from [AsyncSolrCloudClient::get_aliases](crate::clients::async_cloud_client::AsyncSolrCloudClient::get_aliases).
    #[serde(default)]
    #[serde(deserialize_with = "from_alias")]
    pub(crate) aliases: Option<HashMap<String, Vec<String>>>,
    /// The response given by Solr on a select request
    pub(crate) response: Option<SolrDocsResponse>,
    /// The config sets that exist on the server.
    ///
    /// Returned if using [AsyncSolrCloudClient::get_configs](crate::clients::async_cloud_client::AsyncSolrCloudClient::get_configs).
    #[serde(rename = "configSets")]
    pub(crate) config_sets: Option<Vec<String>>,
    /// The collections that exist on the server.
    ///
    /// Returned if using [AsyncSolrCloudClient::get_collections](crate::clients::async_cloud_client::AsyncSolrCloudClient::get_collections).
    pub(crate) collections: Option<Vec<String>>,
    /// Grouping results returned by Solr if `group=true` is passed.
    pub(crate) grouped: Option<HashMap<String, SolrGroupResult>>,
    /// The next cursor mark returned by Solr if [SelectQuery::cursor_mark](crate::queries::select::SelectQuery::cursor_mark) is passed.
    #[serde(rename = "nextCursorMark")]
    pub next_cursor_mark: Option<String>,
    #[serde(rename = "facet_counts")]
    pub(crate) facet_set: Option<SolrFacetSetResult>,
    #[serde(rename = "facets")]
    pub(crate) json_facet: Option<SolrJsonFacetResponse>,
}

impl SolrResponse {
    /// Get the docs returned by a select request.
    /// # Examples
    /// ```no_run
    /// use solrstice::{AsyncSolrCloudClient, SelectQuery, SolrBasicAuth, SolrServerContextBuilder, SolrSingleServerHost};
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).with_auth(SolrBasicAuth::new("solr", Some("SolrRocks"))).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// let response = client.select(&SelectQuery::new(), "collection").await?;
    /// Ok(())
    /// # }
    /// ```
    pub fn get_docs_response(&self) -> Option<&SolrDocsResponse> {
        self.response.as_ref()
    }

    /// Get the groups returned by a select request using the [GroupingComponentBuilder](crate::queries::components::grouping::GroupingComponent).
    ///
    /// # Examples
    /// ```no_run
    /// # use solrstice::{AsyncSolrCloudClient, GroupingComponent, SelectQuery, SolrBasicAuth, SolrServerContextBuilder, SolrSingleServerHost};
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).with_auth(SolrBasicAuth::new("solr", Some("SolrRocks"))).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// let groups = client.select(&SelectQuery::new()
    ///     .grouping(
    ///         &GroupingComponent::new()
    ///             .queries(["age:[0 TO 59]", "age:[60 TO *]"])
    ///             .limit(10),
    ///     ), "collection").await?
    ///     .get_groups().ok_or("No groups returned")?;
    /// Ok(())
    /// # }
    /// ```
    pub fn get_groups(&self) -> Option<&HashMap<String, SolrGroupResult>> {
        self.grouped.as_ref()
    }

    pub fn get_facet_set(&self) -> Option<&SolrFacetSetResult> {
        self.facet_set.as_ref()
    }

    pub fn get_json_facets(&self) -> Option<&SolrJsonFacetResponse> {
        self.json_facet.as_ref()
    }
}

fn from_alias<'de, D>(deserializer: D) -> Result<Option<HashMap<String, Vec<String>>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value_map: Option<HashMap<String, String>> = Deserialize::deserialize(deserializer)?;
    match value_map {
        None => Ok(None),
        Some(value_map) => {
            let mut return_map: HashMap<String, Vec<String>> = HashMap::new();
            for (key, values) in value_map {
                if !values.is_empty() {
                    return_map.insert(key, values.split(',').map(|x| x.to_string()).collect());
                } else {
                    return_map.insert(key, vec![]);
                }
            }
            Ok(Some(return_map))
        }
    }
}
