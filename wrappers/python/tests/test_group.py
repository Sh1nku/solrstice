import pytest
from helpers import (
    Config,
    create_config,
    index_test_data,
    setup_collection,
    teardown_collection,
    wait_for_solr,
)

from solrstice.group import GroupingComponent
from solrstice.queries import SelectQueryBuilder


@pytest.fixture()
def config() -> Config:
    yield create_config()


@pytest.mark.asyncio
async def test_get_group_field_result_works(config: Config):
    name = "GroupFieldQuery"
    wait_for_solr(config.solr_host, 30)

    try:
        await setup_collection(config.context, name, config.config_path)
        await index_test_data(config.context, name)

        group_builder = GroupingComponent(fields=["age"], limit=10)
        select_builder = SelectQueryBuilder(fq=["age:[* TO *]"], grouping=group_builder)
        groups = (await select_builder.execute(config.context, name)).get_groups()
        age_group = groups["age"]
        group = age_group.get_field_result()
        assert age_group.n_groups is None
        assert age_group.matches > 0
        assert len(group) > 0
    finally:
        await teardown_collection(config.context, name)


@pytest.mark.asyncio
async def test_get_group_query_result_works(config: Config):
    name = "GroupQueryQuery"
    wait_for_solr(config.solr_host, 30)

    try:
        await setup_collection(config.context, name, config.config_path)
        await index_test_data(config.context, name)

        group_builder = GroupingComponent(
            queries=["age:[0 TO 59]", "age:[60 TO *]"], limit=10
        )
        select_builder = SelectQueryBuilder(fq=["age:[* TO *]"], grouping=group_builder)
        groups = (await select_builder.execute(config.context, name)).get_groups()
        age_group = groups["age:[0 TO 59]"]
        group = age_group.get_query_result()
        group.docs
        assert len(group.docs) > 0
    finally:
        await teardown_collection(config.context, name)
