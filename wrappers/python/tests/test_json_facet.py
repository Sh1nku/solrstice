"""
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

"""
import pytest
from helpers import (
    Config,
    create_config,
    index_test_data,
    setup_collection,
    teardown_collection,
    wait_for_solr,
)

from solrstice.json_facet import JsonFacetComponent, JsonQueryFacet, JsonStatFacet
from solrstice.queries import SelectQuery


@pytest.fixture()
def config() -> Config:
    yield create_config()


@pytest.mark.asyncio
async def test_json_facet_works(config: Config):
    name = "JsonFacetBasic"
    wait_for_solr(config.solr_host, 30)

    try:
        await setup_collection(config.context, name, config.config_path)
        await index_test_data(config.context, name)

        select_builder = SelectQuery(
            json_facet=JsonFacetComponent(
                facets={"below_60": JsonQueryFacet("age:[0 TO 59]")}
            )
        )
        response = await config.async_client.select(select_builder, name)
        facets = response.get_json_facets()
        below_60 = facets.get_nested_facets().get("below_60")
        assert below_60.get_count() == 4
    finally:
        await teardown_collection(config.context, name)


@pytest.mark.asyncio
async def test_json_facet_sub_works(config: Config):
    name = "JsonFacetSub"
    wait_for_solr(config.solr_host, 30)

    try:
        await setup_collection(config.context, name, config.config_path)
        await index_test_data(config.context, name)

        select_builder = SelectQuery(
            json_facet=JsonFacetComponent(
                facets={
                    "below_60": JsonQueryFacet(
                        "age:[0 TO 59]",
                        facets={"total_people": JsonStatFacet("sum(count)")},
                    )
                }
            )
        )
        response = await config.async_client.select(select_builder, name)
        facets = response.get_json_facets()
        total_people = (
            facets.get_nested_facets()
            .get("below_60")
            .get_flat_facets()
            .get("total_people")
        )
        assert total_people == 750.0
    finally:
        await teardown_collection(config.context, name)
