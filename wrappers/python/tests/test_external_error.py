from typing import Generator

import pytest

from solrstice import SelectQuery

from .helpers import ErrorTestsSetup, create_nginx_error_config


@pytest.fixture()
def config() -> Generator[ErrorTestsSetup, None, None]:
    yield create_nginx_error_config()


@pytest.mark.asyncio
async def test_sensible_error_message_if_not_solr_server(
    config: ErrorTestsSetup,
) -> None:
    try:
        await config.async_client.select(SelectQuery(), "error_collection")
    except Exception as e:
        assert "500" in str(e)


@pytest.mark.asyncio
async def test_sensible_error_message_if_non_existent_collection(
    config: ErrorTestsSetup,
) -> None:
    try:
        await config.async_client.select(SelectQuery(), "notfound_collection")
    except Exception as e:
        assert "404" in str(e)


@pytest.mark.asyncio
async def test_sensible_error_message_if_200_but_not_solr(
    config: ErrorTestsSetup,
) -> None:
    try:
        await config.async_client.select(SelectQuery(), "always_200")
    except Exception as e:
        assert "200" in str(e)
