import os
from dataclasses import dataclass
from pathlib import Path
from typing import Optional

import pytest
from dotenv import load_dotenv
from helpers import wait_for_solr

from solrstice.auth import SolrBasicAuth
from solrstice.config import get_configs, get_configs_blocking
from solrstice.hosts import (
    SolrMultipleServerHost,
    SolrServerContext,
    SolrSingleServerHost,
    ZookeeperEnsembleHostConnector,
)


@dataclass
class Config:
    host: str
    zookeeper_host: str
    auth: Optional[SolrBasicAuth]


@pytest.fixture()
def config() -> Config:
    path = Path("../../test_setup/.env").resolve()
    load_dotenv(path)
    solr_auth = None
    if os.getenv("SOLR_USERNAME") is not None and os.getenv("SOLR_PASSWORD") is not "":
        solr_auth = SolrBasicAuth(
            os.getenv("SOLR_USERNAME"),
            os.getenv("SOLR_PASSWORD"),
        )
    host = os.getenv("SOLR_HOST")

    yield Config(host, os.getenv("ZK_HOST"), solr_auth)


@pytest.mark.asyncio
async def test_zookeeper_connection_works(config: Config):
    wait_for_solr(config.host, 30)
    context = SolrServerContext(
        await ZookeeperEnsembleHostConnector([config.zookeeper_host], 30).connect(),
        config.auth,
    )
    await get_configs(context)


def test_zookeeper_connection_works_blocking(config: Config):
    wait_for_solr(config.host, 30)
    context = SolrServerContext(
        ZookeeperEnsembleHostConnector([config.zookeeper_host], 30).connect_blocking(),
        config.auth,
    )
    get_configs_blocking(context)


@pytest.mark.asyncio
async def test_solr_single_server_works(config: Config):
    wait_for_solr(config.host, 30)
    context = SolrServerContext(SolrSingleServerHost(config.host), config.auth)
    await get_configs(context)


@pytest.mark.asyncio
async def test_solr_single_server_works_with_string(config: Config):
    wait_for_solr(config.host, 30)
    context = SolrServerContext(config.host, config.auth)
    await get_configs(context)


@pytest.mark.asyncio
async def test_multiple_server_works(config: Config):
    wait_for_solr(config.host, 30)
    context = SolrServerContext(SolrMultipleServerHost([config.host], 5), config.auth)
    await get_configs(context)


def test_solr_multiple_server_works_blocking(config: Config):
    wait_for_solr(config.host, 30)
    context = SolrServerContext(SolrMultipleServerHost([config.host], 5), config.auth)
    get_configs_blocking(context)
