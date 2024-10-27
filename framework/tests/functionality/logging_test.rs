use crate::structures::{BaseTestsBuildup, ErrrorTestsSetup, FunctionalityTestsBuildup};
use log::{Metadata, Record};
use serial_test::serial;
use solrstice::Error;
use solrstice::LoggingPolicy;
use solrstice::SolrServerContextBuilder;
use solrstice::{AsyncSolrCloudClient, SelectQuery};
use std::sync::{Arc, Mutex, OnceLock};

struct TestLogger {
    pub messages: Arc<Mutex<Vec<String>>>,
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

static LOGGER_MESSAGES: OnceLock<Arc<Mutex<Vec<String>>>> = OnceLock::new();

pub fn init_logger() -> Arc<Mutex<Vec<String>>> {
    let messages = Arc::new(Mutex::new(Vec::new()));
    let logger = TestLogger {
        messages: Arc::clone(&messages),
    };
    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(log::LevelFilter::Debug);
    messages
}

#[tokio::test]
#[serial]
async fn logging_logs_message() -> Result<(), Error> {
    let config = BaseTestsBuildup::new().await;
    let mut context = SolrServerContextBuilder::new(config.host);
    if config.auth.is_some() {
        context = context.with_auth(config.auth.unwrap());
    }
    let context = context.build();
    let client = AsyncSolrCloudClient::new(context);

    LOGGER_MESSAGES
        .get_or_init(init_logger)
        .lock()
        .unwrap()
        .clear();

    let _ = client.get_configs().await.unwrap();
    let messages = LOGGER_MESSAGES.get().unwrap().lock().unwrap();
    for message in messages.iter() {
        if message.contains("Sending Solr request to") {
            return Ok(());
        }
    }
    Err(Error::Unknown("No log message found".to_string()))
}

#[tokio::test]
#[serial]
async fn logging_does_not_log_message_if_disabled() -> Result<(), Error> {
    let config = BaseTestsBuildup::new().await;
    let mut context =
        SolrServerContextBuilder::new(config.host).with_logging_policy(LoggingPolicy::Off);
    if config.auth.is_some() {
        context = context.with_auth(config.auth.unwrap());
    }
    let context = context.build();
    let client = AsyncSolrCloudClient::new(context);

    LOGGER_MESSAGES
        .get_or_init(init_logger)
        .lock()
        .unwrap()
        .clear();

    let _ = client.get_configs().await.unwrap();

    let messages = LOGGER_MESSAGES.get().unwrap().lock().unwrap();
    for message in messages.iter() {
        if message.contains("Sending Solr request to") {
            return Err(Error::Unknown(format!(
                "Did not expect log message, but got {}",
                message
            )));
        }
    }
    Ok(())
}

#[tokio::test]
#[serial]
async fn logging_works_if_request_fails() -> Result<(), Error> {
    let config = FunctionalityTestsBuildup::build_up("SyntaxErrorLogging")
        .await
        .unwrap();
    let client = AsyncSolrCloudClient::new(config.context);

    LOGGER_MESSAGES
        .get_or_init(init_logger)
        .lock()
        .unwrap()
        .clear();

    let query = SelectQuery::new().fq(["this_is_a_syntax_error::0"]);
    let _ = client.select(query, &config.collection_name).await;
    let messages = LOGGER_MESSAGES.get().unwrap().lock().unwrap();
    for message in messages.iter() {
        if message.contains("Sending Solr request to") {
            return Ok(());
        }
    }
    Err(Error::Unknown("No log message found".to_string()))
}

#[tokio::test]
#[serial]
async fn logging_works_if_request_never_reaches_solr() -> Result<(), Error> {
    let config = ErrrorTestsSetup::new().await;
    let client = AsyncSolrCloudClient::new(config.context);

    LOGGER_MESSAGES
        .get_or_init(init_logger)
        .lock()
        .unwrap()
        .clear();

    let _ = client.select(SelectQuery::new(), "error_collection").await;
    let messages = LOGGER_MESSAGES.get().unwrap().lock().unwrap();
    for message in messages.iter() {
        if message.contains("Sending Solr request to") {
            return Ok(());
        }
    }
    Err(Error::Unknown("No log message found".to_string()))
}
