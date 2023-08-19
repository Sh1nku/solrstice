use crate::models::context::SolrServerContext;
use crate::models::error::{try_solr_error, SolrError};
use crate::models::response::SolrResponse;

pub async fn basic_solr_request<C: AsRef<SolrServerContext>, S: AsRef<str>>(
    context: C,
    url: S,
    query_params: &[(&str, &str)],
) -> Result<SolrResponse, SolrError> {
    let mut request = context
        .as_ref()
        .client
        .get(format!(
            "{}{}",
            &context.as_ref().host.get_solr_node().await?,
            url.as_ref()
        ))
        .query(query_params);
    if let Some(auth) = &context.as_ref().auth {
        request = auth.add_auth_to_request(request);
    }
    let solr_response = request.send().await?.json::<SolrResponse>().await?;
    try_solr_error(&solr_response)?;
    Ok(solr_response)
}
