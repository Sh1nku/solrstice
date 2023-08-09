import pytest

from helpers import Config, wait_for_solr, setup_collection, teardown_collection, create_config
from solrstice.queries import UpdateQueryBuilder, CommitType


@pytest.fixture()
def config() -> Config:
    yield create_config()


@pytest.mark.asyncio
async def test_index_indexes_documents(config: Config):
    name = "IndexIndexesDocuments"
    wait_for_solr(config.solr_host, 30)

    try:
        await setup_collection(config.context, name, config.config_path)

        await UpdateQueryBuilder(handler='update', commit_type=CommitType.Soft).execute(config.context, name, [{'id': 'test'}])
    finally:
        await teardown_collection(config.context, name)
