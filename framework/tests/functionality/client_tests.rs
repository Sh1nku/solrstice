use crate::structures::BaseTestsBuildup;
use serde::{Deserialize, Serialize};
use serial_test::parallel;
use solrstice::AsyncSolrCloudClient;
use solrstice::SelectQuery;
use solrstice::{DeleteQuery, UpdateQuery};
use std::path::Path;
use tokio::join;

#[derive(Serialize, Deserialize, Debug)]
struct TestData {
    id: String,
}

#[tokio::test]
#[parallel]
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
        .index(&UpdateQuery::new(), name, docs.as_slice())
        .await
        .unwrap();

    // Search documents
    let server_docs = client
        .select(&SelectQuery::new().fq(["id:example_document"]), name)
        .await
        .unwrap()
        .get_docs_response()
        .unwrap()
        .get_docs::<TestData>()
        .unwrap();
    assert_eq!(server_docs.len(), 1);

    // Delete documents
    client
        .delete(&DeleteQuery::new().ids(["example_document"]), name)
        .await
        .unwrap();

    // Delete collection
    client.delete_collection(name).await.unwrap();
    // Delete config
    client.delete_config(name).await.unwrap();
}

#[tokio::test]
#[parallel]
pub async fn multiple_clients_test() {
    let name = "multiple_clients_test".to_string();
    let config_1 = BaseTestsBuildup::new().await;
    let config_2 = BaseTestsBuildup::new().await;
    let client_1 = AsyncSolrCloudClient::new(config_1.context);
    let client_2 = AsyncSolrCloudClient::new(config_2.context);

    let _ = client_1.delete_config(&name).await;

    client_1
        .upload_config(&name, Path::new(&config_1.config_path))
        .await
        .unwrap();

    let configs_1_future = client_1.get_configs();
    let configs_2_future = client_2.get_configs();

    let configs_tup = join!(configs_1_future, configs_2_future);

    assert!(configs_tup.0.unwrap().contains(&name));
    assert!(configs_tup.1.unwrap().contains(&name));

    client_1.delete_config(&name).await.unwrap();
}
