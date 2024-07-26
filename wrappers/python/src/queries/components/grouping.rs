use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solrstice::{GroupFormatting, GroupingComponent};

#[pyclass(name = "GroupingComponent", module = "solrstice", subclass)]
#[derive(Clone, Serialize, Deserialize)]
pub struct GroupingComponentWrapper(GroupingComponent);

impl From<GroupingComponentWrapper> for GroupingComponent {
    fn from(wrapper: GroupingComponentWrapper) -> Self {
        wrapper.0
    }
}

impl<'a> From<&'a GroupingComponentWrapper> for &'a GroupingComponent {
    fn from(wrapper: &'a GroupingComponentWrapper) -> Self {
        &wrapper.0
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[pyclass(name = "GroupFormatting")]
pub enum GroupFormattingWrapper {
    Simple,
    Grouped,
}

impl From<GroupFormattingWrapper> for GroupFormatting {
    fn from(wrapper: GroupFormattingWrapper) -> Self {
        match wrapper {
            GroupFormattingWrapper::Simple => GroupFormatting::Simple,
            GroupFormattingWrapper::Grouped => GroupFormatting::Grouped,
        }
    }
}

impl From<GroupFormatting> for GroupFormattingWrapper {
    fn from(format: GroupFormatting) -> Self {
        match format {
            GroupFormatting::Simple => GroupFormattingWrapper::Simple,
            GroupFormatting::Grouped => GroupFormattingWrapper::Grouped,
        }
    }
}

#[pymethods]
impl GroupingComponentWrapper {
    #[new]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        fields: Option<Vec<String>>,
        queries: Option<Vec<String>>,
        limit: Option<usize>,
        offset: Option<usize>,
        sort: Option<Vec<String>>,
        format: Option<GroupFormattingWrapper>,
        main: Option<bool>,
        n_groups: Option<bool>,
        truncate: Option<bool>,
        facet: Option<bool>,
    ) -> Self {
        let mut builder = GroupingComponent::new();
        if let Some(fields) = fields {
            builder = builder.fields(fields);
        }
        if let Some(queries) = queries {
            builder = builder.queries(queries);
        }
        if let Some(limit) = limit {
            builder = builder.limit(limit);
        }
        if let Some(offset) = offset {
            builder = builder.offset(offset);
        }
        if let Some(sort) = sort {
            builder = builder.sort(sort);
        }
        if let Some(format) = format {
            let format: GroupFormatting = format.into();
            builder = builder.format(format);
        }
        if let Some(main) = main {
            builder = builder.main(main);
        }
        if let Some(n_groups) = n_groups {
            builder = builder.n_groups(n_groups);
        }
        if let Some(truncate) = truncate {
            builder = builder.truncate(truncate);
        }
        if let Some(facet) = facet {
            builder = builder.facet(facet);
        }
        Self(builder)
    }
}

impl From<GroupingComponent> for GroupingComponentWrapper {
    fn from(builder: GroupingComponent) -> Self {
        Self(builder)
    }
}
