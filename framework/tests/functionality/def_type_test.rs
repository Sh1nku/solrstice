use crate::structures::{get_test_data, FunctionalityTestsBuildup, Population};
use solrstice::models::error::SolrError;
use solrstice::queries::def_type::{DefType, Dismax, Edismax, Lucene};
use solrstice::queries::index::UpdateQueryBuilder;
use solrstice::queries::select::SelectQueryBuilder;

#[tokio::test]
pub async fn test_dismax_query_parser() -> Result<(), SolrError> {
    let config = FunctionalityTestsBuildup::build_up("Dismax").await.unwrap();
    let update = UpdateQueryBuilder::new();
    update
        .execute(&config.context, &config.collection_name, &get_test_data())
        .await?;

    let query = SelectQueryBuilder::new()
        .q("outdoors")
        .def_type(&DefType::Dismax(
            Dismax::new().qf("interests^20").bq(&["interests:cars^20"]),
        ));
    let response = query
        .execute(&config.context, &config.collection_name)
        .await?;
    let response = response.get_response().ok_or("No response")?;
    let docs = response.get_docs::<Population>()?;
    let first_doc = docs.first().ok_or("No docs in result")?;
    assert_eq!(first_doc.id, "city_Alta_20");

    let _ = config.tear_down().await;
    Ok(())
}

#[tokio::test]
pub async fn test_edismax_query_parser() -> Result<(), SolrError> {
    let config = FunctionalityTestsBuildup::build_up("Edismax")
        .await
        .unwrap();
    let update = UpdateQueryBuilder::new();
    update
        .execute(&config.context, &config.collection_name, &get_test_data())
        .await?;

    let response = SelectQueryBuilder::new()
        .q("outdoors")
        .def_type(&DefType::Edismax(
            Edismax::new().qf("interests^20").bq(&["interests:cars^20"]),
        ))
        .execute(&config.context, &config.collection_name)
        .await?;
    let response = response.get_response().ok_or("No response")?;
    let docs = response.get_docs::<Population>()?;
    let first_doc = docs.first().ok_or("No docs in result")?;
    assert_eq!(first_doc.id, "city_Alta_20");

    let _ = config.tear_down().await;
    Ok(())
}

#[tokio::test]
pub async fn test_lucene_query_parser() -> Result<(), SolrError> {
    let config = FunctionalityTestsBuildup::build_up("Lucene").await.unwrap();
    let update = UpdateQueryBuilder::new();
    update
        .execute(&config.context, &config.collection_name, &get_test_data())
        .await?;

    let response = SelectQueryBuilder::new()
        .q("outdoors")
        .def_type(&DefType::Lucene(Lucene::new().df("interests")))
        .execute(&config.context, &config.collection_name)
        .await?;
    let response = response.get_response().ok_or("No response")?;
    let _ = response.get_docs::<Population>()?;
    let _ = config.tear_down().await;
    Ok(())
}
