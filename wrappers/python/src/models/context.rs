use crate::external::ReqwestClientWrapper;
use crate::hosts::SolrHostWrapper;
use crate::models::auth::SolrAuthWrapper;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solrstice::{LoggingPolicy, SolrSingleServerHost};
use solrstice::{SolrServerContext, SolrServerContextBuilder};

#[derive(FromPyObject)]
pub enum SolrHostUnion {
    SolrHostWrapperEnumValue(SolrHostWrapper),
    String(String),
}

#[pyclass(
    name = "SolrServerContext",
    module = "solrstice",
    subclass,
    from_py_object
)]
#[derive(Clone)]
pub struct SolrServerContextWrapper(SolrServerContext);

#[pymethods]
impl SolrServerContextWrapper {
    #[new]
    #[pyo3(signature = (host, auth=None, logging_policy=None, client=None))]
    pub fn new(
        host: SolrHostUnion,
        auth: Option<SolrAuthWrapper>,
        logging_policy: Option<LoggingPolicyWrapper>,
        client: Option<ReqwestClientWrapper>,
    ) -> Self {
        let mut builder = match host {
            SolrHostUnion::SolrHostWrapperEnumValue(h) => SolrServerContextBuilder::new(h),
            SolrHostUnion::String(s) => SolrServerContextBuilder::new(SolrSingleServerHost::new(s)),
        };
        builder = match auth {
            Some(auth) => builder.with_auth(auth),
            None => builder,
        };
        builder = match logging_policy {
            Some(logging_policy) => builder.with_logging_policy(logging_policy.into()),
            None => builder,
        };
        builder = match client {
            Some(client) => builder.with_client(client.0),
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

#[pyclass(name = "LoggingPolicy", module = "solrstice", subclass, from_py_object)]
#[derive(Clone)]
pub struct LoggingPolicyWrapper(LoggingPolicy);

impl From<LoggingPolicyWrapper> for LoggingPolicy {
    fn from(value: LoggingPolicyWrapper) -> Self {
        value.0
    }
}

impl From<LoggingPolicy> for LoggingPolicyWrapper {
    fn from(value: LoggingPolicy) -> Self {
        Self(value)
    }
}

#[pyclass(name = "OffLoggingPolicy", extends=LoggingPolicyWrapper, module = "solrstice", subclass, from_py_object)]
#[derive(Clone, Serialize, Deserialize)]
pub struct OffLoggingPolicyWrapper {}

#[pymethods]
impl OffLoggingPolicyWrapper {
    #[new]
    pub fn new() -> PyClassInitializer<Self> {
        PyClassInitializer::from(LoggingPolicyWrapper(LoggingPolicy::Off)).add_subclass(Self {})
    }
}

#[pyclass(name = "FastLoggingPolicy", extends=LoggingPolicyWrapper, module = "solrstice", subclass, from_py_object)]
#[derive(Clone, Serialize, Deserialize)]
pub struct FastLoggingPolicyWrapper {}

#[pymethods]
impl FastLoggingPolicyWrapper {
    #[new]
    pub fn new(max_body_length: usize) -> PyClassInitializer<Self> {
        PyClassInitializer::from(LoggingPolicyWrapper(LoggingPolicy::Fast(max_body_length)))
            .add_subclass(Self {})
    }
}

#[pyclass(name = "PrettyLoggingPolicy", extends=LoggingPolicyWrapper, module = "solrstice", subclass, from_py_object)]
#[derive(Clone, Serialize, Deserialize)]
pub struct PrettyLoggingPolicyWrapper {}

#[pymethods]
impl PrettyLoggingPolicyWrapper {
    #[new]
    pub fn new(max_body_length: usize) -> PyClassInitializer<Self> {
        PyClassInitializer::from(LoggingPolicyWrapper(LoggingPolicy::Pretty(max_body_length)))
            .add_subclass(Self {})
    }
}
