use crate::models::context::SolrServerContext;
use crate::models::error::{try_solr_error, SolrError};
use crate::models::response::SolrResponse;
use reqwest::{Body, RequestBuilder, Response};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
enum SolrRequestType {
    Get,
    Post,
}

pub struct SolrRequestBuilder<'a> {
    context: &'a SolrServerContext,
    url: &'a str,
    query_params: Option<&'a [(&'a str, &'a str)]>,
    headers: Option<Vec<(String, String)>>,
}

impl<'a> SolrRequestBuilder<'a> {
    pub fn new(context: &'a SolrServerContext, url: &'a str) -> Self {
        Self {
            context,
            url,
            query_params: None,
            headers: None,
        }
    }

    pub fn with_query_params(mut self, query_params: &'a [(&'a str, &'a str)]) -> Self {
        self.query_params = Some(query_params);
        self
    }

    pub fn with_headers<S: Into<String>, I: IntoIterator<Item = (S, S)>>(
        mut self,
        headers: I,
    ) -> Self {
        self.headers = Some(
            headers
                .into_iter()
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
        );
        self
    }

    pub async fn send_get(self) -> Result<SolrResponse, SolrError> {
        let request = create_standard_request(
            self.context,
            self.url,
            SolrRequestType::Get,
            self.query_params,
            self.headers.as_ref(),
        )
        .await?;
        let response = request.send().await?;
        try_request_auth_error(&response).await?;
        let solr_response = response.json::<SolrResponse>().await?;
        try_solr_error(&solr_response)?;
        Ok(solr_response)
    }

    pub async fn send_post_with_json<T: Serialize + 'a + ?Sized>(
        self,
        json: &T,
    ) -> Result<SolrResponse, SolrError> {
        let mut request = create_standard_request(
            self.context,
            self.url,
            SolrRequestType::Post,
            self.query_params,
            self.headers.as_ref(),
        )
        .await?;
        request = request.json(&json);
        let response = request.send().await?;
        try_request_auth_error(&response).await?;
        let solr_response = response.json::<SolrResponse>().await?;
        try_solr_error(&solr_response)?;
        Ok(solr_response)
    }

    pub async fn send_post_with_body<T: Into<Body>>(
        self,
        data: T,
    ) -> Result<SolrResponse, SolrError> {
        let mut request = create_standard_request(
            self.context,
            self.url,
            SolrRequestType::Post,
            self.query_params,
            self.headers.as_ref(),
        )
        .await?;
        request = request.body(data.into());
        let response = request.send().await?;
        try_request_auth_error(&response).await?;
        let solr_response = response.json::<SolrResponse>().await?;
        try_solr_error(&solr_response)?;
        Ok(solr_response)
    }
}

async fn create_standard_request<'a>(
    context: &'a SolrServerContext,
    url: &'a str,
    request_type: SolrRequestType,
    query_params: Option<&'a [(&'a str, &'a str)]>,
    headers: Option<&Vec<(String, String)>>,
) -> Result<RequestBuilder, SolrError> {
    let mut request = match request_type {
        SolrRequestType::Get => {
            context
                .client
                .get(format!("{}{}", context.host.get_solr_node().await?, url))
        }
        SolrRequestType::Post => {
            context
                .client
                .post(format!("{}{}", context.host.get_solr_node().await?, url))
        }
    };
    if let Some(query_params) = query_params {
        request = request.query(query_params);
    }
    request = request.query(&[("wt", "json")]);
    if let Some(headers) = headers {
        for (key, value) in headers {
            request = request.header(key, value);
        }
    }
    if let Some(auth) = context.auth.as_ref() {
        request = auth.add_auth_to_request(request);
    }
    Ok(request)
}

async fn try_request_auth_error(response: &Response) -> Result<(), SolrError> {
    match response.error_for_status_ref() {
        Ok(_) => Ok(()),
        Err(e) => {
            if e.status().ok_or(SolrError::Unknown(
                "Error while getting response code from request".to_string(),
            ))? == 401
            {
                Err(SolrError::SolrAuthError(
                    "Authentication failed with 401. Check credentials.".to_string(),
                ))
            } else {
                Ok(())
            }
        }
    }
}
