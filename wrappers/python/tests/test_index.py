from typing import Generator

import pytest

from solrstice import CommitType, UpdateQuery

from .helpers import (
    Config,
    create_config,
    setup_collection,
    teardown_collection,
    wait_for_solr,
)


@pytest.fixture()
def config() -> Generator[Config, None, None]:
    yield create_config()


@pytest.mark.asyncio
async def test_index_indexes_documents(config: Config) -> None:
    name = "IndexIndexesDocuments"
    wait_for_solr(config.solr_host, 30)

    try:
        await setup_collection(config.context, name, config.config_path)

        await UpdateQuery(handler="update", commit_type=CommitType.Soft).execute(
            config.context, name, [{"id": "test"}]
        )
    finally:
        await teardown_collection(config.context, name)
