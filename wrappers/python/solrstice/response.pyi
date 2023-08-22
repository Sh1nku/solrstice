from typing import TYPE_CHECKING, Any, Dict, List, Optional

if TYPE_CHECKING:
    from solrstice.facet_set import SolrFacetSetResult
    from solrstice.group import SolrGroupResult
    from solrstice.json_facet import SolrJsonFacetResponse

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

    def get_docs_response(self) -> Optional[SolrDocsResponse]:
        """Get the response from a solr query"""
    def get_groups(self) -> Dict[str, "SolrGroupResult"]:
        """Get the groups from a solr query"""
    def get_next_cursor_mark(self) -> Optional[str]:
        """Get the next cursor mark from a solr query"""
    def get_facet_set(self) -> "SolrFacetSetResult":
        """Get facet counts"""
    def get_json_facets(self) -> Optional["SolrJsonFacetResponse"]:
        """Get json facets"""
