use crate::error::{try_solr_error, Error};
use crate::models::context::SolrServerContext;
use crate::models::response::SolrResponse;
use log::debug;
use reqwest::header::HeaderMap;
use reqwest::{Body, Request, RequestBuilder, Response, Url};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
enum SolrRequestType {
    Get,
    Post,
}

/// How detailed the logs of the requests should be
/// For `Fast` and `Pretty` the number is the maximum length of the body that will be logged
/// Logging will be with the `debug` level
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum LoggingPolicy {
    Off,
    Fast(usize),
    Pretty(usize),
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

    pub async fn send_get(self) -> Result<SolrResponse, Error> {
        let request = create_standard_request(
            self.context,
            self.url,
            SolrRequestType::Get,
            self.query_params,
            self.headers.as_ref(),
        )
        .await?;

        let (client, request) = request.build_split();
        let request = request?;
        log_request_info(&request, self.context.logging_policy);

        let response = client.execute(request).await?;
        try_request_auth_error(&response).await?;
        let solr_response = response.json::<SolrResponse>().await?;
        try_solr_error(&solr_response)?;
        Ok(solr_response)
    }

    pub async fn send_post_with_json<T: Serialize + 'a + ?Sized>(
        self,
        json: &T,
    ) -> Result<SolrResponse, Error> {
        let mut request = create_standard_request(
            self.context,
            self.url,
            SolrRequestType::Post,
            self.query_params,
            self.headers.as_ref(),
        )
        .await?;
        request = request.json(&json);

        let (client, request) = request.build_split();
        let request = request?;
        log_request_info(&request, self.context.logging_policy);

        let response = client.execute(request).await?;
        try_request_auth_error(&response).await?;
        let solr_response = response.json::<SolrResponse>().await?;
        try_solr_error(&solr_response)?;
        Ok(solr_response)
    }

    pub async fn send_post_with_body<T: Into<Body>>(self, data: T) -> Result<SolrResponse, Error> {
        let mut request = create_standard_request(
            self.context,
            self.url,
            SolrRequestType::Post,
            self.query_params,
            self.headers.as_ref(),
        )
        .await?;
        request = request.body(data.into());

        let (client, request) = request.build_split();
        let request = request?;
        log_request_info(&request, self.context.logging_policy);

        let response = client.execute(request).await?;
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
) -> Result<RequestBuilder, Error> {
    let url = format!("{}{}", context.host.get_solr_node().await?, url);
    let mut request = match request_type {
        SolrRequestType::Get => context.client.get(url),
        SolrRequestType::Post => context.client.post(url),
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

async fn try_request_auth_error(response: &Response) -> Result<(), Error> {
    match response.error_for_status_ref() {
        Ok(_) => Ok(()),
        Err(e) => {
            if e.status().ok_or(Error::Unknown(
                "Error while getting response code from request".to_string(),
            ))? == 401
            {
                Err(Error::SolrAuthError(
                    "Authentication failed with 401. Check credentials.".to_string(),
                ))
            } else {
                Ok(())
            }
        }
    }
}

static NO_BODY: &[u8] = "No body".as_bytes();
static ERROR_BODY: &str = "Error while getting body";
fn body_too_long(actual: usize, max: usize) -> String {
    format!("Too long {actual} > {max}")
}

fn log_request_message(url: &Url, headers: &HeaderMap, body: Cow<'_, str>) {
    debug!(
        "Sending Solr request to {}\nHeaders: {:?}\nBody: {}",
        url, headers, body
    );
}

fn log_request_info(request: &Request, logging: LoggingPolicy) {
    if logging == LoggingPolicy::Off {
        return;
    }
    let url = request.url();
    let headers = request.headers();
    let body = request.body().map(|b| b.as_bytes().unwrap_or_default());
    let body = match body {
        None => {
            log_request_message(url, headers, String::from_utf8_lossy(NO_BODY));
            return;
        }
        Some(b) => b,
    };

    match logging {
        LoggingPolicy::Fast(max) => {
            let body: Cow<'_, str> = match body.len() > max {
                true => body_too_long(body.len(), max).into(),
                false => String::from_utf8_lossy(body),
            };
            log_request_message(url, headers, body);
        }
        LoggingPolicy::Pretty(max) => {
            let body: Cow<'_, str> = match body.len() > max {
                true => body_too_long(body.len(), max).into(),
                false => {
                    let body = serde_json::from_slice::<serde_json::Value>(body);
                    match body {
                        Ok(body) => serde_json::to_string_pretty(&body)
                            .unwrap_or(ERROR_BODY.to_string())
                            .into(),
                        Err(_) => ERROR_BODY.into(),
                    }
                }
            };
            log_request_message(url, headers, body)
        }
        _ => {}
    }
}
