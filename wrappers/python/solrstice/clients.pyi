from os import PathLike
from typing import TYPE_CHECKING, Union, List, Optional, Dict, Any

if TYPE_CHECKING:
    from solrstice.hosts import SolrServerContext
    from solrstice.queries import SelectQueryBuilder, UpdateQueryBuilder, DeleteQueryBuilder
    from solrstice.response import SolrResponse
Somepath = Union[PathLike, str]

class AsyncSolrCloudClient:
    """
    A client for interacting with a SolrCloud cluster asynchronously.

    :param context: The context of the Solr server.
    """
    def __init__(self, context: 'SolrServerContext'):
        pass

    async def upload_config(self, config_name: str, config_path: Somepath) -> None:
        """Uploads a Solr config to a Solr instance

            :param config_name: Name of the config
            :param config_path: Path to the config
        """
        pass

    async def get_configs(self) -> List[str]:
        """Gets a list of Solr configs on a Solr instance

            :param context: SolrServerRequest context
        """
    pass

    async def config_exists(self, config_name: str) -> bool:
        """Checks if a Solr config exists on a Solr instance

            :param config_name: Name of the config
        """
    pass

    async def delete_config(self, config_name: str) -> None:
        """Deletes a Solr config from a Solr instance

            :param config_name: Name of the config
        """
    pass

    async def create_collection(self, name: str, config: str, shards: Optional[int] = 1, replication_factor: Optional[int] = 1) -> None:
        """
        Create a collection on the Solr server.

        :param name: The name of the collection to create.
        :param config: The name of the config to use for the collection.
        :param shards: The number of shards to create.
        :param replication_factor: The replication factor to use.
        """

    async def get_collections(self) -> List[str]:
        """
        Get the list of collections on the Solr server.

        :return: The list of collections on the Solr server.
        """

    async def collection_exists(self, name: str) -> bool:
        """
        Check if a collection exists on the Solr server.

        :param name: The name of the collection to check.
        :return: True if the collection exists, False otherwise.
        """

    async def delete_collection(self, name: str) -> None:
        """
        Delete a config from the Solr server.

        :param context: The Solr server context.
        :param name: The name of the collection to delete.
        """

    async def create_alias(self, name: str, collections: List[str]) -> None:
        """
        Create an alias for a collection on the Solr server

        :param name: The name of the alias to create
        :param collections: The collections to alias
        """

    async def get_aliases(self) -> Dict[str, List[str]]:
        """
        Get all aliases on the Solr server

        :return: A dictionary of aliases to collections
        """

    async def alias_exists(self, name: str) -> bool:
        """
        Check if an alias exists on the Solr server

        :param name: The name of the alias to check
        :return: True if the alias exists, False otherwise
        """

    async def delete_alias(self, name: str) -> None:
        """
        Delete an alias from the Solr server

        :param name: The name of the alias to delete
        """

    async def select(self, builder: 'SelectQueryBuilder', collection: str) -> 'SolrResponse':
        """Execute a select query

            :param builder: The query builder
            :param collection: The collection to query
        """

    async def index(self, builder: 'UpdateQueryBuilder', collection: str, data: List[Dict[str, Any]]) -> 'SolrResponse':
        """Execute an index query

            :param builder: The query builder
            :param collection: The collection to index
            :param data: The data to index
        """

    async def delete(self, builder: 'DeleteQueryBuilder', collection: str) -> 'SolrResponse':
        """Execute a delete query

            :param builder: The query builder
            :param collection: The collection to delete from
        """


class BlockingSolrCloudClient:
    """
    A client for interacting with a SolrCloud cluster non-asynchronously.

    :param context: The context of the Solr server.
    """
    def __init__(self, context: 'SolrServerContext'):
        pass

    def upload_config(self, config_name: str, config_path: Somepath) -> None:
        """Uploads a Solr config to a Solr instance

            :param config_name: Name of the config
            :param config_path: Path to the config
        """
        pass

    def get_configs(self) -> List[str]:
        """Gets a list of Solr configs on a Solr instance

            :param context: SolrServerRequest context
        """
    pass

    def config_exists(self, config_name: str) -> bool:
        """Checks if a Solr config exists on a Solr instance

            :param config_name: Name of the config
        """
    pass

    def delete_config(self, config_name: str) -> None:
        """Deletes a Solr config from a Solr instance

            :param config_name: Name of the config
        """
    pass

    def create_collection(self, name: str, config: str, shards: Optional[int] = 1, replication_factor: Optional[int] = 1) -> None:
        """
        Create a collection on the Solr server.

        :param name: The name of the collection to create.
        :param config: The name of the config to use for the collection.
        :param shards: The number of shards to create.
        :param replication_factor: The replication factor to use.
        """

    def get_collections(self) -> List[str]:
        """
        Get the list of collections on the Solr server.

        :return: The list of collections on the Solr server.
        """

    def collection_exists(self, name: str) -> bool:
        """
        Check if a collection exists on the Solr server.

        :param name: The name of the collection to check.
        :return: True if the collection exists, False otherwise.
        """

    def delete_collection(self, name: str) -> None:
        """
        Delete a config from the Solr server.

        :param context: The Solr server context.
        :param name: The name of the collection to delete.
        """

    def create_alias(self, name: str, collections: List[str]) -> None:
        """
        Create an alias for a collection on the Solr server

        :param name: The name of the alias to create
        :param collections: The collections to alias
        """

    def get_aliases(self) -> Dict[str, List[str]]:
        """
        Get all aliases on the Solr server

        :return: A dictionary of aliases to collections
        """

    def alias_exists(self, name: str) -> bool:
        """
        Check if an alias exists on the Solr server

        :param name: The name of the alias to check
        :return: True if the alias exists, False otherwise
        """

    def delete_alias(self, name: str) -> None:
        """
        Delete an alias from the Solr server

        :param name: The name of the alias to delete
        """

    def select(self, builder: 'SelectQueryBuilder', collection: str) -> 'SolrResponse':
        """Execute a select query

            :param builder: The query builder
            :param collection: The collection to query
        """

    def index(self, builder: 'UpdateQueryBuilder', collection: str, data: List[Dict[str, Any]]) -> 'SolrResponse':
        """Execute an index query

            :param builder: The query builder
            :param collection: The collection to index
            :param data: The data to index
        """

    def delete(self, builder: 'DeleteQueryBuilder', collection: str) -> 'SolrResponse':
        """Execute a delete query

            :param builder: The query builder
            :param collection: The collection to delete from
        """