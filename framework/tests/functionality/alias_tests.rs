use crate::structures::BaseTestsBuildup;
use serial_test::parallel;
use solrstice::queries::alias::{alias_exists, create_alias, delete_alias};
use solrstice::queries::collection::{create_collection, delete_collection};
use solrstice::queries::config::{delete_config, upload_config};
use std::path::Path;

#[tokio::test]
#[parallel]
async fn create_alias_creates_alias() {
    let alias_name = "CreateAliasAlias".to_string();
    let collection_name = "CreateAliasCollection".to_string();
    let config_name = "CreateAliasCollection".to_string();

    let config = BaseTestsBuildup::new().await;

    let _ = delete_alias(&config.context, &alias_name).await;
    let _ = delete_collection(&config.context, &collection_name).await;
    let _ = delete_config(&config.context, &config_name).await;

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
        alias_exists(&config.context, &alias_name).await.unwrap(),
        false
    );
    create_alias(&config.context, &alias_name, &[&collection_name])
        .await
        .unwrap();
    assert_eq!(
        alias_exists(&config.context, &alias_name).await.unwrap(),
        true
    );

    let _ = delete_alias(&config.context, &alias_name).await.unwrap();
    let _ = delete_collection(&config.context, &collection_name).await;
    let _ = delete_config(&config.context, &config_name).await;
}
