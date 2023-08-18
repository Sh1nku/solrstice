use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum DefType {
    Lucene(LuceneQuery),
    Dismax(DismaxQuery),
    Edismax(EdismaxQuery),
}

impl From<&DefType> for DefType {
    fn from(def_type: &DefType) -> Self {
        def_type.clone()
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
pub enum QueryOperator {
    AND,
    OR,
}

/// The default query parser
///
/// Documentation can be found at [SolrDocs](https://solr.apache.org/guide/8_11/the-standard-query-parser.html)
/// # Examples
/// ```no_run
/// # use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
/// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
/// # use solrstice::models::context::SolrServerContextBuilder;
/// # use solrstice::queries::def_type::{DefType, LuceneQuery};
/// # use solrstice::queries::select::SelectQuery;
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// # let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
/// # let client = AsyncSolrCloudClient::new(context);
/// let response = client.select(&SelectQuery::new()
///     .q("outdoors")
///     .def_type(&DefType::Lucene(
///         LuceneQuery::new().df("interests")
///     )), "collection_name")
///     .await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LuceneQuery {
    #[serde(rename = "defType")]
    pub def_type: String,
    #[serde(rename = "q.op", skip_serializing_if = "Option::is_none")]
    pub q_op: Option<QueryOperator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub df: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sow: Option<bool>,
}

impl Default for LuceneQuery {
    fn default() -> Self {
        Self {
            def_type: "lucene".to_string(),
            q_op: None,
            df: None,
            sow: None,
        }
    }
}

impl From<LuceneQuery> for DefType {
    fn from(lucene_query: LuceneQuery) -> Self {
        DefType::Lucene(lucene_query)
    }
}

impl LuceneQuery {
    /// Create a new lucene query
    /// # Examples
    /// ```no_run
    /// # use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # use solrstice::queries::def_type::{DefType, LuceneQuery};
    /// # use solrstice::queries::select::SelectQuery;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// # let client = AsyncSolrCloudClient::new(context);
    /// let response = client.select(&SelectQuery::new()
    ///     .q("outdoors")
    ///     .def_type(&DefType::Lucene(
    ///         LuceneQuery::new().df("interests")
    ///     )), "collection_name")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Which query operator to use. Default is OR, but can be AND
    pub fn q_op(mut self, q_op: QueryOperator) -> Self {
        self.q_op = Some(q_op);
        self
    }

    /// Default searchable field
    pub fn df(mut self, df: &str) -> Self {
        self.df = Some(df.to_string());
        self
    }

    /// Split on whitespace
    pub fn sow(mut self, sow: bool) -> Self {
        self.sow = Some(sow);
        self
    }
}

/// The dismax query parser
///
/// Documentation can be found at [SolrDocs](https://solr.apache.org/guide/8_11/the-dismax-query-parser.html)
/// # Examples
/// ```no_run
/// # use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
/// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
/// # use solrstice::models::context::SolrServerContextBuilder;
/// # use solrstice::queries::def_type::{DefType, DismaxQuery};
/// # use solrstice::queries::select::SelectQuery;
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// # let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
/// # let client = AsyncSolrCloudClient::new(context);
/// let response = client.select(&SelectQuery::new()
///     .q("outdoors")
///     .def_type(&DefType::Dismax(
///         DismaxQuery::new().qf("interests^20").bq(&["interests:cars^20"]),
///     )), "collection_name")
///     .await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DismaxQuery {
    #[serde(rename = "defType")]
    pub def_type: String,
    #[serde(rename = "q.alt", skip_serializing_if = "Option::is_none")]
    pub q_alt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qf: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mm: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pf: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ps: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tie: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bq: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bf: Option<Vec<String>>,
}

impl Default for DismaxQuery {
    fn default() -> Self {
        Self {
            def_type: "dismax".to_string(),
            q_alt: None,
            qf: None,
            mm: None,
            pf: None,
            ps: None,
            qs: None,
            tie: None,
            bq: None,
            bf: None,
        }
    }
}

impl From<DismaxQuery> for DefType {
    fn from(dismax_query: DismaxQuery) -> Self {
        DefType::Dismax(dismax_query)
    }
}

impl DismaxQuery {
    /// Create a new dismax query
    ///
    /// ```no_run
    /// # use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
    /// # use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// # use solrstice::models::context::SolrServerContextBuilder;
    /// # use solrstice::queries::def_type::{DefType, DismaxQuery};
    /// # use solrstice::queries::select::SelectQuery;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// # let client = AsyncSolrCloudClient::new(context);
    /// let response = client.select(&SelectQuery::new()
    ///     .q("outdoors")
    ///     .def_type(&DefType::Dismax(
    ///         DismaxQuery::new().qf("interests^20").bq(&["interests:cars^20"]),
    ///     )), "collection_name")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Alternate query
    pub fn q_alt(mut self, q_alt: &str) -> Self {
        self.q_alt = Some(q_alt.to_string());
        self
    }

    /// Query fields
    pub fn qf(mut self, qf: &str) -> Self {
        self.qf = Some(qf.to_string());
        self
    }

    /// Minimum match
    pub fn mm(mut self, mm: &str) -> Self {
        self.mm = Some(mm.to_string());
        self
    }

    /// Phrase fields
    pub fn pf(mut self, pf: &str) -> Self {
        self.pf = Some(pf.to_string());
        self
    }

    /// Phrase slop
    pub fn ps(mut self, ps: &str) -> Self {
        self.ps = Some(ps.to_string());
        self
    }

    /// Query slop
    pub fn qs(mut self, qs: &str) -> Self {
        self.qs = Some(qs.to_string());
        self
    }

    /// Tie breaker
    pub fn tie(mut self, tie: &str) -> Self {
        self.tie = Some(tie.to_string());
        self
    }

    /// Boost query
    pub fn bq(mut self, bq: &[&str]) -> Self {
        self.bq = Some(bq.iter().map(|s| s.to_string()).collect());
        self
    }

    /// Boost functions
    pub fn bf(mut self, bf: &[&str]) -> Self {
        self.bf = Some(bf.iter().map(|s| s.to_string()).collect());
        self
    }
}

/// The extended dismax query parser
///
/// Documentation can be found at [SolrDocs](https://solr.apache.org/guide/8_11/the-extended-dismax-query-parser.html)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct EdismaxQuery {
    #[serde(rename = "defType")]
    pub def_type: String,
    #[serde(rename = "q.alt", skip_serializing_if = "Option::is_none")]
    pub q_alt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qf: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mm: Option<String>,
    #[serde(rename = "mm.autoRelax", skip_serializing_if = "Option::is_none")]
    pub mm_auto_relax: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pf: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pf2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pf3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ps: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ps2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ps3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tie: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bq: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bf: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sow: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lowercase_operators: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopwords: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uf: Option<String>,
}

impl Default for EdismaxQuery {
    fn default() -> Self {
        Self {
            def_type: "edismax".to_string(),
            q_alt: None,
            qf: None,
            mm: None,
            mm_auto_relax: None,
            pf: None,
            pf2: None,
            pf3: None,
            ps: None,
            ps2: None,
            ps3: None,
            qs: None,
            tie: None,
            bq: None,
            bf: None,
            sow: None,
            boost: None,
            lowercase_operators: None,
            stopwords: None,
            uf: None,
        }
    }
}

impl From<EdismaxQuery> for DefType {
    fn from(edismax_query: EdismaxQuery) -> Self {
        DefType::Edismax(edismax_query)
    }
}

impl EdismaxQuery {
    pub fn new() -> Self {
        Self::default()
    }

    /// Alternate query
    pub fn q_alt(mut self, q_alt: &str) -> Self {
        self.q_alt = Some(q_alt.to_string());
        self
    }

    /// Query fields
    pub fn qf(mut self, qf: &str) -> Self {
        self.qf = Some(qf.to_string());
        self
    }

    /// Minimum match
    pub fn mm(mut self, mm: &str) -> Self {
        self.mm = Some(mm.to_string());
        self
    }

    /// Minimum match auto relax
    pub fn mm_auto_relax(mut self, mm_auto_relax: bool) -> Self {
        self.mm_auto_relax = Some(mm_auto_relax);
        self
    }

    /// Phrase fields
    pub fn pf(mut self, pf: &str) -> Self {
        self.pf = Some(pf.to_string());
        self
    }

    /// Phrase fields 2
    pub fn pf2(mut self, pf2: &str) -> Self {
        self.pf2 = Some(pf2.to_string());
        self
    }

    /// Phrase fields 3
    pub fn pf3(mut self, pf3: &str) -> Self {
        self.pf3 = Some(pf3.to_string());
        self
    }

    /// Phrase slop
    pub fn ps(mut self, ps: &str) -> Self {
        self.ps = Some(ps.to_string());
        self
    }

    /// Phrase slop 2
    pub fn ps2(mut self, ps2: &str) -> Self {
        self.ps2 = Some(ps2.to_string());
        self
    }

    /// Phrase slop 3
    pub fn ps3(mut self, ps3: &str) -> Self {
        self.ps3 = Some(ps3.to_string());
        self
    }

    /// Query slop
    pub fn qs(mut self, qs: &str) -> Self {
        self.qs = Some(qs.to_string());
        self
    }

    /// Tie breaker
    pub fn tie(mut self, tie: &str) -> Self {
        self.tie = Some(tie.to_string());
        self
    }

    /// Boost query
    pub fn bq(mut self, bq: &[&str]) -> Self {
        self.bq = Some(bq.iter().map(|s| s.to_string()).collect());
        self
    }

    /// Boost functions
    pub fn bf(mut self, bf: &[&str]) -> Self {
        self.bf = Some(bf.iter().map(|s| s.to_string()).collect());
        self
    }

    /// Split on whitespace
    pub fn sow(mut self, sow: bool) -> Self {
        self.sow = Some(sow);
        self
    }

    /// Boost
    pub fn boost(mut self, boost: &[&str]) -> Self {
        self.boost = Some(boost.iter().map(|s| s.to_string()).collect());
        self
    }

    /// Lowercase operators
    pub fn lowercase_operators(mut self, lowercase_operators: bool) -> Self {
        self.lowercase_operators = Some(lowercase_operators);
        self
    }

    /// Stopwords
    pub fn stopwords(mut self, stopwords: bool) -> Self {
        self.stopwords = Some(stopwords);
        self
    }

    /// User fields
    pub fn uf(mut self, uf: &str) -> Self {
        self.uf = Some(uf.to_string());
        self
    }
}
