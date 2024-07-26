use crate::hosts::solr_host::SolrHost;
use crate::models::auth::SolrAuth;
use crate::queries::request_builder::LoggingPolicy;
use std::sync::Arc;

/// A SolrServerContext specifies how to connect to a solr server, and how to authenticate.
/// # Examples
/// ```
/// use solrstice::{SolrBasicAuth, SolrServerContextBuilder, SolrSingleServerHost};
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
    pub(crate) logging_policy: LoggingPolicy,
}

impl SolrServerContextBuilder {
    /// Create a new SolrServerContextBuilder
    /// # Examples
    /// ```no_run
    /// use solrstice::{SolrServerContextBuilder, SolrSingleServerHost};
    ///
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// ```
    pub fn new<A: SolrHost + Send + Sync + 'static>(host: A) -> Self {
        Self {
            host: Arc::new(host),
            auth: None,
            client: None,
            logging_policy: LoggingPolicy::Fast(512),
        }
    }

    /// Create a new SolrServerContextBuilder
    /// # Examples
    /// ```no_run
    /// use solrstice::{SolrBasicAuth, SolrServerContextBuilder, SolrSingleServerHost};
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
    /// use reqwest::Client;
    /// use solrstice::{SolrServerContextBuilder, SolrSingleServerHost};
    ///
    /// let client = Client::builder().timeout(Duration::from_secs(10)).build().unwrap();
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).with_client(client).build();
    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    /// Set a logging policy
    /// The accepted values are `Logging::Off`, `Logging::Fast(usize)`, `Logging::Pretty(usize)`
    /// `usize` is the maximum length of the body that will be logged in bytes
    /// `Pretty` is expensive as it needs deserialize and reserialize the body a second time.
    /// # Examples
    /// ```
    /// use solrstice::{LoggingPolicy, SolrServerContextBuilder, SolrSingleServerHost};
    ///
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983"))
    ///   .with_logging_policy(LoggingPolicy::Fast(4096))
    ///   .build();
    pub fn with_logging_policy(mut self, logging_policy: LoggingPolicy) -> Self {
        self.logging_policy = logging_policy;
        self
    }

    /// Build a SolrServerContext
    /// # Examples
    /// ```no_run
    /// use solrstice::{SolrServerContextBuilder, SolrSingleServerHost};
    ///
    /// let context = SolrServerContextBuilder::new(SolrSingleServerHost::new("http://localhost:8983")).build();
    /// ```
    pub fn build(self) -> SolrServerContext {
        self.into()
    }
}

/// A SolrServerContext specifies how to connect to a solr server, and how to authenticate.
/// # Examples
/// ```
/// use solrstice::{SolrBasicAuth, SolrServerContextBuilder, SolrSingleServerHost};
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
    pub(crate) logging_policy: LoggingPolicy,
}

impl From<SolrServerContextBuilder> for SolrServerContext {
    fn from(builder: SolrServerContextBuilder) -> Self {
        Self {
            host: builder.host,
            auth: builder.auth,
            client: builder.client.unwrap_or_default(),
            logging_policy: builder.logging_policy,
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
