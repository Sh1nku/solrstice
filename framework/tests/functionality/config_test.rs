use crate::structures::BaseTestsBuildup;
use serial_test::parallel;
use solrstice::queries::config::{config_exists, delete_config, get_configs, upload_config};
use solrstice::Error;
use std::path::Path;

#[tokio::test]
#[parallel]
async fn upload_config_uploads_config() -> Result<(), Error> {
    let config = BaseTestsBuildup::new().await;
    let _ = delete_config(&config.context, "UploadConfig").await;
    assert!(!config_exists(&config.context, "UploadConfig")
        .await
        .unwrap());
    upload_config(
        &config.context,
        "UploadConfig",
        Path::new(&config.config_path),
    )
    .await
    .unwrap();
    assert!(config_exists(&config.context, "UploadConfig")
        .await
        .unwrap());
    delete_config(&config.context, "UploadConfig")
        .await
        .unwrap();
    Ok(())
}

#[tokio::test]
#[parallel]
async fn get_configs_gets_configs() -> Result<(), Error> {
    let config = BaseTestsBuildup::new().await;
    let configs = get_configs(&config.context).await.unwrap();
    assert!(configs.contains(&"_default".to_string()));
    Ok(())
}

#[tokio::test]
#[parallel]
async fn delete_config_deletes_config() -> Result<(), Error> {
    let config = BaseTestsBuildup::new().await;
    let _ = delete_config(&config.context, "DeleteConfig").await;
    upload_config(
        &config.context,
        "DeleteConfig",
        Path::new(&config.config_path),
    )
    .await
    .unwrap();
    assert!(config_exists(&config.context, "DeleteConfig")
        .await
        .unwrap());
    delete_config(&config.context, "DeleteConfig")
        .await
        .unwrap();
    assert!(!config_exists(&config.context, "DeleteConfig")
        .await
        .unwrap());
    Ok(())
}

#[tokio::test]
#[parallel]
async fn config_exists_works_when_config_exists() -> Result<(), Error> {
    let config = BaseTestsBuildup::new().await;
    assert!(config_exists(&config.context, "_default").await.unwrap());
    Ok(())
}

#[tokio::test]
#[parallel]
async fn config_exists_works_when_config_doesent_exist() -> Result<(), Error> {
    let config = BaseTestsBuildup::new().await;
    assert!(!config_exists(&config.context, "_this_does_not_exist")
        .await
        .unwrap());
    Ok(())
}
