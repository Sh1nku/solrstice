use serial_test::parallel;

/// ```rust,no_run
/// # use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
/// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
/// # use solrstice::models::context::SolrServerContextBuilder;
/// # use solrstice::queries::index::UpdateQuery;
/// # use serde::{Serialize, Deserialize};
///
/// #[derive(Serialize, Deserialize, Debug)]
/// struct TestData {
///     id: String,
/// }
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let docs = vec![TestData {
///     id: "example_document".to_string(),
/// }];
///
/// # let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("")).build();
/// # let client = AsyncSolrCloudClient::new(context);
/// client
///     .index(&UpdateQuery::new(), "example_collection", docs.as_slice())
///     .await?;
/// # Ok(())
/// # }
/// ```
async fn index_data_test() {}
