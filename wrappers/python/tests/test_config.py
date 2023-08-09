import pytest

from helpers import create_config, Config
from solrstice.config import upload_config, upload_config_blocking, delete_config, delete_config_blocking, \
    config_exists, config_exists_blocking


@pytest.fixture()
def config() -> Config:
    yield create_config()


@pytest.mark.asyncio
async def test_config_all_async_functions_exported(config: Config):
    try:
        await delete_config(config.context, "UploadConfig")
    except RuntimeError:
        pass
    assert not await config_exists(config.context, "UploadConfig")
    await upload_config(
        config.context,
        "UploadConfig",
        config.config_path,
    )
    assert await config_exists(config.context, "UploadConfig")
    await delete_config(config.context, "UploadConfig")


def test_config_all_blocking_functions_exported(config: Config):
    try:
        delete_config_blocking(config.context, "UploadConfig")
    except RuntimeError:
        pass
    assert not config_exists_blocking(config.context, "UploadConfig")
    upload_config_blocking(
        config.context,
        "UploadConfig",
        config.config_path,
    )
    assert config_exists_blocking(config.context, "UploadConfig")
    delete_config_blocking(config.context, "UploadConfig")
