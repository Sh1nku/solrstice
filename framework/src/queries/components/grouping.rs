use serde::{Deserialize, Serialize};
use std::fmt;

/// How to format groups. The default is GroupFormatting::Grouped.
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
pub enum GroupFormatting {
    #[serde(rename = "grouped")]
    Grouped,
    #[serde(rename = "simple")]
    Simple,
}

impl fmt::Display for GroupFormatting {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

/// Group documents by a field or query.
/// # Examples
/// ```no_run
/// use solrstice::{GroupingComponent, SelectQuery, SolrBasicAuth, SolrSingleServerHost};
/// # use solrstice::SolrServerContextBuilder;
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
#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct GroupingComponent {
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<bool>,
    #[serde(rename = "group.field", skip_serializing_if = "Option::is_none")]
    field: Option<Vec<String>>,
    #[serde(rename = "group.query", skip_serializing_if = "Option::is_none")]
    queries: Option<Vec<String>>,
    #[serde(rename = "group.limit", skip_serializing_if = "Option::is_none")]
    limit: Option<usize>,
    #[serde(rename = "group.offset", skip_serializing_if = "Option::is_none")]
    offset: Option<usize>,
    #[serde(rename = "group.sort", skip_serializing_if = "Option::is_none")]
    sort: Option<Vec<String>>,
    #[serde(rename = "group.format", skip_serializing_if = "Option::is_none")]
    format: Option<GroupFormatting>,
    #[serde(rename = "group.main", skip_serializing_if = "Option::is_none")]
    main: Option<bool>,
    #[serde(rename = "group.ngroups", skip_serializing_if = "Option::is_none")]
    n_groups: Option<bool>,
    #[serde(rename = "group.truncate", skip_serializing_if = "Option::is_none")]
    truncate: Option<bool>,
    #[serde(rename = "group.facet", skip_serializing_if = "Option::is_none")]
    facet: Option<bool>,
}

impl GroupingComponent {
    /// Create a new GroupingComponentBuilder.
    /// # Examples
    /// ```no_run
    /// use solrstice::{GroupingComponent, SelectQuery, SolrBasicAuth, SolrSingleServerHost};
    /// # use solrstice::SolrServerContextBuilder;
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
    pub fn new() -> Self {
        Self {
            group: Some(true),
            field: None,
            queries: None,
            limit: None,
            offset: None,
            sort: None,
            format: None,
            main: None,
            n_groups: None,
            truncate: None,
            facet: None,
        }
    }

    /// Fields to group by.
    /// # Examples
    /// ```rust
    /// use solrstice::GroupingComponent;
    /// GroupingComponent::new().fields(["age"]);
    /// ```
    pub fn fields<S: Into<String>, I: IntoIterator<Item = S>, O: Into<Option<I>>>(
        mut self,
        fields: O,
    ) -> Self {
        self.field = fields
            .into()
            .map(|x| x.into_iter().map(|x| x.into()).collect());
        self
    }

    /// Queries to group by.
    /// # Examples
    /// ```rust
    /// use solrstice::GroupingComponent;
    /// GroupingComponent::new().queries(["age:[0 TO 59]", "age:[60 TO *]"]);
    /// ```
    pub fn queries<S: Into<String>, I: IntoIterator<Item = S>, O: Into<Option<I>>>(
        mut self,
        queries: O,
    ) -> Self {
        self.queries = queries
            .into()
            .map(|x| x.into_iter().map(|x| x.into()).collect());
        self
    }

    /// Maximum number of documents per group.
    /// # Examples
    /// ```rust
    /// use solrstice::GroupingComponent;
    /// GroupingComponent::new().limit(10);
    /// ```
    pub fn limit<O: Into<Option<usize>>>(mut self, limit: O) -> Self {
        self.limit = limit.into();
        self
    }

    /// Initial offset
    /// # Examples
    /// ```rust
    /// use solrstice::GroupingComponent;
    /// GroupingComponent::new().limit(10).offset(10);
    /// ```
    pub fn offset<O: Into<Option<usize>>>(mut self, offset: O) -> Self {
        self.offset = offset.into();
        self
    }

    /// How to sort the documents in the groups.
    /// # Examples
    /// ```rust
    /// use solrstice::GroupingComponent;
    /// GroupingComponent::new().sort(["age asc"]);
    /// ```
    pub fn sort<S: Into<String>, I: IntoIterator<Item = S>, O: Into<Option<I>>>(
        mut self,
        sort: O,
    ) -> Self {
        self.sort = sort
            .into()
            .map(|fq| fq.into_iter().map(|s| s.into()).collect());
        self
    }

    /// How to format the groups.
    /// # Examples
    /// ```rust
    /// use solrstice::{GroupingComponent, GroupFormatting};
    /// GroupingComponent::new().format(GroupFormatting::Simple);
    /// ```
    pub fn format<O: Into<Option<GroupFormatting>>>(mut self, format: O) -> Self {
        self.format = format.into();
        self
    }

    /// Put the results in the main result set.
    /// # Examples
    /// ```rust
    /// use solrstice::GroupingComponent;
    /// GroupingComponent::new().main(true);
    /// ```
    pub fn main<O: Into<Option<bool>>>(mut self, main: O) -> Self {
        self.main = main.into();
        self
    }

    /// Include the number of groups that have matched the query.
    /// # Examples
    /// ```rust
    /// use solrstice::GroupingComponent;
    /// GroupingComponent::new().n_groups(true);
    /// ```
    pub fn n_groups<O: Into<Option<bool>>>(mut self, n_groups: O) -> Self {
        self.n_groups = n_groups.into();
        self
    }

    /// Not really sure what this does.
    pub fn truncate<O: Into<Option<bool>>>(mut self, truncate: O) -> Self {
        self.truncate = truncate.into();
        self
    }

    /// Not really sure what this does.
    pub fn facet<O: Into<Option<bool>>>(mut self, facet: O) -> Self {
        self.facet = facet.into();
        self
    }
}

impl AsRef<GroupingComponent> for GroupingComponent {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl From<&GroupingComponent> for GroupingComponent {
    fn from(component: &GroupingComponent) -> Self {
        component.clone()
    }
}
