use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct JsonFacetComponentBuilder {
    #[serde(rename = "json.facet", serialize_with = "json_facet_as_string")]
    pub facet: HashMap<String, JsonFacetType>,
}

fn json_facet_as_string<S>(
    facet: &HashMap<String, JsonFacetType>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let json_string = serde_json::to_string(facet).map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&json_string)
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum JsonFacetType {
    Terms(Box<JsonTermsFacet>),
    Query(Box<JsonQueryFacet>),
    StringQuery(String),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct JsonTermsFacet {
    #[serde(rename = "type")]
    type_: String,
    field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    facet: Option<HashMap<String, JsonFacetType>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct JsonQueryFacet {
    #[serde(rename = "type")]
    type_: String,
    q: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fq: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    facet: Option<HashMap<String, JsonFacetType>>,
}

impl JsonFacetComponentBuilder {
    pub fn new() -> Self {
        JsonFacetComponentBuilder {
            facet: Default::default(),
        }
    }

    pub fn add_facet(mut self, name: &str, facet: JsonFacetType) -> Self {
        self.facet.insert(name.to_string(), facet);
        self
    }
}

impl Default for JsonFacetComponentBuilder {
    fn default() -> Self {
        JsonFacetComponentBuilder::new()
    }
}

impl From<JsonQueryFacet> for JsonFacetType {
    fn from(facet: JsonQueryFacet) -> Self {
        JsonFacetType::Query(Box::new(facet))
    }
}

impl JsonTermsFacet {
    pub fn new(field: &str) -> Self {
        JsonTermsFacet {
            type_: "terms".to_string(),
            field: field.to_string(),
            offset: None,
            limit: None,
            sort: None,
            facet: None,
        }
    }

    pub fn set_offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn set_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn set_sort(mut self, sort: &str) -> Self {
        self.sort = Some(sort.to_string());
        self
    }

    pub fn set_facets(mut self, facets: HashMap<String, JsonFacetType>) -> Self {
        self.facet = Some(facets);
        self
    }
}

impl JsonQueryFacet {
    pub fn new(q: &str) -> Self {
        JsonQueryFacet {
            type_: "query".to_string(),
            q: q.to_string(),
            limit: None,
            offset: None,
            sort: None,
            fq: None,
            facet: None,
        }
    }

    pub fn set_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn set_offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn set_sort(mut self, sort: &str) -> Self {
        self.sort = Some(sort.to_string());
        self
    }

    pub fn set_fq(mut self, fq: Vec<String>) -> Self {
        self.fq = Some(fq);
        self
    }

    pub fn set_facets(mut self, facets: HashMap<String, JsonFacetType>) -> Self {
        self.facet = Some(facets);
        self
    }

    pub fn add_facet(mut self, name: &str, facet: JsonFacetType) -> Self {
        if let Some(facets) = self.facet.as_mut() {
            facets.insert(name.to_string(), facet);
        } else {
            let mut facets = HashMap::new();
            facets.insert(name.to_string(), facet);
            self.facet = Some(facets);
        }
        self
    }
}