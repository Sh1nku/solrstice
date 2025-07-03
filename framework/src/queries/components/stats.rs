use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct StatsComponent {
    stats: bool,
    #[serde(rename = "stats.field")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    fields: Vec<String>,
}

impl StatsComponent {
    /// Get stats for fields
    /// # Examples
    /// ```no_run
    /// use solrstice::{AsyncSolrCloudClient, StatsComponent, SelectQuery, SolrSingleServerHost};
    /// # use solrstice::SolrServerContextBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    ///  let query = SelectQuery::new().stats(
    ///     StatsComponent::new().fields(["id"]),
    /// );
    /// let response = client
    ///     .select(&query, "collection_name")
    ///     .await?;
    /// let id_field = response.get_stats().unwrap().get_fields().get("id").unwrap();
    ///
    /// assert!(id_field.get_count() > 0);
    /// assert!(id_field.get_min::<String>()?.len() > 0);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> Self {
        StatsComponent {
            stats: true,
            fields: Vec::new(),
        }
    }

    pub fn fields<S: Into<String>, I: IntoIterator<Item = S>>(mut self, fields: I) -> Self {
        self.fields = fields.into_iter().map(|x| x.into()).collect();
        self
    }
}
