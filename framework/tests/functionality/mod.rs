pub mod alias_tests;
pub mod client_tests;
pub mod collection_test;
pub mod config_test;
pub mod def_type_test;
pub mod facetset_test;
pub mod grouping_tests;
pub mod index_test;
pub mod json_facet_test;
pub mod readme_test;
pub mod select_test;
pub mod zk_test;

pub mod auth_test;

pub mod error_test;
pub mod logging_test;

#[cfg(feature = "blocking")]
pub mod blocking_tests;
