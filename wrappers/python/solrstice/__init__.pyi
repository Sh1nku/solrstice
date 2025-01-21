import abc
from abc import ABC
from enum import Enum
from os import PathLike
from typing import TYPE_CHECKING, Any, Dict, List, Optional, Union

if TYPE_CHECKING:
    from solrstice.models import SolrResponse


# region auth
class SolrAuth(ABC):
    """
    Base class for Solr authentication
    Valid implementations are :class:`SolrBasicAuth`
    """


class SolrBasicAuth(SolrAuth):
    """Basic authentication for Solr

    :param username: Username for Solr
    :param password: Password for Solr
    """

    def __init__(self, username: str, password: Optional[str] = None) -> None:
        pass


# endregion

# region def_type
class QueryOperator(Enum):
    """
    The default query operator
    """

    AND = "AND"
    OR = "OR"


class DefType(abc.ABC):
    """
    Specify query type.
    Valid implementations are :class:`LuceneQuery`, :class:`DismaxQuery`, and :class:`EdismaxQuery`
    """

    pass


class LuceneQuery(DefType):
    """
    Create a Lucene query builder.

    :param q_op: Query operator.
    :param df: Default field
    :param sow: Split on whitespace
    """

    def __init__(
            self,
            q_op: Optional[QueryOperator] = None,
            df: Optional[str] = None,
            sow: Optional[bool] = None,
    ):
        pass


class DismaxQuery(DefType):
    """
    Create a DisMax query builder.

    :param q_alt: Alternate query
    :param qf: Query fields
    :param mm: Minimum match
    :param pf: Phrase fields
    :param ps: Phrase slop
    :param qs: Query slop
    :param tie: Tie breaker
    :param bq: Boost query
    :param bf: Boost functions
    """

    def __init__(
            self,
            q_alt: Optional[str] = None,
            qf: Optional[str] = None,
            mm: Optional[str] = None,
            pf: Optional[str] = None,
            ps: Optional[str] = None,
            qs: Optional[str] = None,
            tie: Optional[str] = None,
            bq: Optional[List[str]] = None,
            bf: Optional[List[str]] = None,
    ):
        pass


class EdismaxQuery(DefType):
    """
    Create an Edismax query builder.

    :param q_alt: Alternate query
    :param qf: Query fields
    :param mm: Minimum match
    :param mm_auto_relax: Automatically relax mm
    :param pf: Phrase fields
    :param pf2: Phrase fields 2
    :param pf3: Phrase fields 3
    :param ps: Phrase slop
    :param ps2: Phrase slop 2
    :param ps3: Phrase slop 3
    :param qs: Query slop
    :param tie: Tie breaker
    :param bq: Boost query
    :param bf: Boost functions
    :param sow: Split on whitespace
    :param boost: Boost
    :param lowercase_operators: Lowercase operators
    :param stopwords: Stopwords
    :param uf: User fields
    """

    def __init__(
            self,
            q_alt: Optional[str] = None,
            qf: Optional[str] = None,
            mm: Optional[str] = None,
            mm_auto_relax: Optional[bool] = None,
            pf: Optional[str] = None,
            pf2: Optional[str] = None,
            pf3: Optional[str] = None,
            ps: Optional[str] = None,
            ps2: Optional[str] = None,
            ps3: Optional[str] = None,
            qs: Optional[str] = None,
            tie: Optional[str] = None,
            bq: Optional[List[str]] = None,
            bf: Optional[List[str]] = None,
            sow: Optional[bool] = None,
            boost: Optional[List[str]] = None,
            lowercase_operators: Optional[bool] = None,
            stopwords: Optional[bool] = None,
            uf: Optional[str] = None,
    ):
        pass


# endregion

# region facet_set

class FacetSetComponent:
    """
    Creates a facet set component allowing for counting of facets of different types

    :param queries: A list of queries to facet on
    :param fields: A list of fields to facet on
    :param pivots: A list of pivots to facet on
    """

    def __init__(
            self,
            queries: Optional[List[str]] = None,
            fields: Optional["FieldFacetComponent"] = None,
            pivots: Optional["PivotFacetComponent"] = None,
    ):
        pass


class PivotFacetComponent:
    """
    Allows faceting using pivots

    :param pivots: A list of pivots to facet on
    :param min_count: The minimum count for a facet to be returned
    """

    def __init__(self, pivots: List[str], min_count: Optional[str] = None):
        pass


class FieldFacetComponent:
    """
    Allows faceting using fields

    :param fields: A list of fields to facet on
    :param exclude_terms: Comma separated list of terms to exclude from the facet. Escaping needs to be done manually
    """

    def __init__(
            self,
            fields: List["FieldFacetEntry"],
            exclude_terms: Optional[str] = None,
    ):
        pass


class FieldFacetSort(Enum):
    """
    The sort order for a field facet
    """

    Count = "Count"
    Index = "Index"


class FieldFacetMethod(Enum):
    """
    The method for a field facet
    """

    Enum = "Enum"
    Fc = "Fc"
    Fcs = "Fcs"


class FieldFacetEntry:
    """

    :param field: The field to facet on
    :param prefix: Limit the terms to those starting with the given prefix
    :param contains: Limit the terms to those containing the given substring
    :param contains_ignore_case: Whether to ignore case when filtering by contains
    :param sort: The sort order for the facet
    :param limit: The maximum number of facet entries to return
    :param offset: The offset into the facet entries to return
    :param min_count: The minimum count needed for a facet to be returned
    :param missing: Whether to include a facet for missing values
    :param method: The method to use for the facet. Default is Fc
    :param enum_cache_min_df: The minimum document frequency for a term to be included in the facet cache. Only used for Enum method
    :param exists: Cap facet counts by 1
    """

    def __init__(
            self,
            field: str,
            prefix: Optional[str] = None,
            contains: Optional[str] = None,
            contains_ignore_case: Optional[bool] = None,
            sort: Optional["FieldFacetSort"] = None,
            limit: Optional[int] = None,
            offset: Optional[int] = None,
            min_count: Optional[int] = None,
            missing: Optional[bool] = None,
            method: Optional["FieldFacetMethod"] = None,
            enum_cache_min_df: Optional[int] = None,
            exists: Optional[bool] = None,
    ):
        pass


# endregion

# region json_facet

class JsonFacetComponent:
    """
    Json faceting component

    :param facets: A dictionary of facets to apply to the query
    """

    def __init__(self, facets: Optional[Dict[str, "JsonFacetType"]] = None):
        pass


class JsonFacetType(abc.ABC):
    """
    Base class for a json facet type
    Valid implementations are :class:`JsonTermsFacet`, :class:`JsonQueryFacet`, and :class:`JsonStatFacet`
    """

    pass


class JsonTermsFacet(JsonFacetType):
    """
    Do a terms facet on a field

    :param field: The field to facet on
    :param offset: The offset into the list of terms
    :param limit: The maximum number of terms to return
    :param sort: The sort order for the terms
    """

    def __init__(
            self,
            field: str,
            offset: Optional[int] = None,
            limit: Optional[int] = None,
            sort: Optional[str] = None,
            facets: Optional[Dict[str, JsonFacetType]] = None,
    ):
        pass


class JsonQueryFacet(JsonFacetType):
    """
    Do a query facet

    :param q: The query to facet on
    :param limit: The maximum number of terms to return
    :param offset: The offset into the list of terms
    :param sort: The sort order for the terms
    :param fq: A list of filters to apply to the query
    :param facets: A list of sub-facets to apply to the query
    """

    def __init__(
            self,
            q: str,
            limit: Optional[int] = None,
            offset: Optional[int] = None,
            sort: Optional[str] = None,
            fq: Optional[str] = None,
            facets: Optional[Dict[str, JsonFacetType]] = None,
    ):
        pass


class JsonStatFacet(JsonFacetType):
    """
    Do a stat facet

    :param query: The query to facet on
    """

    def __init__(self, query: str):
        pass


# endregion

# region hosts
class SolrHost(ABC):
    """
    Base class for Solr hosts
    Valid implementations are :class:`SolrSingleServerHost`, :class:`SolrMultipleServerHost`, and :class:`ZookeeperEnsembleHost`
    """


class SolrSingleServerHost(SolrHost):
    """Solr host for a single Solr instance

    :param host: Hostname of the Solr instance
    """

    def __init__(self, host: str) -> None:
        pass


class SolrMultipleServerHost(SolrHost):
    """Solr host for multiple solr instances

    :param hosts: List of Solr instances
    :param timeout: Amount of seconds before declaring a node not responding, and going to the next
    """

    def __init__(self, hosts: List[str], timeout: float) -> None:
        pass


class ZookeeperEnsembleHost(SolrHost):
    """Zookeeper ensemble connection. Cannot be constructed directly, use ZookeeperEnsembleHostConnector instead"""


class ZookeeperEnsembleHostConnector:
    """The builder for a Zookeeper ensemble host

    :param hosts: List of Zookeeper instances
    :param timeout: Timeout for connecting to Zookeeper
    """

    def __init__(self, hosts: List[str], timeout: float) -> None:
        pass

    async def connect(self) -> ZookeeperEnsembleHost:
        """Connect to the Zookeeper ensemble"""
        pass

    def connect_blocking(self) -> ZookeeperEnsembleHost:
        """Connect to the Zookeeper ensemble"""
        pass


class LoggingPolicy(abc.ABC):
    """
    Policy describing how to log solr queries.
    Valid values are :class:`OffLoggingPolicy`, :class:`FastLoggingPolicy`, and :class:`PrettyLoggingPolicy`
    """

    pass


class OffLoggingPolicy(LoggingPolicy):
    """Do not log requests"""

    def __init__(self) -> None:
        pass


class FastLoggingPolicy(LoggingPolicy):
    """For each request create a logging::DEBUG message with url, headers, and body

    :param max_body_length: How long to allow the body to be before dropping to log it
    """

    def __init__(self, max_body_length: int) -> None:
        pass


class PrettyLoggingPolicy(LoggingPolicy):
    """For each request create a logging::DEBUG message with url, headers, and a pretty body

    :param max_body_length: How long to allow the body to be before dropping to log it
    """

    def __init__(self, max_body_length: int) -> None:
        pass


class SolrServerContext:
    """The context for a connection to a solr instance

    :param host: An instance of SolrHost specifying how to connect to a solr instance. If given as a string it creates a :class:`SolrSingleServerHost`
    :param auth: An instance of SolrAuth specifying how to authenticate with the solr instance
    :param logging_policy: How to log solr queries, valid values are :class:`OffLoggingPolicy`, :class:`FastLoggingPolicy`, and :class:`PrettyLoggingPolicy`
    """

    def __init__(
            self,
            host: Union[SolrHost, str],
            auth: Optional[SolrAuth] = None,
            logging_policy: Optional[LoggingPolicy] = None,
    ):
        pass


# endregion

# region group
class GroupFormatting(Enum):
    Simple = "Simple"
    Grouped = "Grouped"


class GroupingComponent:
    """
    Grouping component, used in conjunction with SelectQuery

    :param fields: Fields to group results by
    :param queries: Queries to group by
    :param limit: Limit the number of groups returned for each set of grouped documents
    :param offset: Offset the number of groups returned for each set of grouped documents
    :param sort: Sort the groups
    :param format: The group format, either Simple, or Grouped
    :param main: Should the group result be the main result
    :param n_groups: Should the number of groups be counted
    :param truncate: Truncate
    :param facet: Facet
    """

    def __init__(
            self,
            fields: Optional[List[str]] = None,
            queries: Optional[List[str]] = None,
            limit: Optional[int] = None,
            offset: Optional[int] = None,
            sort: Optional[List[str]] = None,
            format: Optional[GroupFormatting] = None,
            main: Optional[bool] = None,
            n_groups: Optional[bool] = None,
            truncate: Optional[bool] = None,
            facet: Optional[bool] = None,
    ):
        pass


# endregion

# region queries
class SelectQuery:
    """Builder for a select query

    :param q: The query string
    :param fq: The filter queries
    :param fl: The fields to return
    :param sort: The sort order
    :param rows: The number of rows to return
    :param start: Set the start index
    :param cursor_mark: Set the cursor mark
    :param grouping: Set the grouping component
    :param def_type: Set the query type
    :param facet_set: Facet counts
    :param json_facet: Json facets
    :param additional_params: Additional parameters to send with the request, e.g. when using subqueries.
    """

    def __init__(
            self,
            q: Optional[str] = None,
            fq: Optional[List[str]] = None,
            fl: Optional[List[str]] = None,
            sort: Optional[List[str]] = None,
            rows: Optional[int] = None,
            start: Optional[int] = None,
            cursor_mark: Optional[str] = None,
            grouping: Optional["GroupingComponent"] = None,
            def_type: Optional["DefType"] = None,
            facet_set: Optional["FacetSetComponent"] = None,
            json_facet: Optional["JsonFacetComponent"] = None,
            additional_params: Optional[Dict[str, Any]] = None,
    ) -> None:
        pass

    async def execute(
            self, context: "SolrServerContext", collection: str
    ) -> "SolrResponse":
        """Execute the query

        :param context: The context for the connection to the solr instance
        :param collection: The collection to query
        """

    def execute_blocking(
            self, context: "SolrServerContext", collection: str
    ) -> "SolrResponse":
        """Execute the query

        :param context: The context for the connection to the solr instance
        :param collection: The collection to query
        """


class CommitType(Enum):
    Hard = "Hard"
    Soft = "Soft"


class UpdateQuery:
    """Builder for an update query

    :param handler: The handler for the update query
    :param commit_type: The commit type for the update query
    """

    def __init__(
            self,
            handler: Optional[str] = "update",
            commit_type: Optional[CommitType] = CommitType.Hard,
    ) -> None:
        pass

    async def execute(
            self, context: "SolrServerContext", collection: str, data: List[Dict[str, Any]]
    ) -> "SolrResponse":
        """Execute the query

        :param context: The context for the connection to the solr instance
        :param collection: The collection to update
        :param data: The data to update
        """

    def execute_blocking(
            self, context: "SolrServerContext", collection: str, data: List[Dict[str, Any]]
    ) -> "SolrResponse":
        """Execute the query

        :param context: The context for the connection to the solr instance
        :param collection: The collection to update
        :param data: The data to update
        """


class DeleteQuery:
    """Builder for a delete query

    :param handler: The handler for the delete query
    :param commit_type: The commit type for the delete query
    """

    def __init__(
            self,
            handler: Optional[str] = "update",
            commit_type: Optional[CommitType] = CommitType.Hard,
            ids: Optional[List[str]] = None,
            queries: Optional[List[str]] = None,
    ) -> None:
        pass

    async def execute(
            self, context: "SolrServerContext", collection: str
    ) -> "SolrResponse":
        """Execute the query

        :param context: The context for the connection to the solr instance
        :param collection: The collection to delete from
        """

    def execute_blocking(
            self, context: "SolrServerContext", collection: str
    ) -> "SolrResponse":
        """Execute the query

        :param context: The context for the connection to the solr instance
        :param collection: The collection to delete from
        """


# endregion

# region clients

class AsyncSolrCloudClient:
    """
    A client for interacting with a SolrCloud cluster asynchronously.

    :param context: The context of the Solr server.

    >>> from solrstice import SolrServerContext, SolrBasicAuth, AsyncSolrCloudClient
    >>> context = SolrServerContext("http://localhost:8983", SolrBasicAuth("user", "pass"))
    >>> client = AsyncSolrCloudClient(context)

    """

    def __init__(self, context: "SolrServerContext"):
        pass

    async def upload_config(
            self, config_name: str, config_path: Union[PathLike[str], str]
    ) -> None:
        """Uploads a Solr config to a Solr instance

        :param config_name: Name of the config
        :param config_path: Path to the config

        >>> from solrstice import SolrServerContext, AsyncSolrCloudClient
        >>> context = SolrServerContext("http://localhost:8983")
        >>> client = AsyncSolrCloudClient(context)
        >>> async def upload_config() -> None:
        ...     await client.upload_config("config_name", "/path/to/config")

        """

    async def get_configs(self) -> List[str]:
        """Gets a list of Solr configs on a Solr instance
        :returns: A list of Solr configs

        >>> from solrstice import SolrServerContext, AsyncSolrCloudClient
        >>> from typing import List
        >>> context = SolrServerContext("http://localhost:8983")
        >>> client = AsyncSolrCloudClient(context)
        >>> async def get_configs() -> List[str]:
        ...     return await client.get_configs()

        """

    async def config_exists(self, config_name: str) -> bool:
        """Checks if a Solr config exists on a Solr instance

        :param config_name: Name of the config
        :returns: True if the config exists, False otherwise

        >>> from solrstice import SolrServerContext, AsyncSolrCloudClient
        >>> context = SolrServerContext("http://localhost:8983")
        >>> client = AsyncSolrCloudClient(context)
        >>> async def config_exists() -> bool:
        ...     return await client.config_exists("config_name")

        """

    async def delete_config(self, config_name: str) -> None:
        """Deletes a Solr config from a Solr instance
        :param config_name: Name of the config

        >>> from solrstice import SolrServerContext, AsyncSolrCloudClient
        >>> context = SolrServerContext("http://localhost:8983")
        >>> client = AsyncSolrCloudClient(context)
        >>> async def delete_config() -> None:
        ...     await client.delete_config("config_name")

        """

    pass

    async def create_collection(
            self,
            name: str,
            config: str,
            shards: Optional[int] = 1,
            replication_factor: Optional[int] = 1,
    ) -> None:
        """
        Create a collection on the Solr server.

        :param name: The name of the collection to create.
        :param config: The name of the config to use for the collection.
        :param shards: The number of shards to create.
        :param replication_factor: The replication factor to use.

        >>> from solrstice import SolrServerContext, AsyncSolrCloudClient
        >>> context = SolrServerContext("http://localhost:8983")
        >>> client = AsyncSolrCloudClient(context)
        >>> async def create_collection() -> None:
        ...     await client.create_collection("collection_name", "config_name", 1, 1)

        """

    async def get_collections(self) -> List[str]:
        """
        Get the list of collections on the Solr server.

        :return: The list of collections on the Solr server.

        >>> from solrstice import SolrServerContext, AsyncSolrCloudClient
        >>> from typing import List
        >>> context = SolrServerContext("http://localhost:8983")
        >>> client = AsyncSolrCloudClient(context)
        >>> async def get_collections() -> List[str]:
        ...     return await client.get_collections()

        """

    async def collection_exists(self, name: str) -> bool:
        """
        Check if a collection exists on the Solr server.

        :param name: The name of the collection to check.
        :return: True if the collection exists, False otherwise.

        >>> from solrstice import SolrServerContext, AsyncSolrCloudClient
        >>> context = SolrServerContext("http://localhost:8983")
        >>> client = AsyncSolrCloudClient(context)
        >>> async def collection_exists() -> bool:
        ...     return await client.collection_exists("collection_name")

        """

    async def delete_collection(self, name: str) -> None:
        """
        Delete a config from the Solr server.
        :param name: The name of the collection to delete.

        >>> from solrstice import SolrServerContext, AsyncSolrCloudClient
        >>> context = SolrServerContext("http://localhost:8983")
        >>> client = AsyncSolrCloudClient(context)
        >>> async def delete_collection() -> None:
        ...     await client.delete_collection("collection_name")

        """

    async def create_alias(self, name: str, collections: List[str]) -> None:
        """
        Create an alias for a collection on the Solr server

        :param name: The name of the alias to create
        :param collections: The collections to alias

        >>> from solrstice import SolrServerContext, AsyncSolrCloudClient
        >>> context = SolrServerContext("http://localhost:8983")
        >>> client = AsyncSolrCloudClient(context)
        >>> async def create_alias() -> None:
        ...     await client.create_alias("alias_name", ["collection_name"])

        """

    async def get_aliases(self) -> Dict[str, List[str]]:
        """
        Get all aliases on the Solr server

        :return: A dictionary of aliases to collections

        >>> from solrstice import SolrServerContext, AsyncSolrCloudClient
        >>> from typing import Dict, List
        >>> context = SolrServerContext("http://localhost:8983")
        >>> client = AsyncSolrCloudClient(context)
        >>> async def get_aliases() -> Dict[str, List[str]]:
        ...     return await client.get_aliases()

        """

    async def alias_exists(self, name: str) -> bool:
        """
        Check if an alias exists on the Solr server

        :param name: The name of the alias to check
        :return: True if the alias exists, False otherwise

        >>> from solrstice import SolrServerContext, AsyncSolrCloudClient
        >>> context = SolrServerContext("http://localhost:8983")
        >>> client = AsyncSolrCloudClient(context)
        >>> async def alias_exists() -> bool:
        ...     return await client.alias_exists("alias_name")

        """

    async def delete_alias(self, name: str) -> None:
        """
        Delete an alias from the Solr server

        :param name: The name of the alias to delete

        >>> from solrstice import SolrServerContext, AsyncSolrCloudClient
        >>> context = SolrServerContext("http://localhost:8983")
        >>> client = AsyncSolrCloudClient(context)
        >>> async def delete_alias() -> None:
        ...     await client.delete_alias("alias_name")

        """

    async def select(self, builder: "SelectQuery", collection: str) -> "SolrResponse":
        """Execute a select query

        Also see :class:`SelectQuery`

        :param builder: The query builder
        :param collection: The collection to query
        :returns: The response from the query

        >>> from solrstice import SolrServerContext, AsyncSolrCloudClient, SelectQuery
        >>> from solrstice.models import SolrResponse
        >>> context = SolrServerContext("http://localhost:8983")
        >>> client = AsyncSolrCloudClient(context)
        >>> async def select() -> SolrResponse:
        ...     query = SelectQuery(q="*:*", rows=10)
        ...     return await client.select(query, "collection_name")

        """

    async def index(
            self, builder: "UpdateQuery", collection: str, data: List[Dict[str, Any]]
    ) -> "SolrResponse":
        """Execute an index query

        Also see :class:`UpdateQuery`

        :param builder: The query builder
        :param collection: The collection to index
        :param data: The data to index
        :returns: The response from the query

        >>> from solrstice import SolrServerContext, AsyncSolrCloudClient, UpdateQuery
        >>> from solrstice.models import SolrResponse
        >>> context = SolrServerContext("http://localhost:8983")
        >>> client = AsyncSolrCloudClient(context)
        >>> async def index() -> SolrResponse:
        ...     query = UpdateQuery()
        ...     data_to_index = [{"id": "1", "name": "test"}]
        ...     return await client.index(query, "collection_name", data_to_index)

        """

    async def delete(self, builder: "DeleteQuery", collection: str) -> "SolrResponse":
        """Execute a delete query

        Also see :class:`DeleteQuery`

        :param builder: The query builder
        :param collection: The collection to delete from
        :returns: The response from the query

        >>> from solrstice import SolrServerContext, AsyncSolrCloudClient, DeleteQuery
        >>> from solrstice.models import SolrResponse
        >>> context = SolrServerContext("http://localhost:8983")
        >>> client = AsyncSolrCloudClient(context)
        >>> async def delete() -> SolrResponse:
        ...     query = DeleteQuery(ids=["1"])
        ...     return await client.delete(query, "collection_name")

        """


class BlockingSolrCloudClient:
    """
    A client for interacting with a SolrCloud cluster non-asynchronously.

    :param context: The context of the Solr server.

    >>> from solrstice import SolrServerContext, SolrBasicAuth, BlockingSolrCloudClient
    >>> context = SolrServerContext("http://localhost:8983", SolrBasicAuth("user", "pass"))
    >>> client = BlockingSolrCloudClient(context)

    """

    def __init__(self, context: "SolrServerContext"):
        pass

    def upload_config(
            self, config_name: str, config_path: Union[PathLike[str], str]
    ) -> None:
        """Uploads a Solr config to a Solr instance

        :param config_name: Name of the config
        :param config_path: Path to the config

        >>> from solrstice import SolrServerContext, BlockingSolrCloudClient
        >>> context = SolrServerContext("http://localhost:8983")
        >>> client = BlockingSolrCloudClient(context)
        >>> def upload_config() -> None:
        ...     client.upload_config("config_name", "/path/to/config")

        """

    def get_configs(self) -> List[str]:
        """
        Gets a list of Solr configs on a Solr instance
        :returns: A list of Solr configs

        >>> from solrstice import SolrServerContext, BlockingSolrCloudClient
        >>> from typing import List
        >>> context = SolrServerContext("http://localhost:8983")
        >>> client = BlockingSolrCloudClient(context)
        >>> def get_configs() -> List[str]:
        ...     return client.get_configs()

        """

    def config_exists(self, config_name: str) -> bool:
        """Checks if a Solr config exists on a Solr instance

        :param config_name: Name of the config
        :returns: True if the config exists, False otherwise

        >>> from solrstice import SolrServerContext, BlockingSolrCloudClient
        >>> context = SolrServerContext("http://localhost:8983")
        >>> client = BlockingSolrCloudClient(context)
        >>> def config_exists() -> bool:
        ...     return client.config_exists("config_name")

        """

    def delete_config(self, config_name: str) -> None:
        """Deletes a Solr config from a Solr instance

        :param config_name: Name of the config

        >>> from solrstice import SolrServerContext, BlockingSolrCloudClient
        >>> context = SolrServerContext("http://localhost:8983")
        >>> client = BlockingSolrCloudClient(context)
        >>> def delete_config() -> None:
        ...     client.delete_config("config_name")

        """

    def create_collection(
            self,
            name: str,
            config: str,
            shards: Optional[int] = 1,
            replication_factor: Optional[int] = 1,
    ) -> None:
        """
        Create a collection on the Solr server.

        :param name: The name of the collection to create.
        :param config: The name of the config to use for the collection.
        :param shards: The number of shards to create.
        :param replication_factor: The replication factor to use.

        >>> from solrstice import SolrServerContext, BlockingSolrCloudClient
        >>> context = SolrServerContext("http://localhost:8983")
        >>> client = BlockingSolrCloudClient(context)
        >>> def create_collection() -> None:
        ...    client.create_collection("collection_name", "config_name", 1, 1)

        """

    def get_collections(self) -> List[str]:
        """
        Get the list of collections on the Solr server.

        :return: The list of collections on the Solr server.

        >>> from solrstice import SolrServerContext, BlockingSolrCloudClient
        >>> from typing import List
        >>> context = SolrServerContext("http://localhost:8983")
        >>> client = BlockingSolrCloudClient(context)
        >>> def get_collections() -> List[str]:
        ...     return client.get_collections()

        """

    def collection_exists(self, name: str) -> bool:
        """
        Check if a collection exists on the Solr server.

        :param name: The name of the collection to check.
        :return: True if the collection exists, False otherwise.

        >>> from solrstice import SolrServerContext, BlockingSolrCloudClient
        >>> context = SolrServerContext("http://localhost:8983")
        >>> client = BlockingSolrCloudClient(context)
        >>> def collection_exists() -> bool:
        ...     return client.collection_exists("collection_name")

        """

    def delete_collection(self, name: str) -> None:
        """
        Delete a config from the Solr server.

        :param context: The Solr server context.
        :param name: The name of the collection to delete.

        >>> from solrstice import SolrServerContext, BlockingSolrCloudClient
        >>> context = SolrServerContext("http://localhost:8983")
        >>> client = BlockingSolrCloudClient(context)

        >>> def delete_collection() -> None:
        ...     client.delete_collection("collection_name")

        """

    def create_alias(self, name: str, collections: List[str]) -> None:
        """
        Create an alias for a collection on the Solr server

        :param name: The name of the alias to create
        :param collections: The collections to alias

        >>> from solrstice import SolrServerContext, BlockingSolrCloudClient
        >>> context = SolrServerContext("http://localhost:8983")
        >>> client = BlockingSolrCloudClient(context)
        >>> def create_alias() -> None:
        ...     client.create_alias("alias_name", ["collection_name"])

        """

    def get_aliases(self) -> Dict[str, List[str]]:
        """
        Get all aliases on the Solr server

        :return: A dictionary of aliases to collections

        >>> from solrstice import SolrServerContext, BlockingSolrCloudClient
        >>> from typing import Dict, List
        >>> context = SolrServerContext("http://localhost:8983")
        >>> client = BlockingSolrCloudClient(context)
        >>> def get_aliases() -> Dict[str, List[str]]:
        ...     return client.get_aliases()

        """

    def alias_exists(self, name: str) -> bool:
        """
        Check if an alias exists on the Solr server

        :param name: The name of the alias to check
        :return: True if the alias exists, False otherwise

        >>> from solrstice import SolrServerContext, BlockingSolrCloudClient
        >>> context = SolrServerContext("http://localhost:8983")
        >>> client = BlockingSolrCloudClient(context)
        >>> def alias_exists() -> bool:
        ...     return client.alias_exists("alias_name")

        """

    def delete_alias(self, name: str) -> None:
        """
        Delete an alias from the Solr server

        :param name: The name of the alias to delete

        >>> from solrstice import SolrServerContext, BlockingSolrCloudClient
        >>> context = SolrServerContext("http://localhost:8983")
        >>> client = BlockingSolrCloudClient(context)
        >>> def delete_alias() -> None:
        ...     client.delete_alias("alias_name")

        """

    def select(self, builder: "SelectQuery", collection: str) -> "SolrResponse":
        """Execute a select query

        Also see :class:`SelectQuery`

        :param builder: The query builder
        :param collection: The collection to query
        :returns: The response from the query

        >>> from solrstice import SolrServerContext, BlockingSolrCloudClient, SelectQuery
        >>> from solrstice.models import SolrResponse
        >>> context = SolrServerContext("http://localhost:8983")
        >>> client = BlockingSolrCloudClient(context)
        >>> def select() -> SolrResponse:
        ...     query = SelectQuery(q="*:*", rows=10)
        ...     return client.select(query, "collection_name")

        """

    def index(
            self, builder: "UpdateQuery", collection: str, data: List[Dict[str, Any]]
    ) -> "SolrResponse":
        """Execute an index query

        Also see :class:`UpdateQuery`

        :param builder: The query builder
        :param collection: The collection to index
        :param data: The data to index

        >>> from solrstice import SolrServerContext, BlockingSolrCloudClient, UpdateQuery
        >>> from solrstice.models import SolrResponse
        >>> context = SolrServerContext("http://localhost:8983")
        >>> client = BlockingSolrCloudClient(context)
        >>> def index() -> SolrResponse:
        ...     query = UpdateQuery()
        ...     data_to_index = [{"id": "1", "name": "test"}]
        ...     return  client.index(query, "collection_name", data_to_index)

        """

    def delete(self, builder: "DeleteQuery", collection: str) -> "SolrResponse":
        """Execute a delete query

        :param builder: The query builder
        :param collection: The collection to delete from
        :returns: The response from the query

        >>> from solrstice import SolrServerContext, BlockingSolrCloudClient, DeleteQuery
        >>> from solrstice.models import SolrResponse
        >>> context = SolrServerContext("http://localhost:8983")
        >>> client = BlockingSolrCloudClient(context)
        >>> def delete() -> SolrResponse:
        ...     query = DeleteQuery(ids=["1"])
        ...     return client.delete(query, "collection_name")

        """


# endregion

__all__ = [
    "SolrAuth",
    "SolrBasicAuth",
    "QueryOperator",
    "DefType",
    "LuceneQuery",
    "DismaxQuery",
    "EdismaxQuery",
    "FacetSetComponent",
    "PivotFacetComponent",
    "FieldFacetComponent",
    "FieldFacetSort",
    "FieldFacetMethod",
    "FieldFacetEntry",
    "JsonFacetComponent",
    "JsonFacetType",
    "JsonTermsFacet",
    "JsonQueryFacet",
    "JsonStatFacet",
    "SolrHost",
    "SolrSingleServerHost",
    "SolrMultipleServerHost",
    "ZookeeperEnsembleHost",
    "ZookeeperEnsembleHostConnector",
    "LoggingPolicy",
    "OffLoggingPolicy",
    "FastLoggingPolicy",
    "PrettyLoggingPolicy",
    "SolrServerContext",
    "GroupFormatting",
    "GroupingComponent",
    "SelectQuery",
    "CommitType",
    "UpdateQuery",
    "DeleteQuery",
    "AsyncSolrCloudClient",
    "BlockingSolrCloudClient",
]
