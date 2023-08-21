use crate::models::error::SolrError;
use serde::de::{DeserializeOwned, Error};
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct SolrJsonFacetResponse {
    count: usize,
    buckets: Vec<SolrJsonFacetBucketResponse>,
    #[serde(flatten)]
    flat_facets: HashMap<String, serde_json::Value>,
    nested_facets: HashMap<String, SolrJsonFacetResponse>,
}

impl SolrJsonFacetResponse {
    pub fn get_buckets(&self) -> impl Iterator<Item = &SolrJsonFacetBucketResponse> {
        self.buckets.iter()
    }

    pub fn get_flat_facets(&self) -> &HashMap<String, serde_json::Value> {
        &self.flat_facets
    }

    pub fn get_nested_facets(&self) -> &HashMap<String, SolrJsonFacetResponse> {
        &self.nested_facets
    }

    pub fn get_count(&self) -> usize {
        self.count
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SolrJsonFacetBucketResponse {
    val: serde_json::Value,
    count: usize,
}

impl SolrJsonFacetBucketResponse {
    pub fn get_value<T: DeserializeOwned>(&self) -> Result<T, SolrError> {
        serde_json::from_value::<T>(self.val.clone()).map_err(SolrError::from)
    }

    pub fn get_count(&self) -> usize {
        self.count
    }
}

impl<'de> Deserialize<'de> for SolrJsonFacetResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = HashMap::<String, serde_json::Value>::deserialize(deserializer)?;

        let count = map
            .remove("count")
            .and_then(|v| v.as_u64().map(|u| u as usize))
            .ok_or_else(|| D::Error::missing_field("count"))?;

        let buckets = map
            .remove("buckets")
            .and_then(|b| serde_json::from_value::<Vec<SolrJsonFacetBucketResponse>>(b).ok())
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
            count,
            buckets,
            flat_facets,
            nested_facets,
        })
    }
}
