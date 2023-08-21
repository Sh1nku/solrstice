import abc
from typing import Any, Dict, List, Optional, Union

class JsonFacetComponent:
    """
    Json faceting component

    :param facets: A dictionary of facets to apply to the query
    """

    def __init__(self, facets: Optional[Dict[str, JsonFacetType]] = None):
        pass

class JsonFacetType(abc.ABC):
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
