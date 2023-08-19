use crate::hosts::solr_host::SolrHost;
use crate::models::auth::SolrAuth;
use std::sync::Arc;

/// A SolrServerContext specifies how to connect to a solr server, and how to authenticate.
/// # Examples
/// ```
/// use solrstice::models::context::SolrServerContextBuilder;
/// use solrstice::hosts::solr_host::SolrHost;
/// use solrstice::hosts::solr_server_host::SolrSingleServerHost;
/// use solrstice::models::auth::SolrBasicAuth;
///
/// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983"))
///    .with_auth(SolrBasicAuth::new("solr", Some("SolrRocks")))
///    .build();
/// ```
#[derive(Clone)]
pub struct SolrServerContextBuilder {
    pub(crate) host: Arc<dyn SolrHost + Send + Sync>,
    pub(crate) auth: Option<Arc<dyn SolrAuth + Send + Sync>>,
    pub(crate) client: Option<reqwest::Client>,
}

impl SolrServerContextBuilder {
    /// Create a new SolrServerContextBuilder
    /// # Examples
    /// ```no_run
    /// use solrstice::models::context::SolrServerContextBuilder;
    /// use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// ```
    pub fn new<A: SolrHost + Send + Sync + 'static>(host: A) -> Self {
        Self {
            host: Arc::new(host),
            auth: None,
            client: None,
        }
    }

    /// Create a new SolrServerContextBuilder
    /// # Examples
    /// ```no_run
    /// use solrstice::models::context::SolrServerContextBuilder;
    /// use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// use solrstice::models::auth::SolrBasicAuth;
    ///
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983"))
    ///     .with_auth(SolrBasicAuth::new("username", Some("password"))).build();
    /// ```
    pub fn with_auth(mut self, auth: impl SolrAuth + Send + Sync + 'static) -> Self {
        self.auth = Some(Arc::new(auth));
        self
    }

    /// Use a custom reqwest client
    /// # Examples
    /// ```
    /// use std::time::Duration;
    /// use solrstice::models::context::SolrServerContextBuilder;
    /// use reqwest::Client;
    /// use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    ///
    /// let client = Client::builder().timeout(Duration::from_secs(10)).build().unwrap();
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).with_client(client).build();
    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    /// Build a SolrServerContext
    /// # Examples
    /// ```no_run
    /// use solrstice::models::context::SolrServerContextBuilder;
    /// use solrstice::hosts::solr_server_host::SolrSingleServerHost;
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// ```
    pub fn build(self) -> SolrServerContext {
        self.into()
    }
}

/// A SolrServerContext specifies how to connect to a solr server, and how to authenticate.
/// # Examples
/// ```
/// use solrstice::models::context::SolrServerContextBuilder;
/// use solrstice::hosts::solr_host::SolrHost;
/// use solrstice::hosts::solr_server_host::SolrSingleServerHost;
/// use solrstice::models::auth::SolrBasicAuth;
///
/// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983"))
///    .with_auth(SolrBasicAuth::new("solr", Some("SolrRocks")))
///    .build();
/// ```
/// The SolrServerContext is used to create a SolrClient
///
/// Take a look at [SolrServerContextBuilder](crate::models::context::SolrServerContextBuilder) for more information
#[derive(Clone)]
pub struct SolrServerContext {
    pub(crate) host: Arc<dyn SolrHost + Send + Sync>,
    pub(crate) auth: Option<Arc<dyn SolrAuth + Send + Sync>>,
    pub(crate) client: reqwest::Client,
}

impl From<SolrServerContextBuilder> for SolrServerContext {
    fn from(builder: SolrServerContextBuilder) -> Self {
        Self {
            host: builder.host,
            auth: builder.auth,
            client: builder.client.unwrap_or_else(reqwest::Client::new),
        }
    }
}

impl From<&SolrServerContext> for SolrServerContext {
    fn from(context: &SolrServerContext) -> Self {
        context.clone()
    }
}

impl AsRef<SolrServerContext> for SolrServerContext {
    fn as_ref(&self) -> &SolrServerContext {
        self
    }
}
