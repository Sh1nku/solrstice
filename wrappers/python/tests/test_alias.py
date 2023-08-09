import pytest

from helpers import Config, create_config
from solrstice.alias import delete_alias, alias_exists, create_alias, create_alias_blocking, alias_exists_blocking, \
    delete_alias_blocking
from solrstice.collection import delete_collection, collection_exists, create_collection, delete_collection_blocking, \
    collection_exists_blocking, create_collection_blocking
from solrstice.config import delete_config, config_exists, upload_config, delete_config_blocking, \
    upload_config_blocking, config_exists_blocking


@pytest.fixture()
def config() -> Config:
    yield create_config()


@pytest.mark.asyncio
async def test_alias_all_async_functions_exported(config: Config):
    name = "AliasConfig"

    functions = [delete_alias_blocking, delete_collection_blocking, delete_config_blocking]

    for function in functions:
        try:
            function(config.context, name)
        except RuntimeError:
            pass
    assert not await alias_exists(config.context, name)
    assert not await collection_exists(config.context, name)
    assert not await config_exists(config.context, name)
    await upload_config(
        config.context,
        name,
        config.config_path,
    )
    await create_collection(config.context, name, name, 1, 1)
    await create_alias(config.context, name, [name])
    assert await alias_exists(config.context, name)
    await delete_alias(config.context, name)
    await delete_collection(config.context, name)
    await delete_config(config.context, name)


def test_alias_all_blocking_functions_exported(config: Config):
    name = "AliasBlockingConfig"


    functions = [delete_alias_blocking, delete_collection_blocking, delete_config_blocking]

    for function in functions:
        try:
            function(config.context, name)
        except RuntimeError:
            pass
    assert not alias_exists_blocking(config.context, name)
    assert not collection_exists_blocking(config.context, name)
    assert not config_exists_blocking(config.context, name)
    upload_config_blocking(
        config.context,
        name,
        config.config_path,
    )
    create_collection_blocking(config.context, name, name, 1, 1)
    create_alias_blocking(config.context, name, [name])
    assert alias_exists_blocking(config.context, name)
    delete_collection_blocking(config.context, name)
    delete_config_blocking(config.context, name)
