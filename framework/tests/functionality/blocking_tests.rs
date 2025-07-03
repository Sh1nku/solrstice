use crate::structures::{get_test_data, FunctionalityTestsBuildup};
use serial_test::parallel;
use solrstice::queries::config::get_configs_blocking;
use solrstice::{Error, SelectQuery, UpdateQuery};
use std::thread;

#[test]
#[parallel]
fn blocking_works_when_simultaneous_connections_multiple_threads() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let config = runtime.block_on(async {
        FunctionalityTestsBuildup::build_up("BlockingMultipleConsumer")
            .await
            .unwrap()
    });

    let threads = 100;
    let mut handles = Vec::new();
    for _ in 0..threads {
        let server_request = config.context.clone();
        let handle = thread::spawn(move || {
            get_configs_blocking(&server_request).unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    runtime.block_on(async { config.tear_down().await.unwrap() });
}

#[test]
#[parallel]
fn select_raw_works_with_blocking() -> Result<(), Error> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let config = runtime.block_on(async {
        FunctionalityTestsBuildup::build_up("SelectRawBlocking")
            .await
            .unwrap()
    });
    runtime.block_on(async {
        UpdateQuery::new()
            .execute(&config.context, &config.collection_name, &get_test_data())
            .await
            .unwrap();
    });

    let result = SelectQuery::new()
        .execute_blocking_raw(&config.context, &config.collection_name)
        .unwrap();
    assert!(result["response"]["numFound"].as_u64().unwrap() > 0);
    runtime.block_on(async { config.tear_down().await.unwrap() });
    Ok(())
}
