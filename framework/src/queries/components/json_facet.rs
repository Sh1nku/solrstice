use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct JsonFacetComponent {
    #[serde(rename = "json.facet", serialize_with = "json_facet_as_string")]
    facet: HashMap<String, JsonFacetType>,
}

impl AsRef<JsonFacetComponent> for JsonFacetComponent {
    fn as_ref(&self) -> &JsonFacetComponent {
        self
    }
}

impl From<&JsonFacetComponent> for JsonFacetComponent {
    fn from(component: &JsonFacetComponent) -> Self {
        component.clone()
    }
}

impl JsonFacetComponent {
    pub fn new() -> Self {
        JsonFacetComponent {
            facet: Default::default(),
        }
    }

    pub fn facets<K: Into<String>, V: Into<JsonFacetType>, I: IntoIterator<Item = (K, V)>>(
        mut self,
        facets: I,
    ) -> Self {
        self.facet = facets
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        self
    }
}

impl Default for JsonFacetComponent {
    fn default() -> Self {
        JsonFacetComponent::new()
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
    Stat(String),
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

impl From<JsonTermsFacet> for JsonFacetType {
    fn from(facet: JsonTermsFacet) -> Self {
        JsonFacetType::Terms(Box::new(facet))
    }
}

impl JsonTermsFacet {
    pub fn new<S: Into<String>>(field: S) -> Self {
        JsonTermsFacet {
            type_: "terms".to_string(),
            field: field.into(),
            offset: None,
            limit: None,
            sort: None,
            facet: None,
        }
    }

    pub fn offset<O: Into<Option<usize>>>(mut self, offset: O) -> Self {
        self.offset = offset.into();
        self
    }

    pub fn limit<O: Into<Option<usize>>>(mut self, limit: O) -> Self {
        self.limit = limit.into();
        self
    }

    pub fn sort<S: Into<String>, O: Into<Option<S>>>(mut self, sort: O) -> Self {
        self.sort = sort.into().map(|s| s.into());
        self
    }

    pub fn facets<
        K: Into<String>,
        V: Into<JsonFacetType>,
        I: IntoIterator<Item = (K, V)>,
        O: Into<Option<I>>,
    >(
        mut self,
        facets: O,
    ) -> Self {
        self.facet = facets.into().map(|facets| {
            facets
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect()
        });
        self
    }
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

impl From<JsonQueryFacet> for JsonFacetType {
    fn from(facet: JsonQueryFacet) -> Self {
        JsonFacetType::Query(Box::new(facet))
    }
}

impl JsonQueryFacet {
    pub fn new<S: Into<String>>(q: S) -> Self {
        JsonQueryFacet {
            type_: "query".to_string(),
            q: q.into(),
            limit: None,
            offset: None,
            sort: None,
            fq: None,
            facet: None,
        }
    }

    pub fn limit<O: Into<Option<usize>>>(mut self, limit: O) -> Self {
        self.limit = limit.into();
        self
    }

    pub fn offset<O: Into<Option<usize>>>(mut self, offset: O) -> Self {
        self.offset = offset.into();
        self
    }

    pub fn sort<S: Into<String>, O: Into<Option<S>>>(mut self, sort: O) -> Self {
        self.sort = sort.into().map(|s| s.into());
        self
    }

    pub fn fq<S: Into<String>, I: IntoIterator<Item = S>, O: Into<Option<I>>>(
        mut self,
        fq: O,
    ) -> Self {
        self.fq = fq
            .into()
            .map(|fq| fq.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn facets<
        K: Into<String>,
        V: Into<JsonFacetType>,
        I: IntoIterator<Item = (K, V)>,
        O: Into<Option<I>>,
    >(
        mut self,
        facets: O,
    ) -> Self {
        self.facet = facets.into().map(|facets| {
            facets
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect()
        });
        self
    }
}
