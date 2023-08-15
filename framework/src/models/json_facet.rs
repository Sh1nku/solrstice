use serde::de::{DeserializeOwned, Error};
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct JsonFacetResponse {
    pub count: usize,
    pub buckets: Vec<JsonFacetBucket>,
    #[serde(flatten)]
    pub flat_facets: HashMap<String, serde_json::Value>,
    pub nested_facets: HashMap<String, JsonFacetResponse>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct JsonFacetBucket {
    pub val: serde_json::Value,
    pub count: usize,
}

impl<'de> Deserialize<'de> for JsonFacetResponse {
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
            .and_then(|b| serde_json::from_value::<Vec<JsonFacetBucket>>(b).ok())
            .unwrap_or_default();

        let mut flat_facets = HashMap::new();
        let nested_facets: HashMap<String, JsonFacetResponse> = map
            .drain()
            .filter_map(|(key, value)| {
                match serde_json::from_value::<JsonFacetResponse>(value.clone()) {
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

impl JsonFacetResponse {
    pub fn get_buckets(&self) -> &Vec<JsonFacetBucket> {
        &self.buckets
    }

    pub fn get_flat_facets(&self) -> &HashMap<String, serde_json::Value> {
        &self.flat_facets
    }

    pub fn get_nested_facets(&self) -> &HashMap<String, JsonFacetResponse> {
        &self.nested_facets
    }

    pub fn get_count(&self) -> usize {
        self.count
    }
}

impl JsonFacetBucket {
    pub fn get_value<T: DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_value::<T>(self.val.clone())
    }

    pub fn get_count(&self) -> usize {
        self.count
    }
}
