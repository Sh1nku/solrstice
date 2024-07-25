use std::collections::HashMap;

use serde::{Deserialize, Serialize, Serializer};

/// Get self defined facets.
/// # Examples
/// ```no_run
/// use solrstice::{AsyncSolrCloudClient, JsonFacetComponent, JsonQueryFacet, SelectQuery, SolrSingleServerHost};
/// # use solrstice::SolrServerContextBuilder;
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// # let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
/// let client = AsyncSolrCloudClient::new(context);
///  let query = SelectQuery::new().json_facet(
///     JsonFacetComponent::new().facets([("below_60", JsonQueryFacet::new("age:[0 TO 59]"))]),
/// );
/// let response = client
///     .select(&query, "collection_name")
///     .await?;
/// let facets = response.get_json_facets().ok_or("No facets")?;
/// let below_60 = facets
///     .get_nested_facets()
///     .get("below_60")
///     .ok_or("No below_60 facet")?;
/// assert_eq!(below_60.get_count().ok_or("No count")?, 4);
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct JsonFacetComponent {
    #[serde(rename = "json.facet", serialize_with = "json_facet_as_string")]
    facet: HashMap<String, JsonFacetType>,
}

impl AsRef<JsonFacetComponent> for JsonFacetComponent {
    fn as_ref(&self) -> &JsonFacetComponent {
        self
    }
}

impl From<&JsonFacetComponent> for JsonFacetComponent {
    fn from(component: &JsonFacetComponent) -> Self {
        component.clone()
    }
}

impl JsonFacetComponent {
    /// Create a new instance of [JsonFacetComponent].
    /// # Examples
    /// ```no_run
    /// use solrstice::{AsyncSolrCloudClient, JsonFacetComponent, JsonQueryFacet, SelectQuery, SolrSingleServerHost};
    /// # use solrstice::SolrServerContextBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    ///  let query = SelectQuery::new().json_facet(
    ///     JsonFacetComponent::new().facets([("below_60", JsonQueryFacet::new("age:[0 TO 59]"))]),
    /// );
    /// let response = client
    ///     .select(&query, "collection_name")
    ///     .await?;
    /// let facets = response.get_json_facets().ok_or("No facets")?;
    /// let below_60 = facets
    ///     .get_nested_facets()
    ///     .get("below_60")
    ///     .ok_or("No below_60 facet")?;
    /// assert_eq!(below_60.get_count().ok_or("No count")?, 4);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> Self {
        JsonFacetComponent {
            facet: Default::default(),
        }
    }

    /// Set the facets
    pub fn facets<K: Into<String>, V: Into<JsonFacetType>, I: IntoIterator<Item = (K, V)>>(
        mut self,
        facets: I,
    ) -> Self {
        self.facet = facets
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        self
    }
}

impl Default for JsonFacetComponent {
    fn default() -> Self {
        JsonFacetComponent::new()
    }
}

fn json_facet_as_string<S>(
    facet: &HashMap<String, JsonFacetType>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let json_string = serde_json::to_string(facet).map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&json_string)
}

/// The different types of facets supported by JsonFacet
///
/// [JsonTermsFacet] [JsonQueryFacet] [JsonStatFacet]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum JsonFacetType {
    Terms(Box<JsonTermsFacet>),
    Query(Box<JsonQueryFacet>),
    Stat(JsonStatFacet),
}

/// A facet that counts the number of documents that match a query
/// # Examples
/// ```no_run
/// # use solrstice::{AsyncSolrCloudClient, JsonFacetComponent, JsonQueryFacet, JsonStatFacet, JsonTermsFacet, SelectQuery, SolrSingleServerHost};
/// # use solrstice::SolrServerContextBuilder;
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// # let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
/// let client = AsyncSolrCloudClient::new(context);
/// let query = SelectQuery::new()
///     .json_facet(JsonFacetComponent::new().facets([("age", JsonTermsFacet::new("age"))]));
/// let response = client
///     .select(&query, "collection_name")
///     .await?;
/// let facets = response.get_json_facets().ok_or("No facets")?;
/// let age = facets
///     .get_nested_facets()
///     .get("age")
///     .ok_or("No age facet")?;
/// let buckets = age
///     .get_buckets()
///     .collect::<Vec<_>>();
/// assert_eq!(buckets.len(), 3);
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct JsonTermsFacet {
    #[serde(rename = "type")]
    type_: String,
    field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    facet: Option<HashMap<String, JsonFacetType>>,
}

impl From<JsonTermsFacet> for JsonFacetType {
    fn from(facet: JsonTermsFacet) -> Self {
        JsonFacetType::Terms(Box::new(facet))
    }
}

impl JsonTermsFacet {
    /// Create a new terms facet
    /// A facet that counts the number of documents that match a query
    /// # Examples
    /// ```no_run
    /// use solrstice::{AsyncSolrCloudClient, JsonFacetComponent, JsonQueryFacet, JsonStatFacet, JsonTermsFacet, SelectQuery, SolrSingleServerHost};
    /// # use solrstice::SolrServerContextBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// let query = SelectQuery::new()
    ///     .json_facet(JsonFacetComponent::new().facets([("age", JsonTermsFacet::new("age"))]));
    /// let response = client
    ///     .select(&query, "collection_name")
    ///     .await?;
    /// let facets = response.get_json_facets().ok_or("No facets")?;
    /// let age = facets
    ///     .get_nested_facets()
    ///     .get("age")
    ///     .ok_or("No age facet")?;
    /// let buckets = age
    ///     .get_buckets()
    ///     .collect::<Vec<_>>();
    /// assert_eq!(buckets.len(), 3);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new<S: Into<String>>(field: S) -> Self {
        JsonTermsFacet {
            type_: "terms".to_string(),
            field: field.into(),
            offset: None,
            limit: None,
            sort: None,
            facet: None,
        }
    }

    /// Set the offset of the facet
    pub fn offset<O: Into<Option<usize>>>(mut self, offset: O) -> Self {
        self.offset = offset.into();
        self
    }

    /// Limit the number of facet results
    pub fn limit<O: Into<Option<usize>>>(mut self, limit: O) -> Self {
        self.limit = limit.into();
        self
    }

    /// Sort the facet results
    pub fn sort<S: Into<String>, O: Into<Option<S>>>(mut self, sort: O) -> Self {
        self.sort = sort.into().map(|s| s.into());
        self
    }

    /// Add sub-facets to the facet
    pub fn facets<
        K: Into<String>,
        V: Into<JsonFacetType>,
        I: IntoIterator<Item = (K, V)>,
        O: Into<Option<I>>,
    >(
        mut self,
        facets: O,
    ) -> Self {
        self.facet = facets.into().map(|facets| {
            facets
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect()
        });
        self
    }
}

/// A facet that does a query and returns the number of documents that match
/// # Examples
/// ```no_run
/// use solrstice::{AsyncSolrCloudClient, JsonFacetComponent, JsonQueryFacet, JsonStatFacet, SelectQuery, SolrSingleServerHost};
/// # use solrstice::SolrServerContextBuilder;
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// # let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
/// let client = AsyncSolrCloudClient::new(context);
/// let query = SelectQuery::new().json_facet(
///     JsonFacetComponent::new().facets([("below_60", JsonQueryFacet::new("age:[0 TO 59]"))]),
/// );
/// let response = client
///     .select(&query, "collection_name")
///     .await?;
/// let facets = response.get_json_facets().ok_or("No facets")?;
/// let below_60 = facets
///     .get_nested_facets()
///     .get("below_60")
///     .ok_or("No below_60 facet")?;
/// assert_eq!(below_60.get_count().ok_or("No count")?, 4);
/// # Ok(())
/// # }
/// ```

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct JsonQueryFacet {
    #[serde(rename = "type")]
    type_: String,
    q: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fq: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    facet: Option<HashMap<String, JsonFacetType>>,
}

impl From<JsonQueryFacet> for JsonFacetType {
    fn from(facet: JsonQueryFacet) -> Self {
        JsonFacetType::Query(Box::new(facet))
    }
}

impl JsonQueryFacet {
    /// Create a new query facet
    /// # Examples
    /// ```no_run
    /// use solrstice::{AsyncSolrCloudClient, JsonFacetComponent, JsonQueryFacet, JsonStatFacet, SelectQuery, SolrSingleServerHost};
    /// # use solrstice::SolrServerContextBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// let query = SelectQuery::new().json_facet(
    ///     JsonFacetComponent::new().facets([("below_60", JsonQueryFacet::new("age:[0 TO 59]"))]),
    /// );
    /// let response = client
    ///     .select(&query, "collection_name")
    ///     .await?;
    /// let facets = response.get_json_facets().ok_or("No facets")?;
    /// let below_60 = facets
    ///     .get_nested_facets()
    ///     .get("below_60")
    ///     .ok_or("No below_60 facet")?;
    /// assert_eq!(below_60.get_count().ok_or("No count")?, 4);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new<S: Into<String>>(q: S) -> Self {
        JsonQueryFacet {
            type_: "query".to_string(),
            q: q.into(),
            limit: None,
            offset: None,
            sort: None,
            fq: None,
            facet: None,
        }
    }

    /// Limit the number of facet results
    pub fn limit<O: Into<Option<usize>>>(mut self, limit: O) -> Self {
        self.limit = limit.into();
        self
    }

    /// Offset the facet results
    pub fn offset<O: Into<Option<usize>>>(mut self, offset: O) -> Self {
        self.offset = offset.into();
        self
    }

    /// Sort the query facet results
    pub fn sort<S: Into<String>, O: Into<Option<S>>>(mut self, sort: O) -> Self {
        self.sort = sort.into().map(|s| s.into());
        self
    }

    /// Use filter queries to filter the query facet
    pub fn fq<S: Into<String>, I: IntoIterator<Item = S>, O: Into<Option<I>>>(
        mut self,
        fq: O,
    ) -> Self {
        self.fq = fq
            .into()
            .map(|fq| fq.into_iter().map(|s| s.into()).collect());
        self
    }

    /// Add nested facets to the query facet
    pub fn facets<
        K: Into<String>,
        V: Into<JsonFacetType>,
        I: IntoIterator<Item = (K, V)>,
        O: Into<Option<I>>,
    >(
        mut self,
        facets: O,
    ) -> Self {
        self.facet = facets.into().map(|facets| {
            facets
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect()
        });
        self
    }
}

/// A facet that does a query and gets the number of results
/// # Examples
/// ```no_run
/// use solrstice::{AsyncSolrCloudClient, JsonFacetComponent, JsonQueryFacet, JsonStatFacet, SelectQuery, SolrSingleServerHost};
/// # use solrstice::SolrServerContextBuilder;
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// # let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
/// let client = AsyncSolrCloudClient::new(context);
/// let query = SelectQuery::new().json_facet(
///     JsonFacetComponent::new().facets([("total_people", JsonStatFacet::new("sum(count)"))]),
/// );
/// let response = client
///     .select(&query, "collection_name")
///     .await?;
/// let facets = response.get_json_facets().ok_or("No facets")?;
/// let total_people = facets
///     .get_flat_facets()
///     .get("total_people")
///     .ok_or("No total_people facet")?;
/// assert_eq!(total_people.as_f64().ok_or("Not a number")?, 1000.0);
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct JsonStatFacet(String);

impl From<JsonStatFacet> for JsonFacetType {
    fn from(facet: JsonStatFacet) -> Self {
        JsonFacetType::Stat(facet)
    }
}

impl JsonStatFacet {
    /// Create a new JsonStatFacet
    /// # Examples
    /// ```no_run
    /// use solrstice::{AsyncSolrCloudClient, JsonFacetComponent, JsonQueryFacet, JsonStatFacet, SelectQuery, SolrSingleServerHost};
    /// # use solrstice::SolrServerContextBuilder;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// let client = AsyncSolrCloudClient::new(context);
    /// let query = SelectQuery::new().json_facet(
    ///     JsonFacetComponent::new().facets([("total_people", JsonStatFacet::new("sum(count)"))]),
    /// );
    /// let response = client
    ///     .select(&query, "collection_name")
    ///     .await?;
    /// let facets = response.get_json_facets().ok_or("No facets")?;
    /// let total_people = facets
    ///     .get_flat_facets()
    ///     .get("total_people")
    ///     .ok_or("No total_people facet")?;
    /// assert_eq!(total_people.as_f64().ok_or("Not a number")?, 1000.0);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new<S: Into<String>>(stat: S) -> Self {
        JsonStatFacet(stat.into())
    }
}
