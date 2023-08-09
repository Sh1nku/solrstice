use crate::models::context::SolrServerContext;
use crate::models::error::{try_solr_error, SolrError};
use crate::models::response::SolrResponse;

pub async fn basic_solr_request(
    builder: &SolrServerContext,
    url: &str,
    query_params: &[(&str, &str)],
) -> Result<SolrResponse, SolrError> {
    let mut request = builder
        .client
        .get(format!("{}{}", &builder.host.get_solr_node().await?, url))
        .query(query_params);
    if let Some(auth) = &builder.auth {
        request = auth.add_auth_to_request(request);
    }
    let solr_response = request.send().await?.json::<SolrResponse>().await?;
    try_solr_error(&solr_response)?;
    Ok(solr_response)
}
