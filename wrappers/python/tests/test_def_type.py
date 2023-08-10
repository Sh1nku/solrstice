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
    DismaxQueryBuilder,
    EdismaxQueryBuilder,
    LuceneQueryBuilder,
)
from solrstice.queries import SelectQueryBuilder


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

        query_parser = LuceneQueryBuilder(df="population")
        select_builder = SelectQueryBuilder(q="outdoors", def_type=query_parser)
        (await select_builder.execute(config.context, name)).get_response()
    finally:
        await teardown_collection(config.context, name)


@pytest.mark.asyncio
async def test_dismax_query_parser(config: Config):
    name = "DismaxQueryParser"
    wait_for_solr(config.solr_host, 30)

    try:
        await setup_collection(config.context, name, config.config_path)
        await index_test_data(config.context, name)

        query_parser = DismaxQueryBuilder(qf="interests^20", bq=["interests:cars^20"])
        select_builder = SelectQueryBuilder(q="outdoors", def_type=query_parser)
        response = (await select_builder.execute(config.context, name)).get_response()
        first_doc = response.docs[0]
        assert first_doc["id"] == "city_Alta_20"
    finally:
        await teardown_collection(config.context, name)


@pytest.mark.asyncio
async def test_edismax_query_parser(config: Config):
    name = "EDismaxQueryParser"
    wait_for_solr(config.solr_host, 30)

    try:
        await setup_collection(config.context, name, config.config_path)
        await index_test_data(config.context, name)

        query_parser = EdismaxQueryBuilder(qf="interests^20", bq=["interests:cars^20"])
        select_builder = SelectQueryBuilder(q="outdoors", def_type=query_parser)
        response = (await select_builder.execute(config.context, name)).get_response()
        first_doc = response.docs[0]
        assert first_doc["id"] == "city_Alta_20"
    finally:
        await teardown_collection(config.context, name)
