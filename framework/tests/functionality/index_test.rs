use crate::structures::{get_test_data, BaseTestsBuildup, City, FunctionalityTestsBuildup};
use serial_test::parallel;
use solrstice::queries::collection::{create_collection, delete_collection};
use solrstice::queries::config::{delete_config, upload_config};
use solrstice::Error;
use solrstice::SelectQuery;
use solrstice::{DeleteQuery, UpdateQuery};
use std::path::Path;

#[tokio::test]
#[parallel]
async fn index_indexes_documents() -> Result<(), Error> {
    let config = BaseTestsBuildup::new().await;
    let config_name = "IndexConfig".to_string();
    let collection_name = "IndexCollection".to_string();

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

    let update = UpdateQuery::new();
    update
        .execute(&config.context, &collection_name, &get_test_data())
        .await?;

    delete_collection(&config.context, &collection_name)
        .await
        .unwrap();
    delete_config(&config.context, &config_name).await.unwrap();
    Ok(())
}

#[tokio::test]
#[parallel]
async fn index_indexes_correct_documents() -> Result<(), Error> {
    let config = BaseTestsBuildup::new().await;
    let config_name = "IndexCorrectConfig".to_string();
    let collection_name = "IndexCorrectCollection".to_string();

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

    let update = UpdateQuery::new();
    update
        .execute(&config.context, &collection_name, &get_test_data())
        .await?;

    let returned_data = SelectQuery::new()
        .fl(["*", "[child]"])
        .fq(["city_name:[* TO *]"])
        .execute(&config.context, &collection_name)
        .await
        .unwrap()
        .get_docs_response()
        .unwrap()
        .get_docs::<City>()
        .unwrap();
    assert_eq!(returned_data, get_test_data());

    delete_collection(&config.context, &collection_name)
        .await
        .unwrap();
    delete_config(&config.context, &config_name).await.unwrap();
    Ok(())
}

#[tokio::test]
#[parallel]
async fn delete_deletes_documents_by_id() {
    let test_data_name = "DeleteDeletesById".to_string();
    let config = FunctionalityTestsBuildup::build_up(&test_data_name)
        .await
        .unwrap();
    let update = UpdateQuery::new();
    update
        .execute(&config.context, &config.collection_name, &get_test_data())
        .await
        .unwrap();
    let num_found = SelectQuery::new()
        .execute(&config.context, &config.collection_name)
        .await
        .unwrap()
        .get_docs_response()
        .unwrap()
        .get_num_found();
    assert_ne!(num_found, 0);

    DeleteQuery::new()
        .queries(["*:*"])
        .execute(&config.context, &config.collection_name)
        .await
        .unwrap();

    let num_found = SelectQuery::new()
        .execute(&config.context, &config.collection_name)
        .await
        .unwrap()
        .get_docs_response()
        .unwrap()
        .get_num_found();
    assert_eq!(num_found, 0);
}
