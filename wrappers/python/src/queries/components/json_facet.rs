use pyo3::prelude::*;
use solrstice::{
    JsonFacetComponent, JsonFacetSortDirection, JsonFacetType, JsonQueryFacet, JsonStatFacet,
    JsonTermsFacet, JsonTermsFacetMethod,
};
use std::collections::{BTreeMap, HashMap};

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
    #[allow(clippy::too_many_arguments)]
    fn new(
        field: String,
        offset: Option<usize>,
        limit: Option<usize>,
        sort: Option<BTreeMap<String, JsonFacetSortDirectionWrapper>>,
        prelim_sort: Option<BTreeMap<String, JsonFacetSortDirectionWrapper>>,
        overrequest: Option<usize>,
        refine: Option<bool>,
        overrefine: Option<usize>,
        mincount: Option<usize>,
        missing: Option<bool>,
        num_buckets: Option<bool>,
        all_buckets: Option<bool>,
        prefix: Option<String>,
        facets: Option<HashMap<String, JsonFacetTypeWrapper>>,
        method: Option<JsonTermsFacetMethodWrapper>,
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
        if let Some(prelim_sort) = prelim_sort {
            terms = terms.prelim_sort(prelim_sort);
        }
        if let Some(overrequest) = overrequest {
            terms = terms.overrequest(overrequest);
        }
        if let Some(refine) = refine {
            terms = terms.refine(refine);
        }
        if let Some(overrefine) = overrefine {
            terms = terms.overrefine(overrefine);
        }
        if let Some(mincount) = mincount {
            terms = terms.mincount(mincount);
        }
        if let Some(missing) = missing {
            terms = terms.missing(missing);
        }
        if let Some(num_buckets) = num_buckets {
            terms = terms.num_buckets(num_buckets);
        }
        if let Some(all_buckets) = all_buckets {
            terms = terms.all_buckets(all_buckets);
        }
        if let Some(prefix) = prefix {
            terms = terms.prefix(prefix);
        }
        if let Some(facets) = facets {
            terms = terms.facets(facets.iter().map(|(k, v)| (k, JsonFacetType::from(v))));
        }
        if let Some(method) = method {
            terms = terms.method(JsonTermsFacetMethod::from(method));
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
        q: Option<String>,
        limit: Option<usize>,
        offset: Option<usize>,
        sort: Option<String>,
        fq: Option<Vec<String>>,
        facets: Option<HashMap<String, JsonFacetTypeWrapper>>,
    ) -> (Self, JsonFacetTypeWrapper) {
        let mut query_facet = JsonQueryFacet::new();
        if let Some(q) = q {
            query_facet = query_facet.q(q);
        }
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

#[derive(Clone, Copy, Debug, PartialEq)]
#[pyclass(name = "JsonFacetSortDirection")]
pub enum JsonFacetSortDirectionWrapper {
    Asc,
    Desc,
}

impl From<JsonFacetSortDirectionWrapper> for JsonFacetSortDirection {
    fn from(wrapper: JsonFacetSortDirectionWrapper) -> Self {
        match wrapper {
            JsonFacetSortDirectionWrapper::Asc => JsonFacetSortDirection::Asc,
            JsonFacetSortDirectionWrapper::Desc => JsonFacetSortDirection::Desc,
        }
    }
}

impl From<JsonFacetSortDirection> for JsonFacetSortDirectionWrapper {
    fn from(direction: JsonFacetSortDirection) -> Self {
        match direction {
            JsonFacetSortDirection::Asc => JsonFacetSortDirectionWrapper::Asc,
            JsonFacetSortDirection::Desc => JsonFacetSortDirectionWrapper::Desc,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[pyclass(name = "JsonTermsFacetMethod")]
pub enum JsonTermsFacetMethodWrapper {
    DocValues,
    UnInvertedField,
    DocValuesHash,
    Enum,
    Stream,
    Smart,
}

impl From<JsonTermsFacetMethod> for JsonTermsFacetMethodWrapper {
    fn from(method: JsonTermsFacetMethod) -> Self {
        match method {
            JsonTermsFacetMethod::DocValues => JsonTermsFacetMethodWrapper::DocValues,
            JsonTermsFacetMethod::UnInvertedField => JsonTermsFacetMethodWrapper::UnInvertedField,
            JsonTermsFacetMethod::DocValuesHash => JsonTermsFacetMethodWrapper::DocValuesHash,
            JsonTermsFacetMethod::Enum => JsonTermsFacetMethodWrapper::Enum,
            JsonTermsFacetMethod::Stream => JsonTermsFacetMethodWrapper::Stream,
            JsonTermsFacetMethod::Smart => JsonTermsFacetMethodWrapper::Smart,
        }
    }
}

impl From<JsonTermsFacetMethodWrapper> for JsonTermsFacetMethod {
    fn from(wrapper: JsonTermsFacetMethodWrapper) -> Self {
        match wrapper {
            JsonTermsFacetMethodWrapper::DocValues => JsonTermsFacetMethod::DocValues,
            JsonTermsFacetMethodWrapper::UnInvertedField => JsonTermsFacetMethod::UnInvertedField,
            JsonTermsFacetMethodWrapper::DocValuesHash => JsonTermsFacetMethod::DocValuesHash,
            JsonTermsFacetMethodWrapper::Enum => JsonTermsFacetMethod::Enum,
            JsonTermsFacetMethodWrapper::Stream => JsonTermsFacetMethod::Stream,
            JsonTermsFacetMethodWrapper::Smart => JsonTermsFacetMethod::Smart,
        }
    }
}
