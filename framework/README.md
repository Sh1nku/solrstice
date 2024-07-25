# Solrstice: A Solr 8+ Client for Rust and Python

![Lines of code](https://api.badgestore.dev/badge/ef573e3335d97409/local?style=flat-square)

Solrstice is a SolrCloud aware client library written in rust.
It also provides a wrapper to python.
## Features
* Config API
* Collection API
* Alias API
* Select Documents
  * Grouping Component Query
  * DefTypes (lucene, dismax, edismax)
  * Facet Counts (Query, Field, Pivot)
  * Json Facet (Query, Stat, Terms, Nested)
* Indexing Documents
* Deleting Documents
## Examples
Upload a config, create a collection, index a document, select it, and delete it.
```rust
use serde::{Deserialize, Serialize};
use solrstice::AsyncSolrCloudClient;
use solrstice::SolrSingleServerHost;
use solrstice::SolrBasicAuth;
use solrstice::SolrServerContextBuilder;
use solrstice::models::error::SolrError;
use solrstice::{DeleteQuery, UpdateQuery};
use solrstice::SelectQuery;
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