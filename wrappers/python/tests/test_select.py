import pytest
from helpers import (
    Config,
    create_config,
    index_test_data,
    setup_collection,
    teardown_collection,
    wait_for_solr,
)

from solrstice.queries import SelectQueryBuilder


@pytest.fixture()
def config() -> Config:
    yield create_config()


@pytest.mark.asyncio
async def test_get_response_gets_response(config: Config):
    name = "SelectGetResponse"
    wait_for_solr(config.solr_host, 30)

    try:
        await setup_collection(config.context, name, config.config_path)

        await index_test_data(config.context, name)

        builder = SelectQueryBuilder()
        solr_response = await builder.execute(config.context, name)
        docs_response = solr_response.get_response()
        assert docs_response.num_found > 0
        assert docs_response.start == 0
        assert len(docs_response.docs) > 4
    finally:
        await teardown_collection(config.context, name)


@pytest.mark.asyncio
async def test_select_works_when_no_result(config: Config):
    name = "SelectNoResult"
    wait_for_solr(config.solr_host, 30)

    try:
        await setup_collection(config.context, name, config.config_path)

        await index_test_data(config.context, name)

        builder = SelectQueryBuilder(fq=["id:non_existent_id"])
        solr_response = await builder.execute(config.context, name)
        docs_response = solr_response.get_response()
        assert docs_response.num_found == 0
        assert docs_response.start == 0
        assert len(docs_response.docs) == 0
    finally:
        await teardown_collection(config.context, name)


@pytest.mark.asyncio
async def test_select_works_with_cursor_mark(config: Config):
    name = "SelectCursorMark"
    wait_for_solr(config.solr_host, 30)

    try:
        await setup_collection(config.context, name, config.config_path)
        await index_test_data(config.context, name)
        cursor_mark = "*"
        current_iteration = 0
        while True:
            if current_iteration > 100:
                raise Exception("Cursor mark test failed. Too many iterations")
            builder = SelectQueryBuilder(
                fq=["age:[* TO *]"], rows=1, sort=["id desc"], cursor_mark=cursor_mark
            )
            result = await builder.execute(config.context, name)
            if result.next_cursor_mark is not None:
                if cursor_mark == "*":
                    break
                cursor_mark = result.next_cursor_mark
            else:
                raise Exception("Cursor mark test failed. No next cursor mark")
            current_iteration += 1

    finally:
        await teardown_collection(config.context, name)
