use crate::structures::{get_test_data, FunctionalityTestsBuildup};
use serial_test::parallel;
use solrstice::models::SolrJsonFacetResponse;
use solrstice::{Error, SelectQuery, UpdateQuery};
use solrstice::{JsonFacetComponent, JsonQueryFacet, JsonStatFacet, JsonTermsFacet};

#[tokio::test]
#[parallel]
pub async fn test_json_query_facet_works() -> Result<(), Error> {
    let config = FunctionalityTestsBuildup::build_up("JsonFacetQuery")
        .await
        .unwrap();
    let update = UpdateQuery::new();
    update
        .execute(&config.context, &config.collection_name, &get_test_data())
        .await?;

    let query = SelectQuery::new().json_facet(
        JsonFacetComponent::new().facets([("below_60", JsonQueryFacet::new("age:[0 TO 59]"))]),
    );
    let response = config
        .async_client
        .select(&query, &config.collection_name)
        .await?;
    let facets = response.get_json_facets().ok_or("No facets")?;
    let below_60 = facets
        .get_nested_facets()
        .get("below_60")
        .ok_or("No below_60 facet")?;
    assert_eq!(below_60.get_count().ok_or("No count")?, 4);

    let _ = config.tear_down().await;
    Ok(())
}

#[tokio::test]
#[parallel]
pub async fn test_json_stat_facet_works() -> Result<(), Error> {
    let config = FunctionalityTestsBuildup::build_up("JsonFacetStats")
        .await
        .unwrap();
    let update = UpdateQuery::new();
    update
        .execute(&config.context, &config.collection_name, &get_test_data())
        .await?;

    let query = SelectQuery::new().json_facet(
        JsonFacetComponent::new().facets([("total_people", JsonStatFacet::new("sum(count)"))]),
    );
    let response = config
        .async_client
        .select(&query, &config.collection_name)
        .await?;
    let facets = response.get_json_facets().ok_or("No facets")?;
    let total_people = facets
        .get_flat_facets()
        .get("total_people")
        .ok_or("No total_people facet")?;
    assert_eq!(total_people.as_f64().ok_or("Not a number")?, 1000.0);

    let _ = config.tear_down().await;
    Ok(())
}

#[tokio::test]
#[parallel]
pub async fn test_json_terms_facet_works() -> Result<(), Error> {
    let config = FunctionalityTestsBuildup::build_up("JsonFacetTerms")
        .await
        .unwrap();
    let update = UpdateQuery::new();
    update
        .execute(&config.context, &config.collection_name, &get_test_data())
        .await?;

    let query = SelectQuery::new()
        .json_facet(JsonFacetComponent::new().facets([("age", JsonTermsFacet::new("age"))]));
    let response = config
        .async_client
        .select(&query, &config.collection_name)
        .await?;
    let facets = response.get_json_facets().ok_or("No facets")?;
    let age = facets
        .get_nested_facets()
        .get("age")
        .ok_or("No age facet")?;
    let buckets = age.get_buckets().collect::<Vec<&SolrJsonFacetResponse>>();
    assert_eq!(buckets.len(), 3);

    let _ = config.tear_down().await;
    Ok(())
}

#[tokio::test]
#[parallel]
pub async fn test_json_facet_sub_works() -> Result<(), Error> {
    let config = FunctionalityTestsBuildup::build_up("JsonFacetSub")
        .await
        .unwrap();
    let update = UpdateQuery::new();
    update
        .execute(&config.context, &config.collection_name, &get_test_data())
        .await?;

    let query = SelectQuery::new().json_facet(
        JsonFacetComponent::new().facets([(
            "below_60",
            JsonQueryFacet::new("age:[0 TO 59]")
                .facets([("total_people", JsonStatFacet::new("sum(count)"))]),
        )]),
    );
    let response = config
        .async_client
        .select(&query, &config.collection_name)
        .await?;
    let facets = response.get_json_facets().ok_or("No facets")?;
    let total_people = facets
        .get_nested_facets()
        .get("below_60")
        .ok_or("No below_60 facet")?
        .get_flat_facets()
        .get("total_people")
        .ok_or("No total_people facet")?;
    assert_eq!(serde_json::from_value::<f32>(total_people.clone())?, 750.0);

    let _ = config.tear_down().await;
    Ok(())
}
