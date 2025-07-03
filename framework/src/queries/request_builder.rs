use crate::error::{get_solr_error_from_error_response, Error};
use crate::models::context::SolrServerContext;
use crate::models::response::SolrResponse;
use crate::models::SolrResponseError;
use crate::Error::SolrConnectionError;
use log::debug;
use reqwest::header::HeaderMap;
use reqwest::{Body, Request, RequestBuilder, Response, Url};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
enum SolrRequestType {
    Get,
    Post,
}

pub trait SolrResponseType: Serialize + DeserializeOwned {
    fn check_for_error(&self, url: String) -> Result<(), Error>;
}

impl SolrResponseType for HashMap<String, serde_json::Value> {
    fn check_for_error(&self, url: String) -> Result<(), Error> {
        match self.get("error") {
            None => Ok(()),
            Some(err) => {
                let err: SolrResponseError = serde_json::from_value(err.clone())?;
                Err(get_solr_error_from_error_response(url, err))
            }
        }
    }
}
impl SolrResponseType for SolrResponse {
    fn check_for_error(&self, url: String) -> Result<(), Error> {
        match &self.error {
            None => Ok(()),
            Some(e) => Err(get_solr_error_from_error_response(url, e.clone())),
        }
    }
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

    pub async fn send_get<R: SolrResponseType>(self) -> Result<R, Error> {
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
        handle_solr_response::<R>(response).await
    }

    pub async fn send_post_with_json<T: Serialize + 'a + ?Sized, R: SolrResponseType>(
        self,
        json: &T,
    ) -> Result<R, Error> {
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
        handle_solr_response::<R>(response).await
    }

    pub async fn send_post_with_body<T: Into<Body>, R: SolrResponseType>(
        self,
        data: T,
    ) -> Result<R, Error> {
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
        handle_solr_response::<R>(response).await
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

async fn handle_solr_response<R: SolrResponseType>(response: Response) -> Result<R, Error> {
    let url = response.url().clone();
    let status_code = response.status();
    let body = response.text().await.unwrap_or_default();
    let solr_response = serde_json::from_str::<R>(&body);
    if let Ok(r) = solr_response {
        r.check_for_error(url.to_string())?;
        return Ok(r);
    }
    if status_code == 401 {
        return Err(Error::SolrAuthError {
            code: status_code.as_u16(),
            url: url.to_string(),
            msg: body,
        });
    }
    Err(SolrConnectionError {
        url: url.to_string(),
        code: status_code.as_u16(),
        msg: body,
    })
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
