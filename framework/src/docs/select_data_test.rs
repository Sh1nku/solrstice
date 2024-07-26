use serial_test::parallel;

/// ```rust,no_run
/// # use solrstice::AsyncSolrCloudClient;
/// # use solrstice::SolrSingleServerHost;
/// # use solrstice::SolrServerContextBuilder;
/// # use solrstice::SelectQuery;
/// # use serde::{Serialize, Deserialize};
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// # let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
/// # let client = AsyncSolrCloudClient::new(context);
/// # #[derive(Serialize, Deserialize, Debug)]
/// # struct TestData {
/// #     id: String,
/// # }
/// let docs = client
///     .select(
///         &SelectQuery::new()
///         .fq(["id:example_document"]),
///         "example_collection",
///     )
/// .await?
/// .get_docs_response()
/// .ok_or("No response provided")?
/// .get_docs::<TestData>()?;
/// # Ok(())
/// # }
/// ```
async fn select_data_test() {}
