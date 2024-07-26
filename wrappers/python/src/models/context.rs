use crate::hosts::{SolrHostWrapper, SolrSingleServerHostWrapper};
use crate::models::auth::SolrAuthWrapper;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solrstice::LoggingPolicy;
use solrstice::{SolrServerContext, SolrServerContextBuilder};

#[derive(FromPyObject)]
pub enum SolrHostUnion {
    SolrHostWrapperEnumValue(SolrHostWrapper),
    String(String),
}

#[pyclass(name = "SolrServerContext", module = "solrstice", subclass)]
#[derive(Clone)]
pub struct SolrServerContextWrapper(SolrServerContext);

#[pymethods]
impl SolrServerContextWrapper {
    #[new]
    pub fn new(
        host: SolrHostUnion,
        auth: Option<SolrAuthWrapper>,
        logging_policy: Option<LoggingPolicyWrapper>,
    ) -> Self {
        let host = match host {
            SolrHostUnion::SolrHostWrapperEnumValue(h) => h,
            SolrHostUnion::String(s) => SolrSingleServerHostWrapper::new(s).1,
        };
        let mut builder = SolrServerContextBuilder::new(host);
        builder = match auth {
            Some(auth) => builder.with_auth(auth),
            None => builder,
        };
        builder = match logging_policy {
            Some(logging_policy) => builder.with_logging_policy(logging_policy.into()),
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

#[pyclass(name = "LoggingPolicy", module = "solrstice", subclass)]
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

#[pyclass(name = "OffLoggingPolicy", extends=LoggingPolicyWrapper, module = "solrstice", subclass)]
#[derive(Clone, Serialize, Deserialize)]
pub struct OffLoggingPolicyWrapper {}

#[pymethods]
impl OffLoggingPolicyWrapper {
    #[new]
    pub fn new() -> (Self, LoggingPolicyWrapper) {
        (Self {}, LoggingPolicyWrapper(LoggingPolicy::Off))
    }
}

#[pyclass(name = "FastLoggingPolicy", extends=LoggingPolicyWrapper, module = "solrstice", subclass)]
#[derive(Clone, Serialize, Deserialize)]
pub struct FastLoggingPolicyWrapper {}

#[pymethods]
impl FastLoggingPolicyWrapper {
    #[new]
    pub fn new(max_body_length: usize) -> (Self, LoggingPolicyWrapper) {
        (
            Self {},
            LoggingPolicyWrapper(LoggingPolicy::Fast(max_body_length)),
        )
    }
}

#[pyclass(name = "PrettyLoggingPolicy", extends=LoggingPolicyWrapper, module = "solrstice", subclass)]
#[derive(Clone, Serialize, Deserialize)]
pub struct PrettyLoggingPolicyWrapper {}

#[pymethods]
impl PrettyLoggingPolicyWrapper {
    #[new]
    pub fn new(max_body_length: usize) -> (Self, LoggingPolicyWrapper) {
        (
            Self {},
            LoggingPolicyWrapper(LoggingPolicy::Pretty(max_body_length)),
        )
    }
}
