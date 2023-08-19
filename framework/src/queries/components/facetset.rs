use regex::Regex;
use serde::de::Error;
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FacetSetComponent {
    facet: bool,
    #[serde(rename = "facet.query")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    queries: Vec<String>,
    #[serde(
        serialize_with = "serialize_fields",
        deserialize_with = "deserialize_fields",
        flatten
    )]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    fields: Vec<FieldFacetComponent>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pivots: Option<PivotFacetComponent>,
}

fn serialize_fields<S>(fields: &Vec<FieldFacetComponent>, serializer: S) -> Result<S::Ok, S::Error>
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

fn deserialize_fields<'de, D>(deserializer: D) -> Result<Vec<FieldFacetComponent>, D::Error>
where
    D: Deserializer<'de>,
{
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(r"^f\.(.+).facet\.(.+)$").unwrap();
    };
    let map = serde_json::Value::deserialize(deserializer)?;
    let mut fields: HashMap<String, FieldFacetComponent> = HashMap::new();
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
                        .or_insert_with(|| FieldFacetComponent::new(field_name.as_str()));
                    entry.prefix = Some(prefix);
                }
                "contains" => {
                    let contains =
                        serde_json::from_value::<String>(value.clone()).map_err(|e| {
                            Error::custom(format!("Error deserializing field facet: {}", e))
                        })?;
                    let entry = fields
                        .entry(field_name.as_str().to_string())
                        .or_insert_with(|| FieldFacetComponent::new(field_name.as_str()));
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
                        FieldFacetComponent::new(field_name.as_str()),
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
        .collect::<Vec<FieldFacetComponent>>())
}

impl FacetSetComponent {
    pub fn new() -> Self {
        FacetSetComponent {
            facet: true,
            queries: Vec::new(),
            pivots: None,
            fields: Vec::new(),
        }
    }

    pub fn pivots<T: Into<PivotFacetComponent>, O: Into<Option<T>>>(mut self, pivots: O) -> Self {
        self.pivots = pivots.into().map(|x| x.into());
        self
    }

    pub fn queries<S: Into<String>, I: IntoIterator<Item = S>>(mut self, queries: I) -> Self {
        self.queries = queries.into_iter().map(|x| x.into()).collect();
        self
    }

    pub fn fields<S: Into<FieldFacetComponent>, I: IntoIterator<Item = S>>(
        mut self,
        fields: I,
    ) -> Self {
        self.fields = fields.into_iter().map(|x| x.into()).collect();
        self
    }
}

impl Default for FacetSetComponent {
    fn default() -> Self {
        FacetSetComponent::new()
    }
}

impl AsRef<FacetSetComponent> for FacetSetComponent {
    fn as_ref(&self) -> &FacetSetComponent {
        self
    }
}

impl From<&FacetSetComponent> for FacetSetComponent {
    fn from(facet_set: &FacetSetComponent) -> Self {
        facet_set.clone()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PivotFacetComponent {
    /// The field to facet on.
    #[serde(rename = "facet.pivot")]
    pivots: Vec<String>,
    /// The minimum count for a facet to be returned. Default is 1.
    #[serde(rename = "facet.pivot.mincount")]
    min_count: Option<usize>,
}

impl PivotFacetComponent {
    pub fn new<S: Into<String>, I: IntoIterator<Item = S>>(pivots: I) -> Self {
        PivotFacetComponent {
            pivots: pivots.into_iter().map(|x| x.into()).collect(),
            min_count: None,
        }
    }

    pub fn min_count<O: Into<Option<usize>>>(mut self, min_count: O) -> Self {
        self.min_count = min_count.into();
        self
    }
}

impl AsRef<PivotFacetComponent> for PivotFacetComponent {
    fn as_ref(&self) -> &PivotFacetComponent {
        self
    }
}

impl From<&PivotFacetComponent> for PivotFacetComponent {
    fn from(pivot: &PivotFacetComponent) -> Self {
        pivot.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldFacetComponent {
    field: String,
    prefix: Option<String>,
    contains: Option<String>,
}

impl FieldFacetComponent {
    pub fn new<S: Into<String>>(field: S) -> Self {
        FieldFacetComponent {
            field: field.into(),
            prefix: None,
            contains: None,
        }
    }

    pub fn prefix<S: Into<String>, O: Into<Option<S>>>(mut self, prefix: O) -> Self {
        self.prefix = prefix.into().map(|s| s.into());
        self
    }

    pub fn contains<S: Into<String>, O: Into<Option<S>>>(mut self, contains: O) -> Self {
        self.contains = contains.into().map(|s| s.into());
        self
    }
}

impl AsRef<FieldFacetComponent> for FieldFacetComponent {
    fn as_ref(&self) -> &FieldFacetComponent {
        self
    }
}

impl From<&FieldFacetComponent> for FieldFacetComponent {
    fn from(f: &FieldFacetComponent) -> Self {
        f.clone()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::queries::components::facetset::FacetSetComponent;

    #[test]
    fn serialize_fields_works() {
        let builder = FacetSetComponent::new().queries(["age:[* TO *]"]).fields([
            &crate::queries::components::facetset::FieldFacetComponent {
                field: "field".to_string(),
                prefix: Some("prefix".to_string()),
                contains: Some("contains".to_string()),
            },
        ]);
        let serialized = serde_json::to_string_pretty(&builder).unwrap();
        let deserialized = serde_json::from_str::<FacetSetComponent>(&serialized).unwrap();
        assert_eq!(builder, deserialized);
    }
}
