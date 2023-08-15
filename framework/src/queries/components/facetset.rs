use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FacetSetComponentBuilder {
    pub facet: bool,
    #[serde(rename = "facet.query")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queries: Option<Vec<String>>,
    // #[serde(rename = "facet.field")]
    // pub fields: Option<Vec<String>>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pivots: Option<PivotFacetComponentBuilder>,
}

impl FacetSetComponentBuilder {
    pub fn new() -> Self {
        FacetSetComponentBuilder {
            facet: true,
            queries: None,
            pivots: None,
        }
    }

    pub fn pivots(mut self, pivots: &PivotFacetComponentBuilder) -> Self {
        self.pivots = Some(pivots.clone());
        self
    }

    pub fn queries(mut self, queries: &[&str]) -> Self {
        self.queries = Some(queries.iter().map(|s| s.to_string()).collect());
        self
    }
}

impl Default for FacetSetComponentBuilder {
    fn default() -> Self {
        FacetSetComponentBuilder::new()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PivotFacetComponentBuilder {
    /// The field to facet on.
    #[serde(rename = "facet.pivot")]
    pub pivots: Vec<String>,
    /// The minimum count for a facet to be returned. Default is 1.
    #[serde(rename = "facet.pivot.mincount")]
    pub min_count: Option<usize>,
}

impl PivotFacetComponentBuilder {
    pub fn new(pivots: &[&str]) -> Self {
        PivotFacetComponentBuilder {
            pivots: pivots.iter().map(|s| s.to_string()).collect(),
            min_count: None,
        }
    }

    pub fn min_count(mut self, min_count: usize) -> Self {
        self.min_count = Some(min_count);
        self
    }
}
