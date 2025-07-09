use std::collections::{BTreeMap, HashMap};

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
///     JsonFacetComponent::new().facets([("below_60", JsonQueryFacet::new().q("age:[0 TO 59]"))]),
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
    ///     JsonFacetComponent::new().facets([("below_60", JsonQueryFacet::new().q("age:[0 TO 59]"))]),
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum JsonFacetSortDirection {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
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
    sort: Option<BTreeMap<String, JsonFacetSortDirection>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prelim_sort: Option<BTreeMap<String, JsonFacetSortDirection>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overrequest: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refine: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overrefine: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mincount: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    missing: Option<bool>,
    #[serde(rename = "numBuckets", skip_serializing_if = "Option::is_none")]
    num_buckets: Option<bool>,
    #[serde(rename = "allBuckets", skip_serializing_if = "Option::is_none")]
    all_buckets: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    facet: Option<HashMap<String, JsonFacetType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<JsonTermsFacetMethod>,
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
            prelim_sort: None,
            overrequest: None,
            refine: None,
            overrefine: None,
            mincount: None,
            missing: None,
            num_buckets: None,
            all_buckets: None,
            prefix: None,
            facet: None,
            method: None,
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
    pub fn sort<
        K: Into<String>,
        V: Into<JsonFacetSortDirection>,
        I: IntoIterator<Item = (K, V)>,
        O: Into<Option<I>>,
    >(
        mut self,
        sort: O,
    ) -> Self {
        self.sort = sort.into().map(|sort| {
            sort.into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect()
        });
        self
    }

    /// Prelim sort the facet results
    pub fn prelim_sort<
        K: Into<String>,
        V: Into<JsonFacetSortDirection>,
        I: IntoIterator<Item = (K, V)>,
        O: Into<Option<I>>,
    >(
        mut self,
        sort: O,
    ) -> Self {
        self.prelim_sort = sort.into().map(|sort| {
            sort.into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect()
        });
        self
    }

    pub fn overrequest<O: Into<Option<usize>>>(mut self, overrequest: O) -> Self {
        self.overrequest = overrequest.into();
        self
    }

    pub fn refine<O: Into<Option<bool>>>(mut self, refine: O) -> Self {
        self.refine = refine.into();
        self
    }

    pub fn overrefine<O: Into<Option<usize>>>(mut self, overrefine: O) -> Self {
        self.overrefine = overrefine.into();
        self
    }

    /// The minimum count for a bucket to be included
    pub fn mincount<O: Into<Option<usize>>>(mut self, mincount: O) -> Self {
        self.mincount = mincount.into();
        self
    }

    /// Add a special bucket that contains the number of documents that were not counted
    pub fn missing<O: Into<Option<bool>>>(mut self, missing: O) -> Self {
        self.missing = missing.into();
        self
    }

    /// Add a special bucket that contains the number of buckets for the facet
    pub fn num_buckets<O: Into<Option<bool>>>(mut self, num_buckets: O) -> Self {
        self.num_buckets = num_buckets.into();
        self
    }

    /// Add a special bucket
    pub fn all_buckets<O: Into<Option<bool>>>(mut self, all_buckets: O) -> Self {
        self.all_buckets = all_buckets.into();
        self
    }

    /// Only return buckets with the following prefix
    pub fn prefix<S: Into<String>, O: Into<Option<S>>>(mut self, prefix: O) -> Self {
        self.prefix = prefix.into().map(|s| s.into());
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

    /// Which facet algorithm to use
    pub fn method<O: Into<Option<JsonTermsFacetMethod>>>(mut self, method: O) -> Self {
        self.method = method.into();
        self
    }
}

/// The facet algorithm to use
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
pub enum JsonTermsFacetMethod {
    #[serde(rename = "dv")]
    DocValues,
    #[serde(rename = "uif")]
    UnInvertedField,
    #[serde(rename = "dvhash")]
    DocValuesHash,
    #[serde(rename = "enum")]
    Enum,
    #[serde(rename = "stream")]
    Stream,
    #[serde(rename = "smart")]
    Smart,
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
///     JsonFacetComponent::new().facets([("below_60", JsonQueryFacet::new().q("age:[0 TO 59]"))]),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    q: Option<String>,
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
    ///     JsonFacetComponent::new().facets([("below_60", JsonQueryFacet::new().q("age:[0 TO 59]"))]),
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
        JsonQueryFacet {
            type_: "query".to_string(),
            q: None,
            limit: None,
            offset: None,
            sort: None,
            fq: None,
            facet: None,
        }
    }

    /// Set the query for the facet
    pub fn q<S: Into<String>>(mut self, q: S) -> Self {
        self.q = Some(q.into());
        self
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
