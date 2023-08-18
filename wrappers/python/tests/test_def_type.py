import pytest
from helpers import (
    Config,
    create_config,
    index_test_data,
    setup_collection,
    teardown_collection,
    wait_for_solr,
)

from solrstice.def_type import (
    DismaxQuery,
    EdismaxQuery,
    LuceneQuery,
)
from solrstice.queries import SelectQuery


@pytest.fixture()
def config() -> Config:
    yield create_config()


@pytest.mark.asyncio
async def test_lucene_query_parser(config: Config):
    name = "LuceneQueryParser"
    wait_for_solr(config.solr_host, 30)

    try:
        await setup_collection(config.context, name, config.config_path)
        await index_test_data(config.context, name)

        query_parser = LuceneQuery(df="population")
        select_builder = SelectQuery(q="outdoors", def_type=query_parser)
        (await select_builder.execute(config.context, name)).get_docs_response()
    finally:
        await teardown_collection(config.context, name)


@pytest.mark.asyncio
async def test_dismax_query_parser(config: Config):
    name = "DismaxQueryParser"
    wait_for_solr(config.solr_host, 30)

    try:
        await setup_collection(config.context, name, config.config_path)
        await index_test_data(config.context, name)

        query_parser = DismaxQuery(qf="interests^20", bq=["interests:cars^20"])
        select_builder = SelectQuery(q="outdoors", def_type=query_parser)
        response = (
            await select_builder.execute(config.context, name)
        ).get_docs_response()
        first_doc = response.get_docs()[0]
        assert first_doc["id"] == "city_Alta_20"
    finally:
        await teardown_collection(config.context, name)


@pytest.mark.asyncio
async def test_edismax_query_parser(config: Config):
    name = "EdismaxQueryParser"
    wait_for_solr(config.solr_host, 30)

    try:
        await setup_collection(config.context, name, config.config_path)
        await index_test_data(config.context, name)

        query_parser = EdismaxQuery(qf="interests^20", bq=["interests:cars^20"])
        select_builder = SelectQuery(q="outdoors", def_type=query_parser)
        response = (
            await select_builder.execute(config.context, name)
        ).get_docs_response()
        first_doc = response.get_docs()[0]
        assert first_doc["id"] == "city_Alta_20"
    finally:
        await teardown_collection(config.context, name)
