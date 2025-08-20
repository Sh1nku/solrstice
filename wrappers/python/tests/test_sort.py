from typing import Generator

import pytest
from .helpers import (
    Config,
    create_config,
    index_test_data,
    setup_collection,
    teardown_collection,
    wait_for_solr,
)

from solrstice import SelectQuery


@pytest.fixture()
def config() -> Generator[Config, None, None]:
    yield create_config()


@pytest.mark.asyncio
async def test_sort_single_field(config: Config) -> None:
    name = "SortSingleField"
    wait_for_solr(config.solr_host, 30)

    try:
        await setup_collection(config.context, name, config.config_path)
        await index_test_data(config.context, name)

        builder = SelectQuery(
            fq=["city_name:[* TO *]"],
            fl=["*", "[child limit=1000]"],
            sort="city_name asc",
        )
        solr_response = await builder.execute(config.context, name)
        docs_response = solr_response.get_docs_response()
        assert docs_response is not None
        docs = docs_response.get_docs()
        assert docs[0]["city_name"] == "Alta"
        assert docs[-1]["city_name"] == "Tromsø"

        builder = SelectQuery(
            fq=["city_name:[* TO *]"],
            fl=["*", "[child limit=1000]"],
            sort="city_name desc",
        )
        solr_response = await builder.execute(config.context, name)
        docs_response = solr_response.get_docs_response()
        assert docs_response is not None
        docs = docs_response.get_docs()
        assert docs[0]["city_name"] == "Tromsø"
        assert docs[-1]["city_name"] == "Alta"
    finally:
        await teardown_collection(config.context, name)


@pytest.mark.asyncio
async def test_sort_multiple_fields(config: Config) -> None:
    name = "SortMultipleFields"
    wait_for_solr(config.solr_host, 30)

    try:
        await setup_collection(config.context, name, config.config_path)
        await index_test_data(config.context, name)

        builder = SelectQuery(fq=["-city_name:[* TO *]"], sort="age asc, count asc")
        solr_response = await builder.execute(config.context, name)
        docs_response = solr_response.get_docs_response()
        assert docs_response is not None
        docs = docs_response.get_docs()
        assert docs[0]["age"] == docs[1]["age"]
        assert docs[0]["count"] <= docs[1]["count"]

        builder = SelectQuery(fq=["-city_name:[* TO *]"], sort="age desc, count desc")
        solr_response = await builder.execute(config.context, name)
        docs_response = solr_response.get_docs_response()
        assert docs_response is not None
        docs = docs_response.get_docs()
        assert docs[0]["age"] == docs[1]["age"]
        assert docs[0]["count"] >= docs[1]["count"]
    finally:
        await teardown_collection(config.context, name)
