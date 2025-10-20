use crate::structures::ErrrorTestsSetup;
use reqwest::Client;
use serial_test::serial;
use solrstice::{AsyncSolrCloudClient, Error, SelectQuery, SolrServerContextBuilder};

#[tokio::test]
#[serial]
async fn sensible_error_message_if_not_solr_server() -> Result<(), Error> {
    let config = ErrrorTestsSetup::new().await;
    let client = AsyncSolrCloudClient::new(config.http_context);

    let result = client.select(SelectQuery::new(), "error_collection").await;
    assert!(result.is_err() && result.unwrap_err().to_string().contains("500"));
    Ok(())
}

#[tokio::test]
#[serial]
async fn sensible_error_message_if_non_existent_collection() -> Result<(), Error> {
    let config = ErrrorTestsSetup::new().await;
    let client = AsyncSolrCloudClient::new(config.http_context);

    let result = client
        .select(SelectQuery::new(), "notfound_collection")
        .await;
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("404"));
    Ok(())
}

#[tokio::test]
#[serial]
async fn sensible_error_message_if_200_but_not_solr() -> Result<(), Error> {
    let config = ErrrorTestsSetup::new().await;
    let client = AsyncSolrCloudClient::new(config.http_context);

    let result = client.select(SelectQuery::new(), "always_200").await;
    assert!(result.is_err() && result.unwrap_err().to_string().contains("200"));
    Ok(())
}

#[tokio::test]
#[serial]
async fn error_if_invalid_certificate() -> Result<(), Error> {
    let config = ErrrorTestsSetup::new().await;
    let client = AsyncSolrCloudClient::new(SolrServerContextBuilder::new(config.https_host));

    let result = client
        .select(SelectQuery::new(), "notfound_collection")
        .await;
    assert!(result.is_err() && !result.unwrap_err().to_string().contains("404"));
    Ok(())
}

#[tokio::test]
#[serial]
async fn no_error_if_invalid_certificate_but_danger_accept() -> Result<(), Error> {
    let config = ErrrorTestsSetup::new().await;
    let context = SolrServerContextBuilder::new(config.https_host)
        .with_client(
            Client::builder()
                .danger_accept_invalid_certs(true)
                .build()?,
        )
        .build();
    let client = AsyncSolrCloudClient::new(context);

    let result = client.select(SelectQuery::new(), "always_200").await;
    assert!(result.is_err() && result.unwrap_err().to_string().contains("200"));
    Ok(())
}
