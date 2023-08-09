use crate::models::context::SolrServerContext;
use crate::models::error::SolrError;
use crate::queries::helpers::basic_solr_request;
use std::collections::HashMap;

/// Get aliases from the Solr server.
///
/// This is not meant to be used directly, but rather as part of a client.
/// Example usage can be found at [AsyncSolrCloudClient::get_aliases](crate::clients::async_cloud_client::AsyncSolrCloudClient::get_aliases)
pub async fn get_aliases(
    context: &SolrServerContext,
) -> Result<HashMap<String, Vec<String>>, SolrError> {
    let query_params = [("action", "LISTALIASES"), ("wt", "json")];
    let json =
        basic_solr_request(context, "/solr/admin/collections", query_params.as_ref()).await?;
    match json.aliases {
        None => Err(SolrError::Unknown(
            "Could not find alias key in map".to_string(),
        )),
        Some(aliases) => Ok(aliases),
    }
}

/// Create an alias with the given name pointing to a list of collections.
///
/// This is not meant to be used directly, but rather as part of a client.
/// Example usage can be found at [AsyncSolrCloudClient::create_alias](crate::clients::async_cloud_client::AsyncSolrCloudClient::create_alias)
pub async fn create_alias(
    context: &SolrServerContext,
    name: &str,
    collections: &[&str],
) -> Result<(), SolrError> {
    let collections = collections.join(",");
    let query_params = [
        ("action", "CREATEALIAS"),
        ("name", name),
        ("collections", collections.as_str()),
    ];
    basic_solr_request(context, "/solr/admin/collections", query_params.as_ref()).await?;
    Ok(())
}

/// Check if an alias with the given name exists.
///
/// This is not meant to be used directly, but rather as part of a client.
/// Example usage can be found at [AsyncSolrCloudClient::alias_exists](crate::clients::async_cloud_client::AsyncSolrCloudClient::alias_exists)
pub async fn alias_exists(context: &SolrServerContext, name: &str) -> Result<bool, SolrError> {
    let aliases = get_aliases(context).await?;
    Ok(aliases.contains_key(&name.to_string()))
}

/// Delete an alias with the given name.
///
/// This is not meant to be used directly, but rather as part of a client.
/// Example usage can be found at [AsyncSolrCloudClient::delete_alias](crate::clients::async_cloud_client::AsyncSolrCloudClient::delete_alias)
pub async fn delete_alias(context: &SolrServerContext, name: &str) -> Result<(), SolrError> {
    let query_params = [("action", "DELETEALIAS"), ("name", name), ("wt", "json")];
    basic_solr_request(context, "/solr/admin/collections", query_params.as_ref()).await?;
    Ok(())
}

#[cfg(feature = "blocking")]
use crate::runtime::RUNTIME;
#[cfg(feature = "blocking")]
/// Get aliases from the Solr server.
///
/// This is not meant to be used directly, but rather as part of a client.
/// Example usage can be found at [BlockingSolrCloudClient::get_aliases](crate::clients::blocking_cloud_client::BlockingSolrCloudClient::get_aliases)
pub fn get_aliases_blocking(
    context: &SolrServerContext,
) -> Result<HashMap<String, Vec<String>>, SolrError> {
    RUNTIME.handle().block_on(get_aliases(context))
}

#[cfg(feature = "blocking")]
/// Create an alias with the given name pointing to a list of collections.
///
/// This is not meant to be used directly, but rather as part of a client.
/// Example usage can be found at [BlockingSolrCloudClient::create_alias](crate::clients::blocking_cloud_client::BlockingSolrCloudClient::create_alias)
pub fn create_alias_blocking(
    context: &SolrServerContext,
    name: &str,
    collections: &[&str],
) -> Result<(), SolrError> {
    RUNTIME
        .handle()
        .block_on(create_alias(context, name, collections))
}

#[cfg(feature = "blocking")]
/// Check if an alias with the given name exists.
///
/// This is not meant to be used directly, but rather as part of a client.
/// Example usage can be found at [BlockingSolrCloudClient::alias_exists](crate::clients::blocking_cloud_client::BlockingSolrCloudClient::alias_exists)
pub fn alias_exists_blocking(context: &SolrServerContext, name: &str) -> Result<bool, SolrError> {
    RUNTIME.handle().block_on(alias_exists(context, name))
}

#[cfg(feature = "blocking")]
/// Delete an alias with the given name.
///
/// This is not meant to be used directly, but rather as part of a client.
/// Example usage can be found at [BlockingSolrCloudClient::delete_alias](crate::clients::blocking_cloud_client::BlockingSolrCloudClient::delete_alias)
pub fn delete_alias_blocking(context: &SolrServerContext, name: &str) -> Result<(), SolrError> {
    RUNTIME.handle().block_on(delete_alias(context, name))
}
