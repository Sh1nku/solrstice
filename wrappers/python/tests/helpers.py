import json
import os
import time
from dataclasses import dataclass
from typing import Any, Dict, List, Optional
from urllib.error import HTTPError
from urllib.request import urlopen

from dotenv import load_dotenv

from solrstice import (
    AsyncSolrCloudClient,
    OffLoggingPolicy,
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
    error_nginx_host: Optional[str]
    solr_username: Optional[str]
    solr_password: Optional[str]
    solr_auth: Optional[SolrBasicAuth]
    context: SolrServerContext
    config_path: str
    async_client: AsyncSolrCloudClient


def get_path_prefix() -> str:
    iterations = 0
    path_prefix = ""
    while True:
        if not os.path.exists(os.path.join(path_prefix, "test_setup/.env")):
            path_prefix += "../"
        else:
            break
        iterations += 1
        if iterations > 100:
            raise FileNotFoundError("Could not find test_setup/.env")
    return path_prefix


def create_config(logging: bool = False) -> Config:
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
    error_nginx_host = os.getenv("ERROR_NGINX_HOST")
    solr_host = SolrSingleServerHost(host)
    context = SolrServerContext(
        solr_host, solr_auth, OffLoggingPolicy() if not logging else None
    )
    wait_for_solr(host, 30)
    return Config(
        host,
        speedbump_host,
        error_nginx_host,
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


@dataclass
class ErrorTestsSetup:
    error_nginx_host: str
    context: SolrServerContext
    async_client: AsyncSolrCloudClient


def create_nginx_error_config() -> ErrorTestsSetup:
    path = os.path.join(get_path_prefix(), "test_setup/.env")
    load_dotenv(path)
    error_nginx_host = os.getenv("ERROR_NGINX_HOST")
    assert error_nginx_host is not None
    context = SolrServerContext(SolrSingleServerHost(error_nginx_host))
    wait_for_error_nginx(error_nginx_host, 30)
    return ErrorTestsSetup(error_nginx_host, context, AsyncSolrCloudClient(context))


def wait_for_error_nginx(host: str, max_time: int) -> None:
    end = time.time() + max_time
    while time.time() < end:
        try:
            with urlopen(f'{host}{"/status"}') as response:
                if response.status == 200:
                    return
        except Exception:
            pass
        time.sleep(1)
    raise RuntimeError(f"Error nginx did not respond within {max_time} seconds")


@dataclass
class Population:
    id: str
    age: int
    count: int
    interests: List[str]

    @staticmethod
    def from_dict(data: Dict[str, Any]) -> "Population":
        return Population(
            data["id"],
            data["age"],
            data["count"],
            data["interests"] if "interests" in data else [],
        )

    @staticmethod
    def to_dict(population: "Population") -> Dict[str, Any]:
        return {
            "id": population.id,
            "age": population.age,
            "count": population.count,
            "interests": population.interests,
        }


@dataclass
class City:
    id: str
    city_name: str
    population: List[Population]

    @staticmethod
    def from_dict(data: Dict[str, Any]) -> "City":
        return City(
            data["id"],
            data["city_name"],
            (
                [Population.from_dict(x) for x in data["population"]]
                if "population" in data
                else []
            ),
        )

    @staticmethod
    def to_dict(city: "City") -> Dict[str, Any]:
        return {
            "id": city.id,
            "city_name": city.city_name,
            "population": [Population.to_dict(x) for x in city.population],
        }


def load_test_data() -> List[City]:
    with open(os.path.join(get_path_prefix(), "test_setup/test_data.json")) as f:
        return [City.from_dict(x) for x in json.loads(f.read())]


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
