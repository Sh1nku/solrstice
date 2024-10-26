from typing import Generator

import pytest

from solrstice import SelectQuery

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
async def test_syntax_error_returns_sensible_error(config: Config) -> None:
    name = "SyntaxError"
    wait_for_solr(config.solr_host, 30)

    try:
        await setup_collection(config.context, name, config.config_path)
        with pytest.raises(Exception) as e:
            await config.async_client.select(
                SelectQuery(fq=["some_field::syntax_error"]), name
            )
            assert "400" in str(e)
    finally:
        await teardown_collection(config.context, name)
