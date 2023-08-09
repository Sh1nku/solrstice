# Solrstice Python Wrapper
Solrstice is a solr 8+ cloud aware client library written in rust. With this wrapper, you can use it in python.

Both asyncio and blocking clients are provided. All apis have type hints.
## Installation
```bash
pip install solrstice
```
## Basic Usage
### Async
```python
import asyncio
from solrstice.clients import AsyncSolrCloudClient
from solrstice.hosts import SolrSingleServerHost, SolrServerContext
from solrstice.auth import SolrBasicAuth
from solrstice.queries import UpdateQueryBuilder, SelectQueryBuilder, DeleteQueryBuilder

# A SolrServerContext specifies how the library should interact with Solr
context = SolrServerContext(SolrSingleServerHost('localhost:8983'), SolrBasicAuth('solr', 'SolrRocks'))
client = AsyncSolrCloudClient(context)

async def main():
    # Create config and collection
    await client.upload_config('example_config', 'path/to/config')
    await client.create_collection('example_collection', 'example_config', shards=1, replication_factor=1)
    
    # Index a document
    await client.index(UpdateQueryBuilder(), 'example_collection', [{'id': 'example_document', 'title': 'Example document'}])
    
    # Search for the document
    response = await client.select(SelectQueryBuilder(fq=['title:Example document']), 'example_collection')
    docs = response.get_response().docs
    
    # Delete the document
    await client.delete(DeleteQueryBuilder(ids=['example_document']), 'example_collection')
    

asyncio.run(main())
```
### Blocking
```python
from solrstice.clients import BlockingSolrCloudClient
from solrstice.hosts import SolrSingleServerHost, SolrServerContext
from solrstice.auth import SolrBasicAuth
from solrstice.queries import UpdateQueryBuilder, SelectQueryBuilder, DeleteQueryBuilder

# A SolrServerContext specifies how the library should interact with Solr
context = SolrServerContext(SolrSingleServerHost('localhost:8983'), SolrBasicAuth('solr', 'SolrRocks'))
client = BlockingSolrCloudClient(context)

# Create config and collection
client.upload_config('example_config', 'path/to/config')
client.create_collection('example_collection', 'example_config', shards=1, replication_factor=1)

# Index a document
client.index(UpdateQueryBuilder(), 'example_collection', [{'id': 'example_document', 'title': 'Example document'}])

# Search for the document
response = client.select(SelectQueryBuilder(fq=['title:Example document']), 'example_collection')
docs = response.get_response().docs
```

## Notes
* Multiprocessing does not work, and will block forever. Normal multithreading works fine.