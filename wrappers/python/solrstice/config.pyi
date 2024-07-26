from os import PathLike
from typing import TYPE_CHECKING, List, Union

if TYPE_CHECKING:
    from solrstice import SolrServerContext

    Somepath = Union[PathLike[str], str]
else:
    Somepath = Union[PathLike, str]

async def upload_config(
    context: "SolrServerContext", config_name: str, config_path: Somepath
) -> None:
    """Uploads a Solr config to a Solr instance

    :param context: SolrServerRequest context
    :param config_name: Name of the config
    :param config_path: Path to the config
    """
    pass

def upload_config_blocking(
    context: "SolrServerContext", config_name: str, config_path: Somepath
) -> None:
    """Uploads a Solr config to a Solr instance

    :param context: SolrServerRequest context
    :param config_name: Name of the config
    :param config_path: Path to the config
    """
    pass

async def delete_config(context: "SolrServerContext", config_name: str) -> None:
    """Deletes a Solr config from a Solr instance

    :param context: SolrServerRequest context
    :param config_name: Name of the config
    """
    pass

def delete_config_blocking(context: "SolrServerContext", config_name: str) -> None:
    """Deletes a Solr config from a Solr instance

    :param context: SolrServerRequest context
    :param config_name: Name of the config
    """
    pass

async def config_exists(context: "SolrServerContext", config_name: str) -> bool:
    """Checks if a Solr config exists on a Solr instance

    :param context: SolrServerRequest context
    :param config_name: Name of the config
    """
    pass

def config_exists_blocking(context: "SolrServerContext", config_name: str) -> bool:
    """Checks if a Solr config exists on a Solr instance

    :param context: SolrServerRequest context
    :param config_name: Name of the config
    """
    pass

async def get_configs(context: "SolrServerContext") -> List[str]:
    """Gets a list of Solr configs on a Solr instance

    :param context: SolrServerRequest context
    """
    pass

def get_configs_blocking(context: "SolrServerContext") -> List[str]:
    """Gets a list of Solr configs on a Solr instance

    :param context: SolrServerRequest builder
    """
    pass

__all__ = [
    "upload_config",
    "upload_config_blocking",
    "delete_config",
    "delete_config_blocking",
    "config_exists",
    "config_exists_blocking",
    "get_configs",
    "get_configs_blocking",
]
