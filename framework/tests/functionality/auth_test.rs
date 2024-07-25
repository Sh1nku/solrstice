use crate::structures::BaseTestsBuildup;
use serial_test::parallel;
use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
use solrstice::models::auth::SolrBasicAuth;
use solrstice::models::context::SolrServerContextBuilder;
use solrstice::models::error::SolrError;

#[tokio::test]
#[parallel]
async fn auth_gives_sensible_error_when_not_provided() -> Result<(), SolrError> {
    let config = BaseTestsBuildup::new().await;
    if config.auth.is_none() {
        return Ok(());
    }
    let context = SolrServerContextBuilder::new(config.host).build();
    let client = AsyncSolrCloudClient::new(context);
    let response = client.get_collections().await;
    match response {
        Ok(_) => Err(SolrError::Unknown("Should not have succeeded".to_string())),
        Err(e) => match e {
            SolrError::SolrAuthError(_) => Ok(()),
            _ => Err(SolrError::Unknown(
                "Should have been auth error".to_string(),
            )),
        },
    }
}

#[tokio::test]
#[parallel]
async fn auth_gives_sensible_error_when_wrong() -> Result<(), SolrError> {
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
        Ok(_) => Err(SolrError::Unknown("Should not have succeeded".to_string())),
        Err(e) => match e {
            SolrError::SolrAuthError(_) => Ok(()),
            _ => Err(SolrError::Unknown(
                "Should have been auth error".to_string(),
            )),
        },
    }
}
