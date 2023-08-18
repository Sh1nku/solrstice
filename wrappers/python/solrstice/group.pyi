from enum import Enum
from typing import Any, List, Optional

from solrstice.response import SolrDocsResponse

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

class SolrGroupFieldResult:
    """
    Represents a group field result
    """

    def get_group_value(self) -> Any:
        """
        Gets the group value
        :return: Group value
        """
    def get_doc_list(self) -> SolrDocsResponse:
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
    def get_query_result(self) -> Optional[SolrDocsResponse]:
        """
        Gets the query result from a group query
        :return: Query result
        """
    def get_simple_result(self) -> Optional[SolrDocsResponse]:
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
