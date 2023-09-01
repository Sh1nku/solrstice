use crate::models::response::SolrResponse;
use thiserror::Error;

/// Main error type for Solrstice
#[derive(Error, Debug)]
pub enum SolrError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    ZipError(#[from] zip::result::ZipError),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("Error from Solr {code:?}: {msg:?}")]
    SolrResponseError { code: usize, msg: String },
    #[error("Authentication error: {0}")]
    SolrAuthError(String),
    #[error(transparent)]
    ZkError(#[from] zookeeper_async::ZkError),

    #[error(transparent)]
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
