from abc import ABC
from typing import Optional

class SolrAuth(ABC):
    """Base class for Solr authentication"""

class SolrBasicAuth(SolrAuth):
    """Basic authentication for Solr

    :param username: Username for Solr
    :param password: Password for Solr
    """

    def __init__(self, username: str, password: Optional[str] = None) -> None:
        pass
