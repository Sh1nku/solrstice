use crate::structures::BaseTestsBuildup;
use log::{Metadata, Record};
use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
use solrstice::models::context::SolrServerContextBuilder;
use solrstice::models::error::SolrError;
use solrstice::queries::request_builder::LoggingPolicy;
use std::sync::{Arc, Mutex};

struct TestLogger {
    messages: Arc<Mutex<Vec<String>>>,
}

impl log::Log for TestLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut messages = self.messages.lock().unwrap();
            messages.push(format!("{} - {}", record.level(), record.args()));
        }
    }

    fn flush(&self) {}
}

#[tokio::test]
async fn logging_logs_message() -> Result<(), SolrError> {
    let messages = Arc::new(Mutex::new(Vec::new()));
    let logger = TestLogger {
        messages: Arc::clone(&messages),
    };
    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(log::LevelFilter::Debug);

    let config = BaseTestsBuildup::new().await;
    let mut context = SolrServerContextBuilder::new(config.host);
    if config.auth.is_some() {
        context = context.with_auth(config.auth.unwrap());
    }
    let context = context.build();
    let client = AsyncSolrCloudClient::new(context);

    messages.lock().unwrap().clear();

    let _ = client.get_configs().await.unwrap();
    for message in &*messages.lock().unwrap() {
        if message.contains("Sending Solr request to") {
            return Ok(());
        }
    }
    Err(SolrError::Unknown("No log message found".to_string()))
}

#[tokio::test]
async fn logging_does_not_log_message_if_disabled() -> Result<(), SolrError> {
    let messages = Arc::new(Mutex::new(Vec::new()));
    let logger = TestLogger {
        messages: Arc::clone(&messages),
    };
    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(log::LevelFilter::Debug);

    let config = BaseTestsBuildup::new().await;
    let mut context =
        SolrServerContextBuilder::new(config.host).with_logging_policy(LoggingPolicy::Off);
    if config.auth.is_some() {
        context = context.with_auth(config.auth.unwrap());
    }
    let context = context.build();
    let client = AsyncSolrCloudClient::new(context);

    messages.lock().unwrap().clear();

    let _ = client.get_configs().await.unwrap();
    for message in &*messages.lock().unwrap() {
        if message.contains("Sending Solr request to") {
            return Err(SolrError::Unknown(format!(
                "Did not expect log message, but got {}",
                message
            )));
        }
    }
    Ok(())
}
