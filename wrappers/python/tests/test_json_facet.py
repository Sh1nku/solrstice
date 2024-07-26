from typing import Generator

import pytest

from solrstice import (
    JsonFacetComponent,
    JsonQueryFacet,
    JsonStatFacet,
    JsonTermsFacet,
    SelectQuery,
)

from .helpers import (
    Config,
    create_config,
    index_test_data,
    setup_collection,
    teardown_collection,
    wait_for_solr,
)


@pytest.fixture()
def config() -> Generator[Config, None, None]:
    yield create_config()


@pytest.mark.asyncio
async def test_json_query_facet_works(config: Config) -> None:
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
        assert facets is not None
        below_60 = facets.get_nested_facets()["below_60"]
        assert below_60.get_count() == 4
    finally:
        await teardown_collection(config.context, name)


@pytest.mark.asyncio
async def test_json_terms_facet_works(config: Config) -> None:
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
        assert facets is not None
        age_buckets = facets.get_nested_facets()["age"].get_buckets()
        assert len(age_buckets) == 3
    finally:
        await teardown_collection(config.context, name)


@pytest.mark.asyncio
async def test_json_stat_facet_works(config: Config) -> None:
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
        assert facets is not None
        total_people = facets.get_flat_facets()["total_people"]
        assert total_people == 1000
    finally:
        await teardown_collection(config.context, name)


@pytest.mark.asyncio
async def test_json_facet_sub_works(config: Config) -> None:
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
        assert facets is not None
        total_people = facets.get_nested_facets()["below_60"].get_flat_facets()[
            "total_people"
        ]
        assert total_people == 750.0
    finally:
        await teardown_collection(config.context, name)
