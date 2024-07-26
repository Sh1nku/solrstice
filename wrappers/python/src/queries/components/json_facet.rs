use pyo3::prelude::*;
use solrstice::{JsonFacetComponent, JsonFacetType, JsonQueryFacet, JsonStatFacet, JsonTermsFacet};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
#[pyclass(name = "JsonFacetComponent", module = "solrstice", subclass)]
pub struct JsonFacetComponentWrapper(JsonFacetComponent);

#[pymethods]
impl JsonFacetComponentWrapper {
    #[new]
    fn new(facets: Option<HashMap<String, JsonFacetTypeWrapper>>) -> Self {
        let mut component = JsonFacetComponent::new();
        if let Some(facets) = facets {
            component = component.facets(facets.iter().map(|(k, v)| (k, JsonFacetType::from(v))));
        }
        JsonFacetComponentWrapper(component)
    }
}

impl From<JsonFacetComponentWrapper> for JsonFacetComponent {
    fn from(wrapper: JsonFacetComponentWrapper) -> Self {
        wrapper.0
    }
}

impl From<&JsonFacetComponentWrapper> for JsonFacetComponent {
    fn from(wrapper: &JsonFacetComponentWrapper) -> Self {
        wrapper.0.clone()
    }
}

impl From<JsonFacetComponent> for JsonFacetComponentWrapper {
    fn from(component: JsonFacetComponent) -> Self {
        JsonFacetComponentWrapper(component)
    }
}

impl From<&JsonFacetComponent> for JsonFacetComponentWrapper {
    fn from(component: &JsonFacetComponent) -> Self {
        JsonFacetComponentWrapper(component.clone())
    }
}

#[derive(Clone, Debug, PartialEq)]
#[pyclass(name = "JsonFacetType", module = "solrstice", subclass)]
pub struct JsonFacetTypeWrapper(JsonFacetType);

impl From<JsonFacetTypeWrapper> for JsonFacetType {
    fn from(wrapper: JsonFacetTypeWrapper) -> Self {
        wrapper.0
    }
}

impl From<&JsonFacetTypeWrapper> for JsonFacetType {
    fn from(wrapper: &JsonFacetTypeWrapper) -> Self {
        wrapper.0.clone()
    }
}

impl From<JsonFacetType> for JsonFacetTypeWrapper {
    fn from(facet_type: JsonFacetType) -> Self {
        JsonFacetTypeWrapper(facet_type)
    }
}

impl From<&JsonFacetType> for JsonFacetTypeWrapper {
    fn from(facet_type: &JsonFacetType) -> Self {
        JsonFacetTypeWrapper(facet_type.clone())
    }
}

#[derive(Clone, Debug, PartialEq)]
#[pyclass(name = "JsonTermsFacet", extends = JsonFacetTypeWrapper, module = "solrstice", subclass)]
pub struct JsonTermsFacetWrapper {}

#[pymethods]
impl JsonTermsFacetWrapper {
    #[new]
    fn new(
        field: String,
        offset: Option<usize>,
        limit: Option<usize>,
        sort: Option<String>,
        facets: Option<HashMap<String, JsonFacetTypeWrapper>>,
    ) -> (Self, JsonFacetTypeWrapper) {
        let mut terms = JsonTermsFacet::new(field);
        if let Some(offset) = offset {
            terms = terms.offset(offset);
        }
        if let Some(limit) = limit {
            terms = terms.limit(limit);
        }
        if let Some(sort) = sort {
            terms = terms.sort(sort);
        }
        if let Some(facets) = facets {
            terms = terms.facets(facets.iter().map(|(k, v)| (k, JsonFacetType::from(v))));
        }
        (
            Self {},
            JsonFacetTypeWrapper(JsonFacetType::Terms(Box::new(terms))),
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
#[pyclass(name = "JsonQueryFacet", extends = JsonFacetTypeWrapper, module = "solrstice", subclass)]
pub struct JsonQueryFacetWrapper {}

#[pymethods]
impl JsonQueryFacetWrapper {
    #[new]
    #[allow(clippy::too_many_arguments)]
    fn new(
        q: String,
        limit: Option<usize>,
        offset: Option<usize>,
        sort: Option<String>,
        fq: Option<Vec<String>>,
        facets: Option<HashMap<String, JsonFacetTypeWrapper>>,
    ) -> (Self, JsonFacetTypeWrapper) {
        let mut query_facet = JsonQueryFacet::new(q);
        if let Some(limit) = limit {
            query_facet = query_facet.limit(limit);
        }
        if let Some(offset) = offset {
            query_facet = query_facet.offset(offset);
        }
        if let Some(sort) = sort {
            query_facet = query_facet.sort(sort);
        }
        if let Some(fq) = fq {
            query_facet = query_facet.fq(fq);
        }
        if let Some(facets) = facets {
            query_facet =
                query_facet.facets(facets.iter().map(|(k, v)| (k, JsonFacetType::from(v))));
        }
        (
            Self {},
            JsonFacetTypeWrapper(JsonFacetType::Query(Box::new(query_facet))),
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
#[pyclass(name = "JsonStatFacet", extends = JsonFacetTypeWrapper, module = "solrstice", subclass)]
pub struct JsonStatFacetWrapper {}

#[pymethods]
impl JsonStatFacetWrapper {
    #[new]
    fn new(query: String) -> (Self, JsonFacetTypeWrapper) {
        (
            Self {},
            JsonFacetTypeWrapper(JsonFacetType::Stat(JsonStatFacet::new(query))),
        )
    }
}
