use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct JsonFacetComponent {
    #[serde(rename = "json.facet", serialize_with = "json_facet_as_string")]
    pub facet: HashMap<String, JsonFacetType>,
}

impl AsRef<JsonFacetComponent> for JsonFacetComponent {
    fn as_ref(&self) -> &JsonFacetComponent {
        self
    }
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

impl JsonFacetComponent {
    pub fn new() -> Self {
        JsonFacetComponent {
            facet: Default::default(),
        }
    }

    pub fn facets<K: AsRef<str>, V: Into<JsonFacetType> + Clone>(
        mut self,
        facets: &[(K, V)],
    ) -> Self {
        self.facet = facets
            .iter()
            .map(|(name, facet)| (name.as_ref().to_string(), facet.clone().into()))
            .collect();
        self
    }
}

impl Default for JsonFacetComponent {
    fn default() -> Self {
        JsonFacetComponent::new()
    }
}

impl From<JsonQueryFacet> for JsonFacetType {
    fn from(facet: JsonQueryFacet) -> Self {
        JsonFacetType::Query(Box::new(facet))
    }
}

impl JsonTermsFacet {
    pub fn new<T: AsRef<str>>(field: T) -> Self {
        JsonTermsFacet {
            type_: "terms".to_string(),
            field: field.as_ref().to_string(),
            offset: None,
            limit: None,
            sort: None,
            facet: None,
        }
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn sort<T: AsRef<str>>(mut self, sort: T) -> Self {
        self.sort = Some(sort.as_ref().to_string());
        self
    }

    pub fn facets<K: AsRef<str>, V: Into<JsonFacetType> + Clone>(
        mut self,
        facets: &[(K, V)],
    ) -> Self {
        self.facet = Some(
            facets
                .iter()
                .map(|(name, facet)| (name.as_ref().to_string(), facet.clone().into()))
                .collect(),
        );
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

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn sort<T: AsRef<str>>(mut self, sort: T) -> Self {
        self.sort = Some(sort.as_ref().to_string());
        self
    }

    pub fn fq<T: AsRef<str>>(mut self, fq: &[T]) -> Self {
        self.fq = Some(fq.iter().map(|s| s.as_ref().to_string()).collect());
        self
    }

    pub fn facets<K: AsRef<str>, V: Into<JsonFacetType> + Clone>(
        mut self,
        facets: &[(K, V)],
    ) -> Self {
        self.facet = Some(
            facets
                .iter()
                .map(|(name, facet)| (name.as_ref().to_string(), facet.clone().into()))
                .collect(),
        );
        self
    }
}
