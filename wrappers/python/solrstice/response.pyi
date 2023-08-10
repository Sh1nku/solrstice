from typing import TYPE_CHECKING, Any, Dict, List, Optional

if TYPE_CHECKING:
    from solrstice.group import SolrGroupResult

class SolrDocsResponse:
    num_found: int
    start: int
    num_found_exact: bool
    docs: List[Dict[str, Any]]

class SolrResponse:
    """The response from a solr query"""

    next_cursor_mark: Optional[str]

    def get_response(self) -> SolrDocsResponse:
        """Get the response from a solr query

        :raises RuntimeError if no response in query
        """
    def get_groups(self) -> Dict[str, "SolrGroupResult"]:
        """Get the groups from a solr query

        :raises RuntimeError if no groups in query
        """
