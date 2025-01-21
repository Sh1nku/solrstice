use crate::structures::{get_test_data, City, FunctionalityTestsBuildup};
use serial_test::parallel;
use solrstice::models::SolrDocsResponse;
use solrstice::Error;
use solrstice::SelectQuery;
use solrstice::UpdateQuery;
use std::collections::HashMap;

#[tokio::test]
#[parallel]
async fn select_works_when_no_result() -> Result<(), Error> {
    let config = FunctionalityTestsBuildup::build_up("SelectNoResult")
        .await
        .unwrap();

    let result = SelectQuery::new()
        .execute(&config.context, &config.collection_name)
        .await
        .unwrap();
    assert_eq!(
        result
            .get_docs_response()
            .unwrap()
            .get_docs::<City>()
            .unwrap()
            .len(),
        0
    );
    let _ = config.tear_down().await;
    Ok(())
}

#[tokio::test]
#[parallel]
async fn select_works_when_no_result_serde_value() -> Result<(), Error> {
    let config = FunctionalityTestsBuildup::build_up("SelectNoResultSerdeValue")
        .await
        .unwrap();

    let result = SelectQuery::new()
        .execute(&config.context, &config.collection_name)
        .await
        .unwrap();
    assert_eq!(
        result
            .get_docs_response()
            .unwrap()
            .get_docs::<serde_json::Value>()
            .unwrap()
            .len(),
        0
    );
    let _ = config.tear_down().await;
    Ok(())
}

#[tokio::test]
#[parallel]
async fn select_works_using_cursor_mark() -> Result<(), Error> {
    let config = FunctionalityTestsBuildup::build_up("SelectCursorMark")
        .await
        .unwrap();

    UpdateQuery::new()
        .execute(&config.context, &config.collection_name, &get_test_data())
        .await
        .unwrap();

    let mut cursor_mark = "*".to_string();
    let mut current_iteration = 0;
    loop {
        if current_iteration > 100 {
            panic!("Cursor mark test failed. Too many iterations");
        }
        let result = SelectQuery::new()
            .fq(["age:[* TO *]"])
            .rows(1)
            .cursor_mark(cursor_mark.as_str())
            .sort(["id desc"])
            .execute(&config.context, &config.collection_name)
            .await
            .unwrap();
        if let Some(next_cursor_mark) = result.next_cursor_mark {
            if cursor_mark.as_str() == "*" {
                return Ok(());
            }
            cursor_mark = next_cursor_mark;
        } else {
            panic!("Cursor mark test failed. No next cursor mark")
        }
        current_iteration += 1;
    }
}

#[tokio::test]
#[parallel]
async fn select_works_with_additional_params() -> Result<(), Error> {
    let config = FunctionalityTestsBuildup::build_up("SelectAdditionalParams")
        .await
        .unwrap();
    UpdateQuery::new()
        .execute(&config.context, &config.collection_name, &get_test_data())
        .await
        .unwrap();
    let mut params = HashMap::new();
    params.insert("child.q", "*:*");

    let result = SelectQuery::new()
        .q("{!parent which=city_name:*}")
        .fl(["id", "city_name", "child:[subquery]"])
        .additional_params(params)
        .execute(&config.context, &config.collection_name)
        .await
        .unwrap();
    let docs = result
        .get_docs_response()
        .unwrap()
        .get_docs::<serde_json::Value>()
        .unwrap();
    assert_eq!(docs.len(), 2);
    assert!(docs[0].get("child").is_some());

    let child_response = docs[0].get("child").unwrap();
    let child_response: SolrDocsResponse = serde_json::from_value(child_response.clone()).unwrap();
    assert!(child_response.get_num_found() > 0);

    let _ = config.tear_down().await;
    Ok(())
}
