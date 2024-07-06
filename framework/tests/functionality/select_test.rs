use crate::structures::{get_test_data, City, FunctionalityTestsBuildup};
use serial_test::parallel;
use solrstice::models::error::SolrError;
use solrstice::queries::index::UpdateQuery;
use solrstice::queries::select::SelectQuery;

#[tokio::test]
#[parallel]
async fn select_works_when_no_result() -> Result<(), SolrError> {
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
async fn select_works_when_no_result_serde_value() -> Result<(), SolrError> {
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
async fn select_works_using_cursor_mark() -> Result<(), SolrError> {
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
