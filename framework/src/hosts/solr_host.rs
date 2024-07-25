use crate::error::Error;
use async_trait::async_trait;
use dyn_clone::DynClone;
use std::borrow::Cow;

/// SolrHost specifies how to connect to a solr server.
#[async_trait]
pub trait SolrHost: DynClone {
    async fn get_solr_node(&self) -> Result<Cow<str>, Error>;
}
dyn_clone::clone_trait_object!(SolrHost);
