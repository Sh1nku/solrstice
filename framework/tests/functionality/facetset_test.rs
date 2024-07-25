use crate::structures::{get_test_data, FunctionalityTestsBuildup};
use serial_test::parallel;
use solrstice::Error;
use solrstice::SelectQuery;
use solrstice::UpdateQuery;
use solrstice::{FacetSetComponent, FieldFacetComponent, FieldFacetEntry, PivotFacetComponent};

#[tokio::test]
#[parallel]
pub async fn test_facet_pivot_works() -> Result<(), Error> {
    let config = FunctionalityTestsBuildup::build_up("FacetPivot")
        .await
        .unwrap();
    let update = UpdateQuery::new();
    update
        .execute(&config.context, &config.collection_name, &get_test_data())
        .await?;

    let query = SelectQuery::new()
        .facet_set(&FacetSetComponent::new().pivots(&PivotFacetComponent::new(["interests,age"])));
    let response = config
        .async_client
        .select(&query, &config.collection_name)
        .await?;
    let facets = response.get_facet_set().ok_or("No facets")?;
    let pivot = facets.get_pivots();
    let interests_age = pivot.get("interests,age").ok_or("No interests,age pivot")?;
    assert_eq!(interests_age.len(), 3);
    let cars_pivot = interests_age
        .iter()
        .find(|p| match p.get_value::<String>() {
            Ok(val) => val.as_str() == "cars",
            Err(_) => false,
        })
        .ok_or("No cars pivot")?;
    assert_eq!(cars_pivot.get_count(), 1);
    let age_pivot = cars_pivot
        .get_pivots()
        .first()
        .ok_or("No age pivot in cars")?;
    assert_eq!(age_pivot.get_value::<usize>()?, 20);

    let _ = config.tear_down().await;
    Ok(())
}

#[tokio::test]
#[parallel]
pub async fn test_facet_query_works() -> Result<(), Error> {
    let config = FunctionalityTestsBuildup::build_up("FacetQuery")
        .await
        .unwrap();
    let update = UpdateQuery::new();
    update
        .execute(&config.context, &config.collection_name, &get_test_data())
        .await?;

    let query = SelectQuery::new().facet_set(&FacetSetComponent::new().queries(["age:[0 TO 59]"]));
    let response = config
        .async_client
        .select(&query, &config.collection_name)
        .await?;
    let facets = response.get_facet_set().ok_or("No facets")?;
    let queries = facets.get_queries();
    let query = queries.get("age:[0 TO 59]").ok_or("No age query")?;
    assert_eq!(*query, 4);

    let _ = config.tear_down().await;
    Ok(())
}

#[tokio::test]
#[parallel]
pub async fn test_facet_field_works() -> Result<(), Error> {
    let config = FunctionalityTestsBuildup::build_up("FacetField")
        .await
        .unwrap();
    let update = UpdateQuery::new();
    update
        .execute(&config.context, &config.collection_name, &get_test_data())
        .await?;

    let query = SelectQuery::new().facet_set(
        FacetSetComponent::new().fields(FieldFacetComponent::new([FieldFacetEntry::new("age")])),
    );
    let response = config
        .async_client
        .select(&query, &config.collection_name)
        .await?;
    let facets = response.get_facet_set().ok_or("No facets")?;
    let fields = facets.get_fields();
    let age = fields.get("age").ok_or("No age field")?;
    assert_eq!(age.len(), 3);

    let _ = config.tear_down().await;
    Ok(())
}

#[tokio::test]
#[parallel]
pub async fn test_facet_field_exclude_works() -> Result<(), Error> {
    let config = FunctionalityTestsBuildup::build_up("FacetFieldExclude")
        .await
        .unwrap();
    let update = UpdateQuery::new();
    update
        .execute(&config.context, &config.collection_name, &get_test_data())
        .await?;

    let query = SelectQuery::new().facet_set(
        FacetSetComponent::new().fields(
            FieldFacetComponent::new([FieldFacetEntry::new("interests")])
                .exclude_terms("cars,partying"),
        ),
    );
    let response = config
        .async_client
        .select(&query, &config.collection_name)
        .await?;
    let facets = response.get_facet_set().ok_or("No facets")?;
    let fields = facets.get_fields();
    let age = fields.get("interests").ok_or("No interests field")?;
    assert_eq!(age.len(), 1);

    let _ = config.tear_down().await;
    Ok(())
}

#[tokio::test]
#[parallel]
pub async fn test_facet_field_exclude_works_missing() -> Result<(), Error> {
    let config = FunctionalityTestsBuildup::build_up("FacetFieldMissing")
        .await
        .unwrap();
    let update = UpdateQuery::new();
    update
        .execute(&config.context, &config.collection_name, &get_test_data())
        .await?;

    let query =
        SelectQuery::new().facet_set(FacetSetComponent::new().fields(FieldFacetComponent::new([
            FieldFacetEntry::new("interests").missing(true),
        ])));
    let response = config
        .async_client
        .select(&query, &config.collection_name)
        .await?;
    let facets = response.get_facet_set().ok_or("No facets")?;
    let fields = facets.get_fields();
    let interests = fields.get("interests").ok_or("No interests field")?;
    assert_eq!(interests.len(), 4);

    let _ = config.tear_down().await;
    Ok(())
}
