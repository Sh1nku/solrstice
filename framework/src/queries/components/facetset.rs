use regex::Regex;
use serde::de::Error;
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FacetSetComponentBuilder {
    pub facet: bool,
    #[serde(rename = "facet.query")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub queries: Vec<String>,
    #[serde(
        serialize_with = "serialize_fields",
        deserialize_with = "deserialize_fields",
        flatten
    )]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<FieldFacetComponentBuilder>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pivots: Option<PivotFacetComponentBuilder>,
}

fn serialize_fields<S>(
    fields: &Vec<FieldFacetComponentBuilder>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut map = serializer.serialize_map(Some(1))?;
    let mut field_fields = Vec::new();
    for field in fields.iter() {
        field_fields.push(field.field.clone());
        map.serialize_entry(
            format!("f.{}.facet.prefix", field.field).as_str(),
            &field.prefix,
        )?;
        map.serialize_entry(
            format!("f.{}.facet.contains", field.field).as_str(),
            &field.contains,
        )?;
    }
    if !fields.is_empty() {
        map.serialize_entry("facet.field", &field_fields)?;
    }
    map.end()
}

fn deserialize_fields<'de, D>(deserializer: D) -> Result<Vec<FieldFacetComponentBuilder>, D::Error>
where
    D: Deserializer<'de>,
{
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(r"^f\.(.+).facet\.(.+)$").unwrap();
    };
    let map = serde_json::Value::deserialize(deserializer)?;
    let mut fields: HashMap<String, FieldFacetComponentBuilder> = HashMap::new();
    for (key, value) in map.as_object().unwrap().iter() {
        if let Some(caps) = RE.captures(key) {
            let field_name = caps.get(1).ok_or(Error::custom(format!(
                "Invalid field name in facet field: {}",
                key
            )))?;
            let field_type = caps.get(2).ok_or(Error::custom(format!(
                "Invalid field type in facet field: {}",
                key
            )))?;
            match field_type.as_str() {
                "prefix" => {
                    let prefix = serde_json::from_value::<String>(value.clone()).map_err(|e| {
                        Error::custom(format!("Error deserializing field facet: {}", e))
                    })?;
                    let entry = fields
                        .entry(field_name.as_str().to_string())
                        .or_insert_with(|| FieldFacetComponentBuilder::new(field_name.as_str()));
                    entry.prefix = Some(prefix);
                }
                "contains" => {
                    let contains =
                        serde_json::from_value::<String>(value.clone()).map_err(|e| {
                            Error::custom(format!("Error deserializing field facet: {}", e))
                        })?;
                    let entry = fields
                        .entry(field_name.as_str().to_string())
                        .or_insert_with(|| FieldFacetComponentBuilder::new(field_name.as_str()));
                    entry.contains = Some(contains);
                }
                _ => {
                    return Err(Error::custom(format!(
                        "Invalid field type in facet field: {}",
                        key
                    )));
                }
            }
        } else if key == "facet.field" {
            let field_names = serde_json::from_value::<Vec<String>>(value.clone())
                .map_err(|e| Error::custom(format!("Error deserializing field facet: {}", e)))?;
            for field_name in field_names {
                if !fields.contains_key(&field_name) {
                    fields.insert(
                        field_name.clone(),
                        FieldFacetComponentBuilder::new(field_name.as_str()),
                    );
                }
            }
        } else {
            return Err(Error::custom(format!("Invalid facet field: {}", key)));
        }
    }
    Ok(fields
        .into_iter()
        .map(|(_, v)| v)
        .collect::<Vec<FieldFacetComponentBuilder>>())
}

impl FacetSetComponentBuilder {
    pub fn new() -> Self {
        FacetSetComponentBuilder {
            facet: true,
            queries: Vec::new(),
            pivots: None,
            fields: Vec::new(),
        }
    }

    pub fn set_pivots(mut self, pivots: &PivotFacetComponentBuilder) -> Self {
        self.pivots = Some(pivots.clone());
        self
    }

    pub fn set_queries(mut self, queries: &[&str]) -> Self {
        self.queries = queries.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn add_query(mut self, query: &str) -> Self {
        self.queries.push(query.to_string());
        self
    }

    pub fn add_field(mut self, field: &FieldFacetComponentBuilder) -> Self {
        self.fields.push(field.clone());
        self
    }

    pub fn set_fields(mut self, fields: &[&FieldFacetComponentBuilder]) -> Self {
        self.fields = fields
            .iter()
            .map(|f| f.deref().clone())
            .collect::<Vec<FieldFacetComponentBuilder>>();
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

#[derive(Clone, Debug, PartialEq)]
pub struct FieldFacetComponentBuilder {
    pub field: String,
    pub prefix: Option<String>,
    pub contains: Option<String>,
}

impl FieldFacetComponentBuilder {
    pub fn new(field: &str) -> Self {
        FieldFacetComponentBuilder {
            field: field.to_string(),
            prefix: None,
            contains: None,
        }
    }

    pub fn prefix(mut self, prefix: &str) -> Self {
        self.prefix = Some(prefix.to_string());
        self
    }

    pub fn contains(mut self, contains: &str) -> Self {
        self.contains = Some(contains.to_string());
        self
    }
}

#[cfg(test)]
pub mod tests {
    use crate::queries::components::facetset::FacetSetComponentBuilder;

    #[test]
    fn serialize_fields_works() {
        let builder = FacetSetComponentBuilder::new()
            .add_query("age:[* TO *]")
            .add_field(
                &crate::queries::components::facetset::FieldFacetComponentBuilder {
                    field: "field".to_string(),
                    prefix: Some("prefix".to_string()),
                    contains: Some("contains".to_string()),
                },
            );
        let serialized = serde_json::to_string_pretty(&builder).unwrap();
        let deserialized = serde_json::from_str::<FacetSetComponentBuilder>(&serialized).unwrap();
        assert_eq!(builder, deserialized);
    }
}
