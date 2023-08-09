//! Solrstice is a Solr 8+ client for Rust.
//! Take a look at [AsyncSolrCloudClient](crate::clients::async_cloud_client::AsyncSolrCloudClient) and [SelectQueryBuilder](crate::queries::select::SelectQueryBuilder) for more documentation
//! # Examples
//! ```no_run
//! use serde::{Deserialize, Serialize};
//! use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
//! use solrstice::hosts::solr_server_host::SolrSingleServerHost;
//! use solrstice::models::auth::SolrBasicAuth;
//! use solrstice::models::context::SolrServerContext;
//! use solrstice::models::error::SolrError;
//! use solrstice::queries::index::{DeleteQueryBuilder, UpdateQueryBuilder};
//! use solrstice::queries::select::SelectQueryBuilder;
//! use std::path::Path;
//!
//! #[derive(Serialize, Deserialize, Debug)]
//! struct TestData {
//!     id: String,
//! }
//!
//! #[tokio::test]
//! pub async fn example() -> Result<(), SolrError> {
//!
//!     //Create a solr client. You can also use a list of zookeeper hosts instead of a single server.
//!     let context = SolrServerContext::new(SolrSingleServerHost::new("http://localhost:8983"))
//!         .with_auth(SolrBasicAuth::new("solr", Some("SolrRocks")));
//!     let client = AsyncSolrCloudClient::new(context);
//!
//!     // Upload config
//!     client
//!         .upload_config("example_config", Path::new("/path/to/config"))
//!         .await?;
//!
//!     // Create collection
//!     client
//!         .create_collection("example_collection", "example_config", 1, 1)
//!         .await?;
//!
//!     // Index document
//!     let docs = vec![TestData {
//!         id: "example_document".to_string(),
//!     }];
//!     client
//!         .index(
//!             &UpdateQueryBuilder::new(),
//!             "example_collection",
//!             docs.as_slice(),
//!         )
//!         .await?;
//!
//!     // Search and retrieve the document
//!     let docs = client
//!         .select(
//!             &SelectQueryBuilder::new().fq(&["id:example_document"]),
//!             "example_collection",
//!         )
//!         .await?
//!         .get_response()
//!         .ok_or("No response provided")?
//!         .get_docs::<TestData>()?;
//!
//!     // Delete the document
//!     client
//!         .delete(
//!             &DeleteQueryBuilder::new().ids(&["example_document"]),
//!             "example_collection",
//!         )
//!         .await?;
//!     Ok(())
//! }
//! ```

/// Solr Clients
pub mod clients;
/// Host types
pub mod hosts;
/// Model structs
pub mod models;
/// Query types
pub mod queries;
#[cfg(feature = "blocking")]
/// Tokio Runtime for blocking usage
pub mod runtime;
