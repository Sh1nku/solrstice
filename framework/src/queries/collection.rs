use crate::error::Error;
use crate::models::context::SolrServerContext;
use crate::queries::request_builder::SolrRequestBuilder;

pub async fn create_collection<C: AsRef<SolrServerContext>, S: AsRef<str>>(
    context: C,
    name: S,
    config: S,
    shards: usize,
    replication_factor: usize,
) -> Result<(), Error> {
    let query_params = [
        ("action", "CREATE"),
        ("name", name.as_ref()),
        ("numShards", &shards.to_string()),
        ("replicationFactor", &replication_factor.to_string()),
        ("collection.configName", config.as_ref()),
    ];
    SolrRequestBuilder::new(context.as_ref(), "/solr/admin/collections")
        .with_query_params(query_params.as_ref())
        .send_get()
        .await?;
    Ok(())
}

pub async fn get_collections<C: AsRef<SolrServerContext>>(
    context: C,
) -> Result<Vec<String>, Error> {
    let query_params = [("action", "LIST"), ("wt", "json")];
    let json = SolrRequestBuilder::new(context.as_ref(), "/solr/admin/collections")
        .with_query_params(query_params.as_ref())
        .send_get()
        .await?;
    match json.collections {
        None => Err(Error::Unknown("Could not get collections".to_string())),
        Some(collections) => Ok(collections),
    }
}

pub async fn collection_exists<C: AsRef<SolrServerContext>, S: AsRef<str>>(
    context: C,
    name: S,
) -> Result<bool, Error> {
    let collections = get_collections(context).await?;
    Ok(collections.contains(&name.as_ref().to_string()))
}

pub async fn delete_collection<C: AsRef<SolrServerContext>, S: AsRef<str>>(
    context: C,
    name: S,
) -> Result<(), Error> {
    let query_params = [("action", "DELETE"), ("name", name.as_ref())];
    SolrRequestBuilder::new(context.as_ref(), "/solr/admin/collections")
        .with_query_params(query_params.as_ref())
        .send_get()
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
) -> Result<(), Error> {
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
) -> Result<Vec<String>, Error> {
    RUNTIME.handle().block_on(get_collections(context))
}

#[cfg(feature = "blocking")]
pub fn collection_exists_blocking<C: AsRef<SolrServerContext>, S: AsRef<str>>(
    context: C,
    name: S,
) -> Result<bool, Error> {
    RUNTIME.handle().block_on(collection_exists(context, name))
}

#[cfg(feature = "blocking")]
pub fn delete_collection_blocking<C: AsRef<SolrServerContext>, S: AsRef<str>>(
    context: C,
    name: S,
) -> Result<(), Error> {
    RUNTIME.handle().block_on(delete_collection(context, name))
}
