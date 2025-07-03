from typing import Generator

import pytest

from .helpers import Config, create_config, wait_for_solr, setup_collection, index_test_data, teardown_collection
from solrstice import SelectQuery, StatsComponent


@pytest.fixture()
def config() -> Generator[Config, None, None]:
    yield create_config()


@pytest.mark.asyncio
async def test_stats_works(config: Config) -> None:
    name = "StatsWorks"
    wait_for_solr(config.solr_host, 30)

    try:
        await setup_collection(config.context, name, config.config_path)
        await index_test_data(config.context, name)

        select_builder = SelectQuery(stats=StatsComponent(fields=["age"]))
        response = await config.async_client.select(select_builder, name)
        stats = response.get_stats()
        assert stats is not None
        age_stats = stats.get_fields()["age"]
        assert age_stats.get_count() > 0
    finally:
        await teardown_collection(config.context, name)


@pytest.mark.asyncio
async def test_stats_works_string_field(config: Config) -> None:
    name = "StatsWorksStringField"
    wait_for_solr(config.solr_host, 30)

    try:
        await setup_collection(config.context, name, config.config_path)
        await index_test_data(config.context, name)

        select_builder = SelectQuery(stats=StatsComponent(fields=["id"]))
        response = await config.async_client.select(select_builder, name)
        stats = response.get_stats()
        assert stats is not None
        id_stats = stats.get_fields()["id"]
        assert id_stats.get_count() > 0
        assert id_stats.get_mean() is None
        assert isinstance(id_stats.get_min(), str)
        assert len(id_stats.get_min()) > 0
    finally:
        await teardown_collection(config.context, name)
