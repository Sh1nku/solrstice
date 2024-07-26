from typing import Generator

import pytest

from solrstice.collection import (
    collection_exists,
    collection_exists_blocking,
    create_collection,
    create_collection_blocking,
    delete_collection,
    delete_collection_blocking,
)
from solrstice.config import (
    config_exists,
    config_exists_blocking,
    delete_config,
    delete_config_blocking,
    upload_config,
    upload_config_blocking,
)

from .helpers import Config, create_config


@pytest.fixture()
def config() -> Generator[Config, None, None]:
    yield create_config()


@pytest.mark.asyncio
async def test_collection_all_async_functions_exported(config: Config) -> None:
    name = "CollectionConfig"

    try:
        await delete_collection(config.context, name)
    except RuntimeError:
        pass
    try:
        await delete_config(config.context, name)
    except RuntimeError:
        pass
    assert not await collection_exists(config.context, name)
    assert not await config_exists(config.context, name)
    await upload_config(
        config.context,
        name,
        config.config_path,
    )
    await create_collection(config.context, name, name, 1, 1)
    assert await collection_exists(config.context, name)
    await delete_collection(config.context, name)
    await delete_config(config.context, name)


def test_collection_all_blocking_functions_exported(config: Config) -> None:
    name = "CollectionBlockingConfig"

    try:
        delete_collection_blocking(config.context, name)
    except RuntimeError:
        pass
    try:
        delete_config_blocking(config.context, name)
    except RuntimeError:
        pass
    assert not collection_exists_blocking(config.context, name)
    assert not config_exists_blocking(config.context, name)
    upload_config_blocking(
        config.context,
        name,
        config.config_path,
    )
    create_collection_blocking(config.context, name, name, 1, 1)
    assert collection_exists_blocking(config.context, name)
    delete_collection_blocking(config.context, name)
    delete_config_blocking(config.context, name)
