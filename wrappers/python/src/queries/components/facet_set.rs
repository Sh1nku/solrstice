use pyo3::prelude::*;
use solrstice::{
    FacetSetComponent, FieldFacetComponent, FieldFacetEntry, FieldFacetMethod, FieldFacetSort,
    PivotFacetComponent,
};

#[derive(Clone, Debug, PartialEq)]
#[pyclass(name = "FacetSetComponent", module = "solrstice", subclass)]
pub struct FacetSetComponentWrapper(FacetSetComponent);

#[pymethods]
impl FacetSetComponentWrapper {
    #[new]
    pub fn new(
        queries: Option<Vec<String>>,
        fields: Option<FieldFacetComponentWrapper>,
        pivots: Option<PivotFacetComponentWrapper>,
    ) -> Self {
        let mut component = FacetSetComponent::new();
        if let Some(queries) = queries {
            component = component.queries(queries);
        }
        if let Some(fields) = fields {
            component = component.fields(fields);
        }
        if let Some(pivots) = pivots {
            component = component.pivots(pivots);
        }
        FacetSetComponentWrapper(component)
    }
}

impl From<FacetSetComponentWrapper> for FacetSetComponent {
    fn from(wrapper: FacetSetComponentWrapper) -> Self {
        wrapper.0
    }
}

impl From<&FacetSetComponentWrapper> for FacetSetComponent {
    fn from(wrapper: &FacetSetComponentWrapper) -> Self {
        wrapper.0.clone()
    }
}

impl From<FacetSetComponent> for FacetSetComponentWrapper {
    fn from(component: FacetSetComponent) -> Self {
        FacetSetComponentWrapper(component)
    }
}

impl From<&FacetSetComponent> for FacetSetComponentWrapper {
    fn from(component: &FacetSetComponent) -> Self {
        FacetSetComponentWrapper(component.clone())
    }
}

#[derive(Clone, Debug, PartialEq)]
#[pyclass(name = "PivotFacetComponent", module = "solrstice", subclass)]
pub struct PivotFacetComponentWrapper(PivotFacetComponent);

#[pymethods]
impl PivotFacetComponentWrapper {
    #[new]
    pub fn new(pivots: Vec<String>, min_count: Option<usize>) -> Self {
        let mut component = PivotFacetComponent::new(pivots);
        if let Some(min_count) = min_count {
            component = component.min_count(min_count);
        }
        PivotFacetComponentWrapper(component)
    }
}

impl From<PivotFacetComponentWrapper> for PivotFacetComponent {
    fn from(wrapper: PivotFacetComponentWrapper) -> Self {
        wrapper.0
    }
}

impl From<&PivotFacetComponentWrapper> for PivotFacetComponent {
    fn from(wrapper: &PivotFacetComponentWrapper) -> Self {
        wrapper.0.clone()
    }
}

impl From<PivotFacetComponent> for PivotFacetComponentWrapper {
    fn from(component: PivotFacetComponent) -> Self {
        PivotFacetComponentWrapper(component)
    }
}

impl From<&PivotFacetComponent> for PivotFacetComponentWrapper {
    fn from(component: &PivotFacetComponent) -> Self {
        PivotFacetComponentWrapper(component.clone())
    }
}

#[derive(Clone, Debug, PartialEq)]
#[pyclass(name = "FieldFacetComponent", module = "solrstice", subclass)]
pub struct FieldFacetComponentWrapper(FieldFacetComponent);

#[pymethods]
impl FieldFacetComponentWrapper {
    #[new]
    pub fn new(fields: Vec<FieldFacetEntryWrapper>, exclude_terms: Option<String>) -> Self {
        let mut component = FieldFacetComponent::new(fields);
        if let Some(exclude_terms) = exclude_terms {
            component = component.exclude_terms(exclude_terms);
        }
        FieldFacetComponentWrapper(component)
    }
}

impl From<FieldFacetComponentWrapper> for FieldFacetComponent {
    fn from(wrapper: FieldFacetComponentWrapper) -> Self {
        wrapper.0
    }
}

impl From<&FieldFacetComponentWrapper> for FieldFacetComponent {
    fn from(wrapper: &FieldFacetComponentWrapper) -> Self {
        wrapper.0.clone()
    }
}

impl From<FieldFacetComponent> for FieldFacetComponentWrapper {
    fn from(component: FieldFacetComponent) -> Self {
        FieldFacetComponentWrapper(component)
    }
}

impl From<&FieldFacetComponent> for FieldFacetComponentWrapper {
    fn from(component: &FieldFacetComponent) -> Self {
        FieldFacetComponentWrapper(component.clone())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[pyclass(name = "FieldFacetSort")]
pub enum FieldFacetSortWrapper {
    Count,
    Index,
}

impl From<FieldFacetSortWrapper> for FieldFacetSort {
    fn from(wrapper: FieldFacetSortWrapper) -> Self {
        match wrapper {
            FieldFacetSortWrapper::Count => FieldFacetSort::Count,
            FieldFacetSortWrapper::Index => FieldFacetSort::Index,
        }
    }
}

impl From<FieldFacetSort> for FieldFacetSortWrapper {
    fn from(sort: FieldFacetSort) -> Self {
        match sort {
            FieldFacetSort::Count => FieldFacetSortWrapper::Count,
            FieldFacetSort::Index => FieldFacetSortWrapper::Index,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[pyclass(name = "FieldFacetMethod")]
pub enum FieldFacetMethodWrapper {
    Enum,
    Fc,
    Fcs,
}

impl From<FieldFacetMethodWrapper> for FieldFacetMethod {
    fn from(wrapper: FieldFacetMethodWrapper) -> Self {
        match wrapper {
            FieldFacetMethodWrapper::Enum => FieldFacetMethod::Enum,
            FieldFacetMethodWrapper::Fc => FieldFacetMethod::Fc,
            FieldFacetMethodWrapper::Fcs => FieldFacetMethod::Fcs,
        }
    }
}

impl From<FieldFacetMethod> for FieldFacetMethodWrapper {
    fn from(method: FieldFacetMethod) -> Self {
        match method {
            FieldFacetMethod::Enum => FieldFacetMethodWrapper::Enum,
            FieldFacetMethod::Fc => FieldFacetMethodWrapper::Fc,
            FieldFacetMethod::Fcs => FieldFacetMethodWrapper::Fcs,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[pyclass(name = "FieldFacetEntry", module = "solrstice", subclass)]
pub struct FieldFacetEntryWrapper(FieldFacetEntry);

#[pymethods]
impl FieldFacetEntryWrapper {
    #[new]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        field: String,
        prefix: Option<String>,
        contains: Option<String>,
        contains_ignore_case: Option<bool>,
        sort: Option<FieldFacetSortWrapper>,
        limit: Option<usize>,
        offset: Option<usize>,
        min_count: Option<usize>,
        missing: Option<bool>,
        method: Option<FieldFacetMethodWrapper>,
        enum_cache_min_df: Option<usize>,
        exists: Option<bool>,
    ) -> Self {
        let mut entry = FieldFacetEntry::new(field);
        if let Some(prefix) = prefix {
            entry = entry.prefix(prefix);
        }
        if let Some(contains) = contains {
            entry = entry.contains(contains);
        }
        if let Some(contains_ignore_case) = contains_ignore_case {
            entry = entry.contains_ignore_case(contains_ignore_case);
        }
        if let Some(sort) = sort {
            let sort: FieldFacetSort = sort.into();
            entry = entry.sort(sort);
        }
        if let Some(limit) = limit {
            entry = entry.limit(limit);
        }
        if let Some(offset) = offset {
            entry = entry.offset(offset);
        }
        if let Some(min_count) = min_count {
            entry = entry.min_count(min_count);
        }
        if let Some(missing) = missing {
            entry = entry.missing(missing);
        }
        if let Some(method) = method {
            let method: FieldFacetMethod = method.into();
            entry = entry.method(method);
        }
        if let Some(enum_cache_min_df) = enum_cache_min_df {
            entry = entry.enum_cache_min_df(enum_cache_min_df);
        }
        if let Some(exists) = exists {
            entry = entry.exists(exists);
        }
        FieldFacetEntryWrapper(entry)
    }
}

impl From<FieldFacetEntryWrapper> for FieldFacetEntry {
    fn from(wrapper: FieldFacetEntryWrapper) -> Self {
        wrapper.0
    }
}

impl From<&FieldFacetEntryWrapper> for FieldFacetEntry {
    fn from(wrapper: &FieldFacetEntryWrapper) -> Self {
        wrapper.0.clone()
    }
}

impl From<FieldFacetEntry> for FieldFacetEntryWrapper {
    fn from(entry: FieldFacetEntry) -> Self {
        FieldFacetEntryWrapper(entry)
    }
}

impl From<&FieldFacetEntry> for FieldFacetEntryWrapper {
    fn from(entry: &FieldFacetEntry) -> Self {
        FieldFacetEntryWrapper(entry.clone())
    }
}
