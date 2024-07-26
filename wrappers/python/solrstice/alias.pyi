from typing import TYPE_CHECKING, Dict, List

if TYPE_CHECKING:
    from solrstice import SolrServerContext

async def create_alias(
    context: "SolrServerContext", name: str, collections: List[str]
) -> None:
    """
    Create an alias for a collection on the Solr server

    :param context: The Solr server context
    :param name: The name of the alias to create
    :param collections: The collections to alias
    """

def create_alias_blocking(
    context: "SolrServerContext", name: str, collections: List[str]
) -> None:
    """
    Create an alias for a collection on the Solr server

    :param context: The Solr server context
    :param name: The name of the alias to create
    :param collections: The collections to alias
    """

async def get_aliases(context: "SolrServerContext") -> Dict[str, List[str]]:
    """
    Get all aliases on the Solr server

    :param context: The Solr server context
    :return: A dictionary of aliases to collections
    """

def get_aliases_blocking(context: "SolrServerContext") -> Dict[str, List[str]]:
    """
    Get all aliases on the Solr server

    :param context: The Solr server context
    :return: A dictionary of aliases to collections
    """

async def alias_exists(context: "SolrServerContext", name: str) -> bool:
    """
    Check if an alias exists on the Solr server

    :param context: The Solr server context
    :param name: The name of the alias to check
    :return: True if the alias exists, False otherwise
    """

def alias_exists_blocking(context: "SolrServerContext", name: str) -> bool:
    """
    Check if an alias exists on the Solr server

    :param context: The Solr server context
    :param name: The name of the alias to check
    :return: True if the alias exists, False otherwise
    """

async def delete_alias(context: "SolrServerContext", name: str) -> None:
    """
    Delete an alias from the Solr server

    :param context: The Solr server context
    :param name: The name of the alias to delete
    """

def delete_alias_blocking(context: "SolrServerContext", name: str) -> None:
    """
    Delete an alias from the Solr server

    :param context: The Solr server context
    :param name: The name of the alias to delete
    """

__all__ = [
    "create_alias",
    "create_alias_blocking",
    "get_aliases",
    "get_aliases_blocking",
    "alias_exists",
    "alias_exists_blocking",
    "delete_alias",
    "delete_alias_blocking",
]
