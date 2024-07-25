//! Solrstice is a Solr 8+ client for Rust.
//! Take a look at [AsyncSolrCloudClient](crate::clients::async_cloud_client::AsyncSolrCloudClient) and [SelectQuery](crate::queries::select::SelectQuery) for more documentation
//! # Examples
//! ```no_run
//! use serde::{Deserialize, Serialize};
//! use solrstice::AsyncSolrCloudClient;
//! use solrstice::SolrSingleServerHost;
//! use solrstice::SolrBasicAuth;
//! use solrstice::{SolrServerContextBuilder};
//! use solrstice::Error;
//! use solrstice::{DeleteQuery, UpdateQuery};
//! use solrstice::SelectQuery;
//! use std::path::Path;
//!
//! #[derive(Serialize, Deserialize, Debug)]
//! struct TestData {
//!     id: String,
//! }
//!
//! #[tokio::test]
//! pub async fn example() -> Result<(), Error> {
//!
//!     //Create a solr client. You can also use a list of zookeeper hosts instead of a single server.
//!     let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983"))
//!         .with_auth(SolrBasicAuth::new("solr", Some("SolrRocks"))).build();
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
//!             &UpdateQuery::new(),
//!             "example_collection",
//!             docs.as_slice(),
//!         )
//!         .await?;
//!
//!     // Search and retrieve the document
//!     let docs = client
//!         .select(
//!             &SelectQuery::new().fq(["id:example_document"]),
//!             "example_collection",
//!         )
//!         .await?
//!         .get_docs_response()
//!         .ok_or("No response provided")?
//!         .get_docs::<TestData>()?;
//!
//!     // Delete the document
//!     client
//!         .delete(
//!             &DeleteQuery::new().ids(["example_document"]),
//!             "example_collection",
//!         )
//!         .await?;
//!     Ok(())
//! }
//! ```

/// Solr Clients
mod clients;
pub use clients::async_cloud_client::*;
#[cfg(feature = "blocking")]
pub use clients::blocking_cloud_client::*;

/// Host types
mod hosts;
pub use crate::hosts::solr_host::*;
pub use crate::hosts::solr_server_host::*;
pub use crate::hosts::zookeeper_host::*;
/// Model structs
pub mod models;
pub use models::auth::*;
pub use models::commit_type::*;
pub use models::context::*;
/// Query types
pub mod queries;
pub use queries::components::facet_set::*;
pub use queries::components::grouping::*;
pub use queries::components::json_facet::*;
pub use queries::def_type::*;
pub use queries::index::*;
pub use queries::request_builder::*;
pub use queries::select::*;
#[cfg(feature = "blocking")]
/// Tokio Runtime for blocking usage
mod runtime;

#[cfg(doctest)]
pub mod docs;
/// Error types for the library.
pub(crate) mod error;
pub use error::Error;
