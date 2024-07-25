use crate::structures::BaseTestsBuildup;
use serial_test::parallel;
use solrstice::SolrHost;
use solrstice::ZookeeperEnsembleHostConnector;
use std::time::Duration;
use std::vec;

#[tokio::test]
#[parallel]
async fn create_zookeeper_client() {
    BaseTestsBuildup::new().await;
    let zk_hosts = vec![std::env::var("ZK_HOST").unwrap()];
    ZookeeperEnsembleHostConnector::new(zk_hosts, Duration::from_secs(15))
        .connect()
        .await
        .unwrap();
}

#[tokio::test]
#[parallel]
async fn get_solr_node_from_zookeeper() {
    BaseTestsBuildup::new().await;
    let zk_hosts = vec![std::env::var("ZK_HOST").unwrap()];
    let host = ZookeeperEnsembleHostConnector::new(zk_hosts, Duration::from_secs(15))
        .connect()
        .await
        .unwrap();
    let _ = host.get_solr_node().await.unwrap();
}
