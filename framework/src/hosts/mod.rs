//! Host types
//! # Examples
//! ## Connect to a single solr host
//! Good for if you have an external load balancer
//! ```rust
//! use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
//! use solrstice::hosts::solr_server_host::SolrSingleServerHost;
//! use solrstice::models::context::{SolrServerContextBuilder};
//!
//! let host = SolrSingleServerHost::new("localhost:8983");
//! let client = AsyncSolrCloudClient::new(SolrServerContextBuilder::new(host).build());
//! ```
//! ## Connect to zookeeper instances
//! ```no_run
//! use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
//! use solrstice::hosts::zookeeper_host::ZookeeperEnsembleHostConnector;
//! use solrstice::models::context::{SolrServerContextBuilder};
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let host = ZookeeperEnsembleHostConnector::new(&["localhost:8983", "localhost:8984"], std::time::Duration::from_secs(3)).connect().await?;
//! let client = AsyncSolrCloudClient::new(SolrServerContextBuilder::new(host).build());
//! # Ok(())
//! # }
//! ```

/// Solr auth host trait
pub mod solr_host;
/// Direct solr connectors
pub mod solr_server_host;
/// Zookeeper connector
pub mod zookeeper_host;
