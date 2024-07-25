use crate::structures::{get_test_data, FunctionalityTestsBuildup, Population};
use serial_test::parallel;
use solrstice::Error;
use solrstice::SelectQuery;
use solrstice::UpdateQuery;
use solrstice::{DefType, DismaxQuery, EdismaxQuery, LuceneQuery};

#[tokio::test]
#[parallel]
pub async fn test_dismax_query_parser() -> Result<(), Error> {
    let config = FunctionalityTestsBuildup::build_up("Dismax").await.unwrap();
    let update = UpdateQuery::new();
    update
        .execute(&config.context, &config.collection_name, &get_test_data())
        .await?;

    let query = SelectQuery::new().q("outdoors").def_type(
        DismaxQuery::new()
            .qf("interests^20")
            .bq(["interests:cars^20"]),
    );
    let response = query
        .execute(&config.context, &config.collection_name)
        .await?;
    let response = response.get_docs_response().ok_or("No response")?;
    let docs = response.get_docs::<Population>()?;
    let first_doc = docs.first().ok_or("No docs in result")?;
    assert_eq!(first_doc.id, "city_Alta_20");

    let _ = config.tear_down().await;
    Ok(())
}

#[tokio::test]
#[parallel]
pub async fn test_edismax_query_parser() -> Result<(), Error> {
    let config = FunctionalityTestsBuildup::build_up("Edismax")
        .await
        .unwrap();
    let update = UpdateQuery::new();
    update
        .execute(&config.context, &config.collection_name, &get_test_data())
        .await?;

    let response = SelectQuery::new()
        .q("outdoors")
        .def_type(&DefType::Edismax(
            EdismaxQuery::new()
                .qf("interests^20")
                .bq(["interests:cars^20"]),
        ))
        .execute(&config.context, &config.collection_name)
        .await?;
    let response = response.get_docs_response().ok_or("No response")?;
    let docs = response.get_docs::<Population>()?;
    let first_doc = docs.first().ok_or("No docs in result")?;
    assert_eq!(first_doc.id, "city_Alta_20");

    let _ = config.tear_down().await;
    Ok(())
}

#[tokio::test]
#[parallel]
pub async fn test_lucene_query_parser() -> Result<(), Error> {
    let config = FunctionalityTestsBuildup::build_up("Lucene").await.unwrap();
    let update = UpdateQuery::new();
    update
        .execute(&config.context, &config.collection_name, &get_test_data())
        .await?;

    let response = SelectQuery::new()
        .q("outdoors")
        .def_type(&DefType::Lucene(LuceneQuery::new().df("interests")))
        .execute(&config.context, &config.collection_name)
        .await?;
    let response = response.get_docs_response().ok_or("No response")?;
    let _ = response.get_docs::<Population>()?;
    let _ = config.tear_down().await;
    Ok(())
}
