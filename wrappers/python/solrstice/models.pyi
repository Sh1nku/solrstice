from typing import Any, Dict, List, Optional

class SolrFacetSetResult:
    """
    Gets the facet counts from a query
    """

    def get_queries(self) -> Dict[str, int]:
        """
        Gets the query facets

        :return: The query facets

        >>> from solrstice import AsyncSolrCloudClient, SolrServerContext, SelectQuery, FacetSetComponent, FacetSetComponent
        >>> client = AsyncSolrCloudClient(SolrServerContext('localhost:8983'))
        >>> async def query_select() -> None:
        ...     select_builder = SelectQuery(facet_set=FacetSetComponent(queries=["age:[0 TO 59]"]))
        ...     response = await client.select(select_builder, "example_collection")
        ...     facets = response.get_facet_set()
        ...     queries = facets.get_queries()
        ...     query = queries.get("age:[0 TO 59]")

        """
        pass

    def get_pivots(self) -> Dict[str, List["SolrPivotFacetResult"]]:
        """
        Gets the pivot facets

        :return: The pivot facets

        >>> from solrstice import FacetSetComponent, PivotFacetComponent, SelectQuery, SolrServerContext, AsyncSolrCloudClient
        >>> client = AsyncSolrCloudClient(SolrServerContext('localhost:8983'))
        >>> async def pivot_select() -> None:
        ...     select_builder = SelectQuery(facet_set=FacetSetComponent(pivots=PivotFacetComponent(["interests,age"])))
        ...     response = await client.select(select_builder, "example_collection")
        ...     facets = response.get_facet_set()
        ...     pivots = facets.get_pivots()
        ...     interests_age = pivots.get("interests,age")

        """
        pass

    def get_fields(self) -> Dict[str, List["SolrFieldFacetResult"]]:
        """
        Gets the field facets

        :return: The field facets

        >>> from solrstice import FacetSetComponent, FieldFacetComponent, FieldFacetEntry, SelectQuery, SolrServerContext, AsyncSolrCloudClient
        >>> client = AsyncSolrCloudClient(SolrServerContext('localhost:8983'))
        >>> async def field_facet_select() -> None:
        ...     facet_set = FacetSetComponent(fields=FieldFacetComponent(fields=[FieldFacetEntry("age")]))
        ...     select_builder = SelectQuery(facet_set=facet_set)
        ...     response = await client.select(select_builder, "example_collection")
        ...     facets = response.get_facet_set()
        ...     fields = facets.get_fields()
        ...     age = fields.get("age")

        """

class SolrPivotFacetResult:
    """
    Gets the pivot facet counts from a query
    """

    def get_value(self) -> Any:
        """
        Gets the value of the pivot

        :return: The value of the pivot
        """
        pass

    def get_pivots(self) -> List["SolrPivotFacetResult"]:
        """
        Gets additional pivots
        :return: The additional pivots
        """

    def get_queries(self) -> Dict[str, int]:
        """
        Gets the query facets
        :return: The query facets
        """

    def get_count(self) -> int:
        """
        Gets the count of the pivot
        :return: The count of the pivot
        """

class SolrFieldFacetResult:
    def get_key(self) -> Any:
        """
        Gets the key of the facet

        :return: The key of the facet
        """
        pass

    def get_count(self) -> int:
        """
        Gets the count of the facet
        :return: The count of the facet
        """

class SolrGroupFieldResult:
    """
    Represents a group field result
    """

    def get_group_value(self) -> Any:
        """
        Gets the group value
        :return: Group value
        """

    def get_doc_list(self) -> "SolrDocsResponse":
        """
        Gets the document response from solr
        :return: Document response
        """

class SolrGroupResult:
    """
    Represents a group result
    """

    def get_field_result(self) -> Optional[List[SolrGroupFieldResult]]:
        """
        Gets the field results form a group query
        :return: List of group field results
        """

    def get_query_result(self) -> Optional["SolrDocsResponse"]:
        """
        Gets the query result from a group query
        :return: Query result
        """

    def get_simple_result(self) -> Optional["SolrDocsResponse"]:
        """
        Gets the result from a group query where `GroupFormatting.Simple` was used
        :return: Simple result
        """

    def get_matches(self) -> int:
        """
        Gets the number of matches for a group query
        :return: Number of matches
        """

    def get_n_groups(self) -> int:
        """
        Gets the number of groups for a group query
        :return: Number of groups
        """

class SolrJsonFacetResponse:
    """
    A response from a json facet query
    """

    def get_buckets(self) -> List["SolrJsonFacetResponse"]:
        """
        Get the buckets for this facet
        :return: A list of buckets
        """

    def get_flat_facets(self) -> Dict[str, Any]:
        """
        Get stat counts for this facet
        :return: A dictionary of stat counts
        """

    def get_nested_facets(self) -> Dict[str, "SolrJsonFacetResponse"]:
        """
        Get the nested facets for this facet
        :return: A dictionary of nested facets
        """

    def get_count(self) -> Optional[int]:
        """
        Get the count for this facet
        :return: The count for this facet
        """

    def get_val(self) -> Optional[Any]:
        """
        If a bucket facet, this value will be set
        :return: The value for this facet
        """

class SolrDocsResponse:
    def get_num_found(self) -> int:
        """Get the number of documents found in the query"""

    def get_start(self) -> int:
        """Get the start index of the query"""

    def get_num_found_exact(self) -> bool:
        """Get whether the number of documents found is exact. This field only exists on Solr 8.6+. On older versions, this will always be true."""

    def get_docs(self) -> List[Dict[str, Any]]:
        """Get the documents from the query"""

class SolrResponse:
    """The response from a solr query"""

    def get_docs_response(self) -> Optional["SolrDocsResponse"]:
        """Get the response from a solr query"""

    def get_groups(self) -> Dict[str, "SolrGroupResult"]:
        """Get the groups from a solr query"""

    def get_next_cursor_mark(self) -> Optional[str]:
        """Get the next cursor mark from a solr query"""

    def get_facet_set(self) -> "SolrFacetSetResult":
        """Get facet counts"""

    def get_json_facets(self) -> Optional["SolrJsonFacetResponse"]:
        """Get json facets"""

__all__ = [
    "SolrFacetSetResult",
    "SolrPivotFacetResult",
    "SolrFieldFacetResult",
    "SolrGroupFieldResult",
    "SolrGroupResult",
    "SolrJsonFacetResponse",
    "SolrDocsResponse",
    "SolrResponse",
]
