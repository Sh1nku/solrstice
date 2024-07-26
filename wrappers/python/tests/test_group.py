from typing import Generator

import pytest

from solrstice import GroupingComponent, SelectQuery

from .helpers import (
    Config,
    create_config,
    index_test_data,
    setup_collection,
    teardown_collection,
    wait_for_solr,
)


@pytest.fixture()
def config() -> Generator[Config, None, None]:
    yield create_config()


@pytest.mark.asyncio
async def test_get_group_field_result_works(config: Config) -> None:
    name = "GroupFieldQuery"
    wait_for_solr(config.solr_host, 30)

    try:
        await setup_collection(config.context, name, config.config_path)
        await index_test_data(config.context, name)

        group_builder = GroupingComponent(fields=["age"], limit=10)
        select_builder = SelectQuery(fq=["age:[* TO *]"], grouping=group_builder)
        groups = (await select_builder.execute(config.context, name)).get_groups()
        age_group = groups["age"]
        group = age_group.get_field_result()
        assert group is not None
        assert age_group.get_n_groups() is None
        assert age_group.get_matches() > 0
        assert len(group) > 0
    finally:
        await teardown_collection(config.context, name)


@pytest.mark.asyncio
async def test_get_group_query_result_works(config: Config) -> None:
    name = "GroupQueryQuery"
    wait_for_solr(config.solr_host, 30)

    try:
        await setup_collection(config.context, name, config.config_path)
        await index_test_data(config.context, name)

        group_builder = GroupingComponent(
            queries=["age:[0 TO 59]", "age:[60 TO *]"], limit=10
        )
        select_builder = SelectQuery(fq=["age:[* TO *]"], grouping=group_builder)
        groups = (await select_builder.execute(config.context, name)).get_groups()
        age_group = groups["age:[0 TO 59]"]
        group = age_group.get_query_result()
        assert group is not None
        assert len(group.get_docs()) > 0
    finally:
        await teardown_collection(config.context, name)
