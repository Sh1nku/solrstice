use crate::models::context::SolrServerContext;
use crate::models::error::SolrError;
use crate::queries::helpers::basic_solr_request;

pub async fn create_collection(
    builder: &SolrServerContext,
    name: &str,
    config: &str,
    shards: usize,
    replication_factor: usize,
) -> Result<(), SolrError> {
    let query_params = [
        ("action", "CREATE"),
        ("wt", "json"),
        ("name", name),
        ("numShards", &shards.to_string()),
        ("replicationFactor", &replication_factor.to_string()),
        ("collection.configName", config),
    ];
    basic_solr_request(builder, "/solr/admin/collections", query_params.as_ref()).await?;
    Ok(())
}

pub async fn get_collections(builder: &SolrServerContext) -> Result<Vec<String>, SolrError> {
    let query_params = [("action", "LIST"), ("wt", "json")];
    let json = basic_solr_request(
        &builder,
        &format!("/solr/admin/collections"),
        query_params.as_ref(),
    )
    .await?;
    match json.collections {
        None => Err(SolrError::Unknown("Could not get collections".to_string())),
        Some(collections) => Ok(collections),
    }
}

pub async fn collection_exists(builder: &SolrServerContext, name: &str) -> Result<bool, SolrError> {
    let collections = get_collections(builder).await?;
    Ok(collections.contains(&name.to_string()))
}

pub async fn delete_collection(builder: &SolrServerContext, name: &str) -> Result<(), SolrError> {
    let query_params = [("action", "DELETE"), ("name", name)];
    basic_solr_request(
        builder,
        &format!("/solr/admin/collections"),
        query_params.as_ref(),
    )
    .await?;
    Ok(())
}

#[cfg(feature = "blocking")]
use crate::runtime::RUNTIME;
#[cfg(feature = "blocking")]
pub fn create_collection_blocking(
    builder: &SolrServerContext,
    name: &str,
    config: &str,
    shards: usize,
    replication_factor: usize,
) -> Result<(), SolrError> {
    RUNTIME.handle().block_on(create_collection(
        builder,
        name,
        config,
        shards,
        replication_factor,
    ))
}

#[cfg(feature = "blocking")]
pub fn get_collections_blocking(builder: &SolrServerContext) -> Result<Vec<String>, SolrError> {
    RUNTIME.handle().block_on(get_collections(builder))
}

#[cfg(feature = "blocking")]
pub fn collection_exists_blocking(
    builder: &SolrServerContext,
    name: &str,
) -> Result<bool, SolrError> {
    RUNTIME.handle().block_on(collection_exists(builder, name))
}

#[cfg(feature = "blocking")]
pub fn delete_collection_blocking(
    builder: &SolrServerContext,
    name: &str,
) -> Result<(), SolrError> {
    RUNTIME.handle().block_on(delete_collection(builder, name))
}
