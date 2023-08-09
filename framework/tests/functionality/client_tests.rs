use crate::structures::BaseTestsBuildup;
use serde::{Deserialize, Serialize};
use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
use solrstice::queries::index::{DeleteQueryBuilder, UpdateQueryBuilder};
use solrstice::queries::select::SelectQueryBuilder;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct TestData {
    id: String,
}

#[tokio::test]
pub async fn client_example_test() {
    let name = "example_test";
    let config = BaseTestsBuildup::new().await;
    let client = AsyncSolrCloudClient::new(config.context);

    let _ = client.delete_collection(name).await;
    let _ = client.delete_config(name).await;

    // Upload config
    client
        .upload_config(name, Path::new(&config.config_path))
        .await
        .unwrap();
    // Create collection
    client.create_collection(name, name, 1, 1).await.unwrap();

    // Index documents
    let docs = vec![TestData {
        id: "example_document".to_string(),
    }];
    client
        .index(&UpdateQueryBuilder::new(), name, docs.as_slice())
        .await
        .unwrap();

    // Search documents
    let server_docs = client
        .select(
            &SelectQueryBuilder::new().fq(&["id:example_document"]),
            name,
        )
        .await
        .unwrap()
        .get_response()
        .unwrap()
        .get_docs::<TestData>()
        .unwrap();
    assert_eq!(server_docs.len(), 1);

    // Delete documents
    client
        .delete(&DeleteQueryBuilder::new().ids(&["example_document"]), name)
        .await
        .unwrap();

    // Delete collection
    client.delete_collection(name).await.unwrap();
    // Delete config
    client.delete_config(name).await.unwrap();
}
