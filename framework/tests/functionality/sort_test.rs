use crate::structures::{get_test_data, City, FunctionalityTestsBuildup, Population};
use serial_test::parallel;
use solrstice::{Error, SelectQuery, UpdateQuery};

#[tokio::test]
#[parallel]
async fn sort_works_with_single_field() -> Result<(), Error> {
    let config = FunctionalityTestsBuildup::build_up("SortSingleField")
        .await
        .unwrap();
    UpdateQuery::new()
        .execute(&config.context, &config.collection_name, &get_test_data())
        .await
        .unwrap();

    let ascending_result = SelectQuery::new()
        .fq(["city_name:[* TO *]"])
        .fl(["*", "[child limit=1000]"])
        .sort("city_name asc")
        .execute(&config.context, &config.collection_name)
        .await
        .unwrap();
    let docs = ascending_result
        .get_docs_response()
        .unwrap()
        .get_docs::<City>()
        .unwrap();
    assert_eq!(docs[0].city_name, "Alta");
    assert_eq!(docs[docs.len() - 1].city_name, "Tromsø");
    let descending_result = SelectQuery::new()
        .fq(["city_name:[* TO *]"])
        .fl(["*", "[child limit=1000]"])
        .sort("city_name desc")
        .execute(&config.context, &config.collection_name)
        .await
        .unwrap();
    let docs = descending_result
        .get_docs_response()
        .unwrap()
        .get_docs::<City>()
        .unwrap();
    assert_eq!(docs[0].city_name, "Tromsø");
    assert_eq!(docs[docs.len() - 1].city_name, "Alta");

    let _ = config.tear_down().await;
    Ok(())
}

#[tokio::test]
#[parallel]
async fn sort_works_with_multiple_fields() -> Result<(), Error> {
    let config = FunctionalityTestsBuildup::build_up("SortMultipleFields")
        .await
        .unwrap();
    UpdateQuery::new()
        .execute(&config.context, &config.collection_name, &get_test_data())
        .await
        .unwrap();

    let ascending_result = SelectQuery::new()
        .fq(["-city_name:[* TO *]"])
        .sort("age asc, count asc")
        .execute(&config.context, &config.collection_name)
        .await
        .unwrap();
    let docs = ascending_result
        .get_docs_response()
        .unwrap()
        .get_docs::<Population>()
        .unwrap();
    assert_eq!(docs[0].age, docs[1].age);
    assert!(docs[0].count <= docs[1].count);

    let descending_result = SelectQuery::new()
        .fq(["-city_name:[* TO *]"])
        .sort("age desc, count desc")
        .execute(&config.context, &config.collection_name)
        .await
        .unwrap();
    let docs = descending_result
        .get_docs_response()
        .unwrap()
        .get_docs::<Population>()
        .unwrap();
    assert_eq!(docs[0].age, docs[1].age);
    assert!(docs[0].count >= docs[1].count);

    Ok(())
}
