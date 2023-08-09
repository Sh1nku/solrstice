use crate::structures::BaseTestsBuildup;
use serde::{Deserialize, Serialize};
use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
// use solrstice::hosts::solr_server_host::SolrSingleServerHost;
// use solrstice::models::auth::SolrBasicAuth;
// use solrstice::models::context::SolrServerContext;
use solrstice::models::error::SolrError;
use solrstice::queries::index::{DeleteQueryBuilder, UpdateQueryBuilder};
use solrstice::queries::select::SelectQueryBuilder;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct TestData {
    id: String,
}

#[tokio::test]
pub async fn example() -> Result<(), SolrError> {
    let config = BaseTestsBuildup::new().await;

    //Create a solr client. You can also use a list of zookeeper hosts instead of a single server.
    // let context = SolrServerContext::new(SolrSingleServerHost::new("http://localhost:8983"))
    //     .with_auth(SolrBasicAuth::new("solr", Some("SolrRocks")));
    let context = config.context;
    let client = AsyncSolrCloudClient::new(context);

    // Upload config
    client
        .upload_config("example_config", Path::new(&config.config_path))
        .await?;
    // client
    //     .upload_config("example_config", Path::new("/path/to/config"))
    //     .await?;

    // Create collection
    client
        .create_collection("example_collection", "example_config", 1, 1)
        .await?;

    // Index document
    let docs = vec![TestData {
        id: "example_document".to_string(),
    }];
    client
        .index(
            &UpdateQueryBuilder::new(),
            "example_collection",
            docs.as_slice(),
        )
        .await?;

    // Search and retrieve the document
    let docs = client
        .select(
            &SelectQueryBuilder::new().fq(&["id:example_document"]),
            "example_collection",
        )
        .await?
        .get_response()
        .ok_or("No response provided")?
        .get_docs::<TestData>()?;
    assert_eq!(docs.len(), 1);

    // Delete the document
    client
        .delete(
            &DeleteQueryBuilder::new().ids(&["example_document"]),
            "example_collection",
        )
        .await?;

    client.delete_collection("example_collection").await?;
    client.delete_config("example_config").await?;
    Ok(())
}
