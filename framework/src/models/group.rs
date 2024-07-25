use crate::error::Error;
use crate::models::response::SolrDocsResponse;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

/// Struct representing a Solr Grouping response
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SolrGroupResult {
    matches: usize,
    #[serde(rename = "ngroups")]
    n_groups: Option<usize>,
    groups: Option<Vec<SolrGroupFieldResult>>,
    #[serde(rename = "doclist")]
    doc_list: Option<SolrDocsResponse>,
}

impl SolrGroupResult {
    /// Returns a field query result
    /// # Examples
    /// ```no_run
    /// use solrstice::{GroupingComponent, SelectQuery, SolrServerContextBuilder, SolrSingleServerHost};
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let response = SelectQuery::new()
    ///     .fq(["age:[* TO *]"])
    ///     .grouping(&GroupingComponent::new().fields(["age"]).limit(10))
    ///     .execute(&context, "collection_name")
    ///     .await?;
    /// let groups = response.get_groups().ok_or("No groups")?;
    /// let age_group = groups.get("age").ok_or("No age group")?;
    ///
    /// for group in age_group.get_field_result().ok_or("No field result")? {
    ///     println!("Group key: {}", group.get_group_value::<usize>()?);
    ///     let docs = group.get_doc_list().get_docs::<serde_json::Value>()?;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_field_result(&self) -> Option<&[SolrGroupFieldResult]> {
        self.groups.as_deref()
    }

    /// Returns a grouping query result
    /// # Examples
    /// ```no_run
    /// use solrstice::{GroupingComponent, SelectQuery, SolrSingleServerHost};
    /// # use solrstice::SolrServerContextBuilder;
    ///
    ///  async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let response = SelectQuery::new()
    ///     .grouping(
    ///         &GroupingComponent::new()
    ///             .queries(["age:[0 TO 59]", "age:[60 TO *]"])
    ///             .limit(10),
    ///     )
    ///     .execute(&context, "collection_name")
    ///     .await?;
    ///
    /// let groups = response
    ///     .get_groups().ok_or("No groups")?;
    /// let result = groups
    ///     .get("age:[0 TO 59]").ok_or("No age group")?
    ///     .get_query_result().ok_or("No query result")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_query_result(&self) -> Option<&SolrDocsResponse> {
        self.doc_list.as_ref()
    }

    /// If [GroupFormatting::Simple](crate::queries::components::grouping::GroupFormatting::Simple) is used, returns a simple grouping query result. This uses the same logic as [get_query_result](SolrGroupResult::get_query_result)
    /// # Examples
    /// ```no_run
    /// use solrstice::{GroupFormatting, GroupingComponent, SolrBasicAuth, SelectQuery, SolrServerContextBuilder, SolrSingleServerHost};
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let response = SelectQuery::new()
    ///     .fq(["age:[* TO *]"])
    ///     .grouping(&GroupingComponent::new().fields(["age"]).limit(10).format(GroupFormatting::Simple))
    ///     .execute(&context, "collection_name")
    ///     .await?;
    /// let groups = response.get_groups().ok_or("No groups")?;
    /// let age_group = groups.get("age").ok_or("No age group")?;
    ///
    /// let result = age_group.get_simple_result().ok_or("No field result")?;
    /// # Ok(())
    /// # }
    pub fn get_simple_result(&self) -> Option<&SolrDocsResponse> {
        self.doc_list.as_ref()
    }

    /// Returns the number of matches for the query
    pub fn get_matches(&self) -> usize {
        self.matches
    }

    /// Returns the number of groups for the query, if n_groups was given
    pub fn get_n_groups(&self) -> Option<usize> {
        self.n_groups
    }
}

/// Struct representing a Solr Grouping field response
///
/// group_value can be multiple types (int, string), so it is not immediately deserialized
/// # Examples
/// ```no_run
/// use solrstice::{AsyncSolrCloudClient, GroupingComponent, SelectQuery, SolrServerContextBuilder, SolrSingleServerHost};
///
///  async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// # let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
/// # let client = AsyncSolrCloudClient::new(context);
/// let response = client.select(&SelectQuery::new()
///     .fq(["age:[* TO *]"])
///     .grouping(&GroupingComponent::new().fields(["age"]).limit(10)), "collection_name").await?;
/// let groups = response
///     .get_groups()
///     .ok_or("No groups found")?;
/// let age_group = groups.get("age").ok_or("No age group")?;
/// for group in age_group.get_field_result().ok_or("No field result")? {
///     println!("Group key: {}", group.get_group_value::<usize>()?);
///     let docs = group.get_doc_list().get_docs::<serde_json::Value>()?;
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SolrGroupFieldResult {
    /// The key of the field result
    #[serde(rename = "groupValue")]
    group_value: Box<RawValue>,
    /// A list of documents
    #[serde(rename = "doclist")]
    doc_list: SolrDocsResponse,
}

impl SolrGroupFieldResult {
    /// Returns the group key
    /// # Examples
    /// ```no_run
    /// # use solrstice::{AsyncSolrCloudClient, SolrSingleServerHost};
    /// # use solrstice::SolrServerContextBuilder;
    /// # use solrstice::GroupingComponent;
    /// # use solrstice::SelectQuery;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// # let client = AsyncSolrCloudClient::new(context);
    /// let response = client.select(&SelectQuery::new()
    ///     .fq(["age:[* TO *]"])
    ///     .grouping(&GroupingComponent::new().fields(["age"]).limit(10)), "collection_name").await?;
    /// let groups = response
    ///     .get_groups()
    ///     .ok_or("No groups found")?;
    /// let age_group = groups.get("age").ok_or("No age group")?;
    /// for group in age_group.get_field_result().ok_or("No field result")? {
    ///     println!("Group key: {}", group.get_group_value::<usize>()?);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_group_value<K: DeserializeOwned>(&self) -> Result<K, Error> {
        serde_json::from_str(self.group_value.get()).map_err(Error::from)
    }

    /// Returns a list of documents corresponding to the group
    /// # Examples
    /// ```no_run
    /// # use solrstice::{AsyncSolrCloudClient, SolrSingleServerHost};
    /// # use solrstice::SolrServerContextBuilder;
    /// # use solrstice::GroupingComponent;
    /// # use solrstice::SelectQuery;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// # let client = AsyncSolrCloudClient::new(context);
    /// let response = client.select(&SelectQuery::new()
    ///     .fq(["age:[* TO *]"])
    ///     .grouping(&GroupingComponent::new().fields(["age"]).limit(10)), "collection_name").await?;
    /// let groups = response
    ///     .get_groups()
    ///     .ok_or("No groups found")?;
    /// let age_group = groups.get("age").ok_or("No age group")?;
    /// for group in age_group.get_field_result().ok_or("No field result")? {
    ///     println!("Group key: {}", group.get_group_value::<usize>()?);
    ///     let docs = group.get_doc_list().get_docs::<serde_json::Value>()?;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_doc_list(&self) -> &SolrDocsResponse {
        &self.doc_list
    }
}
