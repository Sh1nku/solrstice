import os
import time
from dataclasses import dataclass
from typing import List, Optional
from urllib.error import HTTPError
from urllib.request import urlopen

from dataclasses_json import DataClassJsonMixin, dataclass_json
from dotenv import load_dotenv

from solrstice import (
    AsyncSolrCloudClient,
    SolrBasicAuth,
    SolrServerContext,
    SolrSingleServerHost,
    UpdateQuery,
)
from solrstice.collection import create_collection, delete_collection
from solrstice.config import delete_config, upload_config


@dataclass
class Config:
    solr_host: str
    speedbump_host: Optional[str]
    solr_username: Optional[str]
    solr_password: Optional[str]
    solr_auth: Optional[SolrBasicAuth]
    context: SolrServerContext
    config_path: str
    async_client: AsyncSolrCloudClient


def get_path_prefix() -> str:
    path_prefix = "../../"
    if not os.path.exists(os.path.join(path_prefix, "test_setup/.env")):
        path_prefix = "../../../"
    return path_prefix


def create_config() -> Config:
    path = os.path.join(get_path_prefix(), "test_setup/.env")
    load_dotenv(path)
    solr_auth = None
    solr_username = os.getenv("SOLR_USERNAME")
    solr_password = os.getenv("SOLR_PASSWORD")
    if solr_username:
        solr_auth = SolrBasicAuth(solr_username, solr_password)
    host = os.getenv("SOLR_HOST")
    assert host is not None
    speedbump_host = os.getenv("SPEEDBUMP_HOST")
    solr_host = SolrSingleServerHost(host)
    context = SolrServerContext(solr_host, solr_auth)
    wait_for_solr(host, 30)
    return Config(
        host,
        speedbump_host,
        solr_username,
        solr_password,
        solr_auth,
        context,
        os.path.join(get_path_prefix(), "test_setup/test_collection"),
        AsyncSolrCloudClient(context),
    )


def wait_for_solr(host: str, max_time: int) -> None:
    end = time.time() + max_time
    while time.time() < end:
        try:
            with urlopen(
                f'{host}{"/solr/admin/collections"}?action=CLUSTERSTATUS'
            ) as response:
                if response.status == 200:
                    return
        except HTTPError as e:
            if e.code == 401:
                return
        except Exception:
            pass
        time.sleep(1)
    raise RuntimeError(f"Solr did not respond within {max_time} seconds")


@dataclass_json
@dataclass
class Population(DataClassJsonMixin):
    id: str
    age: int
    count: int
    interests: List[str]


@dataclass_json
@dataclass
class City(DataClassJsonMixin):
    id: str
    city_name: str
    population: List[Population]


def load_test_data() -> List[City]:
    with open(os.path.join(get_path_prefix(), "test_setup/test_data.json")) as f:
        return City.schema().loads(f.read(), many=True)


async def index_test_data(context: SolrServerContext, name: str) -> None:
    data = load_test_data()
    update_builder = UpdateQuery()
    await update_builder.execute(context, name, [City.to_dict(x) for x in data])


async def setup_collection(
    context: SolrServerContext, name: str, config_path: str
) -> None:
    try:
        await delete_collection(context, name)
    except RuntimeError:
        pass
    try:
        await delete_config(context, name)
    except RuntimeError:
        pass
    await upload_config(
        context,
        name,
        config_path,
    )
    await create_collection(context, name, name, 1, 1)


async def teardown_collection(context: SolrServerContext, name: str) -> None:
    try:
        await delete_collection(context, name)
    except RuntimeError:
        pass
    try:
        await delete_config(context, name)
    except RuntimeError:
        pass
