use serde::{Deserialize, Serialize};
use solrstice;
use solrstice::queries::collection::{create_collection, delete_collection};
use solrstice::queries::config::{delete_config, upload_config};
use solrstice::SolrBasicAuth;
use solrstice::SolrRequestBuilder;
use solrstice::SolrSingleServerHost;
use solrstice::{AsyncSolrCloudClient, Error};
use solrstice::{SolrServerContext, SolrServerContextBuilder};
use std::path::Path;
use std::string::ToString;
use std::time::Duration;

pub struct BaseTestsBuildup {
    pub context: SolrServerContext,
    pub config_path: String,
    pub host: SolrSingleServerHost,
    pub auth: Option<SolrBasicAuth>,
}

impl BaseTestsBuildup {
    pub async fn new() -> Self {
        dotenv::from_filename("../test_setup/.env").ok();
        let username = std::env::var("SOLR_USERNAME").unwrap();
        let password = std::env::var("SOLR_PASSWORD").unwrap();
        let auth = match username.is_empty() {
            true => None,
            false => Some(SolrBasicAuth::new(
                username.as_str(),
                Some(password.as_str()),
            )),
        };
        let host = SolrSingleServerHost::new(std::env::var("SOLR_HOST").unwrap().as_str());
        let builder = SolrServerContextBuilder::new(host.clone());
        let context = if let Some(auth) = auth.clone() {
            builder.with_auth(auth).build()
        } else {
            builder.build()
        };
        wait_for_solr(&context, Duration::from_secs(30)).await;
        BaseTestsBuildup {
            context,
            config_path: "../test_setup/test_collection".to_string(),
            host,
            auth,
        }
    }
}

pub struct ErrrorTestsSetup {
    pub context: SolrServerContext,
    pub config_path: String,
    pub host: SolrSingleServerHost,
    pub auth: Option<SolrBasicAuth>,
}

impl ErrrorTestsSetup {
    pub async fn new() -> Self {
        dotenv::from_filename("../test_setup/.env").ok();
        let username = std::env::var("SOLR_USERNAME").unwrap();
        let password = std::env::var("SOLR_PASSWORD").unwrap();
        let auth = match username.is_empty() {
            true => None,
            false => Some(SolrBasicAuth::new(
                username.as_str(),
                Some(password.as_str()),
            )),
        };
        let error_nginx_hostname = std::env::var("ERROR_NGINX_HOST")
            .unwrap()
            .as_str()
            .to_string();
        let host = SolrSingleServerHost::new(&error_nginx_hostname);
        let builder = SolrServerContextBuilder::new(host.clone());
        let context = if let Some(auth) = auth.clone() {
            builder.with_auth(auth).build()
        } else {
            builder.build()
        };
        wait_for_error_nginx(&error_nginx_hostname, Duration::from_secs(30)).await;
        ErrrorTestsSetup {
            context,
            config_path: "../test_setup/test_collection".to_string(),
            host,
            auth,
        }
    }
}

pub struct FunctionalityTestsBuildup {
    pub context: SolrServerContext,
    pub async_client: AsyncSolrCloudClient,
    pub config_path: String,
    pub basename: String,
    pub config_name: String,
    pub collection_name: String,
}

impl FunctionalityTestsBuildup {
    pub async fn build_up(basename: &str) -> Result<Self, Error> {
        dotenv::from_filename("../test_setup/.env").ok();
        let host = std::env::var("SOLR_HOST").unwrap();
        let config_path = "../test_setup/test_collection".to_string();
        let username = std::env::var("SOLR_USERNAME").unwrap();
        let password = std::env::var("SOLR_PASSWORD").unwrap();
        let auth = match username.is_empty() {
            true => {
                return Err(Error::Unknown(
                    "Could not find solr username in tests .env file".to_string(),
                ))
            }
            false => SolrBasicAuth::new(username.as_str(), Some(password.as_str())),
        };

        let config_name = basename.to_owned() + "Config";
        let collection_name = basename.to_owned() + "Collection";

        let solr_request = SolrServerContextBuilder::new(SolrSingleServerHost::new(host.as_str()))
            .with_auth(auth)
            .build();
        wait_for_solr(&solr_request, Duration::from_secs(30)).await;

        let _ = delete_collection(&solr_request, &collection_name).await;
        let _ = delete_config(&solr_request, &config_name).await;

        upload_config(&solr_request, &config_name, Path::new(&config_path))
            .await
            .unwrap();
        create_collection(&solr_request, &collection_name, &config_name, 1, 1)
            .await
            .unwrap();

        Ok(Self {
            context: solr_request.clone(),
            async_client: AsyncSolrCloudClient::new(solr_request),
            basename: basename.to_string(),
            config_path,
            collection_name,
            config_name,
        })
    }

    pub async fn tear_down(&self) -> Result<(), Error> {
        delete_collection(&self.context, &self.collection_name)
            .await
            .unwrap();
        delete_config(&self.context, &self.config_name)
            .await
            .unwrap();
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug)]
pub struct City {
    pub id: String,
    pub city_name: String,
    pub population: Vec<Population>,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug)]
pub struct Population {
    pub id: String,
    pub age: usize,
    pub count: usize,
    pub interests: Vec<String>,
}

pub fn get_test_data() -> Vec<City> {
    let data: Vec<City> =
        serde_json::from_reader(std::fs::File::open("../test_setup/test_data.json").unwrap())
            .unwrap();
    data
}

pub async fn wait_for_solr(context: &SolrServerContext, max_time: Duration) {
    let end: std::time::Instant = std::time::Instant::now() + max_time;
    while std::time::Instant::now() < end {
        let response = SolrRequestBuilder::new(context, "/solr/admin/collections")
            .with_query_params(&[("action", "CLUSTERSTATUS")])
            .send_get()
            .await;
        if response.is_ok() {
            return;
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    panic!("Solr did not respond within {:?} seconds", max_time);
}

pub async fn wait_for_error_nginx(host: &str, max_time: Duration) {
    let end = std::time::Instant::now() + max_time;
    while std::time::Instant::now() < end {
        let response = reqwest::get(format!("{}/status", host)).await.unwrap();
        if response.status().is_success() {
            return;
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
