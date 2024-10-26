use crate::error::Error;
use crate::hosts::solr_host::SolrHost;
use async_trait::async_trait;
use log::debug;
use std::borrow::Cow;
use std::sync::Arc;
use std::time::Duration;
use zookeeper_async::{WatchedEvent, Watcher, ZkResult, ZooKeeper};

/// Connect to zookeeper instances to get a list of solr nodes to connect to. Select randomly from the list of live nodes.
/// The timeout is used to determine how long to wait for a response from a solr host before trying the next one
/// ```no_run
///
/// use solrstice::{AsyncSolrCloudClient, SolrServerContextBuilder, ZookeeperEnsembleHostConnector};
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let host = ZookeeperEnsembleHostConnector::new(["localhost:8983", "localhost:8984"], std::time::Duration::from_secs(3)).connect().await?;
/// let client = AsyncSolrCloudClient::new(SolrServerContextBuilder::new(host).build());
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct ZookeeperEnsembleHostConnector {
    pub hosts: Vec<String>,
    pub timeout: Duration,
}

impl ZookeeperEnsembleHostConnector {
    /// Connect to zookeeper instances to get a list of solr nodes to connect to. Select randomly from the list of live nodes.
    /// The timeout is used to determine how long to wait for a response from a solr host before trying the next one
    /// ```no_run
    /// use solrstice::{AsyncSolrCloudClient, SolrServerContextBuilder, ZookeeperEnsembleHostConnector};
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let host = ZookeeperEnsembleHostConnector::new(["localhost:8983", "localhost:8984"], std::time::Duration::from_secs(3)).connect().await?;
    /// let client = AsyncSolrCloudClient::new(SolrServerContextBuilder::new(host).build());
    /// # Ok(())
    /// # }
    /// ```
    pub fn new<S: Into<String>, V: IntoIterator<Item = S>>(
        hosts: V,
        timeout: Duration,
    ) -> ZookeeperEnsembleHostConnector {
        ZookeeperEnsembleHostConnector {
            hosts: hosts.into_iter().map(|s| s.into()).collect(),
            timeout,
        }
    }

    /// Connect to zookeeper instances to get a list of solr nodes to connect to. Select randomly from the list of live nodes.
    /// The timeout is used to determine how long to wait for a response from a solr host before trying the next one
    /// ```no_run
    ///
    /// use solrstice::{AsyncSolrCloudClient, SolrServerContextBuilder, ZookeeperEnsembleHostConnector};
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let host = ZookeeperEnsembleHostConnector::new(["localhost:8983", "localhost:8984"], std::time::Duration::from_secs(3)).connect().await?;
    /// let client = AsyncSolrCloudClient::new(SolrServerContextBuilder::new(host).build());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn connect(self) -> Result<ZookeeperEnsembleHost, Error> {
        ZookeeperEnsembleHost::new(self.hosts.as_slice(), self.timeout).await
    }
}

#[cfg(feature = "blocking")]
use crate::runtime::RUNTIME;
#[cfg(feature = "blocking")]
impl ZookeeperEnsembleHostConnector {
    /// Connect to zookeeper instances to get a list of solr nodes to connect to. Select randomly from the list of live nodes.
    /// The timeout is used to determine how long to wait for a response from a solr host before trying the next one
    /// ```no_run
    /// use solrstice::{BlockingSolrCloudClient, SolrServerContextBuilder, ZookeeperEnsembleHostConnector};
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let host = ZookeeperEnsembleHostConnector::new(["localhost:8983", "localhost:8984"], std::time::Duration::from_secs(3)).connect_blocking()?;
    /// let client = BlockingSolrCloudClient::new(SolrServerContextBuilder::new(host).build());
    /// # Ok(())
    /// # }
    /// ```
    pub fn connect_blocking(self) -> Result<ZookeeperEnsembleHost, Error> {
        RUNTIME.block_on(self.connect())
    }
}

/// Connect to zookeeper instances to get a list of solr nodes to connect to. Select randomly from the list of live nodes.
/// The timeout is used to determine how long to wait for a response from a solr host before trying the next one
/// ```rust
///
/// use solrstice::{AsyncSolrCloudClient, SolrServerContextBuilder, ZookeeperEnsembleHostConnector};
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let host = ZookeeperEnsembleHostConnector::new(["localhost:8983", "localhost:8984"], std::time::Duration::from_secs(3)).connect().await?;
/// let client = AsyncSolrCloudClient::new(SolrServerContextBuilder::new(host).build());
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct ZookeeperEnsembleHost {
    client: Arc<ZooKeeper>,
}

impl ZookeeperEnsembleHost {
    pub(crate) async fn new<S: Into<String>, V: IntoIterator<Item = S>>(
        hosts: V,
        timeout: Duration,
    ) -> Result<ZookeeperEnsembleHost, Error> {
        let hosts = hosts.into_iter().map(|s| s.into()).collect::<Vec<String>>();
        let hosts = hosts.join(",");
        Ok(ZookeeperEnsembleHost {
            client: Arc::new(ZooKeeper::connect(hosts.as_ref(), timeout, LoggingWatcher).await?),
        })
    }
}

#[async_trait]
impl SolrHost for ZookeeperEnsembleHost {
    async fn get_solr_node(&self) -> Result<Cow<str>, Error> {
        let hosts = get_hosts_from_zookeeper(&self.client).await?;
        match hosts.get(fastrand::usize(0..hosts.len())) {
            None => Err(Error::SolrSetupError(
                "No ready Solr nodes from Zookeeper".to_string(),
            )),
            //TODO Investigate this further. Is it always http://, and do people use auth?
            Some(r) => Ok(Cow::Owned(format!(
                "http://{}",
                r.strip_suffix("_solr").unwrap_or(r)
            ))),
        }
    }
}

pub struct LoggingWatcher;
impl Watcher for LoggingWatcher {
    fn handle(&self, e: WatchedEvent) {
        debug!("{:?}", e)
    }
}

pub(crate) async fn get_hosts_from_zookeeper(client: &ZooKeeper) -> ZkResult<Vec<String>> {
    client.get_children("/live_nodes", true).await
}
