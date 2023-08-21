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
    * Facet Counts (Query, Field, Pivot)
    * Json Facet
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

from solrstice.auth import SolrBasicAuth
from solrstice.clients import AsyncSolrCloudClient
from solrstice.hosts import SolrServerContext, SolrSingleServerHost
from solrstice.queries import DeleteQuery, SelectQuery, UpdateQuery

# A SolrServerContext specifies how the library should interact with Solr
context = SolrServerContext(SolrSingleServerHost('localhost:8983'), SolrBasicAuth('solr', 'SolrRocks'))
client = AsyncSolrCloudClient(context)

async def main():
    # Create config and collection
    await client.upload_config('example_config', 'path/to/config')
    await client.create_collection('example_collection', 'example_config', shards=1, replication_factor=1)
    
    # Index a document
    await client.index(UpdateQuery(), 'example_collection', [{'id': 'example_document', 'title': 'Example document'}])
    
    # Search for the document
    response = await client.select(SelectQuery(fq=['title:Example document']), 'example_collection')
    docs = response.get_docs_response().get_docs()
    
    # Delete the document
    await client.delete(DeleteQuery(ids=['example_document']), 'example_collection')
    

asyncio.run(main())
```
### Blocking
```python
from solrstice.auth import SolrBasicAuth
from solrstice.clients import BlockingSolrCloudClient
from solrstice.hosts import SolrServerContext, SolrSingleServerHost
from solrstice.queries import DeleteQuery, SelectQuery, UpdateQuery

# A SolrServerContext specifies how the library should interact with Solr
context = SolrServerContext(SolrSingleServerHost('localhost:8983'), SolrBasicAuth('solr', 'SolrRocks'))
client = BlockingSolrCloudClient(context)

# Create config and collection
client.upload_config('example_config', 'path/to/config')
client.create_collection('example_collection', 'example_config', shards=1, replication_factor=1)

# Index a document
client.index(UpdateQuery(), 'example_collection', [{'id': 'example_document', 'title': 'Example document'}])

# Search for the document
response = client.select(SelectQuery(fq=['title:Example document']), 'example_collection')
docs = response.get_docs_response().get_docs()

# Delete the document
client.delete(DeleteQuery(ids=['example_document']), 'example_collection')
```

## Notes
* Multiprocessing does not work, and will block forever. Normal multithreading works fine.
## Grouping component
### Field grouping
```python
group_builder = GroupingComponent(fields=["age"], limit=10)
select_builder = SelectQuery(fq=["age:[* TO *]"], grouping=group_builder)
groups = await client.select(select_builder, "example_collection").get_groups()
age_group = groups["age"]
docs = age_group.get_field_result()
```
### Query grouping
```python
group_builder = GroupingComponent(queries=["age:[0 TO 59]", "age:[60 TO *]"], limit=10)
select_builder = SelectQuery(fq=["age:[* TO *]"], grouping=group_builder)
groups = await client.select(select_builder, "example_collection").get_groups()
age_group = groups["age:[0 TO 59]"]
group = age_group.get_query_result()
docs = group.get_docs()
```
## Query parsers
### Lucene
```python
query_parser = LuceneQuery(df="population")
select_builder = SelectQuery(q="outdoors", def_type=query_parser)
await client.select(select_builder, "example_collection")
docs = response.get_docs_response().get_docs()
```
### Dismax
```python
query_parser = DismaxQuery(qf="interests^20", bq=["interests:cars^20"])
select_builder = SelectQuery(q="outdoors", def_type=query_parser)
await client.select(select_builder, "example_collection")
docs = response.get_docs_response().get_docs()
```
### Edismax
```python
query_parser = EdismaxQuery(qf="interests^20", bq=["interests:cars^20"])
select_builder = SelectQuery(q="outdoors", def_type=query_parser)
await client.select(select_builder, "example_collection")
docs = response.get_docs_response().get_docs()
```
## FacetSet Component
### Pivot facet
```python
select_builder = SelectQuery(facet_set=FacetSetComponent(pivots=PivotFacetComponent(["interests,age"])))
await client.select(select_builder, "example_collection")
facets = response.get_facet_set()
pivots = facets.get_pivots()
interests_age = pivot.get("interests,age")
```
### Field facet
```python
facet_set = FacetSetComponent(fields=FieldFacetComponent(fields=[FieldFacetEntry("age")]))
select_builder = SelectQuery(facet_set=facet_set)
response = await client.select(select_builder, name)
facets = response.get_facet_set()
fields = facets.get_fields()
age = fields.get("age")
```
### Query facet
```python
select_builder = SelectQuery(facet_set=FacetSetComponent(queries=["age:[0 TO 59]"]))
response = await config.async_client.select(select_builder, name)
facets = response.get_facet_set()
queries = facets.get_queries()
query = queries.get("age:[0 TO 59]")
```
## Json Facet Component
### Basic
```python
select_builder = SelectQuery(
    json_facet=JsonFacetComponent(
        facets={"below_60": JsonQueryFacet("age:[0 TO 59]")}
    )
)
response = await client.select(select_builder, name)
facets = response.get_json_facets()
below_60 = facets.get_nested_facets().get("below_60")
assert below_60.get_count() == 4
```
### Nested
```python
select_builder = SelectQuery(
    json_facet=JsonFacetComponent(
        facets={
            "below_60": JsonQueryFacet(
                "age:[0 TO 59]",
                facets={"total_people": JsonStatFacet("sum(count)")},
            )
        }
    )
)
response = await client.select(select_builder, name)
facets = response.get_json_facets()
total_people = (
    facets.get_nested_facets()
    .get("below_60")
    .get_flat_facets()
    .get("total_people")
)
assert total_people == 750.0
```