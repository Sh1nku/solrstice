use crate::structures::{get_test_data, FunctionalityTestsBuildup};
use solrstice::models::error::SolrError;
use solrstice::queries::components::json_facet::{
    JsonFacetComponent, JsonFacetType, JsonQueryFacet,
};
use solrstice::queries::index::UpdateQuery;
use solrstice::queries::select::SelectQuery;

#[tokio::test]
pub async fn test_json_facet_works() -> Result<(), SolrError> {
    let config = FunctionalityTestsBuildup::build_up("JsonFacetBasic")
        .await
        .unwrap();
    let update = UpdateQuery::new();
    update
        .execute(&config.context, &config.collection_name, &get_test_data())
        .await?;

    let query = SelectQuery::new().json_facet(
        &JsonFacetComponent::new().facets([("below_60", JsonQueryFacet::new("age:[0 TO 59]"))]),
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
    assert_eq!(below_60.get_count(), 4);

    let _ = config.tear_down().await;
    Ok(())
}

#[tokio::test]
pub async fn test_json_facet_sub_works() -> Result<(), SolrError> {
    let config = FunctionalityTestsBuildup::build_up("JsonFacetSub")
        .await
        .unwrap();
    let update = UpdateQuery::new();
    update
        .execute(&config.context, &config.collection_name, &get_test_data())
        .await?;

    let query = SelectQuery::new().json_facet(&JsonFacetComponent::new().facets([(
        "below_60",
        JsonQueryFacet::new("age:[0 TO 59]").facets([(
            "total_people",
            JsonFacetType::StringQuery("sum(count)".to_string()),
        )]),
    )]));
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
