use serial_test::parallel;

/// ```rust
/// # use solrstice::AsyncSolrCloudClient;
/// # use solrstice::SolrSingleServerHost;
/// # use solrstice::SolrServerContextBuilder;
/// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
/// let client = AsyncSolrCloudClient::new(context);
/// ```
async fn create_client_test() {}
