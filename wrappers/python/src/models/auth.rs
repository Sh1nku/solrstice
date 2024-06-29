use pyo3::prelude::*;
use solrstice::models::auth::{SolrAuth, SolrBasicAuth};
use std::sync::Arc;

#[pymodule]
pub fn auth(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SolrAuthWrapper>()?;
    m.add_class::<SolrBasicAuthWrapper>()?;
    Ok(())
}

#[pyclass(name = "SolrAuth", module = "solrstice.auth", subclass)]
#[derive(Clone)]
pub struct SolrAuthWrapper {
    pub solr_auth: Arc<dyn SolrAuth + Send + Sync>,
}

impl SolrAuth for SolrAuthWrapper {
    fn add_auth_to_request(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        self.solr_auth.add_auth_to_request(request)
    }
}

#[pyclass(name = "SolrBasicAuth", extends=SolrAuthWrapper, module = "solrstice.auth", subclass)]
#[derive(Clone)]
pub struct SolrBasicAuthWrapper {}

#[pymethods]
impl SolrBasicAuthWrapper {
    #[new]
    pub fn new(username: String, password: Option<String>) -> (Self, SolrAuthWrapper) {
        (
            SolrBasicAuthWrapper {},
            SolrAuthWrapper {
                solr_auth: Arc::new(SolrBasicAuth { username, password }),
            },
        )
    }
}
