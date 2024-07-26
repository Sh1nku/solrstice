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
/// # use solrstice::{AsyncSolrCloudClient, SolrSingleServerHost};
/// # use solrstice::SolrServerContextBuilder;
/// # use solrstice::{DefType, LuceneQuery};
/// # use solrstice::SelectQuery;
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
    /// # use solrstice::{AsyncSolrCloudClient, SolrSingleServerHost};
    /// # use solrstice::SolrServerContextBuilder;
    /// # use solrstice::{DefType, LuceneQuery};
    /// # use solrstice::SelectQuery;
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
    pub fn q_op<Q: Into<QueryOperator>, O: Into<Option<Q>>>(mut self, q_op: O) -> Self {
        self.q_op = q_op.into().map(|q| q.into());
        self
    }

    /// Default searchable field
    pub fn df<S: Into<String>, O: Into<Option<S>>>(mut self, df: O) -> Self {
        self.df = df.into().map(|s| s.into());
        self
    }

    /// Split on whitespace
    pub fn sow<O: Into<Option<bool>>>(mut self, sow: O) -> Self {
        self.sow = sow.into();
        self
    }
}

/// The dismax query parser
///
/// Documentation can be found at [SolrDocs](https://solr.apache.org/guide/8_11/the-dismax-query-parser.html)
/// # Examples
/// ```no_run
/// # use solrstice::{AsyncSolrCloudClient, SolrSingleServerHost};
/// # use solrstice::SolrServerContextBuilder;
/// # use solrstice::{DefType, DismaxQuery};
/// # use solrstice::SelectQuery;
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// # let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
/// # let client = AsyncSolrCloudClient::new(context);
/// let response = client.select(&SelectQuery::new()
///     .q("outdoors")
///     .def_type(&DefType::Dismax(
///         DismaxQuery::new().qf("interests^20").bq(["interests:cars^20"]),
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
    /// # use solrstice::{AsyncSolrCloudClient, SolrSingleServerHost};
    /// # use solrstice::SolrServerContextBuilder;
    /// # use solrstice::{DefType, DismaxQuery};
    /// # use solrstice::SelectQuery;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// # let client = AsyncSolrCloudClient::new(context);
    /// let response = client.select(&SelectQuery::new()
    ///     .q("outdoors")
    ///     .def_type(&DefType::Dismax(
    ///         DismaxQuery::new().qf("interests^20").bq(["interests:cars^20"]),
    ///     )), "collection_name")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Alternate query
    pub fn q_alt<S: Into<String>, O: Into<Option<S>>>(mut self, q_alt: O) -> Self {
        self.q_alt = q_alt.into().map(|s| s.into());
        self
    }

    /// Query fields
    pub fn qf<S: Into<String>, O: Into<Option<S>>>(mut self, qf: O) -> Self {
        self.qf = qf.into().map(|s| s.into());
        self
    }

    /// Minimum match
    pub fn mm<S: Into<String>, O: Into<Option<S>>>(mut self, mm: O) -> Self {
        self.mm = mm.into().map(|s| s.into());
        self
    }

    /// Phrase fields
    pub fn pf<S: Into<String>, O: Into<Option<S>>>(mut self, pf: O) -> Self {
        self.pf = pf.into().map(|s| s.into());
        self
    }

    /// Phrase slop
    pub fn ps<S: Into<String>, O: Into<Option<S>>>(mut self, ps: O) -> Self {
        self.ps = ps.into().map(|s| s.into());
        self
    }

    /// Query slop
    pub fn qs<S: Into<String>, O: Into<Option<S>>>(mut self, qs: O) -> Self {
        self.qs = qs.into().map(|s| s.into());
        self
    }

    /// Tie breaker
    pub fn tie<S: Into<String>, O: Into<Option<S>>>(mut self, tie: O) -> Self {
        self.tie = tie.into().map(|s| s.into());
        self
    }

    /// Boost query
    pub fn bq<S: Into<String>, V: Into<Vec<S>>, O: Into<Option<V>>>(mut self, bq: O) -> Self {
        self.bq = bq
            .into()
            .map(|v| v.into().into_iter().map(|s| s.into()).collect());
        self
    }

    /// Boost functions
    pub fn bf<S: Into<String>, V: Into<Vec<S>>, O: Into<Option<V>>>(mut self, bf: O) -> Self {
        self.bf = bf
            .into()
            .map(|v| v.into().into_iter().map(|s| s.into()).collect());
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
    pub fn q_alt<S: Into<String>, O: Into<Option<S>>>(mut self, q_alt: O) -> Self {
        self.q_alt = q_alt.into().map(|s| s.into());
        self
    }

    /// Query fields
    pub fn qf<S: Into<String>, O: Into<Option<S>>>(mut self, qf: O) -> Self {
        self.qf = qf.into().map(|s| s.into());
        self
    }

    /// Minimum match
    pub fn mm<S: Into<String>, O: Into<Option<S>>>(mut self, mm: O) -> Self {
        self.mm = mm.into().map(|s| s.into());
        self
    }

    /// Minimum match auto relax
    pub fn mm_auto_relax<O: Into<Option<bool>>>(mut self, mm_auto_relax: O) -> Self {
        self.mm_auto_relax = mm_auto_relax.into();
        self
    }

    /// Phrase fields
    pub fn pf<S: Into<String>, O: Into<Option<S>>>(mut self, pf: O) -> Self {
        self.pf = pf.into().map(|s| s.into());
        self
    }

    /// Phrase fields 2
    pub fn pf2<S: Into<String>, O: Into<Option<S>>>(mut self, pf2: O) -> Self {
        self.pf2 = pf2.into().map(|s| s.into());
        self
    }

    /// Phrase fields 3
    pub fn pf3<S: Into<String>, O: Into<Option<S>>>(mut self, pf3: O) -> Self {
        self.pf3 = pf3.into().map(|s| s.into());
        self
    }

    /// Phrase slop
    pub fn ps<S: Into<String>, O: Into<Option<S>>>(mut self, ps: O) -> Self {
        self.ps = ps.into().map(|s| s.into());
        self
    }

    /// Phrase slop 2
    pub fn ps2<S: Into<String>, O: Into<Option<S>>>(mut self, ps2: O) -> Self {
        self.ps2 = ps2.into().map(|s| s.into());
        self
    }

    /// Phrase slop 3
    pub fn ps3<S: Into<String>, O: Into<Option<S>>>(mut self, ps3: O) -> Self {
        self.ps3 = ps3.into().map(|s| s.into());
        self
    }

    /// Query slop
    pub fn qs<S: Into<String>, O: Into<Option<S>>>(mut self, qs: O) -> Self {
        self.qs = qs.into().map(|s| s.into());
        self
    }

    /// Tie breaker
    pub fn tie<S: Into<String>, O: Into<Option<S>>>(mut self, tie: O) -> Self {
        self.tie = tie.into().map(|s| s.into());
        self
    }

    /// Boost query
    pub fn bq<S: Into<String>, V: IntoIterator<Item = S>, O: Into<Option<V>>>(
        mut self,
        bq: O,
    ) -> Self {
        self.bq = bq.into().map(|x| x.into_iter().map(|x| x.into()).collect());
        self
    }

    /// Boost functions
    pub fn bf<S: Into<String>, V: IntoIterator<Item = S>, O: Into<Option<V>>>(
        mut self,
        bf: O,
    ) -> Self {
        self.bf = bf.into().map(|x| x.into_iter().map(|x| x.into()).collect());
        self
    }

    /// Split on whitespace
    pub fn sow<O: Into<Option<bool>>>(mut self, sow: O) -> Self {
        self.sow = sow.into();
        self
    }

    /// Boost
    pub fn boost<S: Into<String>, V: IntoIterator<Item = S>, O: Into<Option<V>>>(
        mut self,
        boost: O,
    ) -> Self {
        self.boost = boost
            .into()
            .map(|x| x.into_iter().map(|x| x.into()).collect());
        self
    }

    /// Lowercase operators
    pub fn lowercase_operators<O: Into<Option<bool>>>(mut self, lowercase_operators: O) -> Self {
        self.lowercase_operators = lowercase_operators.into();
        self
    }

    /// Stopwords
    pub fn stopwords<O: Into<Option<bool>>>(mut self, stopwords: O) -> Self {
        self.stopwords = stopwords.into();
        self
    }

    /// User fields
    pub fn uf<S: Into<String>, O: Into<Option<S>>>(mut self, uf: O) -> Self {
        self.uf = uf.into().map(|s| s.into());
        self
    }
}
