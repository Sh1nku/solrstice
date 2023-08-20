from enum import Enum
from typing import Any, Dict, List, Optional

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

    def __init__(self, pivots: List[str], min_count: Optional[str]):
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

class SolrFacetSetResult:
    """
    Gets the facet counts from a query
    """

    def get_queries(self) -> Dict[str, int]:
        """
        Gets the query facets

        :return: The query facets
        """
        pass
    def get_pivots(self) -> Dict[str, List["SolrPivotFacetResult"]]:
        """
        Gets the pivot facets

        :return: The pivot facets
        """
        pass
    def get_fields(self) -> Dict[str, List["SolrFieldFacetResult"]]:
        """
        Gets the field facets

        :return: The field facets
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
