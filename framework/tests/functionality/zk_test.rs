use crate::structures::BaseTestsBuildup;
use solrstice::hosts::solr_host::SolrHost;
use solrstice::hosts::zookeeper_host::ZookeeperEnsembleHostConnector;
use std::time::Duration;
use std::vec;

#[tokio::test]
async fn create_zookeeper_client() {
    BaseTestsBuildup::new().await;
    let zk_hosts = vec![std::env::var("ZK_HOST").unwrap()];
    ZookeeperEnsembleHostConnector::new(zk_hosts, Duration::from_secs(15))
        .connect()
        .await
        .unwrap();
}

#[tokio::test]
async fn get_solr_node_from_zookeeper() {
    BaseTestsBuildup::new().await;
    let zk_hosts = vec![std::env::var("ZK_HOST").unwrap()];
    let host = ZookeeperEnsembleHostConnector::new(zk_hosts, Duration::from_secs(15))
        .connect()
        .await
        .unwrap();
    let _ = host.get_solr_node().await.unwrap();
}
