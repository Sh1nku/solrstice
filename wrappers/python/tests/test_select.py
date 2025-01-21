from typing import Generator

import pytest

from solrstice import SelectQuery

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
async def test_get_response_gets_response(config: Config) -> None:
    name = "SelectGetResponse"
    wait_for_solr(config.solr_host, 30)

    try:
        await setup_collection(config.context, name, config.config_path)

        await index_test_data(config.context, name)

        builder = SelectQuery()
        solr_response = await builder.execute(config.context, name)
        docs_response = solr_response.get_docs_response()
        assert docs_response is not None
        assert docs_response.get_num_found() > 0
        assert docs_response.get_start() == 0
        assert len(docs_response.get_docs()) > 4
    finally:
        await teardown_collection(config.context, name)


@pytest.mark.asyncio
async def test_select_works_when_no_result(config: Config) -> None:
    name = "SelectNoResult"
    wait_for_solr(config.solr_host, 30)

    try:
        await setup_collection(config.context, name, config.config_path)

        await index_test_data(config.context, name)

        builder = SelectQuery(fq=["id:non_existent_id"])
        solr_response = await builder.execute(config.context, name)
        docs_response = solr_response.get_docs_response()
        assert docs_response is not None
        assert docs_response.get_num_found() == 0
        assert docs_response.get_start() == 0
        assert len(docs_response.get_docs()) == 0
    finally:
        await teardown_collection(config.context, name)


@pytest.mark.asyncio
async def test_select_works_with_cursor_mark(config: Config) -> None:
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
            builder = SelectQuery(
                fq=["age:[* TO *]"], rows=1, sort=["id desc"], cursor_mark=cursor_mark
            )
            result = await builder.execute(config.context, name)
            if result.get_next_cursor_mark() is not None:
                if cursor_mark == "*":
                    break
                cursor_mark = result.get_next_cursor_mark()  # type: ignore
            else:
                raise Exception("Cursor mark test failed. No next cursor mark")
            current_iteration += 1

    finally:
        await teardown_collection(config.context, name)


@pytest.mark.asyncio
async def test_select_works_with_additional_params(config: Config) -> None:
    name = "SelectAdditionalParams"
    wait_for_solr(config.solr_host, 30)

    try:
        await setup_collection(config.context, name, config.config_path)

        await index_test_data(config.context, name)

        builder = SelectQuery(
            q='{!parent which=city_name:*}',
            fl=["id", "city_name", "child:[subquery]"],
            additional_params={
                'child.q': '*:*',
                'child.fl': ['id'],
                'child.rows': 1,
            }
        )
        solr_response = await builder.execute(config.context, name)
        docs_response = solr_response.get_docs_response()
        assert docs_response is not None
        assert docs_response.get_num_found() > 0
        child_result = docs_response.get_docs()[0].get("child")
        assert child_result is not None
        assert child_result['numFound'] > 0
        assert len(child_result['docs']) == 1
    finally:
        await teardown_collection(config.context, name)
