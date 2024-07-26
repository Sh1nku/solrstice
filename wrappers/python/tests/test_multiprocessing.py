import dataclasses
import random
from multiprocessing.pool import ThreadPool
from typing import Generator, Optional

import pytest

from solrstice import (
    BlockingSolrCloudClient,
    SolrBasicAuth,
    SolrServerContext,
    SolrSingleServerHost,
    UpdateQuery,
)

from .helpers import Config, create_config


@pytest.fixture()
def config() -> Generator[Config, None, None]:
    yield create_config()


@dataclasses.dataclass
class PickableConfig:
    solr_host: str
    solr_username: Optional[str]
    solr_password: Optional[str]


def create_client(config: PickableConfig) -> BlockingSolrCloudClient:
    auth = (
        None
        if not config.solr_username
        else SolrBasicAuth(config.solr_username, config.solr_password)
    )
    return BlockingSolrCloudClient(
        SolrServerContext(SolrSingleServerHost(config.solr_host), auth)
    )


def index_independent(config: PickableConfig, collection_name: str) -> None:
    client = create_client(config)
    client.index(
        UpdateQuery(),
        collection_name,
        [{"id": str(random.randint(0, 10000000000))}],
    )


# TODO This does not work, and blocks forever
# def test_blocking_multiprocessing_works(config: Config):
#     name = "BlockingMultiprocessingWorks"
#
#     pickable_config = PickableConfig(config.solr_host, config.solr_username, config.solr_password)
#     client = create_client(pickable_config)
#     try:
#         client.delete_collection(name)
#     except:
#         pass
#     try:
#         client.delete_config(name)
#     except:
#         pass
#
#     client.upload_config(name, config.config_path)
#     client.create_collection(name, name, shards=1, replication_factor=1)
#
#     with Pool(processes=4) as pool:
#         tasks = [pool.apply_async(index_independent, (pickable_config, name)) for _ in range(10)]
#         [task.get(15) for task in tasks]
#
#     try:
#         client.delete_collection(name)
#     except:
#         pass
#     try:
#         client.delete_config(name)
#     except:
#         pass


def test_blocking_multithreading_works(config: Config) -> None:
    name = "BlockingMultithreadingWorks"

    pickable_config = PickableConfig(
        config.solr_host, config.solr_username, config.solr_password
    )
    client = create_client(pickable_config)
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

    with ThreadPool(processes=4) as pool:
        tasks = [
            pool.apply_async(index_independent, (pickable_config, name))
            for _ in range(10)
        ]
        [task.get(15) for task in tasks]

    try:
        client.delete_collection(name)
    except:
        pass
    try:
        client.delete_config(name)
    except:
        pass
