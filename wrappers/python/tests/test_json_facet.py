import pytest
from helpers import (
    Config,
    create_config,
    index_test_data,
    setup_collection,
    teardown_collection,
    wait_for_solr,
)

from solrstice.json_facet import (
    JsonFacetComponent,
    JsonQueryFacet,
    JsonStatFacet,
    JsonTermsFacet,
)
from solrstice.queries import SelectQuery


@pytest.fixture()
def config() -> Config:
    yield create_config()


@pytest.mark.asyncio
async def test_json_query_facet_works(config: Config):
    name = "JsonQueryFacet"
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
async def test_json_terms_facet_works(config: Config):
    name = "JsonTermsFacet"
    wait_for_solr(config.solr_host, 30)

    try:
        await setup_collection(config.context, name, config.config_path)
        await index_test_data(config.context, name)

        select_builder = SelectQuery(
            json_facet=JsonFacetComponent(facets={"age": JsonTermsFacet("age")})
        )
        response = await config.async_client.select(select_builder, name)
        facets = response.get_json_facets()
        age_buckets = facets.get_nested_facets().get("age").get_buckets()
        assert len(age_buckets) == 3
    finally:
        await teardown_collection(config.context, name)


@pytest.mark.asyncio
async def test_json_stat_facet_works(config: Config):
    name = "JsonStatFacet"
    wait_for_solr(config.solr_host, 30)

    try:
        await setup_collection(config.context, name, config.config_path)
        await index_test_data(config.context, name)

        select_builder = SelectQuery(
            json_facet=JsonFacetComponent(
                facets={"total_people": JsonStatFacet("sum(count)")}
            )
        )
        response = await config.async_client.select(select_builder, name)
        facets = response.get_json_facets()
        total_people = facets.get_flat_facets().get("total_people")
        assert total_people == 1000
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
