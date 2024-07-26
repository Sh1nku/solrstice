//! The different query types this library supports.

/// Alias API
pub mod alias;
/// Collection API
pub mod collection;
/// Components for select queries
pub mod components;
/// Config API
pub mod config;
/// Def types for select queries. Eg: `luscene`, `edismax`
pub(crate) mod def_type;
/// Index and Delete API
pub(crate) mod index;
/// Request builder for queries
pub(crate) mod request_builder;
/// Select query API
pub(crate) mod select;
