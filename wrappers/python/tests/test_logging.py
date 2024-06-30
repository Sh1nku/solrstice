import logging

import pytest
from _pytest.logging import LogCaptureFixture

from helpers import Config, create_config, wait_for_solr, setup_collection, index_test_data, teardown_collection
from solrstice.queries import SelectQuery


@pytest.fixture()
def config() -> Config:
    yield create_config()


@pytest.mark.asyncio
async def test_sending_select_query_writes_message(config: Config, caplog: LogCaptureFixture):
    caplog.set_level(logging.DEBUG)
    name = "SendingSelectQueryWritesMessage"
    wait_for_solr(config.solr_host, 30)

    try:
        await setup_collection(config.context, name, config.config_path)

        await index_test_data(config.context, name)

        builder = SelectQuery()
        await builder.execute(config.context, name)

        for record in caplog.records:
            print(record)

    finally:
        await teardown_collection(config.context, name)
