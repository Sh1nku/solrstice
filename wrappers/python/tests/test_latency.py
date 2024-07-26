# Test that the GIL is not blocked while waiting for a response.
import asyncio
import time
from multiprocessing.pool import ThreadPool
from typing import Generator, List, Optional

import pytest

from solrstice import (
    AsyncSolrCloudClient,
    BlockingSolrCloudClient,
    SolrBasicAuth,
    SolrServerContext,
    SolrSingleServerHost,
)

from .helpers import Config, create_config


@pytest.fixture()
def config() -> Generator[Config, None, None]:
    yield create_config()


def test_blocking_client_does_not_block_gil_config(config: Config) -> None:
    processes = 8
    if not config.speedbump_host:
        pytest.skip("No speedbump host configured")
    with ThreadPool(processes) as pool:
        start_time = time.time()
        tasks = [
            pool.apply_async(
                get_configs_blocking,
                (config.speedbump_host, config.solr_username, config.solr_password),
            )
            for _ in range(processes)
        ]
        for task in tasks:
            task.get(timeout=processes * 3)

    elapsed_seconds = time.time() - start_time
    # Presume since the latency is 2s, that it should have completed in max 1s, given no GIL blocking.
    assert elapsed_seconds < processes


@pytest.mark.asyncio
async def test_async_client_does_not_block_event_loop(config: Config) -> None:
    processes = 8
    if not config.speedbump_host:
        pytest.skip("No speedbump host configured")
    start_time = time.time()
    tasks = [
        asyncio.create_task(
            get_configs_async(
                config.speedbump_host, config.solr_username, config.solr_password
            )
        )
        for _ in range(processes)
    ]
    await asyncio.gather(*tasks)
    elapsed_seconds = time.time() - start_time
    # Presume since the latency is 2s, that it should have completed in max 1s, given no GIL blocking.
    assert elapsed_seconds < processes


def get_configs_blocking(
    host: str, username: Optional[str], password: Optional[str]
) -> List[str]:
    auth = None if not username else SolrBasicAuth(username, password)
    client = BlockingSolrCloudClient(
        SolrServerContext(SolrSingleServerHost(host), auth)
    )
    return client.get_configs()


async def get_configs_async(
    host: str, username: Optional[str], password: Optional[str]
) -> List[str]:
    auth = None if not username else SolrBasicAuth(username, password)
    client = AsyncSolrCloudClient(SolrServerContext(SolrSingleServerHost(host), auth))
    return await client.get_configs()
