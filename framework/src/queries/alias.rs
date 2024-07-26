use crate::error::Error;
use crate::models::context::SolrServerContext;
use crate::queries::request_builder::SolrRequestBuilder;
use std::collections::HashMap;

/// Get aliases from the Solr server.
///
/// This is not meant to be used directly, but rather as part of a client.
/// Example usage can be found at [AsyncSolrCloudClient::get_aliases](crate::clients::async_cloud_client::AsyncSolrCloudClient::get_aliases)
pub async fn get_aliases<C: AsRef<SolrServerContext>>(
    context: C,
) -> Result<HashMap<String, Vec<String>>, Error> {
    let json = SolrRequestBuilder::new(context.as_ref(), "/solr/admin/collections")
        .with_query_params(&[("action", "LISTALIASES")])
        .send_get()
        .await?;
    match json.aliases {
        None => Err(Error::Unknown(
            "Could not find alias key in map".to_string(),
        )),
        Some(aliases) => Ok(aliases),
    }
}

/// Create an alias with the given name pointing to a list of collections.
///
/// This is not meant to be used directly, but rather as part of a client.
/// Example usage can be found at [AsyncSolrCloudClient::create_alias](crate::clients::async_cloud_client::AsyncSolrCloudClient::create_alias)
pub async fn create_alias<C: AsRef<SolrServerContext>, S: AsRef<str>>(
    context: C,
    name: S,
    collections: &[S],
) -> Result<(), Error> {
    let collections = collections
        .iter()
        .map(|x| x.as_ref())
        .collect::<Vec<&str>>()
        .join(",");
    let query_params = [
        ("action", "CREATEALIAS"),
        ("name", name.as_ref()),
        ("collections", collections.as_str()),
    ];
    SolrRequestBuilder::new(context.as_ref(), "/solr/admin/collections")
        .with_query_params(query_params.as_ref())
        .send_get()
        .await?;
    Ok(())
}

/// Check if an alias with the given name exists.
///
/// This is not meant to be used directly, but rather as part of a client.
/// Example usage can be found at [AsyncSolrCloudClient::alias_exists](crate::clients::async_cloud_client::AsyncSolrCloudClient::alias_exists)
pub async fn alias_exists<C: AsRef<SolrServerContext>, S: AsRef<str>>(
    context: C,
    name: S,
) -> Result<bool, Error> {
    let aliases = get_aliases(context).await?;
    Ok(aliases.contains_key(name.as_ref()))
}

/// Delete an alias with the given name.
///
/// This is not meant to be used directly, but rather as part of a client.
/// Example usage can be found at [AsyncSolrCloudClient::delete_alias](crate::clients::async_cloud_client::AsyncSolrCloudClient::delete_alias)
pub async fn delete_alias<C: AsRef<SolrServerContext>, S: AsRef<str>>(
    context: C,
    name: S,
) -> Result<(), Error> {
    let query_params = [("action", "DELETEALIAS"), ("name", name.as_ref())];
    SolrRequestBuilder::new(context.as_ref(), "/solr/admin/collections")
        .with_query_params(query_params.as_ref())
        .send_get()
        .await?;
    Ok(())
}

#[cfg(feature = "blocking")]
use crate::runtime::RUNTIME;
#[cfg(feature = "blocking")]
/// Get aliases from the Solr server.
///
/// This is not meant to be used directly, but rather as part of a client.
/// Example usage can be found at [BlockingSolrCloudClient::get_aliases](crate::clients::blocking_cloud_client::BlockingSolrCloudClient::get_aliases)
pub fn get_aliases_blocking<C: AsRef<SolrServerContext>>(
    context: C,
) -> Result<HashMap<String, Vec<String>>, Error> {
    RUNTIME.handle().block_on(get_aliases(context))
}

#[cfg(feature = "blocking")]
/// Create an alias with the given name pointing to a list of collections.
///
/// This is not meant to be used directly, but rather as part of a client.
/// Example usage can be found at [BlockingSolrCloudClient::create_alias](crate::clients::blocking_cloud_client::BlockingSolrCloudClient::create_alias)
pub fn create_alias_blocking<C: AsRef<SolrServerContext>, S: AsRef<str>>(
    context: C,
    name: S,
    collections: &[S],
) -> Result<(), Error> {
    RUNTIME
        .handle()
        .block_on(create_alias(context, name, collections))
}

#[cfg(feature = "blocking")]
/// Check if an alias with the given name exists.
///
/// This is not meant to be used directly, but rather as part of a client.
/// Example usage can be found at [BlockingSolrCloudClient::alias_exists](crate::clients::blocking_cloud_client::BlockingSolrCloudClient::alias_exists)
pub fn alias_exists_blocking<C: AsRef<SolrServerContext>, S: AsRef<str>>(
    context: C,
    name: S,
) -> Result<bool, Error> {
    RUNTIME.handle().block_on(alias_exists(context, name))
}

#[cfg(feature = "blocking")]
/// Delete an alias with the given name.
///
/// This is not meant to be used directly, but rather as part of a client.
/// Example usage can be found at [BlockingSolrCloudClient::delete_alias](crate::clients::blocking_cloud_client::BlockingSolrCloudClient::delete_alias)
pub fn delete_alias_blocking<C: AsRef<SolrServerContext>, S: AsRef<str>>(
    context: C,
    name: S,
) -> Result<(), Error> {
    RUNTIME.handle().block_on(delete_alias(context, name))
}
