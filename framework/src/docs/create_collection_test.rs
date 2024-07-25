use serial_test::parallel;

/// ```rust,no_run
/// # use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
/// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
/// # use solrstice::models::context::SolrServerContextBuilder;
/// # use std::path::Path;
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// # let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("")).build();
/// # let client = AsyncSolrCloudClient::new(context);
/// client
///    .upload_config("example_config", Path::new("/path/to/config"))
///    .await?;
/// client
///    .create_collection("example_collection", "example_config", 1, 1)
///    .await?;
///     # Ok(())
///  # }
/// ```
async fn create_collection_test() {}
