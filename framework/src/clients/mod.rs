//! Clients for interacting with Solr.
//! # Examples
//! ## Async client for SolrCloud
//! ```rust
//! use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
//! use solrstice::hosts::solr_server_host::SolrSingleServerHost;
//! use solrstice::models::context::SolrServerContextBuilder;
//!
//! let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
//! let client = AsyncSolrCloudClient::new(context);
//! ```
//! ## Blocking client for SolrCloud
//! ```rust
//! use solrstice::clients::blocking_cloud_client::BlockingSolrCloudClient;
//! use solrstice::hosts::solr_server_host::SolrSingleServerHost;
//! use solrstice::models::context::SolrServerContextBuilder;
//!
//! let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
//! let client = BlockingSolrCloudClient::new(context);
//! ```

/// Client for interacting asynchronously with SolrCloud.
pub mod async_cloud_client;
/// Client for interacting blocking with SolrCloud.
#[cfg(feature = "blocking")]
pub mod blocking_cloud_client;
