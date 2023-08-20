use regex::Regex;
use serde::de::Error;
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FacetSetComponent {
    facet: bool,
    #[serde(rename = "facet.query")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    queries: Vec<String>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    fields: Option<FieldFacetComponent>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pivots: Option<PivotFacetComponent>,
}

impl FacetSetComponent {
    pub fn new() -> Self {
        FacetSetComponent {
            facet: true,
            queries: Vec::new(),
            pivots: None,
            fields: None,
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

    pub fn fields<T: Into<FieldFacetComponent>, O: Into<Option<T>>>(mut self, fields: O) -> Self {
        self.fields = fields.into().map(|x| x.into());
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

#[derive(Clone, Debug, PartialEq)]
pub struct FieldFacetComponent {
    fields: Vec<FieldFacetEntry>,
    exclude_terms: Option<String>,
}

impl Serialize for FieldFacetComponent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        let mut field_fields = Vec::new();
        for field in self.fields.iter() {
            field_fields.push(field.field.clone());
            if let Some(prefix) = &field.prefix {
                map.serialize_entry(format!("f.{}.facet.prefix", field.field).as_str(), prefix)?;
            }
            if let Some(contains) = &field.contains {
                map.serialize_entry(
                    format!("f.{}.facet.contains", field.field).as_str(),
                    contains,
                )?;
            }
            if let Some(contains_ignore_case) = &field.contains_ignore_case {
                map.serialize_entry(
                    format!("f.{}.facet.contains.ignoreCase", field.field).as_str(),
                    contains_ignore_case,
                )?;
            }
            if let Some(sort) = &field.sort {
                map.serialize_entry(format!("f.{}.facet.sort", field.field).as_str(), sort)?;
            }
            if let Some(limit) = &field.limit {
                map.serialize_entry(format!("f.{}.facet.limit", field.field).as_str(), limit)?;
            }
            if let Some(offset) = &field.offset {
                map.serialize_entry(format!("f.{}.facet.offset", field.field).as_str(), offset)?;
            }
            if let Some(min_count) = &field.min_count {
                map.serialize_entry(
                    format!("f.{}.facet.mincount", field.field).as_str(),
                    min_count,
                )?;
            }
            if let Some(missing) = &field.missing {
                map.serialize_entry(format!("f.{}.facet.missing", field.field).as_str(), missing)?;
            }
            if let Some(method) = &field.method {
                map.serialize_entry(format!("f.{}.facet.method", field.field).as_str(), method)?;
            }
            if let Some(enum_cache_min_df) = &field.enum_cache_min_df {
                map.serialize_entry(
                    format!("f.{}.facet.enum.cache.minDf", field.field).as_str(),
                    enum_cache_min_df,
                )?;
            }
            if let Some(exists) = &field.exists {
                map.serialize_entry(format!("f.{}.facet.exists", field.field).as_str(), exists)?;
            }
        }
        if !self.fields.is_empty() {
            map.serialize_entry("facet.field", &field_fields)?;
        }
        if let Some(exclude_terms) = &self.exclude_terms {
            map.serialize_entry("facet.excludeTerms", exclude_terms.as_str())?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for FieldFacetComponent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        lazy_static::lazy_static! {
            static ref RE: Regex = Regex::new(r"^f\.(.+).facet\.(.+)$").unwrap();
        };
        let mut component = FieldFacetComponent::new(Vec::<FieldFacetEntry>::new());
        let map = serde_json::Value::deserialize(deserializer)?;
        let mut fields: HashMap<String, FieldFacetEntry> = HashMap::new();
        for (key, value) in map
            .as_object()
            .ok_or(Error::custom("Could not map object"))?
            .iter()
        {
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
                    "prefix" | "contains" => {
                        let content =
                            serde_json::from_value::<String>(value.clone()).map_err(|e| {
                                Error::custom(format!("Error deserializing field facet: {}", e))
                            })?;
                        let entry = get_or_insert_entry(field_name.as_str(), &mut fields);
                        match field_type.as_str() {
                            "prefix" => entry.prefix = Some(content),
                            _ => entry.contains = Some(content),
                        }
                    }
                    "contains.ignoreCase" | "missing" | "exists" => {
                        let content =
                            serde_json::from_value::<bool>(value.clone()).map_err(|e| {
                                Error::custom(format!("Error deserializing field facet: {}", e))
                            })?;
                        let entry = get_or_insert_entry(field_name.as_str(), &mut fields);
                        match field_type.as_str() {
                            "contains.ignoreCase" => entry.contains_ignore_case = Some(content),
                            "exists" => entry.exists = Some(content),
                            _ => entry.missing = Some(content),
                        }
                    }
                    "sort" => {
                        let content = serde_json::from_value::<FieldFacetSort>(value.clone())
                            .map_err(|e| {
                                Error::custom(format!("Error deserializing field facet: {}", e))
                            })?;
                        let entry = get_or_insert_entry(field_name.as_str(), &mut fields);
                        entry.sort = Some(content);
                    }
                    "limit" | "offset" | "mincount" | "enum.cache.minDf" => {
                        let content =
                            serde_json::from_value::<usize>(value.clone()).map_err(|e| {
                                Error::custom(format!("Error deserializing field facet: {}", e))
                            })?;
                        let entry = get_or_insert_entry(field_name.as_str(), &mut fields);
                        match field_type.as_str() {
                            "limit" => entry.limit = Some(content),
                            "offset" => entry.offset = Some(content),
                            "mincount" => entry.min_count = Some(content),
                            _ => entry.enum_cache_min_df = Some(content),
                        }
                    }
                    "method" => {
                        let content = serde_json::from_value::<FieldFacetMethod>(value.clone())
                            .map_err(|e| {
                                Error::custom(format!("Error deserializing field facet: {}", e))
                            })?;
                        let entry = get_or_insert_entry(field_name.as_str(), &mut fields);
                        entry.method = Some(content);
                    }
                    _ => {
                        return Err(Error::custom(format!(
                            "Invalid field type in facet field: {}",
                            key
                        )));
                    }
                }
            } else if key == "facet.field" {
                let field_names =
                    serde_json::from_value::<Vec<String>>(value.clone()).map_err(|e| {
                        Error::custom(format!("Error deserializing field facet: {}", e))
                    })?;
                for field_name in field_names {
                    if !fields.contains_key(&field_name) {
                        fields.insert(
                            field_name.clone(),
                            FieldFacetEntry::new(field_name.as_str()),
                        );
                    }
                }
            } else if key == "facet.excludeTerms" {
                let exclude_terms =
                    serde_json::from_value::<String>(value.clone()).map_err(|e| {
                        Error::custom(format!("Error deserializing field facet: {}", e))
                    })?;
                component = component.exclude_terms(exclude_terms);
            } else {
                return Err(Error::custom(format!("Invalid facet field: {}", key)));
            }
        }
        component = component.fields(fields.into_values());
        if component.fields.is_empty() {
            return Err(Error::custom("No facet fields specified"));
        }
        Ok(component)
    }
}

fn get_or_insert_entry<'a>(
    field_name: &'a str,
    fields: &'a mut HashMap<String, FieldFacetEntry>,
) -> &'a mut FieldFacetEntry {
    fields
        .entry(field_name.to_string())
        .or_insert_with(|| FieldFacetEntry::new(field_name))
}

impl FieldFacetComponent {
    pub fn new<T: Into<FieldFacetEntry>, I: IntoIterator<Item = T>>(fields: I) -> Self {
        FieldFacetComponent {
            fields: fields.into_iter().map(|x| x.into()).collect(),
            exclude_terms: None,
        }
    }

    pub fn fields<T: Into<FieldFacetEntry>, I: IntoIterator<Item = T>>(
        mut self,
        fields: I,
    ) -> Self {
        self.fields = fields.into_iter().map(|x| x.into()).collect();
        self
    }

    pub fn exclude_terms<S: Into<String>, O: Into<Option<S>>>(mut self, exclude_terms: O) -> Self {
        self.exclude_terms = exclude_terms.into().map(|x| x.into());
        self
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum FieldFacetSort {
    #[serde(rename = "count")]
    Count,
    #[serde(rename = "index")]
    Index,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum FieldFacetMethod {
    #[serde(rename = "enum")]
    Enum,
    #[serde(rename = "fc")]
    Fc,
    #[serde(rename = "fcs")]
    Fcs,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldFacetEntry {
    field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contains: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contains_ignore_case: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<FieldFacetSort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    missing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<FieldFacetMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enum_cache_min_df: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exists: Option<bool>,
}

impl FieldFacetEntry {
    pub fn new<S: Into<String>>(field: S) -> Self {
        FieldFacetEntry {
            field: field.into(),
            prefix: None,
            contains: None,
            contains_ignore_case: None,
            sort: None,
            limit: None,
            offset: None,
            min_count: None,
            missing: None,
            method: None,
            enum_cache_min_df: None,
            exists: None,
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

    pub fn contains_ignore_case<O: Into<Option<bool>>>(mut self, contains_ignore_case: O) -> Self {
        self.contains_ignore_case = contains_ignore_case.into();
        self
    }

    pub fn sort<S: Into<FieldFacetSort>, O: Into<Option<S>>>(mut self, sort: O) -> Self {
        self.sort = sort.into().map(|s| s.into());
        self
    }

    pub fn limit<O: Into<Option<usize>>>(mut self, limit: O) -> Self {
        self.limit = limit.into();
        self
    }

    pub fn offset<O: Into<Option<usize>>>(mut self, offset: O) -> Self {
        self.offset = offset.into();
        self
    }

    pub fn min_count<O: Into<Option<usize>>>(mut self, min_count: O) -> Self {
        self.min_count = min_count.into();
        self
    }

    pub fn missing<O: Into<Option<bool>>>(mut self, missing: O) -> Self {
        self.missing = missing.into();
        self
    }

    pub fn method<S: Into<FieldFacetMethod>, O: Into<Option<S>>>(mut self, method: O) -> Self {
        self.method = method.into().map(|s| s.into());
        self
    }

    pub fn enum_cache_min_df<O: Into<Option<usize>>>(mut self, enum_cache_min_df: O) -> Self {
        self.enum_cache_min_df = enum_cache_min_df.into();
        self
    }

    pub fn exists<O: Into<Option<bool>>>(mut self, exists: O) -> Self {
        self.exists = exists.into();
        self
    }
}

impl AsRef<FieldFacetEntry> for FieldFacetEntry {
    fn as_ref(&self) -> &FieldFacetEntry {
        self
    }
}

impl From<&FieldFacetEntry> for FieldFacetEntry {
    fn from(f: &FieldFacetEntry) -> Self {
        f.clone()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::queries::components::facet_set::{FacetSetComponent, FieldFacetComponent};

    #[test]
    fn serialize_fields_works() {
        let builder = FacetSetComponent::new().queries(["age:[* TO *]"]).fields(
            FieldFacetComponent::new([
                &crate::queries::components::facet_set::FieldFacetEntry::new("field_field")
                    .prefix("prefix")
                    .contains("contains")
                    .method(crate::queries::components::facet_set::FieldFacetMethod::Enum)
                    .sort(crate::queries::components::facet_set::FieldFacetSort::Count)
                    .limit(10)
                    .offset(10)
                    .min_count(10)
                    .missing(true)
                    .enum_cache_min_df(10)
                    .exists(true),
            ])
            .exclude_terms("exclude_terms"),
        );
        let serialized = serde_json::to_string_pretty(&builder).unwrap();
        let deserialized = serde_json::from_str::<FacetSetComponent>(&serialized).unwrap();
        assert_eq!(builder, deserialized);
    }

    #[test]
    fn serialize_field_facet_works_empty() {
        let builder = FacetSetComponent::new().queries(["age:[* TO *]"]);
        let serialized = serde_json::to_string_pretty(&builder).unwrap();
        let deserialized = serde_json::from_str::<FacetSetComponent>(&serialized).unwrap();
        assert_eq!(builder, deserialized);
    }
}
