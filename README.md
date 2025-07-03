# Solrstice: A Solr 8+ Client for Rust and Python

![Lines of code](https://api.badgestore.dev/badge/ef573e3335d97409/local?style=flat-square)

Solrstice is a SolrCloud aware client library written in rust.
It also provides a wrapper to python.

Use the [Rust documentation](https://docs.rs/solrstice) or
the [Python documentation](https://pypi.org/project/solrstice/) for more information.

## Features

* Config API
* Collection API
* Alias API
* Select Documents
    * Grouping Component Query
    * Stats Component
    * DefTypes (lucene, dismax, edismax)
    * Facet Counts (Query, Field, Pivot)
    * Json Facet (Query, Stat, Terms, Nested)
* Indexing Documents
* Deleting Documents

## Examples

Upload a config, create a collection, index a document, select it, and delete it.

### Rust

```rust
use serde::{Deserialize, Serialize};
use solrstice::{AsyncSolrCloudClient, SolrSingleServerHost, SolrBasicAuth,
                SolrServerContextBuilder, Error, DeleteQuery, UpdateQuery, SelectQuery};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct TestData {
    id: String,
}

#[tokio::test]
pub async fn example() -> Result<(), Error> {

    //Create a solr client. You can also use a list of zookeeper hosts instead of a single server.
    let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983"))
        .with_auth(SolrBasicAuth::new("solr", Some("SolrRocks"))).build();
    let client = AsyncSolrCloudClient::new(context);

    // Upload config
    client
        .upload_config("example_config", Path::new("/path/to/config"))
        .await?;

    // Create collection
    client
        .create_collection("example_collection", "example_config", 1, 1)
        .await?;

    // Index document
    let docs = vec![TestData {
        id: "example_document".to_string(),
    }];
    client
        .index(
            &UpdateQuery::new(),
            "example_collection",
            docs.as_slice(),
        )
        .await?;

    // Search and retrieve the document
    let docs = client
        .select(
            &SelectQuery::new().fq(["id:example_document"]),
            "example_collection",
        )
        .await?
        .get_docs_response()
        .ok_or("No response provided")?
        .get_docs::<TestData>()?;

    // Delete the document
    client
        .delete(
            &DeleteQuery::new().ids(["example_document"]),
            "example_collection",
        )
        .await?;
    Ok(())
}
```

### Python

```python
import asyncio
from solrstice import AsyncSolrCloudClient, SolrSingleServerHost, SolrServerContext, \
    SolrBasicAuth, UpdateQuery, SelectQuery, DeleteQuery

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