from enum import Enum
from typing import Any, List, Optional

from solrstice.response import SolrDocsResponse

class GroupFormatting(Enum):
    Simple = "Simple"
    Grouped = "Grouped"

class GroupingComponent:
    """
    Grouping component, used in conjunction with SelectQueryBuilder

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
    fields: Optional[List[str]]
    queries: Optional[List[str]]
    limit: Optional[int]
    offset: Optional[int]
    sort: Optional[List[str]]
    format: Optional[GroupFormatting]
    main: Optional[bool]
    n_groups: Optional[bool]
    truncate: Optional[bool]
    facet: Optional[bool]

class SolrGroupFieldResult:
    """
    Represents a group field result
    """

    group_value: Any
    doc_list: SolrDocsResponse

class SolrGroupResult:
    """
    Represents a group result
    """

    matches: int
    n_groups: Optional[int]

    def get_field_result(self) -> List[SolrGroupFieldResult]:
        """
        Gets the field results form a group query
        :return: List of group field results

        :raises: RuntimeError if conversion failed, or no field result existed
        """
    def get_query_result(self) -> SolrDocsResponse:
        """
        Gets the query result from a group query
        :return: Query result

        :raises: RuntimeError if conversion failed, or no query result existed
        """
    def get_simple_result(self) -> SolrDocsResponse:
        """
        Gets the result from a group query where `GroupFormatting.Simple` was used
        :return: Simple result

        :raises: RuntimeError if conversion failed, or no simple result existed
        """
