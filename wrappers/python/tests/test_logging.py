import logging

import pytest
from _pytest.logging import LogCaptureFixture
from helpers import Config, create_config, wait_for_solr

from solrstice import AsyncSolrCloudClient
from solrstice import OffLoggingPolicy, SolrServerContext


@pytest.fixture()
def config() -> Config:
    yield create_config()


# TODO This test fails if run in parallel with the rest of the test suite, but not if run alone
# @pytest.mark.asyncio
# async def test_logging_logs_message(config: Config, caplog: LogCaptureFixture):
#     wait_for_solr(config.solr_host, 30)
#
#     with caplog.at_level(logging.DEBUG):
#         caplog.clear()
#         assert not any(
#             "Sending Solr request to" in msg for msg in [x.getMessage() for x in caplog.records]), "Logs are not empty"
#         await config.async_client.get_configs()
#         assert any(
#             "Sending Solr request to" in msg for msg in
#             [x.getMessage() for x in caplog.records]), "Expected log message not found"


@pytest.mark.asyncio
async def test_logging_does_not_log_message_if_disabled(
        config: Config, caplog: LogCaptureFixture
):
    wait_for_solr(config.solr_host, 30)

    context = SolrServerContext(config.solr_host, config.solr_auth, OffLoggingPolicy())
    client = AsyncSolrCloudClient(context)

    with caplog.at_level(logging.DEBUG):
        caplog.clear()
        assert not any(
            "Sending Solr request to" in msg
            for msg in [x.getMessage() for x in caplog.records]
        ), "Logs are not empty"
        await client.get_configs()
        assert not any(
            "Sending Solr request to" in msg
            for msg in [x.getMessage() for x in caplog.records]
        ), "Logs are not empty"
