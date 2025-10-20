from typing import Optional

__all__ = [
    "ReqwestClient",
]

class ReqwestClient:
    """
    A wrapper around the Rust `reqwest::Client` for making HTTP requests.

    :param timeout: Overall request timeout in milliseconds.
    :param read_timeout: Read timeout in milliseconds.
    :param connect_timeout: Connection timeout in milliseconds.
    :param danger_accept_invalid_hostnames: If True, accepts invalid hostnames.
    :param danger_accept_invalid_certs: If True, accepts invalid SSL certificates.

    >>> from solrstice import SolrServerContext, AsyncSolrCloudClient
    >>> from solrstice.external import ReqwestClient
    >>> context = SolrServerContext("https://localhost:8983", client=ReqwestClient(timeout=5000, danger_accept_invalid_certs=True))
    >>> client = AsyncSolrCloudClient(context)
    >>> async def config_exists() -> bool:
    ...     return await client.config_exists("config_name")

    """

    def __init__(
        self,
        *,
        timeout: Optional[int] = None,
        read_timeout: Optional[int] = None,
        connect_timeout: Optional[int] = None,
        danger_accept_invalid_hostnames: Optional[bool] = None,
        danger_accept_invalid_certs: Optional[bool] = None
    ) -> None:
        pass
