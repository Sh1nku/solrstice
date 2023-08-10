# Solrstice: A Solr 8+ Client for Rust and Python
Solrstice is a solr client library written in rust. With this wrapper you can use it in python.

Both asyncio and blocking clients are provided. All apis have type hints.
Documentation can be found at [sh1nku.github.io/solrstice/python](https://sh1nku.github.io/solrstice/python)
## Features
* Config API
* Collection API
* Alias API
* Select Documents
    * Grouping Component Query
    * DefTypes (lucene, dismax, edismax)
* Indexing Documents
* Deleting Documents
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

# Delete the document
client.delete(DeleteQueryBuilder(ids=['example_document']), 'example_collection')
```

## Notes
* Multiprocessing does not work, and will block forever. Normal multithreading works fine.
## Grouping component
### Field grouping
```python
group_builder = GroupingComponent(fields=["age"], limit=10)
select_builder = SelectQueryBuilder(fq=["age:[* TO *]"], grouping=group_builder)
groups = await client.select(select_builder, "example_collection").get_groups()
age_group = groups["age"]
docs = age_group.get_field_result()
```
### Query grouping
```python
group_builder = GroupingComponent(queries=["age:[0 TO 59]", "age:[60 TO *]"], limit=10)
select_builder = SelectQueryBuilder(fq=["age:[* TO *]"], grouping=group_builder)
groups = await client.select(select_builder, "example_collection").get_groups()
age_group = groups["age:[0 TO 59]"]
group = age_group.get_query_result()
docs = group.docs
```
## Query parsers
### Lucene
```python
query_parser = LuceneQueryBuilder(df="population")
select_builder = SelectQueryBuilder(q="outdoors", def_type=query_parser)
await client.select(select_builder, "example_collection")
docs = response.get_response().docs
```
### Dismax
```python
query_parser = DismaxQueryBuilder(qf="interests^20", bq=["interests:cars^20"])
select_builder = SelectQueryBuilder(q="outdoors", def_type=query_parser)
await client.select(select_builder, "example_collection")
docs = response.get_response().docs
```
### Edismax
```python
query_parser = EdismaxQueryBuilder(qf="interests^20", bq=["interests:cars^20"])
select_builder = SelectQueryBuilder(q="outdoors", def_type=query_parser)
await client.select(select_builder, "example_collection")
docs = response.get_response().docs
```