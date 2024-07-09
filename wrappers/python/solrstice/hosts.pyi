import abc
from abc import ABC
from typing import List, Optional, Union

from solrstice.auth import SolrAuth

class SolrHost(ABC):
    """Base class for Solr hosts"""

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
    """Policy describing how to log solr queries. Valid values are :class:`OffLoggingPolicy`, :class:`FastLoggingPolicy`, and :class:`PrettyLoggingPolicy`"""

    pass

class OffLoggingPolicy(LoggingPolicy):
    """Do not log requests"""

    def __init__(self):
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
