//! Models used by the Solr Client.

/// All authentication types supported by the library.
pub mod auth;
/// Commit types for Solr's update and delete queries.
pub mod commit_type;
/// Context for the solr Client. Specifying how to connect.
pub mod context;
/// Error types for the library.
pub mod error;
/// Facet
pub mod facetset;
/// Models used by the GroupingComponent.
pub mod group;
/// Facets returned by JSON faceting.
pub mod json_facet;
/// Models used to get responses from Solr
pub mod response;
