import os
from dataclasses import dataclass
from pathlib import Path
from typing import Generator, Optional

import pytest
from dotenv import load_dotenv

from solrstice import (
    SolrBasicAuth,
    SolrMultipleServerHost,
    SolrServerContext,
    SolrSingleServerHost,
    ZookeeperEnsembleHostConnector,
)
from solrstice.config import get_configs, get_configs_blocking

from .helpers import wait_for_solr


@dataclass
class Config:
    host: str
    zookeeper_host: str
    auth: Optional[SolrBasicAuth]


@pytest.fixture()
def config() -> Generator[Config, None, None]:
    path = Path("../../test_setup/.env").resolve()
    load_dotenv(path)
    solr_auth = None
    solr_username = os.getenv("SOLR_USERNAME")
    solr_password = os.getenv("SOLR_PASSWORD")

    if solr_username is not None and solr_password != "":
        solr_auth = SolrBasicAuth(
            solr_username,
            solr_password,
        )
    host = os.getenv("SOLR_HOST")
    assert host is not None
    zookeeper_host = os.getenv("ZK_HOST")
    assert zookeeper_host is not None

    yield Config(host, zookeeper_host, solr_auth)


@pytest.mark.asyncio
async def test_zookeeper_connection_works(config: Config) -> None:
    wait_for_solr(config.host, 30)
    context = SolrServerContext(
        await ZookeeperEnsembleHostConnector([config.zookeeper_host], 30).connect(),
        config.auth,
    )
    await get_configs(context)


def test_zookeeper_connection_works_blocking(config: Config) -> None:
    wait_for_solr(config.host, 30)
    context = SolrServerContext(
        ZookeeperEnsembleHostConnector([config.zookeeper_host], 30).connect_blocking(),
        config.auth,
    )
    get_configs_blocking(context)


@pytest.mark.asyncio
async def test_solr_single_server_works(config: Config) -> None:
    wait_for_solr(config.host, 30)
    context = SolrServerContext(SolrSingleServerHost(config.host), config.auth)
    await get_configs(context)


@pytest.mark.asyncio
async def test_solr_single_server_works_with_string(config: Config) -> None:
    wait_for_solr(config.host, 30)
    context = SolrServerContext(config.host, config.auth)
    await get_configs(context)


@pytest.mark.asyncio
async def test_multiple_server_works(config: Config) -> None:
    wait_for_solr(config.host, 30)
    context = SolrServerContext(SolrMultipleServerHost([config.host], 5), config.auth)
    await get_configs(context)


def test_solr_multiple_server_works_blocking(config: Config) -> None:
    wait_for_solr(config.host, 30)
    context = SolrServerContext(SolrMultipleServerHost([config.host], 5), config.auth)
    get_configs_blocking(context)
