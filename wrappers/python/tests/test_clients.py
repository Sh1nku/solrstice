import asyncio

import pytest

from helpers import Config, create_config
from solrstice.clients import AsyncSolrCloudClient, BlockingSolrCloudClient
from solrstice.queries import UpdateQueryBuilder, SelectQueryBuilder, DeleteQueryBuilder


@pytest.fixture()
def config() -> Config:
    yield create_config()


@pytest.mark.asyncio
async def test_async_client_works(config: Config):
    name = "AsyncClientWorks"

    client = AsyncSolrCloudClient(config.context)
    try:
        await client.delete_collection(name)
    except:
        pass
    try:
        await client.delete_config(name)
    except:
        pass

    await client.upload_config(name, config.config_path)
    await client.create_collection(name, name, shards=1, replication_factor=1)
    await client.index(UpdateQueryBuilder(), name, [{'id': 'example_document'}])
    response = await client.select(SelectQueryBuilder(fq=['id:example_document']), name)
    docs = response.get_response()
    assert docs.num_found == 1

    await client.delete(DeleteQueryBuilder(ids=['example_document']), name)
    response = await client.select(SelectQueryBuilder(fq=['id:example_document']), name)
    docs = response.get_response()
    assert docs.num_found == 0

    await client.delete_collection(name)
    await client.delete_config(name)


def test_blocking_client_works(config: Config):
    name = "BlockingClientWorks"

    client = BlockingSolrCloudClient(config.context)
    try:
        client.delete_collection(name)
    except:
        pass
    try:
        client.delete_config(name)
    except:
        pass

    client.upload_config(name, config.config_path)
    client.create_collection(name, name, shards=1, replication_factor=1)
    client.index(UpdateQueryBuilder(), name, [{'id': 'example_document'}])
    response = client.select(SelectQueryBuilder(fq=['id:example_document']), name)
    docs = response.get_response()
    assert docs.num_found == 1

    client.delete(DeleteQueryBuilder(ids=['example_document']), name)
    response = client.select(SelectQueryBuilder(fq=['id:example_document']), name)
    docs = response.get_response()
    assert docs.num_found == 0

    client.delete_collection(name)
    client.delete_config(name)
