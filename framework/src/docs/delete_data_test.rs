use serial_test::parallel;

/// ```rust,no_run
/// # use solrstice::AsyncSolrCloudClient;
/// # use solrstice::SolrSingleServerHost;
/// # use solrstice::SolrServerContextBuilder;
/// # use solrstice::DeleteQuery;
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// # let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
/// # let client = AsyncSolrCloudClient::new(context);
/// client
///     .delete(
///         &DeleteQuery::new().ids(["example_document"]),
///         "example_collection",
///     )
/// .await?;
/// # Ok(())
/// # }
/// ```
async fn delete_data_test() {}
