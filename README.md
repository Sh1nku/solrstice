# Solrstice: A Solr 8+ Client for Rust and Python

![Lines of code](https://api.badgestore.dev/badge/ef573e3335d97409/local?style=flat-square)

Solrstice is a SolrCloud aware client library written in rust.
It also provides a wrapper to python.

Use the [Rust documentation](https://docs.rs/solrstice) or the [Python documentation](https://sh1nku.github.io/solrstice/python) for more information.
## Features
* Config API
* Collection API
* Alias API
* Select Documents
  * Grouping Component Query
  * DefTypes (lucene, dismax, edismax)
* Indexing Documents
* Deleting Documents
## Examples
Upload a config, create a collection, index a document, select it, and delete it.
### Rust
```rust
use serde::{Deserialize, Serialize};
use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
use solrstice::hosts::solr_server_host::SolrSingleServerHost;
use solrstice::models::auth::SolrBasicAuth;
use solrstice::models::context::SolrServerContextBuilder;
use solrstice::models::error::SolrError;
use solrstice::queries::index::{DeleteQueryBuilder, UpdateQueryBuilder};
use solrstice::queries::select::SelectQueryBuilder;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct TestData {
    id: String,
}

#[tokio::test]
pub async fn example() -> Result<(), SolrError> {
  
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
          &UpdateQueryBuilder::new(),
          "example_collection",
          docs.as_slice(),
        )
        .await?;
    
    // Search and retrieve the document
    let docs = client
        .select(
          &SelectQueryBuilder::new().fq(&["id:example_document"]),
          "example_collection",
        )
        .await?
        .get_response()
        .ok_or("No response provided")?
        .get_docs::<TestData>()?;
    
    // Delete the document
    client
        .delete(
          &DeleteQueryBuilder::new().ids(&["example_document"]),
          "example_collection",
        )
        .await?;
    Ok(())
}
```
### Python
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