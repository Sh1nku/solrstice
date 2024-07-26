from typing import Generator

import pytest

from solrstice import (
    FacetSetComponent,
    FieldFacetComponent,
    FieldFacetEntry,
    PivotFacetComponent,
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
async def test_facet_pivot_works(config: Config) -> None:
    name = "FacetPivot"
    wait_for_solr(config.solr_host, 30)

    try:
        await setup_collection(config.context, name, config.config_path)
        await index_test_data(config.context, name)

        pivot = PivotFacetComponent(["interests,age"])
        facet_set = FacetSetComponent(pivots=pivot)
        select_builder = SelectQuery(facet_set=facet_set)
        response = await config.async_client.select(select_builder, name)
        facets = response.get_facet_set()
        pivot_result = facets.get_pivots()
        interests_age = pivot_result["interests,age"]
        cars_pivot = next(p for p in interests_age if p.get_value() == "cars")
        assert cars_pivot.get_count() == 1
        age_pivot = cars_pivot.get_pivots()[0]
        assert age_pivot.get_value() == 20
    finally:
        await teardown_collection(config.context, name)


@pytest.mark.asyncio
async def test_facet_query_works(config: Config) -> None:
    name = "FacetQuery"
    wait_for_solr(config.solr_host, 30)

    try:
        await setup_collection(config.context, name, config.config_path)
        await index_test_data(config.context, name)

        facet_set = FacetSetComponent(queries=["age:[0 TO 59]"])
        select_builder = SelectQuery(facet_set=facet_set)
        response = await config.async_client.select(select_builder, name)
        facets = response.get_facet_set()
        queries = facets.get_queries()
        query = queries.get("age:[0 TO 59]")
        assert query == 4
    finally:
        await teardown_collection(config.context, name)


@pytest.mark.asyncio
async def test_facet_field_works(config: Config) -> None:
    name = "FacetField"
    wait_for_solr(config.solr_host, 30)

    try:
        await setup_collection(config.context, name, config.config_path)
        await index_test_data(config.context, name)

        fields = FieldFacetComponent(fields=[FieldFacetEntry("age")])
        facet_set = FacetSetComponent(fields=fields)
        select_builder = SelectQuery(facet_set=facet_set)
        response = await config.async_client.select(select_builder, name)
        facets = response.get_facet_set()
        fields_result = facets.get_fields()
        age = fields_result["age"]
        assert len(age) == 3
    finally:
        await teardown_collection(config.context, name)
