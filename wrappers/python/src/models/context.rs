use crate::hosts::SolrHostWrapper;
use crate::models::auth::SolrAuthWrapper;
use pyo3::prelude::*;
use solrstice::models::context::{SolrServerContext, SolrServerContextBuilder};

#[pyclass(name = "SolrServerContext", module = "solrstice.hosts", subclass)]
#[derive(Clone)]
pub struct SolrServerContextWrapper(SolrServerContext);

#[pymethods]
impl SolrServerContextWrapper {
    #[new]
    pub fn new(host: SolrHostWrapper, auth: Option<SolrAuthWrapper>) -> Self {
        let mut builder = SolrServerContextBuilder::new(host);
        builder = match auth {
            Some(auth) => builder.with_auth(auth),
            None => builder,
        };
        SolrServerContextWrapper(builder.build())
    }
}

impl From<SolrServerContextWrapper> for SolrServerContext {
    fn from(value: SolrServerContextWrapper) -> Self {
        value.0
    }
}

impl<'a> From<&'a SolrServerContextWrapper> for &'a SolrServerContext {
    fn from(value: &'a SolrServerContextWrapper) -> Self {
        &value.0
    }
}
