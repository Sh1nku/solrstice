use crate::models::error::PyErrWrapper;
use pyo3::prelude::*;
use solrstice::Error;
use std::time::Duration;

#[pyclass(name = "ReqwestClient", module = "solrstice.external")]
#[derive(Clone)]
pub struct ReqwestClientWrapper(pub reqwest::Client);

#[pymethods]
impl ReqwestClientWrapper {
    #[new]
    pub fn new(
        timeout: Option<u64>,
        read_timeout: Option<u64>,
        connect_timeout: Option<u64>,
        danger_accept_invalid_hostnames: Option<bool>,
        danger_accept_invalid_certs: Option<bool>,
    ) -> PyResult<Self> {
        let mut builder = reqwest::Client::builder();
        if let Some(timeout) = timeout {
            builder = builder.timeout(Duration::from_millis(timeout));
        }
        if let Some(read_timeout) = read_timeout {
            builder = builder.read_timeout(Duration::from_millis(read_timeout));
        }
        if let Some(connect_timeout) = connect_timeout {
            builder = builder.connect_timeout(Duration::from_millis(connect_timeout));
        }
        if let Some(accept_invalid_hostnames) = danger_accept_invalid_hostnames {
            builder = builder.danger_accept_invalid_hostnames(accept_invalid_hostnames);
        }
        if let Some(accept_invalid_certs) = danger_accept_invalid_certs {
            builder = builder.danger_accept_invalid_certs(accept_invalid_certs);
        }
        Ok(ReqwestClientWrapper(
            builder
                .build()
                .map_err(Error::from)
                .map_err(PyErrWrapper::from)?,
        ))
    }
}
