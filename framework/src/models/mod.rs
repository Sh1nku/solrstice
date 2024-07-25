//! Models used by the Solr Client.

/// All authentication types supported by the library.
pub(crate) mod auth;
/// Commit types for Solr's update and delete queries.
pub(crate) mod commit_type;
/// Context for the solr Client. Specifying how to connect.
pub(crate) mod context;
/// Facet
pub(crate) mod facet_set;
pub use facet_set::*;
/// Models used by the GroupingComponent.
pub(crate) mod group;
pub use group::*;

/// Facets returned by JSON faceting.
pub(crate) mod json_facet;
pub use json_facet::*;
/// Models used to get responses from Solr
pub(crate) mod response;
pub use response::*;
