use crate::error::Error;
use crate::hosts::solr_host::SolrHost;
use async_trait::async_trait;
use std::borrow::Cow;
use std::time::Duration;

/// Connect to a single solr host
/// Good for if you have an external load balancer
/// ```rust
/// use solrstice::{AsyncSolrCloudClient, SolrServerContextBuilder, SolrSingleServerHost};
///
/// let host = SolrSingleServerHost::new("localhost:8983");
/// let client = AsyncSolrCloudClient::new(SolrServerContextBuilder::new(host).build());
/// ```
#[derive(Clone)]
pub struct SolrSingleServerHost {
    pub host: String,
}

#[async_trait]
impl SolrHost for SolrSingleServerHost {
    async fn get_solr_node(&self) -> Result<Cow<str>, Error> {
        Ok(Cow::Borrowed(&self.host))
    }
}

impl SolrSingleServerHost {
    /// Connect to a single solr host
    /// Good for if you have an external load balancer
    /// ```rust
    /// use solrstice::{AsyncSolrCloudClient, SolrServerContextBuilder, SolrSingleServerHost};
    ///
    /// let host = SolrSingleServerHost::new("localhost:8983");
    /// let client = AsyncSolrCloudClient::new(SolrServerContextBuilder::new(host).build());
    /// ```
    pub fn new<S: Into<String>>(host: S) -> SolrSingleServerHost {
        SolrSingleServerHost { host: host.into() }
    }
}

/// Connect to multiple solr hosts. Acts as a load balancer with random selection
///
/// It would be better to use [ZookeeperEnsembleHostConnector](crate::hosts::zookeeper_host::ZookeeperEnsembleHostConnector) instead.
/// The timeout is used to determine how long to wait for a response from a solr host before trying the next one
/// ```rust
/// use solrstice::{AsyncSolrCloudClient, SolrMultipleServerHost, SolrServerContextBuilder};
///
/// let host = SolrMultipleServerHost::new(["localhost:8983", "localhost:8984"], std::time::Duration::from_secs(3));
/// let client = AsyncSolrCloudClient::new(SolrServerContextBuilder::new(host).build());
/// ```
#[derive(Clone)]
pub struct SolrMultipleServerHost {
    pub hosts: Vec<String>,
    pub timeout: Duration,
}

#[async_trait]
impl SolrHost for SolrMultipleServerHost {
    async fn get_solr_node(&self) -> Result<Cow<str>, Error> {
        let mut server_indices: Vec<usize> = (0..self.hosts.len()).collect();
        if server_indices.is_empty() {
            return Err(Error::SolrSetupError("No Solr Host Specified".to_string()));
        }
        fastrand::shuffle(&mut server_indices);
        for i in server_indices {
            match self.hosts.get(i) {
                None => continue,
                Some(r) => {
                    //TODO There might be a better way to do this
                    let client = reqwest::Client::new();
                    let res = client
                        .get(format!("{}/solr/", r))
                        .timeout(self.timeout)
                        .send()
                        .await;
                    if res.is_err() {
                        continue;
                    }
                    return Ok(Cow::Borrowed(r));
                }
            }
        }
        Err(Error::SolrSetupError("No Solr Host answered".to_string()))
    }
}

impl SolrMultipleServerHost {
    /// Connect to multiple solr hosts. Acts as a load balancer with random selection
    ///
    /// It would be better to use [ZookeeperEnsembleHostConnector](crate::hosts::zookeeper_host::ZookeeperEnsembleHostConnector) instead.
    /// The timeout is used to determine how long to wait for a response from a solr host before trying the next one
    /// ```rust
    /// use solrstice::{AsyncSolrCloudClient, SolrMultipleServerHost, SolrServerContextBuilder};
    ///
    /// let host = SolrMultipleServerHost::new(["localhost:8983", "localhost:8984"], std::time::Duration::from_secs(3));
    /// let client = AsyncSolrCloudClient::new(SolrServerContextBuilder::new(host).build());
    /// ```
    pub fn new<S: Into<String>, V: IntoIterator<Item = S>>(
        hosts: V,
        timeout: Duration,
    ) -> SolrMultipleServerHost {
        SolrMultipleServerHost {
            hosts: hosts.into_iter().map(|s| s.into()).collect(),
            timeout,
        }
    }
}
