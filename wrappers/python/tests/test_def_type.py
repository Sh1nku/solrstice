import pytest

from helpers import Config, create_config, wait_for_solr, setup_collection, teardown_collection, index_test_data
from solrstice.def_type import DefTypeLucene
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

        query_parser = DefTypeLucene(df="population")
        select_builder = SelectQueryBuilder(q='outdoors', def_type=query_parser)
        (await select_builder.execute(config.context, name)).get_response()
    finally:
        await teardown_collection(config.context, name)
