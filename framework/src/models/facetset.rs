use crate::models::error::SolrError;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FacetSet {
    #[serde(rename = "facet_queries", default)]
    queries: HashMap<String, usize>,
    #[serde(rename = "facet_pivot", default)]
    pivots: HashMap<String, Vec<PivotFacetResult>>,
    #[serde(rename = "facet_fields", default)]
    fields: HashMap<String, usize>,
}

impl FacetSet {
    pub fn get_queries(&self) -> &HashMap<String, usize> {
        &self.queries
    }

    pub fn get_pivots(&self) -> &HashMap<String, Vec<PivotFacetResult>> {
        &self.pivots
    }

    pub fn get_fields(&self) -> &HashMap<String, usize> {
        &self.fields
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PivotFacetResult {
    pub field: String,
    value: serde_json::Value,
    pub count: usize,
    #[serde(rename = "pivot")]
    pivots: Option<Vec<PivotFacetResult>>,
    queries: Option<HashMap<String, usize>>,
}

impl PivotFacetResult {
    pub fn get_value<T: DeserializeOwned>(&self) -> Result<T, SolrError> {
        Ok(serde_json::from_value::<T>(self.value.clone())?)
    }

    pub fn get_pivots(&self) -> Option<&Vec<PivotFacetResult>> {
        self.pivots.as_ref()
    }

    pub fn get_queries(&self) -> Option<&HashMap<String, usize>> {
        self.queries.as_ref()
    }
}
