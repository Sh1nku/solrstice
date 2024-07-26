from typing import TYPE_CHECKING, List, Optional

if TYPE_CHECKING:
    from solrstice import SolrServerContext

async def create_collection(
    context: "SolrServerContext",
    name: str,
    config: str,
    shards: Optional[int] = 1,
    replication_factor: Optional[int] = 1,
) -> None:
    """
    Create a collection on the Solr server.

    :param context: The Solr server context.
    :param name: The name of the collection to create.
    :param config: The name of the config to use for the collection.
    :param shards: The number of shards to create.
    :param replication_factor: The replication factor to use.
    """

def create_collection_blocking(
    context: "SolrServerContext",
    name: str,
    config: str,
    shards: Optional[int] = 1,
    replication_factor: Optional[int] = 1,
) -> None:
    """
    Create a collection on the Solr server.

    :param context: The Solr server context.
    :param name: The name of the collection to create.
    :param config: The name of the config to use for the collection.
    :param shards: The number of shards to create.
    :param replication_factor: The replication factor to use.
    """

async def get_collections(context: "SolrServerContext") -> List[str]:
    """
    Get the list of collections on the Solr server.

    :param context: The Solr server context.
    :return: The list of collections on the Solr server.
    """

def get_collections_blocking(context: "SolrServerContext") -> List[str]:
    """
    Get the list of collections on the Solr server.

    :param context: The Solr server context.
    :return: The list of collections on the Solr server.
    """

async def collection_exists(context: "SolrServerContext", name: str) -> bool:
    """
    Check if a collection exists on the Solr server.

    :param context: The Solr server context.
    :param name: The name of the collection to check.
    :return: True if the collection exists, False otherwise.
    """

def collection_exists_blocking(context: "SolrServerContext", name: str) -> bool:
    """
    Check if a collection exists on the Solr server.

    :param context: The Solr server context.
    :param name: The name of the collection to check.
    :return: True if the collection exists, False otherwise.
    """

async def delete_collection(context: "SolrServerContext", name: str) -> None:
    """
    Delete a config from the Solr server.

    :param context: The Solr server context.
    :param name: The name of the collection to delete.
    """

def delete_collection_blocking(context: "SolrServerContext", name: str) -> None:
    """
    Delete a config from the Solr server.

    :param context: The Solr server context.
    :param name: The name of the collection to delete.
    """

__all__ = [
    "create_collection",
    "create_collection_blocking",
    "get_collections",
    "get_collections_blocking",
    "collection_exists",
    "collection_exists_blocking",
    "delete_collection",
    "delete_collection_blocking",
]
