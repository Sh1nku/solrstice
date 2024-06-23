use crate::models::context::SolrServerContextWrapper;
use crate::models::error::PyErrWrapper;
use async_trait::async_trait;
use pyo3::prelude::*;
use solrstice::hosts::solr_host::SolrHost;
use solrstice::hosts::solr_server_host::{SolrMultipleServerHost, SolrSingleServerHost};
use solrstice::hosts::zookeeper_host::ZookeeperEnsembleHostConnector;
use solrstice::models::error::SolrError;
use std::borrow::Cow;
use std::sync::Arc;
use std::time::Duration;

#[pymodule]
pub fn hosts(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<SolrHostWrapper>()?;
    m.add_class::<SolrSingleServerHostWrapper>()?;
    m.add_class::<SolrMultipleServerHostWrapper>()?;
    m.add_class::<ZookeeperEnsembleHostWrapper>()?;
    m.add_class::<ZookeeperEnsembleHostConnectorWrapper>()?;
    m.add_class::<SolrServerContextWrapper>()?;
    Ok(())
}

#[pyclass(name = "SolrHost", subclass, module = "solrstice.hosts")]
#[derive(Clone)]
pub struct SolrHostWrapper {
    pub solr_host: Arc<dyn SolrHost + Send + Sync>,
}

#[async_trait]
impl SolrHost for SolrHostWrapper {
    async fn get_solr_node(&self) -> Result<Cow<str>, SolrError> {
        self.solr_host.get_solr_node().await
    }
}

#[pyclass(name = "SolrSingleServerHost", extends = SolrHostWrapper, module= "solrstice.hosts")]
#[derive(Clone)]
pub struct SolrSingleServerHostWrapper;

#[pymethods]
impl SolrSingleServerHostWrapper {
    #[new]
    pub fn new(host: String) -> (Self, SolrHostWrapper) {
        (
            SolrSingleServerHostWrapper {},
            SolrHostWrapper {
                solr_host: Arc::new(SolrSingleServerHost::new(host.as_str())),
            },
        )
    }
}

#[pyclass(name = "SolrMultipleServerHost", extends = SolrHostWrapper, module= "solrstice.hosts")]
#[derive(Clone)]
pub struct SolrMultipleServerHostWrapper;

#[pymethods]
impl SolrMultipleServerHostWrapper {
    #[new]
    pub fn new(hosts: Vec<String>, timeout: f32) -> (Self, SolrHostWrapper) {
        (
            SolrMultipleServerHostWrapper {},
            SolrHostWrapper {
                solr_host: Arc::new(SolrMultipleServerHost::new(
                    hosts,
                    Duration::from_secs_f32(timeout),
                )),
            },
        )
    }
}

#[pyclass(name = "ZookeeperEnsembleHost", extends = SolrHostWrapper, module= "solrstice.hosts")]
#[derive(Clone)]
pub struct ZookeeperEnsembleHostWrapper;

#[pyclass(name = "ZookeeperEnsembleHostConnector")]
#[derive(Clone)]
pub struct ZookeeperEnsembleHostConnectorWrapper(ZookeeperEnsembleHostConnector);

#[pymethods]
impl ZookeeperEnsembleHostConnectorWrapper {
    #[new]
    pub fn new(hosts: Vec<String>, timeout: f32) -> Self {
        ZookeeperEnsembleHostConnectorWrapper(ZookeeperEnsembleHostConnector {
            hosts,
            timeout: Duration::from_secs_f32(timeout),
        })
    }

    pub fn connect<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let connector = self.0.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let host = SolrHostWrapper {
                solr_host: Arc::new(connector.connect().await.map_err(PyErrWrapper::from)?),
            };
            Ok(Python::with_gil(|_| host))
        })
    }

    pub fn connect_blocking(&self) -> PyResult<SolrHostWrapper> {
        let host = SolrHostWrapper {
            solr_host: Arc::new(
                self.0
                    .clone()
                    .connect_blocking()
                    .map_err(PyErrWrapper::from)?,
            ),
        };
        Ok(Python::with_gil(|_| host))
    }
}
