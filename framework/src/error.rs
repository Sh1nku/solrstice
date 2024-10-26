use crate::models::response::SolrResponse;
use thiserror::Error;

/// Main error type for Solrstice
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    ZipError(#[from] zip::result::ZipError),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    ZkError(#[from] zookeeper_async::ZkError),

    #[error(transparent)]
    StripPrefixError(#[from] std::path::StripPrefixError),

    #[error("Solr setup error: {0}")]
    SolrSetupError(String),
    #[error("Solr connection error: {code:?} - {url:?}\n{msg:?}")]
    SolrConnectionError { code: u16, url: String, msg: String },
    #[error("Solr response error: {code:?} - {url:?}\n{msg:?}")]
    SolrResponseError { code: u16, url: String, msg: String },
    #[error("Solr auth error: {code:?} - {url:?}\n{msg:?}")]
    SolrAuthError { code: u16, url: String, msg: String },

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::Unknown(err.to_string())
    }
}

/// Helper function to check if a SolrResponse contains an error
pub fn try_solr_error(url: String, response: &SolrResponse) -> Result<(), Error> {
    match &response.error {
        None => Ok(()),
        Some(err) => {
            let mut msg = "Unknown Solr Error".to_string();
            if err.msg.is_some() {
                msg = err.msg.clone().unwrap();
            } else if err.trace.is_some() {
                msg = err.trace.clone().unwrap();
            }
            Err(Error::SolrResponseError {
                code: err.code,
                url,
                msg,
            })
        }
    }
}
