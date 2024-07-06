use crate::structures::BaseTestsBuildup;
use serial_test::parallel;
use solrstice::queries::collection::{collection_exists, create_collection, delete_collection};
use solrstice::queries::config::{delete_config, upload_config};
use std::path::Path;

#[tokio::test]
#[parallel]
async fn create_collection_creates_collection() {
    let config_name = "CreateCollectionConfig".to_string();
    let collection_name = "CreateCollectionCollection".to_string();

    let config = BaseTestsBuildup::new().await;
    let _ = delete_collection(&config.context, &collection_name).await;
    let _ = delete_config(&config.context, &config_name).await;

    assert_eq!(
        collection_exists(&config.context, &collection_name)
            .await
            .unwrap(),
        false
    );
    upload_config(
        &config.context,
        &config_name,
        Path::new(&config.config_path),
    )
    .await
    .unwrap();
    create_collection(&config.context, &collection_name, &config_name, 1, 1)
        .await
        .unwrap();
    assert_eq!(
        collection_exists(&config.context, &collection_name)
            .await
            .unwrap(),
        true
    );

    let _ = delete_collection(&config.context, &collection_name)
        .await
        .unwrap();
    let _ = delete_config(&config.context, &config_name);
}
