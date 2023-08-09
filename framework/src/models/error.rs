use crate::models::response::SolrResponse;
use thiserror::Error;

/// Main error type for Solrstice
#[derive(Error, Debug)]
pub enum SolrError {
    #[error("HTTP Request failed: {}", .0)]
    ReqwestError(#[from] reqwest::Error),
    #[error("IO Error: {}", .0)]
    IOError(#[from] std::io::Error),
    #[error("Zip Error: {}", .0)]
    ZipError(#[from] zip::result::ZipError),

    #[error("Serde failed: {}", .0)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("Error from Solr {code:?}: {msg:?}")]
    SolrResponseError { code: usize, msg: String },
    #[error("Zookeeper error: {}", .0)]
    ZkError(#[from] zookeeper_async::ZkError),

    #[error("Strip prefix error: {}", .0)]
    StripPrefixError(#[from] std::path::StripPrefixError),

    #[error("Solr Connection error: {0}")]
    SolrConnectionError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<&str> for SolrError {
    fn from(err: &str) -> Self {
        SolrError::Unknown(err.to_string())
    }
}

/// Helper function to check if a SolrResponse contains an error
pub fn try_solr_error(response: &SolrResponse) -> Result<(), SolrError> {
    match &response.error {
        None => Ok(()),
        Some(err) => {
            let mut msg = "Unknown Solr Error".to_string();
            if err.msg.is_some() {
                msg = err.msg.clone().unwrap();
            } else if err.trace.is_some() {
                msg = err.trace.clone().unwrap();
            }
            Err(SolrError::SolrResponseError {
                code: err.code,
                msg,
            })
        }
    }
}
