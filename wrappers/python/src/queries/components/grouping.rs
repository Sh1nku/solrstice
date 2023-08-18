use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solrstice::queries::components::grouping::{GroupFormatting, GroupingComponent};

#[pyclass(name = "GroupingComponent", module = "solrstice.group")]
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
#[pyclass(name = "GroupFormatting", module = "solrstice.group")]
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
    pub fn new(
        fields: Option<Vec<&str>>,
        queries: Option<Vec<&str>>,
        limit: Option<usize>,
        offset: Option<usize>,
        sort: Option<Vec<&str>>,
        format: Option<GroupFormattingWrapper>,
        main: Option<bool>,
        n_groups: Option<bool>,
        truncate: Option<bool>,
        facet: Option<bool>,
    ) -> Self {
        let builder = GroupingComponent::new();
        let mut s = Self(builder);
        s.set_fields(fields);
        s.set_queries(queries);
        s.set_limit(limit);
        s.set_offset(offset);
        s.set_sort(sort);
        s.set_format(format);
        s.set_main(main);
        s.set_n_groups(n_groups);
        s.set_truncate(truncate);
        s.set_facet(facet);
        s
    }

    #[setter]
    pub fn set_fields(&mut self, fields: Option<Vec<&str>>) {
        self.0.field = fields.map_or(None, |x| {
            Some(x.into_iter().map(|x| x.to_string()).collect())
        });
    }

    #[getter]
    pub fn get_fields(&self) -> Option<Vec<String>> {
        self.0.field.clone()
    }

    #[setter]
    pub fn set_queries(&mut self, queries: Option<Vec<&str>>) {
        self.0.queries = queries.map_or(None, |x| {
            Some(x.into_iter().map(|x| x.to_string()).collect())
        });
    }

    #[getter]
    pub fn get_queries(&self) -> Option<Vec<String>> {
        self.0.queries.clone()
    }

    #[setter]
    pub fn set_limit(&mut self, limit: Option<usize>) {
        self.0.limit = limit;
    }

    #[getter]
    pub fn get_limit(&self) -> Option<usize> {
        self.0.limit
    }

    #[setter]
    pub fn set_offset(&mut self, offset: Option<usize>) {
        self.0.offset = offset;
    }

    #[getter]
    pub fn get_offset(&self) -> Option<usize> {
        self.0.offset
    }

    #[setter]
    pub fn set_sort(&mut self, sort: Option<Vec<&str>>) {
        self.0.sort = sort.map_or(None, |x| {
            Some(x.into_iter().map(|x| x.to_string()).collect())
        });
    }

    #[getter]
    pub fn get_sort(&self) -> Option<Vec<String>> {
        self.0.sort.clone()
    }

    #[setter]
    pub fn set_format(&mut self, format: Option<GroupFormattingWrapper>) {
        self.0.format = format.map_or(None, |x| Some(x.into()));
    }

    #[getter]
    pub fn get_format(&self) -> Option<GroupFormattingWrapper> {
        self.0.format.map_or(None, |x| Some(x.into()))
    }

    #[setter]
    pub fn set_main(&mut self, main: Option<bool>) {
        self.0.main = main
    }

    #[getter]
    pub fn get_main(&self) -> Option<bool> {
        self.0.main
    }

    #[setter]
    pub fn set_n_groups(&mut self, n_groups: Option<bool>) {
        self.0.n_groups = n_groups
    }

    #[getter]
    pub fn get_n_groups(&self) -> Option<bool> {
        self.0.n_groups
    }

    #[setter]
    pub fn set_truncate(&mut self, truncate: Option<bool>) {
        self.0.truncate = truncate
    }

    #[getter]
    pub fn get_truncate(&self) -> Option<bool> {
        self.0.truncate
    }

    #[setter]
    pub fn set_facet(&mut self, facet: Option<bool>) {
        self.0.facet = facet
    }

    #[getter]
    pub fn get_facet(&self) -> Option<bool> {
        self.0.facet
    }
}

impl From<GroupingComponent> for GroupingComponentWrapper {
    fn from(builder: GroupingComponent) -> Self {
        Self(builder)
    }
}
