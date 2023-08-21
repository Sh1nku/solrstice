# Solrstice Docs
* [Rust docs](https://docs.rs/solrstice/) on docs.rs has better documentation
* [Python docs](https://sh1nku.github.io/solrstice/python) 

## Introduction
Solrstice is meant to be a library for interacting with an Apache Solr cluster
Currently version `8` and `9` is supported.

You can install the library by putting this in your `Cargo.toml`
```toml
solrstice = { version = "0.1", features = ["blocking"] }
```
If the `blocking` feature is not provided, only async will work.

## Getting started
### Creating a client
```rust
let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983"))
    .with_auth(SolrBasicAuth::new("solr", Some("SolrRocks"))).build();
let client = AsyncSolrCloudClient::new(context);
```
### Creating a collection
```rust
client
    .upload_config("example_config", Path::new("/path/to/config"))
    .await?;
client
    .create_collection("example_collection", "example_config", 1, 1)
    .await?;
```
### Indexing data
```rust
#[derive(Serialize, Deserialize, Debug)]
struct TestData {
    id: String,
}
let docs = vec![TestData {
    id: "example_document".to_string(),
}];
client
    .index( &UpdateQuery::new(), "example_collection", docs.as_slice())
    .await?;
```
### Selecting data
```rust
let docs = client
    .select(
        &SelectQuery::new().fq(["id:example_document"]),
        "example_collection",
    )
    .await?
    .get_docs_response()
    .ok_or("No response provided")?
    .get_docs::<TestData>()?;
```
### Deleting data
```rust
client
    .delete(
        &DeleteQuery::new().ids(["example_document"]),
        "example_collection",
    )
    .await?;
```