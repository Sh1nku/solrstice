use crate::structures::{get_test_data, FunctionalityTestsBuildup};
use serial_test::parallel;
use solrstice::{Error, SelectQuery, StatsComponent, UpdateQuery};

#[tokio::test]
#[parallel]
async fn stats_works() -> Result<(), Error> {
    let config = FunctionalityTestsBuildup::build_up("StatsWorks")
        .await
        .unwrap();
    UpdateQuery::new()
        .execute(&config.context, &config.collection_name, &get_test_data())
        .await
        .unwrap();

    let result = SelectQuery::new()
        .stats(StatsComponent::new().fields(["age"]))
        .execute(&config.context, &config.collection_name)
        .await
        .unwrap();
    assert!(result.get_stats().unwrap().get_fields()["age"].get_count() > 0);
    let _ = config.tear_down().await;
    Ok(())
}

#[tokio::test]
#[parallel]
async fn stats_works_string_field() -> Result<(), Error> {
    let config = FunctionalityTestsBuildup::build_up("StatsWorksStringField")
        .await
        .unwrap();
    UpdateQuery::new()
        .execute(&config.context, &config.collection_name, &get_test_data())
        .await
        .unwrap();

    let result = SelectQuery::new()
        .stats(StatsComponent::new().fields(["id"]))
        .execute(&config.context, &config.collection_name)
        .await
        .unwrap();
    assert!(result.get_stats().unwrap().get_fields()["id"].get_count() > 0);
    assert!(result.get_stats().unwrap().get_fields()["id"]
        .get_mean::<String>()
        .is_none());
    assert!(
        result.get_stats().unwrap().get_fields()["id"]
            .get_min::<String>()
            .unwrap()
            .len()
            > 0
    );
    let _ = config.tear_down().await;
    Ok(())
}
