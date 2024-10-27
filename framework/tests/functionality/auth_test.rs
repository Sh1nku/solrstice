use crate::structures::BaseTestsBuildup;
use serial_test::parallel;
use solrstice::AsyncSolrCloudClient;
use solrstice::Error;
use solrstice::SolrBasicAuth;
use solrstice::SolrServerContextBuilder;

#[tokio::test]
#[parallel]
async fn auth_gives_sensible_error_when_not_provided() -> Result<(), Error> {
    let config = BaseTestsBuildup::new().await;
    if config.auth.is_none() {
        return Ok(());
    }
    let context = SolrServerContextBuilder::new(config.host).build();
    let client = AsyncSolrCloudClient::new(context);
    let response = client.get_collections().await;
    match response {
        Ok(_) => Err(Error::Unknown("Should not have succeeded".to_string())),
        Err(e) => match e {
            Error::SolrAuthError { .. } => Ok(()),
            _ => Err(Error::Unknown("Should have been auth error".to_string())),
        },
    }
}

#[tokio::test]
#[parallel]
async fn auth_gives_sensible_error_when_wrong() -> Result<(), Error> {
    let config = BaseTestsBuildup::new().await;
    if config.auth.is_none() {
        return Ok(());
    }
    let context = SolrServerContextBuilder::new(config.host)
        .with_auth(SolrBasicAuth::new("BAD", "BAD"))
        .build();
    let client = AsyncSolrCloudClient::new(context);
    let response = client.get_collections().await;
    match response {
        Ok(_) => Err(Error::Unknown("Should not have succeeded".to_string())),
        Err(e) => match e {
            Error::SolrAuthError { .. } => Ok(()),
            _ => Err(Error::Unknown("Should have been auth error".to_string())),
        },
    }
}
