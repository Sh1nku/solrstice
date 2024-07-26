import asyncio

import pytest
from helpers import Config, create_config
from typing_extensions import Optional

from solrstice import AsyncSolrCloudClient, UpdateQuery, SelectQuery, DeleteQuery, BlockingSolrCloudClient, \
    SolrServerContext, SolrSingleServerHost, SolrAuth, SolrBasicAuth


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
    await client.index(UpdateQuery(), name, [{"id": "example_document"}])
    response = await client.select(SelectQuery(fq=["id:example_document"]), name)
    docs = response.get_docs_response()
    assert docs.get_num_found() == 1

    await client.delete(DeleteQuery(ids=["example_document"]), name)
    response = await client.select(SelectQuery(fq=["id:example_document"]), name)
    docs = response.get_docs_response()
    assert docs.get_num_found() == 0

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
    client.index(UpdateQuery(), name, [{"id": "example_document"}])
    response = client.select(SelectQuery(fq=["id:example_document"]), name)
    docs = response.get_docs_response()
    assert docs.get_num_found() == 1

    client.delete(DeleteQuery(ids=["example_document"]), name)
    response = client.select(SelectQuery(fq=["id:example_document"]), name)
    docs = response.get_docs_response()
    assert docs.get_num_found() == 0

    client.delete_collection(name)
    client.delete_config(name)


@pytest.mark.asyncio
async def test_multiple_clients_works():
    name = "MultipleClientWorks"

    config_1 = create_config()
    config_2 = create_config()

    client = AsyncSolrCloudClient(config_1.context)
    client_2 = AsyncSolrCloudClient(config_2.context)

    try:
        await client.delete_config(name)
    except:
        pass

    await client.upload_config(name, config_1.config_path)

    results = await asyncio.gather(*[client.get_configs(), client_2.get_configs()])
    assert name in results[0]
    assert name in results[1]

    await client.delete_config(name)


@pytest.mark.asyncio
async def test_subclassing_client_works():
    class SolrClient(AsyncSolrCloudClient):
        def __new__(cls, host: str, auth: Optional[SolrAuth] = None):
            context = SolrServerContext(SolrSingleServerHost(host), auth)
            return super().__new__(cls, context=context)

        def test_method(self) -> str:
            return "test"

    name = "SubclassingClientWorks"

    config = create_config()

    client = SolrClient(
        config.solr_host, SolrBasicAuth(config.solr_username, config.solr_password)
    )

    try:
        await client.delete_config(name)
    except:
        pass

    await client.upload_config(name, config.config_path)
    assert client.test_method() == "test"

    await client.delete_config(name)
