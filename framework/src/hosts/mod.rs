//! Host types
//! # Examples
//! ## Connect to a single solr host
//! Good for if you have an external load balancer
//! ```rust
//! use solrstice::{AsyncSolrCloudClient, SolrServerContextBuilder, SolrSingleServerHost};
//!
//! let host = SolrSingleServerHost::new("localhost:8983");
//! let client = AsyncSolrCloudClient::new(SolrServerContextBuilder::new(host).build());
//! ```
//! ## Connect to zookeeper instances
//! ```no_run
//! use solrstice::{AsyncSolrCloudClient, SolrServerContextBuilder, ZookeeperEnsembleHostConnector};
//!
//! async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let host = ZookeeperEnsembleHostConnector::new(["localhost:8983", "localhost:8984"], std::time::Duration::from_secs(3)).connect().await?;
//! let client = AsyncSolrCloudClient::new(SolrServerContextBuilder::new(host).build());
//! # Ok(())
//! # }
//! ```

/// Solr auth host trait
pub(crate) mod solr_host;
/// Direct solr connectors
pub(crate) mod solr_server_host;
/// Zookeeper connector
pub(crate) mod zookeeper_host;
