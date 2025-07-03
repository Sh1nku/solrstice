use crate::Error;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SolrStatsResult {
    stats_fields: HashMap<String, SolrStatsFieldResult>,
}

impl SolrStatsResult {
    pub fn get_fields(&self) -> &HashMap<String, SolrStatsFieldResult> {
        &self.stats_fields
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SolrStatsFieldResult {
    min: Value,
    max: Value,
    count: u64,
    missing: u64,
    sum: Option<f64>,
    mean: Option<Value>,
    #[serde(rename = "sumOfSquares")]
    sum_of_squares: Option<f64>,
    stddev: Option<f64>,
}

impl SolrStatsFieldResult {
    pub fn get_min<T: DeserializeOwned>(&self) -> Result<T, Error> {
        Ok(serde_json::from_value::<T>(self.min.clone())?)
    }

    pub fn get_max<T: DeserializeOwned>(&self) -> Result<T, Error> {
        Ok(serde_json::from_value::<T>(self.max.clone())?)
    }

    pub fn get_count(&self) -> u64 {
        self.count
    }

    pub fn get_missing(&self) -> u64 {
        self.missing
    }

    pub fn get_sum(&self) -> Option<f64> {
        self.sum
    }

    pub fn get_mean<T: DeserializeOwned>(&self) -> Option<Result<T, Error>> {
        self.mean
            .as_ref()
            .map(|mean| serde_json::from_value::<T>(mean.clone()).map_err(|e| Error::from(e)))
    }

    pub fn get_sum_of_squares(&self) -> Option<f64> {
        self.sum_of_squares
    }

    pub fn get_stddev(&self) -> Option<f64> {
        self.stddev
    }
}
