use crate::structures::ErrrorTestsSetup;
use serial_test::serial;
use solrstice::{AsyncSolrCloudClient, Error, SelectQuery};

#[tokio::test]
#[serial]
async fn sensible_error_message_if_not_solr_server() -> Result<(), Error> {
    let config = ErrrorTestsSetup::new().await;
    let client = AsyncSolrCloudClient::new(config.context);

    let result = client.select(SelectQuery::new(), "error_collection").await;
    assert!(
        result.is_err()
            && result
                .unwrap_err()
                .to_string()
                .contains("500 Internal Server Error")
    );
    Ok(())
}

#[tokio::test]
#[serial]
async fn sensible_error_message_if_non_existent_collection() -> Result<(), Error> {
    let config = ErrrorTestsSetup::new().await;
    let client = AsyncSolrCloudClient::new(config.context);

    let result = client
        .select(SelectQuery::new(), "notfound_collection")
        .await;
    assert!(result.is_err() && result.unwrap_err().to_string().contains("404 Not Found"));
    Ok(())
}

#[tokio::test]
#[serial]
async fn sensible_error_message_if_200_but_not_solr() -> Result<(), Error> {
    let config = ErrrorTestsSetup::new().await;
    let client = AsyncSolrCloudClient::new(config.context);

    let result = client.select(SelectQuery::new(), "always_200").await;
    assert!(result.is_err() && result.unwrap_err().to_string().contains("200 OK"));
    Ok(())
}
