from typing import Generator

import pytest
from solrstice import SelectQuery, SolrServerContext, AsyncSolrCloudClient
from solrstice.external import ReqwestClient

from .helpers import ErrorTestsSetup, create_nginx_error_config


@pytest.fixture()
def config() -> Generator[ErrorTestsSetup, None, None]:
    yield create_nginx_error_config()


@pytest.mark.asyncio
async def test_sensible_error_message_if_not_solr_server(
    config: ErrorTestsSetup,
) -> None:
    try:
        await config.http_client.select(SelectQuery(), "error_collection")
    except Exception as e:
        assert "500" in str(e)


@pytest.mark.asyncio
async def test_sensible_error_message_if_non_existent_collection(
    config: ErrorTestsSetup,
) -> None:
    try:
        await config.http_client.select(SelectQuery(), "notfound_collection")
    except Exception as e:
        assert "404" in str(e)


@pytest.mark.asyncio
async def test_sensible_error_message_if_200_but_not_solr(
    config: ErrorTestsSetup,
) -> None:
    try:
        await config.http_client.select(SelectQuery(), "always_200")
    except Exception as e:
        assert "200" in str(e)


@pytest.mark.asyncio
async def test_certificate_error_if_invalid_certificate(
    config: ErrorTestsSetup,
) -> None:
    try:
        https_context = SolrServerContext(config.error_nginx_https_host)
        client = AsyncSolrCloudClient(https_context)
        await client.select(SelectQuery(), "notfound_collection")
    except Exception as e:
        assert "404" not in str(e)


@pytest.mark.asyncio
async def test_certificate_error_can_be_bypassed(
    config: ErrorTestsSetup,
) -> None:
    try:
        await config.https_client.select(
            SelectQuery(), "notfound_collection"
        )
    except Exception as e:
        assert "404" in str(e)
