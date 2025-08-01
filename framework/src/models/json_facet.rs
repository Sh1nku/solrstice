use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Get self defined facets.
/// # Examples
/// ```no_run
/// # use solrstice::{AsyncSolrCloudClient, JsonFacetComponent, JsonQueryFacet, SelectQuery, SolrServerContextBuilder, SolrSingleServerHost};
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// # let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
/// let client = AsyncSolrCloudClient::new(context);
///  let query = SelectQuery::new().json_facet(
///     JsonFacetComponent::new().facets([("below_60", JsonQueryFacet::new().q("age:[0 TO 59]"))]),
/// );
/// let response = client
///     .select(&query, "collection_name")
///     .await?;
/// let facets = response.get_json_facets().ok_or("No facets")?;
/// let below_60 = facets
///     .get_nested_facets()
///     .get("below_60")
///     .ok_or("No below_60 facet")?;
/// assert_eq!(below_60.get_count().ok_or("No count")?, 4);
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct SolrJsonFacetResponse {
    val: Option<Value>,
    count: Option<usize>,
    num_buckets: Option<usize>,
    missing: Option<Box<SolrJsonFacetResponse>>,
    all_buckets: Option<Box<SolrJsonFacetResponse>>,
    #[serde(default)]
    buckets: Vec<SolrJsonFacetResponse>,
    #[serde(flatten)]
    flat_facets: HashMap<String, Value>,
    #[serde(default)]
    nested_facets: HashMap<String, SolrJsonFacetResponse>,
}

impl SolrJsonFacetResponse {
    /// Returned if the facet is a bucket.
    pub fn get_val(&self) -> Option<&Value> {
        self.val.as_ref()
    }

    /// Get buckets of the facet.
    pub fn get_buckets(&self) -> impl Iterator<Item = &SolrJsonFacetResponse> {
        self.buckets.iter()
    }

    /// Get the number of buckets if specified with `numBuckets`.
    pub fn get_num_buckets(&self) -> Option<usize> {
        self.num_buckets
    }

    /// Get the missing special facet, populated if it was specified in the query.
    pub fn get_missing(&self) -> Option<usize> {
        self.missing.as_deref()?.get_count()
    }

    /// Get the all buckets special facet, populated if it was specified in the query.
    pub fn get_all_buckets(&self) -> Option<usize> {
        self.all_buckets.as_deref()?.get_count()
    }

    /// Get flat facets.
    pub fn get_flat_facets(&self) -> &HashMap<String, Value> {
        &self.flat_facets
    }

    /// Get nested facets.
    pub fn get_nested_facets(&self) -> &HashMap<String, SolrJsonFacetResponse> {
        &self.nested_facets
    }

    /// Get count of the facet.
    pub fn get_count(&self) -> Option<usize> {
        self.count
    }
}

impl<'de> Deserialize<'de> for SolrJsonFacetResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = HashMap::<String, Value>::deserialize(deserializer)?;

        let count = map
            .remove("count")
            .and_then(|v| v.as_u64().map(|u| u as usize));

        let val = map.remove("val");

        let num_buckets = map
            .remove("numBuckets")
            .and_then(|v| v.as_u64().map(|u| u as usize));
        let missing = map
            .remove("missing")
            .and_then(|v| serde_json::from_value::<SolrJsonFacetResponse>(v).ok());

        let all_buckets = map
            .remove("allBuckets")
            .and_then(|v| serde_json::from_value::<SolrJsonFacetResponse>(v).ok());

        let buckets = map
            .remove("buckets")
            .and_then(|b| serde_json::from_value::<Vec<SolrJsonFacetResponse>>(b).ok())
            .unwrap_or_default();

        let mut flat_facets = HashMap::new();
        let nested_facets: HashMap<String, SolrJsonFacetResponse> = map
            .drain()
            .filter_map(|(key, value)| {
                match serde_json::from_value::<SolrJsonFacetResponse>(value.clone()) {
                    Ok(v) => Some((key, v)),
                    Err(_) => {
                        flat_facets.insert(key, value);
                        None
                    }
                }
            })
            .collect();

        Ok(Self {
            val,
            count,
            num_buckets,
            missing: missing.map(Box::new),
            all_buckets: all_buckets.map(Box::new),
            buckets,
            flat_facets,
            nested_facets,
        })
    }
}
