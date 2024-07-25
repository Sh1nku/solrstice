use crate::structures::FunctionalityTestsBuildup;
use serial_test::parallel;
use solrstice::queries::config::get_configs_blocking;
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
