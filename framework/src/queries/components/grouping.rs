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
/// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
/// use solrstice::models::auth::SolrBasicAuth;
/// # use solrstice::models::context::SolrServerContextBuilder;
/// use solrstice::queries::components::grouping::GroupingComponentBuilder;
/// use solrstice::queries::select::SelectQueryBuilder;
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// # let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
/// let response = SelectQueryBuilder::new()
///     .fq(&["age:[* TO *]"])
///     .grouping(&GroupingComponentBuilder::new().fields(&["age"]).limit(10))
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
pub struct GroupingComponentBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<bool>,
    #[serde(rename = "group.field", skip_serializing_if = "Option::is_none")]
    pub field: Option<Vec<String>>,
    #[serde(rename = "group.query", skip_serializing_if = "Option::is_none")]
    pub queries: Option<Vec<String>>,
    #[serde(rename = "group.limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(rename = "group.offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    #[serde(rename = "group.sort", skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<String>>,
    #[serde(rename = "group.format", skip_serializing_if = "Option::is_none")]
    pub format: Option<GroupFormatting>,
    #[serde(rename = "group.main", skip_serializing_if = "Option::is_none")]
    pub main: Option<bool>,
    #[serde(rename = "group.ngroups", skip_serializing_if = "Option::is_none")]
    pub n_groups: Option<bool>,
    #[serde(rename = "group.truncate", skip_serializing_if = "Option::is_none")]
    pub truncate: Option<bool>,
    #[serde(rename = "group.facet", skip_serializing_if = "Option::is_none")]
    pub facet: Option<bool>,
}

impl GroupingComponentBuilder {
    /// Create a new GroupingComponentBuilder.
    /// # Examples
    /// ```no_run
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// use solrstice::models::auth::SolrBasicAuth;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// use solrstice::queries::components::grouping::GroupingComponentBuilder;
    /// use solrstice::queries::select::SelectQueryBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let response = SelectQueryBuilder::new()
    ///     .fq(&["age:[* TO *]"])
    ///     .grouping(&GroupingComponentBuilder::new().fields(&["age"]).limit(10))
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
    /// use solrstice::queries::components::grouping::GroupingComponentBuilder;
    /// GroupingComponentBuilder::new().fields(&["age"]);
    /// ```
    pub fn fields(mut self, fields: &[&str]) -> Self {
        self.field = Some(fields.into_iter().map(|x| x.to_string()).collect());
        self
    }

    /// Queries to group by.
    /// # Examples
    /// ```rust
    /// use solrstice::queries::components::grouping::GroupingComponentBuilder;
    /// GroupingComponentBuilder::new().queries(&["age:[0 TO 59]", "age:[60 TO *]"]);
    /// ```
    pub fn queries(mut self, queries: &[&str]) -> Self {
        self.queries = Some(queries.into_iter().map(|x| x.to_string()).collect());
        self
    }

    /// Maximum number of documents per group.
    /// # Examples
    /// ```rust
    /// use solrstice::queries::components::grouping::GroupingComponentBuilder;
    /// GroupingComponentBuilder::new().limit(10);
    /// ```
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Initial offset
    /// # Examples
    /// ```rust
    /// use solrstice::queries::components::grouping::GroupingComponentBuilder;
    /// GroupingComponentBuilder::new().limit(10).offset(10);
    /// ```
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    /// How to sort the documents in the groups.
    /// # Examples
    /// ```rust
    /// use solrstice::queries::components::grouping::GroupingComponentBuilder;
    /// GroupingComponentBuilder::new().sort(&["age asc"]);
    /// ```
    pub fn sort(mut self, sort: &[&str]) -> Self {
        self.sort = Some(sort.into_iter().map(|x| x.to_string()).collect());
        self
    }

    /// How to format the groups.
    /// # Examples
    /// ```rust
    /// use solrstice::queries::components::grouping::{GroupingComponentBuilder, GroupFormatting};
    /// GroupingComponentBuilder::new().format(GroupFormatting::Simple);
    /// ```
    pub fn format(mut self, format: GroupFormatting) -> Self {
        self.format = Some(format);
        self
    }

    /// Put the results in the main result set.
    /// # Examples
    /// ```rust
    /// use solrstice::queries::components::grouping::GroupingComponentBuilder;
    /// GroupingComponentBuilder::new().main(true);
    /// ```
    pub fn main(mut self, main: bool) -> Self {
        self.main = Some(main);
        self
    }

    /// Include the number of groups that have matched the query.
    /// # Examples
    /// ```rust
    /// use solrstice::queries::components::grouping::GroupingComponentBuilder;
    /// GroupingComponentBuilder::new().n_groups(true);
    /// ```
    pub fn n_groups(mut self, n_groups: bool) -> Self {
        self.n_groups = Some(n_groups);
        self
    }

    /// Not really sure what this does.
    pub fn truncate(mut self, truncate: bool) -> Self {
        self.truncate = Some(truncate);
        self
    }

    /// Not really sure what this does.
    pub fn facet(mut self, facet: bool) -> Self {
        self.facet = Some(facet);
        self
    }
}
