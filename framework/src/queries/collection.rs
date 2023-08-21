use crate::models::context::SolrServerContext;
use crate::models::error::SolrError;
use crate::queries::helpers::basic_solr_request;

pub async fn create_collection<C: AsRef<SolrServerContext>, S: AsRef<str>>(
    context: C,
    name: S,
    config: S,
    shards: usize,
    replication_factor: usize,
) -> Result<(), SolrError> {
    let query_params = [
        ("action", "CREATE"),
        ("wt", "json"),
        ("name", name.as_ref()),
        ("numShards", &shards.to_string()),
        ("replicationFactor", &replication_factor.to_string()),
        ("collection.configName", config.as_ref()),
    ];
    basic_solr_request(context, "/solr/admin/collections", query_params.as_ref()).await?;
    Ok(())
}

pub async fn get_collections<C: AsRef<SolrServerContext>>(
    context: C,
) -> Result<Vec<String>, SolrError> {
    let query_params = [("action", "LIST"), ("wt", "json")];
    let json = basic_solr_request(
        context,
        &format!("/solr/admin/collections"),
        query_params.as_ref(),
    )
    .await?;
    match json.collections {
        None => Err(SolrError::Unknown("Could not get collections".to_string())),
        Some(collections) => Ok(collections),
    }
}

pub async fn collection_exists<C: AsRef<SolrServerContext>, S: AsRef<str>>(
    context: C,
    name: S,
) -> Result<bool, SolrError> {
    let collections = get_collections(context).await?;
    Ok(collections.contains(&name.as_ref().to_string()))
}

pub async fn delete_collection<C: AsRef<SolrServerContext>, S: AsRef<str>>(
    context: C,
    name: S,
) -> Result<(), SolrError> {
    let query_params = [("action", "DELETE"), ("name", name.as_ref())];
    basic_solr_request(
        context,
        &format!("/solr/admin/collections"),
        query_params.as_ref(),
    )
    .await?;
    Ok(())
}

#[cfg(feature = "blocking")]
use crate::runtime::RUNTIME;
#[cfg(feature = "blocking")]
pub fn create_collection_blocking<C: AsRef<SolrServerContext>, S: AsRef<str>>(
    context: C,
    name: S,
    config: S,
    shards: usize,
    replication_factor: usize,
) -> Result<(), SolrError> {
    RUNTIME.handle().block_on(create_collection(
        context,
        name,
        config,
        shards,
        replication_factor,
    ))
}

#[cfg(feature = "blocking")]
pub fn get_collections_blocking<C: AsRef<SolrServerContext>>(
    context: C,
) -> Result<Vec<String>, SolrError> {
    RUNTIME.handle().block_on(get_collections(context))
}

#[cfg(feature = "blocking")]
pub fn collection_exists_blocking<'a, C: AsRef<SolrServerContext>, S: AsRef<str>>(
    context: C,
    name: S,
) -> Result<bool, SolrError> {
    RUNTIME.handle().block_on(collection_exists(context, name))
}

#[cfg(feature = "blocking")]
pub fn delete_collection_blocking<C: AsRef<SolrServerContext>, S: AsRef<str>>(
    context: C,
    name: S,
) -> Result<(), SolrError> {
    RUNTIME.handle().block_on(delete_collection(context, name))
}
