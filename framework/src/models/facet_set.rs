use crate::error::Error;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct SolrFacetSetResult {
    #[serde(rename = "facet_queries", default)]
    queries: HashMap<String, usize>,
    #[serde(rename = "facet_pivot", default)]
    pivots: HashMap<String, Vec<SolrPivotFacetResult>>,
    #[serde(
        rename = "facet_fields",
        default,
        deserialize_with = "fields_deserializer"
    )]
    fields: HashMap<String, Vec<SolrFieldFacetResult>>,
}

fn fields_deserializer<'de, D>(
    deserializer: D,
) -> Result<HashMap<String, Vec<SolrFieldFacetResult>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    /*
           "facet_fields": {
           "age": [
               "20",
               2,
               "40",
               2,
               "60",
               2
           ]
       },
    */
    let mut map = HashMap::new();
    let mut raw_map: HashMap<String, Vec<serde_json::Value>> =
        serde::Deserialize::deserialize(deserializer)?;
    for (key, value) in raw_map.drain() {
        let mut field_facets = Vec::new();
        // Get in pairs
        for i in 0..value.len() / 2 {
            let key = value
                .get(i * 2)
                .ok_or(serde::de::Error::custom(format!(
                    "Non-Conformant value while deserializing facet field {}",
                    key
                )))?
                .clone();
            let count = serde_json::from_value::<usize>(
                value
                    .get(i * 2 + 1)
                    .ok_or(serde::de::Error::custom(format!(
                        "Non-Conformant value while deserializing facet field {}",
                        key
                    )))?
                    .clone(),
            )
            .map_err(|e| {
                serde::de::Error::custom(format!("Error deserializing field facet: {}", e))
            })?;
            field_facets.push(SolrFieldFacetResult { key, count });
        }
        map.insert(key, field_facets);
    }
    Ok(map)
}

impl SolrFacetSetResult {
    pub fn get_queries(&self) -> &HashMap<String, usize> {
        &self.queries
    }

    pub fn get_pivots(&self) -> &HashMap<String, Vec<SolrPivotFacetResult>> {
        &self.pivots
    }

    pub fn get_fields(&self) -> &HashMap<String, Vec<SolrFieldFacetResult>> {
        &self.fields
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SolrPivotFacetResult {
    field: String,
    value: serde_json::Value,
    count: usize,
    #[serde(rename = "pivot", default)]
    pivots: Vec<SolrPivotFacetResult>,
    #[serde(default)]
    queries: HashMap<String, usize>,
}

impl SolrPivotFacetResult {
    pub fn get_value<T: DeserializeOwned>(&self) -> Result<T, Error> {
        Ok(serde_json::from_value::<T>(self.value.clone())?)
    }

    pub fn get_pivots(&self) -> &[SolrPivotFacetResult] {
        self.pivots.as_slice()
    }

    pub fn get_queries(&self) -> &HashMap<String, usize> {
        &self.queries
    }

    pub fn get_count(&self) -> usize {
        self.count
    }

    pub fn get_field(&self) -> &str {
        &self.field
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SolrFieldFacetResult {
    key: serde_json::Value,
    count: usize,
}

impl SolrFieldFacetResult {
    pub fn get_key<T: DeserializeOwned>(&self) -> Result<T, Error> {
        Ok(serde_json::from_value::<T>(self.key.clone())?)
    }

    pub fn get_count(&self) -> usize {
        self.count
    }
}
